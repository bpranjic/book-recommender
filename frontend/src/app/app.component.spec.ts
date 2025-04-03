import { ComponentFixture, TestBed } from '@angular/core/testing';
import { AppComponent } from './app.component';
import { AuthService } from './services/auth.service';
import { MatDialog } from '@angular/material/dialog';
import { ActivatedRoute } from '@angular/router';
import { of } from 'rxjs';

describe('AppComponent', () => {
  let component: AppComponent;
  let fixture: ComponentFixture<AppComponent>;
  let authServiceMock: jasmine.SpyObj<AuthService>;
  let matDialogMock: jasmine.SpyObj<MatDialog>;
  let activatedRouteMock: Partial<ActivatedRoute>;

  beforeEach(async () => {
    authServiceMock = jasmine.createSpyObj('AuthService', ['getUser', 'setUser']);
    matDialogMock = jasmine.createSpyObj('MatDialog', ['open']);
    window.onbeforeunload = jasmine.createSpy();
    activatedRouteMock = {
      params: of({ id: 1 })
    };

    await TestBed.configureTestingModule({
      imports: [AppComponent],
      providers: [
        { provide: AuthService, useValue: authServiceMock },
        { provide: MatDialog, useValue: matDialogMock },
        { provide: ActivatedRoute, useValue: activatedRouteMock }, 
      ],
    }).compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(AppComponent);
    component = fixture.componentInstance;
  });

  it('should create the component', () => {
    expect(component).toBeTruthy();
  });

  it('should check if user is logged in', () => {
    authServiceMock.getUser.and.returnValue({ id: 1, name: 'John Doe' });
    expect(component.isUserLoggedIn()).toBeFalse();
  });

  it('should open login dialog', () => {
    component.openLoginDialog();
    expect(matDialogMock.open).toHaveBeenCalled();
  });

  it('should open register dialog', () => {
    component.openRegisterDialog();
    expect(matDialogMock.open).toHaveBeenCalled();
  });

});
