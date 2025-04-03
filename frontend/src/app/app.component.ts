import { Component, inject, effect } from '@angular/core';
import { RouterLink, RouterLinkActive, RouterOutlet } from '@angular/router';
import { environment } from '../environments/environment';
import { LoginComponent } from './components/login/login.component';
import { MatButtonModule } from '@angular/material/button';
import { MatDialog } from '@angular/material/dialog';
import { RegisterComponent } from './components/register/register.component';
import { AuthService } from './services/auth.service';

interface User {
  id: number;
  name: string;
}

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, RouterLink, RouterLinkActive, MatButtonModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  api = environment.api;
  private authService = inject(AuthService);
  currentUser?: User | null = null;
  readonly loginDialog = inject(MatDialog);
  readonly registerDialog = inject(MatDialog);

  constructor() {
    effect(() => {
      this.currentUser = this.authService.getUser();
    });
  }

  isUserLoggedIn(): boolean {
    return this.currentUser !== null;
  }

  openLoginDialog(): void {
    this.loginDialog.open(LoginComponent);
  }

  openRegisterDialog(): void {
    this.registerDialog.open(RegisterComponent);
  }

  logout(): void {
    this.authService.setUser(null);
    location.reload();
  }
}
