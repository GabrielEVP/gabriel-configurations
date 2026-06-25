---
name: primeng
description: Comprehensive PrimeNG v21 — components, theming, configuration, forms, tables, overlays, menus, and integration patterns with Angular 21.
license: MIT
compatibility: opencode
trigger: primeng, p-table, p-button, primefaces, primeicons, primeNG component, primeng mcp
---

# PrimeNG — The Most Complete Angular UI Component Library

## Overview

PrimeNG is a rich set of open-source native Angular UI components. This skill covers **all aspects** of PrimeNG including setup, theming, every component category, integration patterns, and the MCP server.

**Current version**: PrimeNG v21  
**Angular compatibility**: Angular 19+

> **Complementary skill**: `primeng-tailwind` covers PrimeNG + Tailwind CSS v4 specific integration patterns (responsive layouts, custom styling, etc.). Load both skills for full coverage.

---

## 1. Setup & Installation

### 1.1 Install Dependencies

```bash
npm install primeng @primeng/themes @primeng/icons
npm install primeicons  # optional — legacy icon set
```

### 1.2 Provide PrimeNG Config

```typescript
// app.config.ts
import { ApplicationConfig } from '@angular/core';
import { provideAnimationsAsync } from '@angular/platform-browser/animations-async';
import { providePrimeNG } from 'primeng/config';
import Aura from '@primeng/themes/aura';

export const appConfig: ApplicationConfig = {
  providers: [
    provideAnimationsAsync(),
    providePrimeNG({
      theme: {
        preset: Aura,
        options: {
          darkModeSelector: '.dark',     // or '.p-dark'
          cssLayer: {
            name: 'primeng',
            order: 'tailwind-base, primeng, tailwind-utilities'
          }
        }
      },
      ripple: true
    })
  ]
};
```

### 1.3 Global Styles (styles.scss)

```scss
@import 'primeicons/primeicons.css';
@layer tailwind-base, primeng, tailwind-utilities;
@import 'tailwindcss' layer(tailwind-base);
```

### 1.4 Import Components

Components are **standalone** and tree-shakeable from v19+:

```typescript
import { ButtonModule } from 'primeng/button';
import { TableModule } from 'primeng/table';
// etc.
```

---

## 2. Theming & Configuration

### 2.1 Presets

| Preset | Description |
|--------|-------------|
| `Aura` | Modern, minimal design (default) |
| `Lara` | Classic PrimeNG look |
| `Nora` | Bold, high-contrast |
| `Material` | Material Design inspired |

```typescript
import Material from '@primeng/themes/material';
```

### 2.2 Dark Mode

Toggle via CSS class:

```typescript
// Add/remove class on document root
document.documentElement.classList.toggle('p-dark');
```

### 2.3 Pass Through (PT) API

Directly style PrimeNG internal elements without overriding CSS:

```typescript
providePrimeNG({
  theme: { preset: Aura },
  pt: {
    button: {
      root: { class: 'bg-blue-500 hover:bg-blue-600 text-white' }
    }
  }
});
```

Per-instance override:

```html
<p-button label="Save" [pt]="{ root: { class: 'custom-btn' } }" />
```

### 2.4 Unstyled Mode

```typescript
providePrimeNG({ theme: { preset: Aura, options: { unstyled: true } } });
```

Then style everything via PT or Tailwind.

### 2.5 Icons

- **Default**: PrimeIcons (`pi pi-check`, `pi pi-user`, etc.)
- **Custom**: Use `providePrimeNG({ icons: { ... } })` to register custom icon sets.

---

## 3. Component Reference

### 3.1 Form Components

