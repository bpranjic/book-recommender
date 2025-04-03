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
import { v4 as uuidv4 } from 'uuid';
import { MatIconModule } from '@angular/material/icon';

@Component({
  selector: 'app-add-book',
  imports: [MatDialogContent, MatFormFieldModule, MatInputModule, MatButtonModule, MatIconModule, FormsModule],
  templateUrl: './add-book.component.html',
  styleUrl: './add-book.component.css'
})
export class AddBookComponent {
  readonly dialogRef = inject(MatDialogRef<AddBookComponent>);
  api = environment.api;
  local = "http://localhost:8000/api";
  private _snackbar = inject(MatSnackBar);

  onAdd(title: string, author: string, genre: string, cover: string) {
    let id = uuidv4();
    const myHeaders = new Headers();
    myHeaders.append("Content-Type", "application/json");
    myHeaders.append("Accept", "application/json");

    const payload = {
      id: id,
      title: title,
      author: author,
      genre: genre,
      cover: cover
    };

    fetch(`${this.api}/books`, {
      method: "POST",
      headers: myHeaders,
      body: JSON.stringify(payload),
      redirect: "follow"
    })
      .then((response) => response.text())
      .then((text) => {
        location.reload();
        this._snackbar.open("Book has been added successfully.", "OK", {
          duration: 3000
        })
      })
      .catch(error => {
        console.error(error);
        this._snackbar.open("There was an error when adding a book.", "OK", {
          duration: 3000
        })
      })
      this.dialogRef.close();
  }

  onNoClick(): void {
    this.dialogRef.close();
  }
}
