---
name: master-clickup
description: "Trigger: clickup, task, tarea, modulo, module, parsun, lexand, proyecto, workspace, doc, documentacion. ClickUp workspace management for Parsun & Lexand. Create module tasks with grouped subtasks by layer (📁 Database Seeder → 📁 Backend → 📁 Frontend → 📁 Reports), each with its own priority; document modules in the Modulos doc following the standard template (DB structure, migrations + seed, folder structure, business rules, endpoints, tests — backend + frontend with coverage), assign to Gabriel, set priority, sync doc on completion."
license: Apache-2.0
metadata:
  author: gabriel
  version: "2.2"
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

A parent task represents a complete module/feature. Usa tags para categorizar y una descripción que indique el propósito y referencias a diseño:

```json
{
  "name": "Módulo {Name}",
  "list_id": "901217812567",
  "priority": "high",
  "assignees": [296620700],
  "tags": ["parsun"],
  "markdown_description": "Implementación completa del Módulo de {Name} para Parsun.\n\nIncluye: {summary}.\n\n**Guía de diseño:** {referencia de diseño, ej: \"Copiar el diseño de páginas y componentes del frontend del módulo Customers\"}.\n\nVer documento de diseño en ClickUp: \"{DocName}\" en el doc Módulos."
}
```

### Subtask Pattern — Grouped by Layer (Orders-style)

Las subtareas se organizan en **agrupadores por capa** (no por fases numéricas). Cada agrupador es una subtask que contiene subtareas hijas específicas.

Cada agrupador tiene su propia **prioridad** según la carga de trabajo de esa capa.

```
{Módulo}
│
├── 📁 Database Seeder          (priority: low)
│   ├── Create Flyway migrations for {module} tables
│   └── Seed {module} with {statuses or initial data}
│
├── 📁 Backend                  (priority: high)
│   ├── {UseCase}: {description}
│   ├── {UseCase}: {description}
│   └── Implement {Entity}Controller
│
├── 📁 Frontend                 (priority: high)
│   ├── {Module} Routing & Shell
│   ├── {Module}Service (HTTP Client)
│   ├── {Entity}ListComponent
│   ├── {Entity}FormComponent (Create & Edit)
│   ├── {Entity}DetailComponent
│   └── {specific components}
│
└── 📁 Reports                  (priority: normal, opcional)
    ├── {Report} PDF Template Design
    └── {Report} Controller / Download Action
```

**Reglas:**
- **Database Seeder**: Siempre priority `low` (es rápida). Incluye migraciones DDL + seed DML.
- **Backend**: Siempre priority `high` (mayor complejidad técnica).
- **Frontend**: Siempre priority `high` (múltiples componentes + servicios).
- **Reports**: Priority `normal` (opcional, solo si el módulo genera reportes PDF).
- Todas las subtasks (agrupadores e hijas): assignee = Gabriel (`296620700`).
- Tags en las subtasks hijas: `["parsun", "{module-name}"]` (solo en las hojas, no en los agrupadores).
- **No se usan dependencias** entre agrupadores — cada uno se trabaja de forma independiente.

### Agrupador vs Fase: ¿Cuándo usar cada uno?

| Situación | Patrón |
|-----------|--------|
| Módulo con capas bien definidas (DB, Backend, Frontend) | 📁 **Grouped by Layer** (Orders-style) |
| Funcionalidad pequeña sin frontend (ej. solo backend) | Subtask directa sin agrupador |
| Módulo con entregas progresivas (Fase 1, luego Fase 2) | **Fases** (Lexand-style) |

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
| Template | Tabular (DB tables, migrations, seed) | SQL blocks, more narrative |
| Task naming | `Fase N — Layer: desc` | `Fase N — Module: desc` |
| Tags | `parsun` | `lexand` |
| Migrations | Incluye seed data versionado | Incluye seed data versionado |
| Tests | Unitarios + API + Integración + Frontend | Unitarios + API + Integración + Frontend |
| Cobertura | Mínimo 80% frontend, ~90% API | Mínimo 80% frontend, ~90% API |

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