| Component | Selector | Use Case |
|-----------|----------|----------|
| AutoComplete | `<p-autocomplete>` | Real-time suggestions |
| CascadeSelect | `<p-cascadeselect>` | Nested option selection |
| Checkbox | `<p-checkbox>` | Boolean input |
| ColorPicker | `<p-colorpicker>` | Color selection |
| DatePicker | `<p-datepicker>` | Date/time selection |
| Editor | `<p-editor>` | Rich text (Quill-based) |
| FloatLabel | `<p-floatlabel>` | Floating label wrapper |
| IconField | `<p-iconfield>` | Input with embedded icon |
| IftaLabel | `<p-iftalabel>` | In-field top-aligned label |
| InputGroup | `<p-inputgroup>` | Group content with input |
| InputMask | `<p-inputmask>` | Formatted input (phone, date, etc.) |
| InputNumber | `<p-inputnumber>` | Numeric input |
| InputOtp | `<p-inputotp>` | One-time password input |
| InputText | `<input pInputText>` | Standard text input (directive) |
| KeyFilter | `<input pKeyFilter>` | Restrict keystrokes (directive) |
| Knob | `<p-knob>` | Dial input |
| Listbox | `<p-listbox>` | Single/multi select list |
| MultiSelect | `<p-multiselect>` | Multi-select dropdown |
| Password | `<p-password>` | Password with strength meter |
| RadioButton | `<p-radiobutton>` | Radio selection |
| Rating | `<p-rating>` | Star rating |
| Select | `<p-select>` | Single select dropdown |
| SelectButton | `<p-selectbutton>` | Button-based selection |
| Slider | `<p-slider>` | Range slider |
| Textarea | `<textarea pTextarea>` | Multi-line text (directive) |
| ToggleButton | `<p-togglebutton>` | Boolean button |
| ToggleSwitch | `<p-toggleswitch>` | Toggle switch |
| Fluid | `<p-fluid>` | Full-width child components |

### 3.2 Data Components

| Component | Selector | Use Case |
|-----------|----------|----------|
| DataView | `<p-dataview>` | Grid/list layout with pagination |
| OrderList | `<p-orderlist>` | Reorderable list |
| Paginator | `<p-paginator>` | Pagination control |
| PickList | `<p-picklist>` | Transfer between lists |
| Table | `<p-table>` | **Full-featured data table** — sorting, filtering, pagination, editing, row expansion, column groups |
| Tree | `<p-tree>` | Hierarchical data display |
| TreeSelect | `<p-treeselect>` | Tree-based dropdown |
| TreeTable | `<p-treetable>` | Hierarchical tabular data |
| VirtualScroller | `<p-virtualscroller>` | Virtual scroll for huge datasets |

### 3.3 Overlay Components

| Component | Selector | Use Case |
|-----------|----------|----------|
| ConfirmDialog | `<p-confirmdialog>` | Confirmation modal (service-based) |
| ConfirmPopup | `<p-confirmpopup>` | Confirmation popup near target |
| Dialog | `<p-dialog>` | Modal dialog |
| Drawer | `<p-drawer>` | Slide-in panel |
| DynamicDialog | `<p-dynamicdialog>` | Programmatic dialog (DialogService) |
| Overlay | `<p-overlay>` | Generic overlay panel |
| Popover | `<p-popover>` | Popover near target |
| Tooltip | `<span pTooltip>` | Tooltip directive |

### 3.4 Menu Components

| Component | Selector | Use Case |
|-----------|----------|----------|
| Breadcrumb | `<p-breadcrumb>` | Page hierarchy |
| ContextMenu | `<p-contextmenu>` | Right-click menu |
| Dock | `<p-dock>` | macOS-style dock |
| MegaMenu | `<p-megamenu>` | Multi-column menu |
| Menu | `<p-menu>` | Simple popup/static menu |
| Menubar | `<p-menubar>` | Horizontal menu bar |
| PanelMenu | `<p-panelmenu>` | Accordion + tree hybrid |
| SpeedDial | `<p-speeddial>` | Floating action button |
| SplitButton | `<p-splitbutton>` | Button with dropdown |
| TieredMenu | `<p-tieredmenu>` | Nested overlay menu |

### 3.5 Panel Components

| Component | Selector | Use Case |
|-----------|----------|----------|
| Accordion | `<p-accordion>` | Collapsible sections |
| Card | `<p-card>` | Content card container |
| Deferred | `<div pDeferred>` | Deferred content loading (directive) |
| Divider | `<p-divider>` | Visual separator |
| Fieldset | `<p-fieldset>` | Grouped content with toggle |
| Panel | `<p-panel>` | Container with header |
| ScrollPanel | `<p-scrollpanel>` | Custom scrollbar |
| Splitter | `<p-splitter>` | Resizable panels |
| Stepper | `<p-stepper>` | Multi-step wizard |
| Tabs | `<p-tabs>` | Tabbed interface |
| Toolbar | `<p-toolbar>` | Button/action toolbar |

### 3.6 Media Components

| Component | Selector | Use Case |
|-----------|----------|----------|
| Avatar | `<p-avatar>` | User avatar (image/icon/initials) |
| Carousel | `<p-carousel>` | Image/content carousel |
| Galleria | `<p-galleria>` | Full-featured gallery |
| Image | `<p-image>` | Image with preview |
| ImageCompare | `<p-imagecompare>` | Side-by-side image comparison |

