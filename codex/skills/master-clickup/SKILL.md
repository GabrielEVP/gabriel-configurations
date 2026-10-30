---
name: master-clickup
description: "Trigger: clickup, task, tarea, modulo, module, parsun, lexand, proyecto, workspace, doc, documentacion. ClickUp workspace management for Parsun & Lexand. Create module tasks with phase subtasks (Fase N — Layer: Description), document modules in the Modulos doc following the standard template (DB structure, migrations, folder structure, business rules, endpoints), assign to Gabriel, set priority, create dependencies between subtasks, sync doc on completion."
---

# Master ClickUp

## Workspace Structure

```
Workspace
├── Lexand (space: 90127428094)
│   └── Lexand (folder: 901210971562)
│       └── Board (list: 901217812839)
├── Parsun (space: 90127428101)
│   └── Proyecto (list: 901217812567)
└── Personal (space: 90127428460)
    ├── Dinero (folder)
    ├── Libros (folder)
    ├── Ejercicio (folder)
    ├── Linux (folder)
    ├── Software (folder)
    └── Proyectos (folder)
```

## User & Assignee Reference

| Name | User ID |
|------|---------|
| Gabriel Vargas | `296620700` |

## Priorities

| Label | Priority value |
|-------|----------------|
| Urgent | `urgent` |
| High | `high` |
| Normal | `normal` |
| Low | `low` |

---

## Pre-Flight: Read Before Write (MANDATORY)

Before creating or updating ANY task or doc, ALWAYS:

1. **Search** for existing items first: `clickup_search(keywords: "{name}", filters: {location: {subcategories: ["{list_id}"]}})`
2. If found → inform the user: "Ya existe {item}. ¿Querés actualizarlo o crear uno nuevo?"
3. **Read existing doc pages** before updating: `clickup_clickup_get_document_pages(document_id, page_ids, content_format: "text/md")`
4. Compare current content with planned changes — ONLY update sections that changed
5. Never overwrite content that hasn't changed

---

## Parsun Task Structure

### List ID: `901217812567` (Proyecto)

### Parent Task Pattern

A parent task represents a complete module/feature:

```json
{
  "name": "Módulo {Name}",
  "list_id": "901217812567",
  "priority": "urgent|high|normal|low",
  "assignees": [296620700],
  "tags": ["parsun"],
  "markdown_description": "Implementación completa del Módulo de {Name} para Parsun.\n\nIncluye: {summary}.\n\nVer documento de diseño en ClickUp: \"{DocName}\" en el doc Módulos."
}
```

### Subtask Pattern (Phases)

```
Fase {N} — Database: {description of DB changes}
Fase {N} — Backend: {description of backend changes}
Fase {N} — Frontend: {description of frontend changes}
```

- Each phase (Fase 1, 2, 3...) has up to 3 subtasks: Database, Backend, Frontend
- Database tasks go first (migrations), then Backend, then Frontend
- All subtasks: assignee = Gabriel (296620700), same priority as parent, tags = ["parsun", "{module-name}"]

### Adding Dependencies Between Subtasks

After creating subtasks, add dependencies so phases execute in order:

| Dependency | Type | Why |
|------------|------|-----|
| Fase 1 DB → Fase 1 Backend | `waiting_on` | Backend needs DB tables |
| Fase 1 Backend → Fase 1 Frontend | `waiting_on` | Frontend needs API endpoints |
| Fase 1 Frontend → Fase 2 DB | `waiting_on` | Phase 2 needs Phase 1 complete |

Use `clickup_add_task_dependency(task_id, depends_on, type: "waiting_on")` for each pair.

### Syncing Docs on Task Completion

When a task is marked complete, check if it has a corresponding module doc page:

1. **Search the Modulos doc** for a page matching the module name
2. If found → read the current page content
3. Update the relevant sections (add/update endpoints, business rules, implementation notes)
4. If NOT found → create a new page (see "Creating New Doc Pages")

---

## Lexand Task Structure

### List ID: `901217812839` (Board)

### Parent Task Pattern

```json
{
  "name": "Módulo {Name}",
  "list_id": "901217812839",
  "priority": "urgent|high|normal|low",
  "assignees": [296620700],
  "tags": ["lexand"],
  "markdown_description": "Implementación del Módulo de {Name} para Lexand.\n\nIncluye: {summary}."
}
```

### Lexand Subtask Pattern

Lexand uses a simpler phase structure (no fixed "Fase N" convention — tasks are organized by implementation priority):

```
Fase {N} — {Module}: {Specific feature or component}
```

### Lexand Doc Structure

The Lexand doc (id: `2kxurc2r-832`) has a different template than Parsun:

