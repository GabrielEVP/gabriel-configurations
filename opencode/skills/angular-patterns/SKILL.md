---
name: angular-patterns
description: Angular 21 patterns — Signals, Standalone Components, new control flow syntax (@if/@for/@defer), inject() function, OnPush, lazy loading, strict TypeScript.
license: MIT
compatibility: opencode
---

# Angular 21 Patterns

## Standalone Components (Default)

Every component is standalone. No NgModule ever.

```typescript
@Component({
  selector: 'app-customer-list',
  standalone: true,
  imports: [TableModule, ButtonModule, InputTextModule, RouterModule],
  templateUrl: './customer-list.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class CustomerListComponent {
  private readonly customerService = inject(CustomerService);
  private readonly router = inject(Router);

  readonly customers = toSignal(this.customerService.findAll(), { initialValue: [] });
  readonly searchTerm = signal('');
  readonly isLoading = signal(false);

  readonly filtered = computed(() =>
    this.customers().filter(c =>
      c.name.toLowerCase().includes(this.searchTerm().toLowerCase())
    )
  );
}
```

---

## Signals — Complete Reference

### signal() — Writable State
```typescript
readonly count = signal(0);
readonly name = signal('');
readonly items = signal<Item[]>([]);

// Update methods
this.count.set(5);                       // Replace value
this.count.update(n => n + 1);          // Transform value
this.items.update(list => [...list, newItem]);  // Immutable update
```

### computed() — Derived State
```typescript
readonly total = computed(() => this.items().reduce((sum, i) => sum + i.price, 0));
readonly hasItems = computed(() => this.items().length > 0);
readonly displayName = computed(() => this.name().trim() || 'Anonymous');
```

### toSignal() — Observable to Signal
```typescript
// Synchronous with initialValue
readonly products = toSignal(this.productService.findAll(), { initialValue: [] });

// With error handling
readonly order = toSignal(
  this.orderService.findById(this.orderId()),
  { initialValue: null }
);
```

### input() / output() — Component API
```typescript
export class ProductCardComponent {
  // Required input
  readonly product = input.required<ProductResponse>();

  // Optional input with default
  readonly compact = input(false);

  // Output event
  readonly selected = output<ProductResponse>();
  readonly quantityChanged = output<{ productId: string; quantity: number }>();

  select(): void {
    this.selected.emit(this.product());
  }
}
```

### resource() — Async Data Loading (Angular 19+)
```typescript
export class ProductDetailComponent {
  readonly id = input.required<string>();

  readonly product = resource({
    request: () => ({ id: this.id() }),
    loader: async ({ request }) => {
      const response = await fetch(`/api/v1/products/${request.id}`);
      return response.json() as Promise<ProductResponse>;
    }
  });

  // Access: this.product.value(), this.product.isLoading(), this.product.error()
}
```

### effect() — Side Effects
```typescript
constructor() {
  // Runs when searchTerm changes, tracks dependencies automatically
  effect(() => {
    const term = this.searchTerm();
    if (term.length >= 3) {
      this.performSearch(term);
    }
  });

  // Cleanup
  effect((onCleanup) => {
    const subscription = someObservable.subscribe();
    onCleanup(() => subscription.unsubscribe());
  });
}
```

---

## New Control Flow Syntax

### @if / @else if / @else
```html
@if (isLoading()) {
  <app-loading-skeleton rows="5" />
} @else if (error()) {
  <app-error-state [message]="error()!.message" (retry)="reload()" />
} @else if (customers().length === 0) {
  <app-empty-state
    title="No customers found"
    subtitle="Start by adding your first customer"
  />
} @else {
  <p-table [value]="customers()" />
}
```

### @for with track
```html
@for (customer of filtered(); track customer.id) {
  <app-customer-card
    [customer]="customer"
    (selected)="navigateTo(customer.id)"
  />
} @empty {
  <p class="text-gray-500 text-center py-8">No results match your search</p>
}
```

### @switch
```html
@switch (orderStatus()) {
  @case ('PENDING') {
    <span class="badge-warning">Pending Confirmation</span>
  }
  @case ('CONFIRMED') {
    <span class="badge-info">Confirmed</span>
  }
  @case ('SHIPPED') {
    <span class="badge-primary">Shipped</span>
  }
  @case ('DELIVERED') {
    <span class="badge-success">Delivered</span>
  }
  @case ('CANCELLED') {
    <span class="badge-error">Cancelled</span>
  }
}
```

