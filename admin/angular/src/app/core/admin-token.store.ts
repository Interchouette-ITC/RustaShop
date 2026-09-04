import { Injectable, computed, signal } from '@angular/core';

const STORAGE_KEY = 'rs.adminApiToken';

/** Session-scoped admin bearer (never committed; paste from env locally). */
@Injectable({ providedIn: 'root' })
export class AdminTokenStore {
  private readonly tokenSignal = signal(readStoredToken());

  readonly token = this.tokenSignal.asReadonly();
  readonly hasToken = computed(() => this.tokenSignal().length > 0);

  setToken(value: string): void {
    const trimmed = value.trim();
    this.tokenSignal.set(trimmed);
    if (trimmed) {
      sessionStorage.setItem(STORAGE_KEY, trimmed);
    } else {
      sessionStorage.removeItem(STORAGE_KEY);
    }
  }

  clear(): void {
    this.setToken('');
  }
}

function readStoredToken(): string {
  try {
    return sessionStorage.getItem(STORAGE_KEY)?.trim() ?? '';
  } catch {
    return '';
  }
}
