/** Development defaults (replaced in production builds). */
export const environment = {
  production: false,
  /** Commerce API prefix (dev server proxies `/api` → Actix). */
  apiBaseUrl: '/api',
  /**
   * Operator API URI segment (`/v1/{this}/…`).
   * Must match `RUSTASHOP_ADMIN_API_PREFIX` on the API (local default `admin`).
   */
  adminApiPrefix: 'admin',
};
