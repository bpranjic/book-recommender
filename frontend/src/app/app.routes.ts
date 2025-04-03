import { Routes } from '@angular/router';
import { LoginComponent } from './components/login/login.component';
import { RegisterComponent } from './components/register/register.component';
import { HomeComponent } from './components/home/home.component';
import { BooksComponent } from './components/books/books.component';
import { UsersComponent } from './components/users/users.component';

export const routes: Routes = [
    { path: "login", component: LoginComponent },
    { path: "register", component: RegisterComponent },
    { path: "home", component: HomeComponent },
    { path: "books", component: BooksComponent },
    { path: "users", component: UsersComponent },
    { path: "", component: HomeComponent }
];
