import { HttpErrorResponse } from '@angular/common/http';

/** Maps HTTP / unknown errors to a short admin-facing message. */
export function formatApiError(err: unknown, fallback = 'Request failed.'): string {
  if (err instanceof HttpErrorResponse) {
    if (err.status === 0) {
      return 'API unreachable. Is the Commerce API running?';
    }
    if (err.status === 401) {
      return 'Unauthorized. Check the admin bearer token.';
    }
    if (err.status === 404) {
      return 'Not found.';
    }
    if (err.status === 422) {
      return 'Invalid status.';
    }
    return `Request failed (${err.status}).`;
  }
  if (err && typeof err === 'object' && 'message' in err) {
    const message = String((err as { message: unknown }).message).trim();
    if (message) {
      return message;
    }
  }
  return fallback;
}
