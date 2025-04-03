import { Component, inject, model } from '@angular/core';
import { AuthService } from '../../services/auth.service';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import { MatDialogContent } from '@angular/material/dialog';
import { FormsModule } from '@angular/forms';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatButtonModule } from '@angular/material/button';
import { environment } from '../../../environments/environment';
import { MatSnackBar } from '@angular/material/snack-bar';
import { v4 as uuidv4 } from 'uuid';
import { MatIconModule } from '@angular/material/icon';

interface Book {
  id: string;
  title: string;
  author: string;
  genre: string;
  cover: string;
}

@Component({
  selector: 'app-edit-book',
  imports: [MatDialogContent, MatFormFieldModule, MatInputModule, MatButtonModule, MatIconModule, FormsModule],
  templateUrl: './edit-book.component.html',
  styleUrl: './edit-book.component.css'
})
export class EditBookComponent {
  readonly dialogRef = inject(MatDialogRef<EditBookComponent>);
  readonly book = inject<Book>(MAT_DIALOG_DATA);
  api = environment.api;
  local = "http://localhost:8000/api";
  private _snackbar = inject(MatSnackBar);

  onEdit(): void {
    const myHeaders = new Headers();
    myHeaders.append("Content-Type", "application/json");
    myHeaders.append("Accept", "application/json");

    const requestOptions = {
      method: "PUT",
      headers: myHeaders,
      body: JSON.stringify(this.book),
      redirect: "follow"
    };

    fetch(`${this.api}/books/${this.book.id}`, {
      method: "PUT",
      headers: myHeaders,
      body: JSON.stringify(this.book),
      redirect: "follow"
    })
    .then((response) => response.json())
    .then((json) => {
      if (json) {
        location.reload()
        this._snackbar.open("Successfully modified book.", "OK", {
          duration: 3000
        });
      } else {
        this._snackbar.open("Could not modify book.", "OK", {
          duration: 3000
        });
      }
    })
    .catch((error) => console.error(`There was an error modifying the book: ${error}`));
  }
}