**Nota:** Las migraciones incluyen tanto DDL (creación de tablas) como DML (seed data).
Cada módulo tiene sus propios seed scripts versionados que se ejecutan después de las tablas.

### Seed Data

| Versión | Archivo | Descripción |
|---------|---------|-------------|
| V{N} | `V{N}__seed_{module}.sql` | Datos iniciales del módulo |

Los seed scripts se colocan en `src/main/resources/db/migration/` con el versionado
Flyway estándar. Cada módulo tiene su seed junto a las migraciones de estructura.

* * *

## Estructura de Carpetas

### Backend

```
{módulo}/
├── domain/
│   ├── entities/
│   │   ├── {Entity}.java
│   │   └── {ChildEntity}.java
│   ├── valueobjects/
│   │   ├── {Entity}Id.java
│   │   ├── {Field}Value.java
│   │   └── {Type}Enum.java          # Enum
│   └── repositories/
│       ├── {Entity}Repository.java
│       └── {ChildEntity}Repository.java
├── application/
│   ├── usecases/
│   │   ├── Register{Entity}UseCase.java
│   │   ├── Update{Entity}UseCase.java
│   │   ├── Get{Entity}UseCase.java
│   │   ├── List{Entity}sUseCase.java
│   │   ├── Delete{Entity}UseCase.java
│   │   └── ...UseCase.java
│   ├── dto/
│   │   ├── request/
│   │   │   ├── Create{Entity}Request.java
│   │   │   ├── Update{Entity}Request.java
│   │   │   └── {Child}Request.java
│   │   └── response/
│   │       ├── {Entity}ListResponse.java
│   │       ├── {Entity}DetailResponse.java
│   │       ├── {Entity}CreateResponse.java
│   │       └── {Child}Response.java
│   └── mappers/
│       └── {Entity}Mapper.java
└── infrastructure/
    ├── web/
    │   └── {Entity}Controller.java
    ├── persistence/
    │   ├── entities/
    │   │   ├── {Entity}Entity.java
    │   │   └── {Child}Entity.java
    │   └── repositories/
    │       ├── Jpa{Entity}Repository.java
    │       ├── SpringData{Entity}Repository.java
    │       └── SpringData{Child}Repository.java
    ├── services/
    │   └── Sequential{Field}Generator.java
    └── config/
        └── {Entity}Config.java
```

### Backend Tests (junto al source, `src/test/java/...`)

```
{módulo}/
├── domain/
│   ├── entities/
│   │   └── {Entity}Test.java
│   └── valueobjects/
│       ├── {Field}ValueTest.java
│       └── {Type}Test.java
├── application/
│   └── usecases/
│       ├── Register{Entity}UseCaseTest.java
│       ├── Update{Entity}UseCaseTest.java
│       └── List{Entity}sUseCaseTest.java
└── infrastructure/
    ├── web/
    │   └── {Entity}ControllerTest.java
    └── persistence/
        └── {Entity}RepositoryAdapterTest.java
```

### Frontend