```markdown
# Módulo: {Module Name}

## Propósito

{What the module does, one paragraph.}

* * *

## Patrón Aggregate (DDD)  (if applicable)

{Entity relationships and aggregate root}

## Tipos/Enums

{If the module has type enums}

## Esquema de Base de Datos

### Tabla: `{table_name}`

```sql
CREATE TABLE {table_name} (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    ...
);
```

### Tabla: `{child_table}`

* * *

## Reglas de Negocio

1. {Rule 1}
2. {Rule 2}

## DTOs

### Request DTOs (SIN IDs)

### Response DTOs (CON IDs)

## Esquema de Clases (Domain)

## API Endpoints

```
GET    /api/{module}
POST   /api/{module}
...
```

## Casos de Prueba Críticos

1. {Test case 1}
2. {Test case 2}
```

### Key Differences from Parsun

| Aspect | Parsun | Lexand |
|--------|--------|--------|
| Doc ID | `2kxurc2r-1012` | `2kxurc2r-832` |
| IDs | `BINARY(16)` UUID | `BIGINT AUTO_INCREMENT` |
| Template | Tabular (DB tables, migrations) | SQL blocks, more narrative |
| Task naming | `Fase N — Layer: desc` | `Fase N — Module: desc` |
| Tags | `parsun` | `lexand` |

When documenting, ALWAYS check which project the task belongs to and use the corresponding template.

---

## Module Documentation (Modulos Doc — Parsun)

The doc "Modulos" (id: `2kxurc2r-1012`) in the Parsun space contains all module documentation. Each module has its own page.

### Doc Pages Reference

| Page Name | Page ID |
|-----------|---------|
| Customers | `2kxurc2r-1492` |
| Users | `2kxurc2r-1572` |
| Auth | `2kxurc2r-1592` |
| Categories | `2kxurc2r-1612` |
| Products | `2kxurc2r-1632` |
| Warehouses | `2kxurc2r-1652` |
| Price Types | `2kxurc2r-1672` |
| Orders | `2kxurc2r-1692` |
| Reports — Orders | `2kxurc2r-1872` (subpage of Orders) |
| Invoices | `2kxurc2r-1892` |
| Reports — Invoices | `2kxurc2r-1912` (subpage of Invoices) |
| Quotes | `2kxurc2r-1932` |
| Reports — Quotes | `2kxurc2r-1952` (subpage of Quotes) |
| Payment Methods | `2kxurc2r-2032` |
| Payments | `2kxurc2r-2052` |
| Credit Notes | `2kxurc2r-2072` |
| Debit Notes | `2kxurc2r-2092` |
| Returns | `2kxurc2r-2112` |
| Cash Register | `2kxurc2r-2132` |
| Receivables | `2kxurc2r-2152` |
| Reports | `2kxurc2r-1792` |

### Parsun Module Template

```markdown
Módulo encargado de la {gestión/funcionalidad} de {entidad} en Parsun. Permite {acciones principales}.

* * *

## Estructura de Base de Datos

### Tabla: `{table_name}`

| Columna | Tipo | Restricciones | Descripción |
|---------|------|---------------|-------------|
| `{col}` | `{type}` | {PK, FK, NOT NULL, etc.} | {description} |

**Índices:** {indexes}
* * *

### {child table if any}

* * *

## Migraciones Flyway

| Versión | Archivo | Descripción |
|---------|---------|-------------|
| V{N} | `V{N}__{description}.sql` | {what it creates} |

* * *

## Estructura de Carpetas

### Backend

```
{module}/
├── domain/
│   ├── entities/
│   │   └── {Entity}.java
│   ├── valueobjects/
│   │   └── ...
│   └── repositories/
│       └── {Entity}Repository.java
├── application/
│   ├── usecases/
│   │   └── ...
│   ├── dto/
│   │   ├── request/
│   │   └── response/
│   └── mappers/
└── infrastructure/
    ├── web/
    ├── persistence/
    └── config/
```

### Frontend

```
features/{module}/
├── pages/
├── components/
├── services/
├── models/
└── {module}.routes.ts
```

* * *

## Reglas de Negocio

- {Rule 1}
- {Rule 2}

* * *

## Endpoints REST

**Base URL:** `/api/{module}`

| Método | Ruta | Descripción | Roles |
|--------|------|-------------|-------|
| `GET` | `/` | Listar | {roles} |
| `POST` | `/` | Crear | {roles} |

* * *

## Notas de Implementación

- IDs: `BINARY(16)` en MySQL, UUID string en API
- Arquitectura Hexagonal: dominio sin Spring ni JPA
- Tests con `BaseIntegrationTest` (Testcontainers) y `BaseWebControllerTest`
- Frontend: Vitest `.spec.ts` junto al source
```

