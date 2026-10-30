---
name: tailwind-design
description: Tailwind CSS v4 design patterns. Utility-first CSS, responsive design, dark mode, custom theme tokens, component patterns without CSS files.
---

# Tailwind CSS v4 Design Patterns

## v4 Key Changes

```css
/* app.css — Tailwind v4 uses CSS-first configuration */
@import "tailwindcss";

@theme {
  /* Custom design tokens */
  --color-brand-50: #f8f9fa;
  --color-brand-900: #0d1117;

  --font-sans: 'Inter', ui-sans-serif, system-ui;

  --radius-card: 0.75rem;
  --shadow-card: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);
}
```

---

## Utility-First Philosophy

Never write custom CSS. Compose Tailwind utilities instead.

```html
<!-- WRONG: Custom CSS class -->
<div class="product-card">...</div>

<!-- CORRECT: Utility composition -->
<div class="bg-white rounded-xl border border-gray-200 shadow-sm p-6 hover:shadow-md transition-shadow">
  ...
</div>
```

---

## Responsive Design

Mobile-first approach with breakpoint prefixes:

```html
<!-- Stack on mobile, side by side on desktop -->
<div class="flex flex-col md:flex-row gap-6">
  <aside class="w-full md:w-64 lg:w-72">...</aside>
  <main class="flex-1">...</main>
</div>

<!-- Hide on mobile, show on desktop -->
<div class="hidden lg:block">Desktop only content</div>

<!-- Show on mobile, hide on desktop -->
<div class="block md:hidden">Mobile only content</div>

<!-- Responsive typography -->
<h1 class="text-2xl md:text-3xl lg:text-4xl font-bold">Title</h1>

<!-- Responsive grid -->
<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
```

**Breakpoints:**
| Prefix | Min width |
|--------|----------|
| `sm:` | 640px |
| `md:` | 768px |
| `lg:` | 1024px |
| `xl:` | 1280px |
| `2xl:` | 1536px |

---

## Component Patterns

### Card
```html
<div class="bg-white rounded-xl border border-gray-200 shadow-sm overflow-hidden">
  <div class="p-6">
    <h3 class="text-lg font-semibold text-gray-900">Card Title</h3>
    <p class="text-sm text-gray-500 mt-1">Card description goes here</p>
  </div>
  <div class="px-6 py-4 bg-gray-50 border-t border-gray-200 flex justify-end gap-3">
    <button class="px-4 py-2 text-sm font-medium text-gray-700 hover:text-gray-900">Cancel</button>
    <button class="px-4 py-2 text-sm font-medium bg-gray-900 text-white rounded-lg hover:bg-gray-700">Save</button>
  </div>
</div>
```

### Empty State
```html
<div class="flex flex-col items-center justify-center py-16 text-center">
  <div class="w-12 h-12 rounded-full bg-gray-100 flex items-center justify-center mb-4">
    <i class="pi pi-inbox text-gray-400 text-xl"></i>
  </div>
  <h3 class="text-lg font-semibold text-gray-900">No orders yet</h3>
  <p class="text-sm text-gray-500 mt-1 max-w-sm">
    Once orders are created, they'll appear here.
  </p>
  <button class="mt-4 px-4 py-2 bg-gray-900 text-white text-sm font-medium rounded-lg hover:bg-gray-700">
    Create First Order
  </button>
</div>
```

### Loading Skeleton
```html
<div class="animate-pulse space-y-4">
  @for (i of [1,2,3,4,5]; track i) {
    <div class="flex items-center gap-4 p-4 bg-white rounded-lg border border-gray-200">
      <div class="w-10 h-10 bg-gray-200 rounded-full"></div>
      <div class="flex-1 space-y-2">
        <div class="h-4 bg-gray-200 rounded w-1/3"></div>
        <div class="h-3 bg-gray-100 rounded w-1/2"></div>
      </div>
      <div class="h-6 bg-gray-200 rounded w-20"></div>
    </div>
  }
</div>
```

