---
name: gabriel-design-system
description: Gabriel design system — minimalist black and white base, semantic colors only for success/error/warning/info. Typography, spacing, component patterns.
---

# Gabriel Design System

## Core Philosophy

**Minimalist. Intentional. Semantic.**

The Gabriel design system uses a black and white base with semantic color only where it carries meaning. No decorative color. Every design decision should have a purpose.

---

## Color Palette

### Base (Black & White)
```css
--color-background: #ffffff;      /* Page background */
--color-surface: #f8f9fa;         /* Cards, panels */
--color-border: #e9ecef;          /* Borders, dividers */
--color-text-primary: #0d1117;    /* Main text */
--color-text-secondary: #6c757d;  /* Supporting text */
--color-text-disabled: #adb5bd;   /* Disabled states */
```

### Semantic Colors (Use ONLY for state communication)
```css
/* Success — completed actions, active states */
--color-success-bg: #d1fae5;
--color-success-text: #065f46;
--color-success-border: #6ee7b7;

/* Error — failures, invalid data, destructive actions */
--color-error-bg: #fee2e2;
--color-error-text: #991b1b;
--color-error-border: #fca5a5;

/* Warning — attention needed, non-blocking issues */
--color-warning-bg: #fef9c3;
--color-warning-text: #854d0e;
--color-warning-border: #fde047;

/* Info — neutral information, guidance */
--color-info-bg: #dbeafe;
--color-info-text: #1e40af;
--color-info-border: #93c5fd;
```

### When to Use Color
| Color | Use | Never Use For |
|-------|-----|---------------|
| Success (green) | Saved, created, active, paid | Section titles, decorative accents |
| Error (red) | Failed, invalid, deleted, overdue | Warnings or non-critical states |
| Warning (yellow) | Attention needed, low stock | Errors or success states |
| Info (blue) | Informational messages, hints | Actions or status badges |

---

## Typography

### Scale
```
Display:    3rem (48px) / font-bold / tracking-tight
H1:         2.25rem (36px) / font-bold / tracking-tight
H2:         1.5rem (24px) / font-semibold
H3:         1.25rem (20px) / font-semibold
H4:         1rem (16px) / font-semibold
Body:       0.875rem (14px) / font-normal / leading-relaxed
Small:      0.75rem (12px) / font-normal
Micro:      0.6875rem (11px) / font-medium / tracking-wide / uppercase
```

### Font Family
- **UI**: Inter (primary), system-ui (fallback)
- **Mono**: JetBrains Mono, ui-monospace (fallback)

### Usage Examples
```html
<!-- Page title -->
<h1 class="text-3xl font-bold tracking-tight text-gray-900">Customers</h1>

<!-- Section heading -->
<h2 class="text-xl font-semibold text-gray-900">Recent Orders</h2>

<!-- Data label -->
<dt class="text-xs font-medium uppercase tracking-wide text-gray-400">Total Revenue</dt>

<!-- Data value -->
<dd class="text-2xl font-bold text-gray-900 mt-1">$24,500</dd>
```

---

## Spacing System

Uses Tailwind's default scale (4px base):

```
xs:   4px  (gap-1, p-1)
sm:   8px  (gap-2, p-2)
md:   12px (gap-3, p-3) — most common
lg:   16px (gap-4, p-4)
xl:   24px (gap-6, p-6) — card padding
2xl:  32px (gap-8, p-8) — section spacing
3xl:  48px (gap-12)     — page sections
```

---

## Component Patterns

### Primary Button
```html
<button class="inline-flex items-center gap-2 px-4 py-2 bg-gray-900 text-white text-sm font-medium rounded-lg hover:bg-gray-700 active:bg-gray-800 disabled:opacity-50 disabled:cursor-not-allowed transition-colors">
  <i class="pi pi-plus text-xs"></i>
  Create Order
</button>
```

### Secondary Button
```html
<button class="inline-flex items-center gap-2 px-4 py-2 bg-white text-gray-700 text-sm font-medium rounded-lg border border-gray-300 hover:bg-gray-50 active:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed transition-colors">
  Cancel
</button>
```

