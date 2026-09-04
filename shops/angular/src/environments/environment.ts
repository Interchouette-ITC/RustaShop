/** Development defaults (replaced in production builds). */
export const environment = {
  production: false,
  /** Commerce API prefix (dev server proxies `/api` → Actix). */
  apiBaseUrl: '/api',
};