---

## Module Documentation (Lexand Doc)

The Lexand doc (id: `2kxurc2r-832`) covers architecture, modules, and technology.

### Doc Pages Reference

| Page Name | Page ID | Description |
|-----------|---------|-------------|
| Lexand | `2kxurc2r-612` | Home/overview page |
| Módulos | `2kxurc2r-632` | Module table of contents with status |
| Tecnología | `2kxurc2r-652` | Tech stack |
| Arquitectura Backend | `2kxurc2r-672` | Backend architecture |
| Arquitectura Frontend | `2kxurc2r-692` | Frontend architecture |
| Customer | `2kxurc2r-712` | Customer module docs |
| Invoice | `2kxurc2r-732` | Invoice module docs |
| Order | `2kxurc2r-752` | Order module docs |
| Purchase | `2kxurc2r-772` | Purchase module docs |
| Product | `2kxurc2r-792` | Product module docs |
| Supplier | `2kxurc2r-812` | Supplier module docs |
| User | `2kxurc2r-832` | User module docs |
| Warehouse | `2kxurc2r-852` | Warehouse module docs |

### Lexand Module Template

```markdown
# Módulo: {Module Name}

## Propósito

{What the module does.}

* * *

## Patrón Aggregate (DDD)

{Entity relationships.}

## Esquema de Base de Datos

### Tabla: `{table_name}`

```sql
CREATE TABLE {table_name} (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    ...
);
```

* * *

## Reglas de Negocio

1. {Rule 1}

## DTOs

### Request (SIN IDs)
### Response (CON IDs)

## Esquema de Clases (Domain)

## API Endpoints

## Casos de Prueba Críticos
```

---

## Creating New Doc Pages

When a module doesn't have a doc page yet:

### For Parsun

```javascript
// 1. Create the new page
clickup_create_document_page(
  document_id: "2kxurc2r-1012",
  name: "{ModuleName}",
  content: "{Parsun template content filled}",
  content_format: "text/md"
)

// 2. Add it to the doc structure at the right position
// The page is auto-added as a top-level page
```

### For Lexand

```javascript
clickup_create_document_page(
  document_id: "2kxurc2r-832",
  name: "{ModuleName}",
  content: "{Lexand template content filled}",
  content_format: "text/md"
)

// 2. Update the Módulos table of contents page (id: 2kxurc2r-632)
// Read current content, add the new module row to the table
clickup_update_document_page(
  document_id: "2kxurc2r-832",
  page_id: "2kxurc2r-632",
  content: "{updated TOC with new module}"
)
```

---

## Workflow: Complete Module Creation

When asked to create a new module in ClickUp:

```
1. SEARCH for existing task/docs first (pre-flight)
2. CREATE parent task: Módulo {Name} in correct list
3. CREATE subtasks organized by layers and phases
4. ADD dependencies: DB → Backend → Frontend per phase, and between phases
5. CREATE doc page using the correct template (Parsun or Lexand)
6. UPDATE the Módulos table of contents if Lexand
7. ASSIGN all to Gabriel with correct priority
```

## Workflow: Update Module Doc on Task Complete

When a task is completed:

```
1. IDENTIFY which module the task belongs to
2. DETERMINE if it's Parsun or Lexand (check list_id)
3. READ the current doc page content via get_document_pages
4. COMPARE with planned content — update ONLY changed sections
5. CALL update_document_page with the new content
6. If the module has a status table (Lexand Módulos page) → update status
```

---

## Useful ClickUp MCP Commands Reference

```javascript
// Search
clickup_search(keywords: "Módulo Finanzas", filters: {location: {subcategories: ["901217812567"]}})

// Get task with subtasks
clickup_get_task(task_id: "869dcxukd", include: ["subtasks", "description"])

// Create task
clickup_create_task(name: "Módulo {Name}", list_id: "901217812567", priority: "high", assignees: ["296620700"], tags: ["parsun"], markdown_description: "...")

// Add dependency
clickup_add_task_dependency(task_id: "subtask-2", depends_on: "subtask-1", type: "waiting_on")

// Update task (mark complete)
clickup_update_task(task_id: "...", status: "complete")

// Get doc pages
clickup_get_document_pages(document_id: "2kxurc2r-1012", page_ids: ["2kxurc2r-1492"], content_format: "text/md")

// Create doc page
clickup_create_document_page(document_id: "2kxurc2r-1012", name: "NewModule", content: "...", content_format: "text/md")

// Update doc page
clickup_update_document_page(document_id: "2kxurc2r-1012", page_id: "2kxurc2r-1492", content: "...", content_format: "text/md")
```
