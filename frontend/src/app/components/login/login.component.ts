import { Component, inject } from '@angular/core';
import { AuthService } from '../../services/auth.service';
import { MatDialogRef } from '@angular/material/dialog';
import { MatDialogContent } from '@angular/material/dialog';
import { FormsModule } from '@angular/forms';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatButtonModule } from '@angular/material/button';
import { environment } from '../../../environments/environment';
import { MatSnackBar } from '@angular/material/snack-bar';

@Component({
  selector: 'app-login',
  imports: [MatDialogContent, MatFormFieldModule, MatButtonModule, FormsModule, MatInputModule],
  templateUrl: './login.component.html',
  styleUrl: './login.component.css'
})
export class LoginComponent {
  readonly dialogRef = inject(MatDialogRef<LoginComponent>);
  private authService = inject(AuthService);
  api = environment.api;
  private _snackbar = inject(MatSnackBar);

  onLogin(username: string, password: string) {
    const loginHeaders = new Headers();
    loginHeaders.append("Content-Type", "application/x-www-form-urlencoded");

    const urlencoded = new URLSearchParams();
    urlencoded.append("username", username);
    urlencoded.append("password", password);

    fetch(`${this.api}/login`, {
      method: "POST",
      headers: loginHeaders,
      body: urlencoded,
      redirect: "follow"
    })
    .then(response => response.json())
    .then(data => {
      if (data.id && data.username) {
        this.authService.setUser({ id: data.id, name: data.username });
        this._snackbar.open("Successfully logged in.", "OK", {
          duration: 3000
        })
      }
      this.dialogRef.close();
      location.reload();
    })
    .catch(error => {
      this.dialogRef.close();
      this._snackbar.open("Wrong username or password.", "OK", {
        duration: 3000
      })
    });
  }

  onNoClick(): void {
    this.dialogRef.close();
  }
}
