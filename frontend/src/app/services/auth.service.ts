import { Injectable, signal, inject } from '@angular/core';
import { PLATFORM_ID } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';

interface User {
  id: number;
  name: string;
}

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private platformId = inject(PLATFORM_ID);
  private userSignal = signal<User | null>(this.loadUserFromSession());

  private loadUserFromSession(): User | null {
    if (isPlatformBrowser(this.platformId)) { 
      const userJson = sessionStorage.getItem('currentUser');
      return userJson ? JSON.parse(userJson) : null;
    }
    return null;
  }

  setUser(user: User | null) {
    this.userSignal.set(user);
    if (isPlatformBrowser(this.platformId)) {
      if (user) {
        sessionStorage.setItem('currentUser', JSON.stringify(user));
      } else {
        sessionStorage.removeItem('currentUser');
      }
    }
  }

  getUser() {
    return this.userSignal();
  }
}
