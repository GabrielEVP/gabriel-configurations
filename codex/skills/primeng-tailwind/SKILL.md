---
name: primeng-tailwind
description: PrimeNG components integration with Tailwind CSS v4. Component styling, theming, responsive layouts, form components, data tables, dialogs.
---

# PrimeNG + Tailwind CSS v4

## Setup

```typescript
// app.config.ts
import { providePrimeNG } from 'primeng/config';
import Aura from '@primeng/themes/aura';

export const appConfig: ApplicationConfig = {
  providers: [
    providePrimeNG({
      theme: {
        preset: Aura,
        options: {
          darkModeSelector: '.dark',
          cssLayer: {
            name: 'primeng',
            order: 'tailwind-base, primeng, tailwind-utilities'
          }
        }
      }
    }),
    provideAnimationsAsync()
  ]
};
```

---

## Data Table

```html
<p-table
  [value]="orders()"
  [paginator]="true"
  [rows]="20"
  [rowsPerPageOptions]="[10, 20, 50]"
  [loading]="isLoading()"
  [globalFilterFields]="['id', 'customerName', 'status']"
  dataKey="id"
  styleClass="w-full"
>
  <ng-template pTemplate="caption">
    <div class="flex items-center justify-between">
      <h2 class="text-xl font-semibold text-gray-900">Orders</h2>
      <div class="flex gap-3">
        <p-iconField>
          <p-inputIcon styleClass="pi pi-search" />
          <input
            pInputText
            type="text"
            placeholder="Search orders..."
            class="w-64"
            (input)="table.filterGlobal($event.target.value, 'contains')"
          />
        </p-iconField>
        <p-button
          label="New Order"
          icon="pi pi-plus"
          (onClick)="navigateToNew()"
          styleClass="bg-gray-900 border-gray-900 hover:bg-gray-700"
        />
      </div>
    </div>
  </ng-template>

  <ng-template pTemplate="header">
    <tr>
      <th pSortableColumn="id" class="w-32">
        <div class="flex items-center gap-2">Order ID <p-sortIcon field="id" /></div>
      </th>
      <th pSortableColumn="customerName">
        <div class="flex items-center gap-2">Customer <p-sortIcon field="customerName" /></div>
      </th>
      <th pSortableColumn="status">Status</th>
      <th pSortableColumn="totalAmount" class="text-right">Total</th>
      <th pSortableColumn="createdAt">Date</th>
      <th class="w-24">Actions</th>
    </tr>
  </ng-template>

  <ng-template pTemplate="body" let-order>
    <tr class="hover:bg-gray-50 transition-colors">
      <td class="font-mono text-sm text-gray-500">{{ order.id | slice:0:8 }}...</td>
      <td class="font-medium">{{ order.customerName }}</td>
      <td><app-order-status-badge [status]="order.status" /></td>
      <td class="text-right font-medium">{{ order.totalAmount | currency:'USD' }}</td>
      <td class="text-gray-500 text-sm">{{ order.createdAt | date:'MMM d, y' }}</td>
      <td>
        <div class="flex gap-1">
          <p-button
            icon="pi pi-eye"
            [text]="true"
            [rounded]="true"
            size="small"
            (onClick)="view(order.id)"
          />
          <p-button
            icon="pi pi-times"
            [text]="true"
            [rounded]="true"
            severity="danger"
            size="small"
            (onClick)="confirmCancel(order)"
            [disabled]="order.status !== 'PENDING'"
          />
        </div>
      </td>
    </tr>
  </ng-template>

  <ng-template pTemplate="emptymessage">
    <tr>
      <td colspan="6" class="text-center py-12 text-gray-400">No orders found</td>
    </tr>
  </ng-template>
</p-table>
```

---

## Form Components

