/** Install funnel state and API calls (controller). */

import { computed, onMounted, ref } from 'vue';

export function useInstallFlow() {
  const step = ref(0);
  const status = ref(null);
  const error = ref('');
  const wipeConfirmed = ref(false);
  const result = ref(null);
  const busy = ref(false);

  const wipeRequired = computed(() => status.value?.wipe_required === true);

  async function loadStatus() {
    try {
      const res = await fetch('/install/api/status');
      status.value = await res.json();
    } catch (err) {
      error.value = String(err);
    }
  }

  function goConfirm() {
    step.value = 1;
  }

  async function complete() {
    error.value = '';
    if (wipeRequired.value && !wipeConfirmed.value) {
      error.value = 'Confirm the wipe checkbox to continue.';
      return;
    }
    busy.value = true;
    try {
      const res = await fetch('/install/api/complete', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ wipe_confirmed: wipeConfirmed.value }),
      });
      const body = await res.json();
      if (!res.ok) {
        error.value = body.message || body.error || res.statusText;
        return;
      }
      result.value = body;
      step.value = 2;
    } catch (err) {
      error.value = String(err);
    } finally {
      busy.value = false;
    }
  }

  onMounted(() => {
    void loadStatus();
  });

  return {
    step,
    status,
    error,
    wipeConfirmed,
    result,
    busy,
    wipeRequired,
    goConfirm,
    complete,
  };
}
