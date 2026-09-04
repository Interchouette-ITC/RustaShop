import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

/** Same override story as shops/angular `proxy.conf.js`. */
function apiProxyTarget() {
  const fromProxy = process.env.RUSTASHOP_API_PROXY?.trim();
  if (fromProxy) {
    return fromProxy.replace(/\/$/, '');
  }
  const bind = process.env.RUSTASHOP_BIND || '127.0.0.1:8080';
  if (bind.startsWith('http://') || bind.startsWith('https://')) {
    return bind.replace(/\/$/, '');
  }
  return `http://${bind}`;
}

const installDevPort = Number(process.env.RUSTASHOP_INSTALL_DEV_PORT || 5173);

export default defineConfig({
  plugins: [vue()],
  base: '/install/',
  build: {
    outDir: 'dist',
    emptyOutDir: true,
  },
  server: {
    port: installDevPort,
    proxy: {
      // Only for `npm run dev`. Production build is served by Actix (same origin).
      '/install/api': {
        target: apiProxyTarget(),
        changeOrigin: true,
      },
    },
  },
});
