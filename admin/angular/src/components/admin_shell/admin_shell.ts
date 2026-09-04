import { Component, inject, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RouterOutlet } from '@angular/router';

import { AdminTokenStore } from '@rustashop/admin-core';
import { template as adminShellTpl, styles as adminShellStyles } from '@generated/admin_shell.ng';

@Component({
  selector: 'rs-admin-shell',
  imports: [RouterOutlet, FormsModule],
  template: adminShellTpl,
  styles: adminShellStyles,
})
export class AdminShell {
  private readonly tokens = inject(AdminTokenStore);

  protected readonly tokenDraft = signal(this.tokens.token());

  protected saveToken(): void {
    this.tokens.setToken(this.tokenDraft());
  }

  protected clearToken(): void {
    this.tokenDraft.set('');
    this.tokens.clear();
  }
}