### 3.7 Display Components

| Component | Selector | Use Case |
|-----------|----------|----------|
| Badge | `<p-badge>` | Status indicator badge |
| BlockUI | `<p-blockui>` | Block content overlay |
| Chart | `<p-chart>` | Charts.js based charts |
| Chip | `<p-chip>` | Tag/chip entity |
| Inplace | `<p-inplace>` | Inline editing |
| MeterGroup | `<p-metergroup>` | Scalar measurement display |
| ProgressBar | `<p-progressbar>` | Determinate progress |
| ProgressSpinner | `<p-progressspinner>` | Indeterminate spinner |
| Skeleton | `<p-skeleton>` | Loading placeholder |
| Tag | `<p-tag>` | Category/label tag |
| Timeline | `<p-timeline>` | Vertical event timeline |

### 3.8 Directives & Utilities

| Component | Selector | Use Case |
|-----------|----------|----------|
| AnimateOnScroll | `pAnimateOnScroll` | Scroll-triggered animations |
| AutoFocus | `pAutoFocus` | Auto-focus on load |
| Draggable | `pDraggable` | Drag behavior |
| Droppable | `pDroppable` | Drop target |
| FocusTrap | `<p-focustrap>` | Trap focus within element |
| Ripple | `pRipple` | Material ripple effect |
| ScrollTop | `<p-scrolltop>` | Back-to-top button |
| StyleClass | `pStyleClass` | Declarative CSS class management |

### 3.9 Messages

| Component | Selector | Use Case |
|-----------|----------|----------|
| Message | `<p-message>` | Inline message |
| Toast | `<p-toast>` | Global notification overlay |

---

## 4. Common Patterns

### 4.1 Form Validation with Reactive Forms

```typescript
@Component({
  standalone: true,
  imports: [ReactiveFormsModule, InputTextModule, SelectModule, ButtonModule],
  template: `
    <form [formGroup]="form" (ngSubmit)="onSubmit()" class="flex flex-col gap-4">
      <div class="flex flex-col gap-1">
        <label for="name">Name</label>
        <input id="name" pInputText formControlName="name" />
        @if (form.get('name')?.invalid && form.get('name')?.touched) {
          <small class="text-red-600">Name is required (min 3 chars)</small>
        }
      </div>

      <div class="flex flex-col gap-1">
        <label>Category</label>
        <p-select
          formControlName="category"
          [options]="categories()"
          optionLabel="name"
          optionValue="id"
          placeholder="Select..."
          styleClass="w-full"
        />
      </div>

      <p-button
        label="Submit"
        type="submit"
        [disabled]="form.invalid"
        [loading]="isSubmitting()"
      />
    </form>
  `
})
export class MyFormComponent {
  private fb = inject(FormBuilder);
  isSubmitting = signal(false);
  categories = signal<Category[]>([]);

  form = this.fb.nonNullable.group({
    name: ['', [Validators.required, Validators.minLength(3)]],
    category: ['' as const, Validators.required]
  });

  onSubmit() { /* ... */ }
}
```

### 4.2 Data Table with Signals

```typescript
@Component({
  standalone: true,
  imports: [TableModule, ButtonModule, IconFieldModule, InputIconModule, InputTextModule, TagModule],
  template: `
    <p-table
      [value]="items()"
      [loading]="loading()"
      [paginator]="true"
      [rows]="20"
      [globalFilterFields]="['name', 'status', 'email']"
      dataKey="id"
      styleClass="w-full"
    >
      <ng-template pTemplate="caption">
        <div class="flex items-center justify-between">
          <p-iconfield>
            <p-inputicon styleClass="pi pi-search" />
            <input pInputText (input)="dt.filterGlobal($event, 'contains')" placeholder="Search..." />
          </p-iconfield>
          <p-button label="New" icon="pi pi-plus" (onClick)="openNew()" />
        </div>
      </ng-template>

      <ng-template pTemplate="header">
        <tr>
          <th pSortableColumn="name">Name <p-sortIcon field="name" /></th>
          <th pSortableColumn="status">Status</th>
          <th pSortableColumn="email">Email</th>
          <th>Actions</th>
        </tr>
      </ng-template>

      <ng-template pTemplate="body" let-item>
        <tr>
          <td>{{ item.name }}</td>
          <td><p-tag [value]="item.status" [severity]="getSeverity(item.status)" /></td>
          <td>{{ item.email }}</td>
          <td>
            <p-button icon="pi pi-pencil" [text]="true" [rounded]="true" (onClick)="edit(item)" />
            <p-button icon="pi pi-trash" [text]="true" [rounded]="true" severity="danger" (onClick)="delete(item)" />
          </td>
        </tr>
      </ng-template>

      <ng-template pTemplate="emptymessage">
        <tr><td colspan="4" class="text-center py-8 text-gray-400">No records found</td></tr>
      </ng-template>
    </p-table>
  `
})
export class DataTableComponent {
  dt = viewChild.required(Table);
  items = signal<Item[]>([]);
  loading = signal(false);
}
```

