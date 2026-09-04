import { Pipe, PipeTransform } from '@angular/core';

/** Formats API money (`amount_minor` + ISO currency) for display. */
@Pipe({ name: 'rsMoney' })
export class MoneyPipe implements PipeTransform {
  transform(amountMinor: number | null | undefined, currency = 'EUR'): string {
    if (amountMinor == null || Number.isNaN(amountMinor)) {
      return '-';
    }
    try {
      return new Intl.NumberFormat(undefined, {
        style: 'currency',
        currency,
      }).format(amountMinor / 100);
    } catch {
      return `${(amountMinor / 100).toFixed(2)} ${currency}`;
    }
  }
}