### @defer — Lazy Loading in Template
```html
@defer (on viewport; prefetch on idle) {
  <app-analytics-chart [data]="chartData()" />
} @placeholder (minimum 100ms) {
  <div class="h-64 bg-gray-50 rounded-lg" />
} @loading (minimum 200ms; after 100ms) {
  <app-chart-skeleton />
} @error {
  <app-error-state message="Failed to load analytics" />
}
```

**@defer trigger options:**
- `on idle` — when browser is idle
- `on viewport` — when element enters viewport
- `on interaction` — on click/focus
- `on hover` — on mouse hover
- `on timer(2s)` — after delay
- `when condition` — when boolean signal is true

---

## inject() — Dependency Injection in Components

```typescript
export class OrderFormComponent {
  // Services
  private readonly orderService = inject(OrderService);
  private readonly customerService = inject(CustomerService);
  private readonly router = inject(Router);
  private readonly route = inject(ActivatedRoute);
  private readonly fb = inject(FormBuilder);

  // Angular utilities
  private readonly destroyRef = inject(DestroyRef);
  private readonly changeDetector = inject(ChangeDetectorRef);
  private readonly injector = inject(Injector);

  // Tokens
  private readonly apiUrl = inject(API_URL_TOKEN);
}
```

---

## OnPush Change Detection

OnPush components only check for changes when:
1. An `@Input` reference changes
2. An event originates from the component
3. An Observable bound with `async` pipe emits
4. A Signal changes
5. `markForCheck()` is called manually

```typescript
// This is why signals + OnPush is so powerful:
// Only the specific component using the signal re-renders
@Component({
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `<span>{{ count() }}</span>`
})
export class CounterComponent {
  readonly count = input.required<number>();
}
```

---

## Lazy Loading

### Route-Level Lazy Loading
```typescript
export const routes: Routes = [
  {
    path: 'customers',
    loadChildren: () =>
      import('@features/customers/customers.routes').then(m => m.CUSTOMERS_ROUTES),
    canActivate: [authGuard]
  },
  {
    path: 'orders',
    loadComponent: () =>
      import('@features/orders/ui/order-list/order-list.component')
        .then(m => m.OrderListComponent)
  }
];
```

### Component-Level Lazy Loading
```typescript
// Programmatic lazy loading
const { HeavyComponent } = await import('./heavy/heavy.component');
```

---

## Reactive Forms Best Practices

```typescript
export class CustomerFormComponent {
  private readonly fb = inject(FormBuilder);

  readonly form = this.fb.group({
    name: ['', [Validators.required, Validators.minLength(2), Validators.maxLength(100)]],
    email: ['', [Validators.required, Validators.email]],
    phone: ['', [Validators.pattern(/^\+?[1-9]\d{1,14}$/)]],
    address: this.fb.group({
      street: ['', Validators.required],
      city: ['', Validators.required],
      country: ['', Validators.required]
    })
  });

  // Type-safe field access
  get nameControl() { return this.form.get('name')!; }
  get emailControl() { return this.form.get('email')!; }

  // Signal-based submission state
  readonly isSubmitting = signal(false);
  readonly submitError = signal<string | null>(null);
}
```

---

## HTTP with Interceptors

```typescript
// Functional interceptors (Angular 17+)
export const authInterceptor: HttpInterceptorFn = (req, next) => {
  const authService = inject(AuthService);
  const token = authService.accessToken();

  if (!token) return next(req);

  return next(req.clone({
    setHeaders: { Authorization: `Bearer ${token}` }
  }));
};

// app.config.ts
export const appConfig: ApplicationConfig = {
  providers: [
    provideHttpClient(withInterceptors([authInterceptor, errorInterceptor])),
    provideRouter(routes),
    provideAnimationsAsync()
  ]
};
```

---

## TypeScript Strict Patterns

```typescript
// Never use any — use proper types
type ApiStatus = 'idle' | 'loading' | 'success' | 'error';

interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  pageSize: number;
}

// Discriminated union for state management
type AsyncState<T> =
  | { status: 'idle' }
  | { status: 'loading' }
  | { status: 'success'; data: T }
  | { status: 'error'; error: Error };

// Use satisfies for object type checking
const config = {
  apiUrl: '/api/v1',
  timeout: 5000
} satisfies Record<string, string | number>;
```
