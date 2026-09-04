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

  it('records loadStatus fetch failures', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockRejectedValue(new Error('status down')),
    );

    const flow = useInstallFlow();
    await Promise.resolve();
    await Promise.resolve();

    expect(flow.error.value).toContain('status down');
    expect(flow.status.value).toBeNull();
  });

  it('surfaces complete HTTP and network errors', async () => {
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({ available: true, wipe_required: false }),
      })
      .mockResolvedValueOnce({
        ok: false,
        statusText: 'Conflict',
        json: async () => ({ message: 'wipe required' }),
      })
      .mockRejectedValueOnce(new Error('network boom'));
    vi.stubGlobal('fetch', fetchMock);

    const flow = useInstallFlow();
    await Promise.resolve();
    await Promise.resolve();

    await flow.complete();
    expect(flow.error.value).toBe('wipe required');
    expect(flow.busy.value).toBe(false);

    await flow.complete();
    expect(flow.error.value).toContain('network boom');
    expect(flow.busy.value).toBe(false);
  });
});
