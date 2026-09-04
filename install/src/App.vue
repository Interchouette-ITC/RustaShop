<script setup>
import { useInstallFlow } from './composables/useInstallFlow.js';

const {
  step,
  status,
  error,
  wipeConfirmed,
  result,
  busy,
  wipeRequired,
  goConfirm,
  complete,
} = useInstallFlow();
</script>

<template>
  <main class="wrap">
    <Transition name="fade" mode="out-in">
      <section v-if="step === 0" key="welcome" class="card">
        <h1>rustashop install</h1>
        <p>Creates an opaque admin API path and bearer token in <code>.env</code>.</p>
        <p v-if="status">Wipe required: <strong>{{ wipeRequired ? 'yes' : 'no' }}</strong></p>
        <button type="button" @click="goConfirm">Continue</button>
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