### 4.3 Dynamic Dialog (Service Pattern)

```typescript
@Component({
  standalone: true,
  template: `<p-dynamicdialog />`
})
export class AppComponent {
  private dialogService = inject(DialogService);

  openDetail(item: Item) {
    const ref = this.dialogService.open(DetailComponent, {
      header: 'Item Detail',
      width: '600px',
      modal: true,
      closable: true,
      data: { item }
    });

    ref.onClose.subscribe((result) => {
      if (result) this.refreshList();
    });
  }
}
```

### 4.4 Confirmation Dialog

```typescript
@Component({
  standalone: true,
  imports: [ConfirmDialogModule, ButtonModule],
  template: `
    <p-confirmdialog />
    <p-button label="Delete" severity="danger" (onClick)="confirmDelete()" />
  `
})
export class DeleteComponent {
  private confirmationService = inject(ConfirmationService);
  private messageService = inject(MessageService);

  confirmDelete() {
    this.confirmationService.confirm({
      message: 'Are you sure you want to delete this item?',
      header: 'Confirm Deletion',
      icon: 'pi pi-exclamation-triangle',
      accept: () => {
        this.messageService.add({
          severity: 'success',
          summary: 'Deleted',
          detail: 'Item deleted successfully',
          life: 3000
        });
      }
    });
  }
}
```

---

## 5. PrimeNG MCP Server

PrimeNG provides an MCP (Model Context Protocol) server that gives AI assistants comprehensive access to PrimeNG component documentation, properties, events, and templates — directly from your editor.

**Configured in your OpenCode MCP** under the key `primeng`:

```json
"primeng": {
  "command": ["npx", "-y", "@primeng/mcp"],
  "enabled": true,
  "type": "local"
}
```

The MCP server offers tools to:
- Query component documentation (properties, events, slots/templates)
- Get usage examples
- Access theming and styling info
- Check migration notes

Use the MCP server when you need specific API details about any PrimeNG component.

### LLMs.txt

PrimeNG also provides LLM-optimized documentation at:
- <https://primeng.org/llms>

This endpoint returns structured, machine-readable documentation of all components for AI consumption.

---

## 6. Migration Guide (v19 → v20 → v21)

### Key Breaking Changes

- **v19**: Standalone components by default, `providePrimeNG()` replaces `PrimeNGConfig`
- **v20**: CSS layers introduced, removed legacy styles
- **v21**: Theme system overhaul, new presets (Aura/Nora/Material), Pass Through API improved

### Migration Steps

```bash
npm install primeng@latest @primeng/themes@latest @primeng/icons@latest
```

1. Replace `import { PrimeNGConfig }` with `providePrimeNG()`
2. Replace theme imports to `@primeng/themes/aura` (or `lara`/`nora`/`material`)
3. Add CSS layers in your styles
4. Remove `node_modules/primeng/resources/*` imports — no longer needed

---

## 7. Accessibility

PrimeNG components target **WCAG 2.1 AA** compliance:
- Full keyboard navigation
- ARIA attributes on all interactive components
- Screen reader support (aria-live regions, roles, states)
- Focus management in overlays and dialogs

---

## 8. Performance Tips

| Technique | How |
|-----------|-----|
| Lazy load modules | Import only what you use — components are tree-shakeable |
| VirtualScroll | Use `<p-virtualscroller>` for lists > 1000 items |
| Table `virtualScroll` | Enable virtual scrolling in `<p-table>` for large datasets |
| Lazy loading data | Use `onLazyLoad` event in `<p-table>` for server-side pagination |
| Deferred loading | Use `pDeferred` directive for below-fold content |
| OnPush strategy | All PrimeNG components support `ChangeDetectionStrategy.OnPush` |