### Danger Button
```html
<button class="inline-flex items-center gap-2 px-4 py-2 bg-red-600 text-white text-sm font-medium rounded-lg hover:bg-red-700 active:bg-red-800 transition-colors">
  <i class="pi pi-trash text-xs"></i>
  Delete
</button>
```

### Text Button (tertiary)
```html
<button class="text-sm font-medium text-gray-600 hover:text-gray-900 transition-colors">
  View Details
</button>
```

---

### Status Badges

```html
<!-- Generic badge pattern -->
<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium">

<!-- Active / Success -->
<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
  Active
</span>

<!-- Inactive / Neutral -->
<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-600">
  Inactive
</span>

<!-- Pending / Warning -->
<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
  Pending
</span>

<!-- Error / Cancelled -->
<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800">
  Cancelled
</span>
```

---

### Form Fields

```html
<div class="space-y-5">
  <div class="flex flex-col gap-1.5">
    <!-- Label -->
    <label class="text-sm font-medium text-gray-700">
      Full Name
      <span class="text-red-500 ml-0.5">*</span>
    </label>

    <!-- Input -->
    <input
      type="text"
      class="w-full px-3 py-2 text-sm text-gray-900 bg-white border border-gray-300 rounded-lg
             placeholder:text-gray-400
             focus:outline-none focus:ring-2 focus:ring-gray-900 focus:border-transparent
             disabled:bg-gray-50 disabled:text-gray-400
             aria-invalid:border-red-500 aria-invalid:focus:ring-red-500"
      placeholder="Enter full name"
    />

    <!-- Error message -->
    <p class="text-xs text-red-600 flex items-center gap-1">
      <i class="pi pi-times-circle text-xs"></i>
      Name is required
    </p>

    <!-- Help text (when no error) -->
    <p class="text-xs text-gray-400">2-100 characters</p>
  </div>
</div>
```

---

### Data Table Row Patterns

```html
<tr class="border-b border-gray-100 hover:bg-gray-50/50 transition-colors group">
  <!-- ID column -->
  <td class="px-4 py-3">
    <span class="font-mono text-xs text-gray-400">{{ id | slice:0:8 }}...</span>
  </td>

  <!-- Name column -->
  <td class="px-4 py-3">
    <span class="text-sm font-medium text-gray-900">{{ name }}</span>
  </td>

  <!-- Amount column -->
  <td class="px-4 py-3 text-right">
    <span class="text-sm font-medium text-gray-900 tabular-nums">{{ amount | currency }}</span>
  </td>

  <!-- Actions column (visible on hover) -->
  <td class="px-4 py-3">
    <div class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
      <button class="p-1 rounded hover:bg-gray-100 text-gray-400 hover:text-gray-700">
        <i class="pi pi-pencil text-xs"></i>
      </button>
    </div>
  </td>
</tr>
```

---

## Layout Templates

### Dashboard Layout
```html
<div class="min-h-screen flex bg-gray-50">
  <!-- Sidebar -->
  <aside class="w-60 bg-gray-900 flex flex-col flex-shrink-0">
    <!-- Logo -->
    <div class="h-16 flex items-center px-6 border-b border-gray-800">
      <span class="text-white font-bold text-lg">Parsun</span>
    </div>
    <!-- Nav -->
    <nav class="flex-1 px-3 py-4 space-y-1">
      <!-- nav items -->
    </nav>
  </aside>

  <!-- Main -->
  <div class="flex-1 flex flex-col overflow-hidden">
    <!-- Top bar -->
    <header class="h-16 bg-white border-b border-gray-200 flex items-center justify-between px-6">
      <!-- breadcrumb, user menu -->
    </header>

    <!-- Content -->
    <main class="flex-1 overflow-auto p-6">
      <router-outlet />
    </main>
  </div>
</div>
```

---

## Rules

1. **No color for decoration** — only for meaning
2. **Consistent spacing** — use the defined scale, don't invent new values
3. **Text hierarchy** — always clear what's primary, secondary, tertiary
4. **Hover states** — every interactive element must have a visible hover state
5. **Focus states** — visible focus ring for accessibility (focus:ring-2 focus:ring-gray-900)
6. **Disabled states** — opacity-50 + cursor-not-allowed
7. **Empty states** — every list view must have an empty state design
8. **Loading states** — use skeleton loaders, not spinners where possible