```
features/{module}/
├── pages/
│   ├── list/
│   │   ├── {entity}-list.component.ts
│   │   └── {entity}-list.component.spec.ts
│   ├── form/
│   │   ├── {entity}-form.component.ts
│   │   └── {entity}-form.component.spec.ts
│   └── details/
│       ├── {entity}-details.component.ts
│       └── {entity}-details.component.spec.ts
├── components/
│   ├── general-section/
│   │   ├── general-section.component.ts
│   │   └── general-section.component.spec.ts
│   ├── contact-section/
│   │   ├── contact-section.component.ts
│   │   └── contact-section.component.spec.ts
│   └── addresses-section/
│       ├── addresses-section.component.ts
│       └── addresses-section.component.spec.ts
├── services/
│   ├── api/
│   │   ├── {entity}-api.service.ts
│   │   └── {entity}-api.service.spec.ts
│   └── mappers/
│       ├── {entity}.mapper.ts
│       └── {entity}.mapper.spec.ts
├── validators/
│   ├── {entity}.validators.ts
│   └── {entity}.validators.spec.ts
├── data/
│   └── {entity-geo}.data.ts
├── models/
│   └── {entity}.model.ts
└── {entity}s.routes.ts
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

## Tests

### Backend Tests

| Tipo | Framework | Cobertura esperada |
|------|-----------|--------------------|
| Unitarios (Domain + ValueObjects) | JUnit 5 + Mockito | 100% — lógica de dominio y reglas de negocio |
| Use Cases | JUnit 5 + Mockito | 100% — flujos felices y bordes |
| Controller (API) | `@WebMvcTest` + MockMvc | ~90% — status codes, validación, serialización |
| Integración (Repository) | Testcontainers + `@DataJpaTest` | ~80% — queries nativas, proyecciones |

Ejecución: `mvnw test -pl {module}`

### Frontend Tests

| Tipo | Framework | Cobertura esperada |
|------|-----------|--------------------|
| Componentes | Vitest + Testing Library | ~80% — render, interacción, estados |
| Servicios API | Vitest + MSW (Mock Service Worker) | 100% — mapeo de requests/responses |
| Validators | Vitest | 100% — reglas de validación |
| Mappers | Vitest | 100% — transformaciones DTO ↔ Model |

Ejecución: `npx vitest run --reporter=verbose`

Los tests `.spec.ts` viven junto al source que prueban (collocation).

* * *

## Notas de Implementación

- IDs: `BINARY(16)` en MySQL, UUID string en API
- Arquitectura Hexagonal: dominio sin Spring ni JPA
- Tests con `BaseIntegrationTest` (Testcontainers) y `BaseWebControllerTest`
- Seed data versionado con Flyway junto a las migraciones DDL
- Frontend: Vitest `.spec.ts` junto al source, cobertura mínima 80%
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

## Migraciones Flyway

| Versión | Archivo | Descripción |
|---------|---------|-------------|
| V{N} | `V{N}__{description}.sql` | DDL |
| V{N}+1 | `V{N+1}__seed_{module}.sql` | Seed data inicial |

## Reglas de Negocio

1. {Rule 1}

## DTOs

### Request (SIN IDs)
### Response (CON IDs)

## Esquema de Clases (Domain)

### Backend

```
{módulo}/
├── domain/
├── application/
│   ├── usecases/
│   ├── dto/
│   │   ├── request/
│   │   └── response/
│   └── mappers/
└── infrastructure/
    ├── web/
    ├── persistence/
    └── config/
```

### Backend Tests

```
{módulo}/
├── domain/
│   └── {Entity}Test.java
├── application/
│   └── usecases/
│       └── ...UseCaseTest.java
└── infrastructure/
    ├── web/
    │   └── {Entity}ControllerTest.java
    └── persistence/
        └── {Entity}RepositoryAdapterTest.java
```

### Frontend

```
features/{module}/
├── pages/
│   ├── list/
│   │   ├── {entity}-list.component.ts
│   │   └── {entity}-list.component.spec.ts
│   ├── form/
│   │   ├── {entity}-form.component.ts
│   │   └── {entity}-form.component.spec.ts
│   └── details/
│       ├── {entity}-details.component.ts
│       └── {entity}-details.component.spec.ts
├── components/
│   ├── ...-section/
│   │   ├── ...-section.component.ts
│   │   └── ...-section.component.spec.ts
├── services/
│   ├── api/
│   ├── mappers/
├── validators/
├── models/
└── {entity}s.routes.ts
```

## API Endpoints

## Tests

### Backend: JUnit 5 + Mockito + Testcontainers
### Frontend: Vitest + Testing Library, `.spec.ts` junto al source

## Casos de Prueba Críticos

1. {Test case 1}
2. {Test case 2}
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
   - Priority: high | Tags: ["parsun"] | Assignee: Gabriel
   - Description with summary and design reference
3. CREATE grouped subtasks (Orders-style):
   a. 📁 Database Seeder (priority: low)
      → Create Flyway migrations for {module} tables
      → Seed {module} with initial data
   b. 📁 Backend (priority: high)
      → Use cases, DTOs, mappers, controller
   c. 📁 Frontend (priority: high)
      → Routing, service, pages (list/form/detail), components
   d. 📁 Reports (priority: normal, optional)
      → PDF templates, download actions
4. No dependencies needed between groupers (trabajo independiente)
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
