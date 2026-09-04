import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('vue', async () => {
  const actual = await vi.importActual('vue');
  return {
    ...actual,
    onMounted: (fn) => {
      fn();
    },
  };
});

import { useInstallFlow } from './useInstallFlow.js';

describe('useInstallFlow', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it('loads status on mount', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue({
        ok: true,
        json: async () => ({ available: true, wipe_required: false }),
      }),
    );

    const flow = useInstallFlow();
    await Promise.resolve();
    await Promise.resolve();

    expect(fetch).toHaveBeenCalledWith('/install/api/status');
    expect(flow.status.value).toEqual({ available: true, wipe_required: false });
    expect(flow.wipeRequired.value).toBe(false);
  });

  it('blocks complete when wipe is required and not confirmed', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue({
        ok: true,
        json: async () => ({ available: true, wipe_required: true }),
      }),
    );

    const flow = useInstallFlow();
    await Promise.resolve();
    await Promise.resolve();

    await flow.complete();
    expect(flow.error.value).toContain('wipe');
    expect(fetch).toHaveBeenCalledTimes(1);
  });

  it('posts complete and advances to success step', async () => {
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({ available: true, wipe_required: false }),
      })
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          admin_prefix: 'admin',
          admin_token: 'tok',
          env_path: '.env',
          next_step: 'mv install install.off',
        }),
      });
    vi.stubGlobal('fetch', fetchMock);

    const flow = useInstallFlow();
    await Promise.resolve();
    await Promise.resolve();

    flow.goConfirm();
    expect(flow.step.value).toBe(1);

    await flow.complete();
    expect(flow.step.value).toBe(2);
    expect(flow.result.value.admin_prefix).toBe('admin');
    expect(fetchMock).toHaveBeenCalledWith(
      '/install/api/complete',
      expect.objectContaining({ method: 'POST' }),
    );
  });
});
