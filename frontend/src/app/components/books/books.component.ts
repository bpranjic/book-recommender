import { Component, computed, inject } from '@angular/core';
import { AuthService } from '../../services/auth.service';
import { MatTabsModule } from '@angular/material/tabs';
import { environment } from '../../../environments/environment';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatDialog } from '@angular/material/dialog';
import { AddBookComponent } from '../add-book/add-book.component';
import { MatSnackBar } from '@angular/material/snack-bar';
import { EditBookComponent } from '../edit-book/edit-book.component';

interface Book {
  id: string;
  title: string;
  author: string;
  genre: string;
  cover: string;
}

@Component({
  selector: 'app-books',
  imports: [MatTabsModule, MatCardModule, MatButtonModule, MatIconModule],
  templateUrl: './books.component.html',
  styleUrl: './books.component.css'
})
export class BooksComponent {
  api = environment.api;
  local = "http://localhost:8000/api";
  allBooks: Book[] = [];
  myBooks: Book[] = [];
  recommendations: Book[] = [];
  private authService = inject(AuthService);
  private _snackbar = inject(MatSnackBar);
  readonly addBookDialog = inject(MatDialog);
  readonly editBookDialog = inject(MatDialog);
  currentUser = computed(() => this.authService.getUser());

  constructor() { }

  ngOnInit(): void {
    this.getAllBooks();
    this.getAllBooksPerUser();
    this.getRecommendations();
  }

  addBookToUser(id: string): void {
    const user = this.authService.getUser();
    if (!user) {
      return;
    }
    const headers = new Headers();
    headers.append("Content-Type", "application/json");
    headers.append("Accept", "application/json");

    fetch(`${this.api}/users/${user.id}/books`, {
      method: "POST",
      headers: headers,
      body: id,
      redirect: "follow"
    })
      .then((response) => response.json())
      .then((json) => {
        if (json) {
          location.reload()
          this._snackbar.open("Successfully added book to user.", "OK", {
            duration: 3000
          });
        } else {
          this._snackbar.open("Could not add book to user.", "OK", {
            duration: 3000
          });
        }
      })
      .catch((error) => console.error(`There was an error fetching books: ${error}`));
  }

  deleteBook(book: Book): void {
    const headers = new Headers();
    headers.append("Content-Type", "application/json");
    headers.append("Accept", "application/json");

    fetch(`${this.api}/books/${book.id}`, {
      method: "DELETE",
      headers: headers
    })
      .then((response) => response.json())
      .then((json) => {
        if (json) {
          location.reload()
          this._snackbar.open("Successfully deleted book from database.", "OK", {
            duration: 3000
          });
        } else {
          this._snackbar.open("Successfully deleted book from database.", "OK", {
            duration: 3000
          });
        }
      })
      .catch((error) => console.error(`There was an error deleting the book: ${error}`));
  }

  editBook(book: Book): void {
    this.editBookDialog.open(EditBookComponent, {data: book});
  }

  getAllBooks(): void {
    const headers = new Headers();
    headers.append("Content-Type", "application/json");
    headers.append("Accept", "application/json");

    fetch(`${this.api}/books`, {
      method: "GET",
      headers: headers
    })
      .then((response) => response.json())
      .then((json) => {
        this.allBooks = json;
      })
      .catch((error) => console.error(`There was an error fetching books: ${error}`));
  }

  getAllBooksPerUser(): void {
    const headers = new Headers();
    headers.append("Content-Type", "application/json");
    headers.append("Accept", "application/json");

    const user = this.authService.getUser();
    if (user) {
      fetch(`${this.api}/users/${user.id}/books`, {
        method: "GET",
        headers: headers
      })
        .then((response) => response.json())
        .then((json) => {
          this.myBooks = json;
        })
        .catch((error) => console.error(`There was an error fetching books: ${error}`));
    }
  }

  getRecommendations(): void {
    const headers = new Headers();
    headers.append("Content-Type", "application/json");
    headers.append("Accept", "application/json");

    const user = this.authService.getUser();
    if (user) {
      fetch(`${this.api}/users/${user.id}/recommendations`, {
        method: "GET",
        headers: headers
      })
        .then((response) => response.json())
        .then((json) => {
          this.recommendations = json;
        })
        .catch((error) => console.error(`There was an error fetching books: ${error}`));
    }
  }

  openAddBookDialog(): void {
    if (this.currentUser() !== null) {
      this.addBookDialog.open(AddBookComponent);
    } else {
      this._snackbar.open("You need to be logged in to add books.", "OK", {
        duration: 3000
      })
    }
  }

  removeBook(id: string): void {
    const user = this.authService.getUser();
    if (!user) {
      return;
    }
    const headers = new Headers();
    headers.append("Content-Type", "application/json");
    headers.append("Accept", "application/json");

    fetch(`${this.api}/users/${user.id}/books/`, {
      method: "DELETE",
      headers: headers,
      body: id,
      redirect: "follow"
    })
      .then((response) => response.json())
      .then((json) => {
        if (json) {
          location.reload()
          this._snackbar.open("Successfully removed book from user.", "OK", {
            duration: 3000
          });
        } else {
          this._snackbar.open("Could not remove book from user.", "OK", {
            duration: 3000
          });
        }
      })
      .catch((error) => console.error(`There was an error removing the book: ${error}`));
  }
}