### Alert / Notification Banner
```html
<!-- Error -->
<div class="flex items-start gap-3 p-4 bg-red-50 border border-red-200 rounded-lg">
  <i class="pi pi-times-circle text-red-500 mt-0.5"></i>
  <div>
    <p class="text-sm font-medium text-red-800">Error Title</p>
    <p class="text-sm text-red-600 mt-0.5">Error description here.</p>
  </div>
</div>

<!-- Warning -->
<div class="flex items-start gap-3 p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
  <i class="pi pi-exclamation-triangle text-yellow-500 mt-0.5"></i>
  <p class="text-sm text-yellow-800">Warning message here.</p>
</div>

<!-- Success -->
<div class="flex items-start gap-3 p-4 bg-green-50 border border-green-200 rounded-lg">
  <i class="pi pi-check-circle text-green-500 mt-0.5"></i>
  <p class="text-sm text-green-800">Operation completed successfully.</p>
</div>

<!-- Info -->
<div class="flex items-start gap-3 p-4 bg-blue-50 border border-blue-200 rounded-lg">
  <i class="pi pi-info-circle text-blue-500 mt-0.5"></i>
  <p class="text-sm text-blue-800">Informational message here.</p>
</div>
```

---

## Semantic Color System

Only use color to convey meaning, never decoratively:

```typescript
// TypeScript helper for consistent color usage
const semanticColors = {
  success: 'bg-green-100 text-green-800 border-green-200',
  error:   'bg-red-100 text-red-800 border-red-200',
  warning: 'bg-yellow-100 text-yellow-800 border-yellow-200',
  info:    'bg-blue-100 text-blue-800 border-blue-200',
  neutral: 'bg-gray-100 text-gray-800 border-gray-200'
} as const;
```

---

## Typography Scale

```html
<!-- Page title -->
<h1 class="text-3xl font-bold tracking-tight text-gray-900">Page Title</h1>

<!-- Section heading -->
<h2 class="text-xl font-semibold text-gray-900">Section Heading</h2>

<!-- Card title -->
<h3 class="text-base font-semibold text-gray-900">Card Title</h3>

<!-- Body text -->
<p class="text-sm text-gray-600 leading-relaxed">Body text</p>

<!-- Supporting text -->
<p class="text-xs text-gray-400">Supporting / meta text</p>

<!-- Label -->
<label class="text-sm font-medium text-gray-700">Field Label</label>

<!-- Code / mono -->
<code class="font-mono text-sm bg-gray-100 px-1.5 py-0.5 rounded text-gray-800">code</code>
```

---

## Spacing & Layout Utilities

```html
<!-- Page layout -->
<main class="min-h-screen bg-gray-50">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <!-- content -->
  </div>
</main>

<!-- Section spacing -->
<section class="space-y-6">
  <div>...</div>
  <div>...</div>
</section>

<!-- Inline elements -->
<div class="flex items-center gap-2">
  <icon />
  <span>Label</span>
</div>

<!-- Divider -->
<hr class="border-gray-200 my-6" />
```

---

## Dark Mode Support

```html
<!-- Components that respect dark mode -->
<div class="bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100">
  <p class="text-gray-600 dark:text-gray-400">Secondary text</p>
  <div class="border border-gray-200 dark:border-gray-700">...</div>
</div>
```

Toggle dark mode by adding `.dark` class to `<html>` element.

---

## Performance Tips

1. **Avoid dynamic class construction** — Tailwind purges unused classes:
   ```typescript
   // WRONG — Tailwind can't detect this
   const color = `text-${dynamicColor}-500`;

   // CORRECT — full class names in source
   const classMap = { red: 'text-red-500', blue: 'text-blue-500' };
   const cls = classMap[color];
   ```

2. **Use `@apply` sparingly** — only for truly repeated patterns in CSS, not in templates

3. **Tailwind v4 is faster** — CSS-native cascade layers, no PostCSS plugin overhead
