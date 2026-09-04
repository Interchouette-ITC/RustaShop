<script setup>
import { computed, onMounted, ref } from 'vue';

const step = ref(0);
const status = ref(null);
const error = ref('');
const wipeConfirmed = ref(false);
const result = ref(null);
const busy = ref(false);

const wipeRequired = computed(() => status.value?.wipe_required === true);

onMounted(async () => {
  try {
    const res = await fetch('/install/api/status');
    status.value = await res.json();
  } catch (err) {
    error.value = String(err);
  }
});

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
</script>

<template>
  <main class="wrap">
    <Transition name="fade" mode="out-in">
      <section v-if="step === 0" key="welcome" class="card">
        <h1>rustashop install</h1>
        <p>Creates an opaque admin API path and bearer token in <code>.env</code>.</p>
        <p v-if="status">Wipe required: <strong>{{ wipeRequired ? 'yes' : 'no' }}</strong></p>
        <button type="button" @click="step = 1">Continue</button>
      </section>

      <section v-else-if="step === 1" key="confirm" class="card">
        <h1>Write secrets</h1>
        <label v-if="wipeRequired" class="check">
          <input v-model="wipeConfirmed" type="checkbox" />
          I understand this will wipe my shop files and database.
        </label>
        <p v-if="!wipeRequired" class="muted">First-time install: no wipe confirmation needed.</p>
        <p v-if="error" class="err">{{ error }}</p>
        <button type="button" :disabled="busy" @click="complete">
          {{ busy ? 'Working…' : 'Finish install' }}
        </button>
      </section>

      <section v-else key="done" class="card">
        <h1>Done</h1>
        <p>Save these values:</p>
        <pre>{{ result?.admin_prefix }}</pre>
        <pre>{{ result?.admin_token }}</pre>
        <p class="next">{{ result?.next_step }}</p>
        <p class="muted">Only supported disable: <code>mv install install.off</code></p>
      </section>
    </Transition>
  </main>
</template>

<style>
:root {
  color-scheme: light;
  font-family: "Segoe UI", system-ui, sans-serif;
  --bg: #0f1419;
  --card: #1a222c;
  --fg: #e8eef5;
  --accent: #3b82f6;
  --err: #f87171;
}
body {
  margin: 0;
  background: radial-gradient(1200px 600px at 20% 0%, #1e3a5f, var(--bg));
  color: var(--fg);
  min-height: 100vh;
}
.wrap {
  max-width: 32rem;
  margin: 0 auto;
  padding: 3rem 1.25rem;
}
.card {
  background: var(--card);
  border-radius: 0.75rem;
  padding: 1.5rem;
  box-shadow: 0 12px 40px rgb(0 0 0 / 35%);
}
h1 {
  margin-top: 0;
  font-size: 1.5rem;
}
button {
  margin-top: 1rem;
  background: var(--accent);
  color: white;
  border: 0;
  border-radius: 0.4rem;
  padding: 0.65rem 1rem;
  cursor: pointer;
  font-weight: 600;
}
button:disabled {
  opacity: 0.6;
  cursor: wait;
}
.check {
  display: flex;
  gap: 0.5rem;
  align-items: flex-start;
  margin: 1rem 0;
}
.err {
  color: var(--err);
}
.muted {
  opacity: 0.75;
  font-size: 0.9rem;
}
.next {
  font-weight: 600;
}
pre {
  background: #0b0f14;
  padding: 0.5rem 0.75rem;
  border-radius: 0.35rem;
  overflow-x: auto;
}
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.25s ease, transform 0.25s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateY(0.5rem);
}
</style>
