import { ComponentFixture, TestBed } from '@angular/core/testing';
import { AddBookComponent } from './add-book.component';
import { MatDialogRef } from '@angular/material/dialog';
import { AuthService } from '../../services/auth.service';

describe('AddBookComponent', () => {
  let component: AddBookComponent;
  let fixture: ComponentFixture<AddBookComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AddBookComponent],
      providers: [
        { provide: MatDialogRef, useValue: { close: jasmine.createSpy('close') } },
        { provide: AuthService, useValue: { setUser: jasmine.createSpy('setUser') } }
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(AddBookComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