```html
<form [formGroup]="form" (ngSubmit)="submit()" class="space-y-6">

  <!-- Text Input -->
  <div class="flex flex-col gap-1.5">
    <label for="name" class="text-sm font-medium text-gray-700">
      Customer Name <span class="text-red-500">*</span>
    </label>
    <input
      id="name"
      pInputText
      formControlName="name"
      placeholder="Enter customer name"
      [class.ng-dirty]="nameControl.dirty"
    />
    @if (nameControl.dirty && nameControl.errors?.['required']) {
      <small class="text-red-600">Name is required</small>
    }
    @if (nameControl.dirty && nameControl.errors?.['minlength']) {
      <small class="text-red-600">Name must be at least 2 characters</small>
    }
  </div>

  <!-- Dropdown -->
  <div class="flex flex-col gap-1.5">
    <label class="text-sm font-medium text-gray-700">Category</label>
    <p-select
      formControlName="categoryId"
      [options]="categories()"
      optionLabel="name"
      optionValue="id"
      placeholder="Select a category"
      styleClass="w-full"
    />
  </div>

  <!-- Multi-select -->
  <div class="flex flex-col gap-1.5">
    <label class="text-sm font-medium text-gray-700">Roles</label>
    <p-multiselect
      formControlName="roles"
      [options]="availableRoles"
      optionLabel="label"
      optionValue="value"
      placeholder="Select roles"
      styleClass="w-full"
    />
  </div>

  <!-- Submit -->
  <div class="flex justify-end gap-3 pt-4 border-t border-gray-200">
    <p-button
      label="Cancel"
      [outlined]="true"
      severity="secondary"
      type="button"
      (onClick)="cancel()"
    />
    <p-button
      label="Save Customer"
      type="submit"
      [loading]="isSubmitting()"
      [disabled]="form.invalid"
      styleClass="bg-gray-900 border-gray-900 hover:bg-gray-700"
    />
  </div>
</form>
```

---

## Dialog / Modal

```typescript
export class CustomerListComponent {
  private readonly dialogService = inject(DialogService);
  private readonly confirmationService = inject(ConfirmationService);

  openCreateDialog(): void {
    const ref = this.dialogService.open(CustomerFormComponent, {
      header: 'New Customer',
      width: '600px',
      contentStyle: { padding: '1.5rem' },
      modal: true,
      closable: true
    });

    ref.onClose.subscribe((created: CustomerResponse | undefined) => {
      if (created) {
        this.loadCustomers();
      }
    });
  }

  confirmDelete(customer: CustomerResponse): void {
    this.confirmationService.confirm({
      message: `Are you sure you want to delete "${customer.name}"?`,
      header: 'Confirm Delete',
      icon: 'pi pi-exclamation-triangle',
      acceptButtonStyleClass: 'p-button-danger',
      accept: () => this.deleteCustomer(customer.id)
    });
  }
}
```

```html
<!-- In template -->
<p-confirmDialog />
<p-dynamicDialog />
```

---

## Toast Notifications

```typescript
export class OrderFormComponent {
  private readonly messageService = inject(MessageService);

  private showSuccess(message: string): void {
    this.messageService.add({
      severity: 'success',
      summary: 'Success',
      detail: message,
      life: 3000
    });
  }

  private showError(message: string): void {
    this.messageService.add({
      severity: 'error',
      summary: 'Error',
      detail: message,
      life: 5000
    });
  }
}
```

```html
<!-- In app root component -->
<p-toast position="bottom-right" />
```

---

## Status Badge Component

```typescript
@Component({
  selector: 'app-order-status-badge',
  standalone: true,
  template: `
    <span [class]="badgeClass()">{{ label() }}</span>
  `
})
export class OrderStatusBadgeComponent {
  readonly status = input.required<OrderStatus>();

  readonly label = computed(() => {
    const labels: Record<OrderStatus, string> = {
      PENDING: 'Pending',
      CONFIRMED: 'Confirmed',
      SHIPPED: 'Shipped',
      DELIVERED: 'Delivered',
      CANCELLED: 'Cancelled'
    };
    return labels[this.status()];
  });

  readonly badgeClass = computed(() => {
    const base = 'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium';
    const variants: Record<OrderStatus, string> = {
      PENDING:   `${base} bg-yellow-100 text-yellow-800`,
      CONFIRMED: `${base} bg-blue-100 text-blue-800`,
      SHIPPED:   `${base} bg-purple-100 text-purple-800`,
      DELIVERED: `${base} bg-green-100 text-green-800`,
      CANCELLED: `${base} bg-red-100 text-red-800`
    };
    return variants[this.status()];
  });
}
```

---

## Responsive Layout Patterns

```html
<!-- Responsive grid -->
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
  @for (product of products(); track product.id) {
    <app-product-card [product]="product" />
  }
</div>

<!-- Sidebar layout -->
<div class="flex h-screen overflow-hidden">
  <aside class="w-64 bg-gray-900 text-white flex-shrink-0 hidden lg:block">
    <app-sidebar />
  </aside>
  <main class="flex-1 overflow-auto bg-gray-50">
    <router-outlet />
  </main>
</div>

<!-- Page header pattern -->
<div class="px-6 py-4 border-b border-gray-200 bg-white">
  <div class="flex items-center justify-between max-w-7xl mx-auto">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Orders</h1>
      <p class="text-sm text-gray-500 mt-0.5">Manage and track all orders</p>
    </div>
    <div class="flex gap-3">
      <ng-content select="[actions]" />
    </div>
  </div>
</div>
```
