/** Production build defaults (same-origin `/api` behind the admin reverse proxy). */
export const environment = {
  production: true,
  apiBaseUrl: '/api',
  /** Must match the install `RUSTASHOP_ADMIN_API_PREFIX`. */
  adminApiPrefix: 'admin',
  /** Deploy behind `/{segment}/`; set to `/{segment}/` to match install. */
  adminPublicPath: '/',
};
