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
  selector: 'app-register',
  imports: [MatDialogContent, MatFormFieldModule, MatButtonModule, FormsModule, MatInputModule],
  templateUrl: './register.component.html',
  styleUrl: './register.component.css'
})
export class RegisterComponent {
  readonly dialogRef = inject(MatDialogRef<RegisterComponent>);
  private authService = inject(AuthService);
  api = environment.api;
  private _snackbar = inject(MatSnackBar);

  onRegister(username: string, password: string) {
    const registerHeaders = new Headers();
    registerHeaders.append("Content-Type", "application/x-www-form-urlencoded");

    const urlencoded = new URLSearchParams();
    urlencoded.append("username", username);
    urlencoded.append("password", password);

    fetch(`${this.api}/register`, {
      method: "POST",
      headers: registerHeaders,
      body: urlencoded,
      redirect: "follow"
    })
    .then(response => response.json())
    .then(data => {
      if (data.id && data.username) {
        const headers = new Headers();
        headers.append("Content-Type", "application/json");
        headers.append("Accept", "application/json");
        const payload = {id: Number(data.id), name: data.username};

        fetch(`${this.api}/users`, {
          method: "POST",
          headers: headers,
          body: JSON.stringify(payload),
          redirect: "follow"
        })
          .then((resp) => resp.text())
          .then((_) => {
            this._snackbar.open("User Registered Successfully.", "OK", {
              duration: 3000
            });
          })
      }
      this.dialogRef.close();
    })
    .catch(error => {
      this.dialogRef.close();
      this._snackbar.handsetCssClass
      this._snackbar.open("User already exists.", "OK", {
        duration: 3000
      });
    });
  }

  onNoClick(): void {
    this.dialogRef.close();
  }
}
