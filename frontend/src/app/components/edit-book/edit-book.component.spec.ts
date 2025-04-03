import { ComponentFixture, TestBed } from '@angular/core/testing';
import { EditBookComponent } from './edit-book.component';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { AuthService } from '../../services/auth.service';

describe('EditBookComponent', () => {
  let component: EditBookComponent;
  let fixture: ComponentFixture<EditBookComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [EditBookComponent],
      providers: [
        { provide: MatDialogRef, useValue: { close: jasmine.createSpy('close') } },
        { provide: MAT_DIALOG_DATA, useValue: {} },
        { provide: AuthService, useValue: { setUser: jasmine.createSpy('setUser') } }
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(EditBookComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
