---
name: api-design
description: "Trigger: api, rest, openapi, swagger, endpoint, controller, http, api versioning, api contract. REST API design — OpenAPI/Swagger, versioning, status codes, request/response patterns, error handling, pagination, HATEOAS, API contracts."
---

# REST API Design

## URL Conventions

```
# Base URL pattern
/api/v1/{resource}
/api/v1/{resource}/{id}
/api/v1/{resource}/{id}/{subresource}

# Examples
GET    /api/v1/orders              # list
POST   /api/v1/orders              # create
GET    /api/v1/orders/{id}          # read
PUT    /api/v1/orders/{id}          # full update
PATCH  /api/v1/orders/{id}          # partial update
DELETE /api/v1/orders/{id}          # delete

# Sub-resources
GET    /api/v1/orders/{id}/lines
POST   /api/v1/orders/{id}/lines

# Actions (when not CRUD)
POST   /api/v1/orders/{id}/cancel
POST   /api/v1/orders/{id}/refund
```

### Naming Rules
- **Plural nouns**: `/orders`, `/customers`, NOT `/order`, `/getOrders`
- **Kebab-case**: `/order-lines`, NOT `/orderLines`, `/order_lines`
- **Lowercase** always
- **No verbs in URL** for CRUD — verbs are HTTP methods
- **Version prefix**: `/api/v1/`, `/api/v2/`
- **Query params for filtering/sorting/pagination**

## HTTP Status Codes

| Code | When to use |
|------|-------------|
| **200** | GET/PUT/PATCH success |
| **201** | POST created |
| **204** | DELETE success, no body |
| **301** | Resource moved permanently |
| **302** | Temporary redirect |
| **304** | Not modified (caching) |
| **400** | Bad request (validation error) |
| **401** | Unauthenticated (no token) |
| **403** | Forbidden (valid token, no permission) |
| **404** | Resource not found |
| **409** | Conflict (duplicate, state conflict) |
| **422** | Unprocessable entity (validation of business rules) |
| **429** | Rate limited |
| **500** | Internal server error (unexpected) |

## Standard Response Format

### Success
```json
{
  "data": { ... },
  "message": "Order created successfully"
}
```

### List with pagination
```json
{
  "data": [ ... ],
  "pagination": {
    "page": 1,
    "pageSize": 20,
    "totalItems": 156,
    "totalPages": 8,
    "hasNext": true,
    "hasPrev": false
  }
}
```

### Error
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid request data",
    "details": [
      {
        "field": "email",
        "message": "Email format is invalid",
        "code": "INVALID_FORMAT"
      }
    ],
    "traceId": "abc-123-def"
  }
}
```

## Pagination

```java
// Request: GET /api/v1/orders?page=1&pageSize=20&sort=createdAt,desc

public record PageRequest(int page, int pageSize, String sort) {
    public PageRequest {
        page = Math.max(0, page);
        pageSize = Math.min(100, Math.max(1, pageSize));
    }
}

public record PageResponse<T>(
    List<T> data,
    long totalItems,
    int page,
    int pageSize,
    int totalPages,
    boolean hasNext,
    boolean hasPrev
) {
    public static <T> PageResponse<T> from(Page<T> springPage) {
        return new PageResponse<>(
            springPage.getContent(),
            springPage.getTotalElements(),
            springPage.getNumber(),
            springPage.getSize(),
            springPage.getTotalPages(),
            springPage.hasNext(),
            springPage.hasPrevious()
        );
    }
}
```

### Cursor-based Pagination (for large datasets)
```java
// Request: GET /api/v1/orders?cursor=2024-06-15T10:30:00Z&limit=20

public record CursorResponse<T>(
    List<T> data,
    String nextCursor,
    boolean hasMore
) {}
```

## API Versioning

### URI Path (recommended for breaking changes)
```
/api/v1/orders     # old
/api/v2/orders     # new — breaking changes
```

### When to version
- Breaking changes to response format
- Removing fields
- Changing required fields
- Changing endpoint behavior

### When NOT to version
- Adding new fields (clients should ignore unknown fields)
- Adding new endpoints
- Bug fixes that don't change the contract

## Filtering, Sorting & Searching

```
GET /api/v1/orders?status=PENDING&customerId=123
GET /api/v1/orders?createdAtFrom=2024-01-01&createdAtTo=2024-06-30
GET /api/v1/orders?search=john+doe
GET /api/v1/orders?sort=createdAt,desc&sort=customerName,asc
GET /api/v1/orders?fields=id,status,totalAmount
```

### Spring Boot implementation
```java
@GetMapping
public PageResponse<OrderResponse> findAll(
    @RequestParam(defaultValue = "0") int page,
    @RequestParam(defaultValue = "20") int pageSize,
    @RequestParam(defaultValue = "createdAt,desc") String[] sort,
    @RequestParam(required = false) String status,
    @RequestParam(required = false) String search
) {
    // ...
}
```

## Input Validation

```java
// Request validation with Jakarta
public record CreateOrderRequest(
    @NotNull UUID customerId,
    @NotEmpty @Valid List<CreateOrderLineRequest> lines
) {
    public record CreateOrderLineRequest(
        @NotNull UUID productId,
        @Min(1) @Max(999) int quantity
    ) {}
}
```

## Idempotency

```java
// POST /api/v1/orders
// Header: Idempotency-Key: <uuid>

// Implementation:
// 1. Check if key was processed → return 200 with cached response
// 2. If not → process and cache response
// 3. On network error → client retries with same key
```

## Rate Limiting

```java
// Response headers
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 87
X-RateLimit-Reset: 1718500000

// 429 Too Many Requests response
{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Too many requests. Try again in 30 seconds."
  }
}
```

## OpenAPI / Swagger

```java
@Tag(name = "Orders", description = "Order management endpoints")
@RestController
@RequestMapping("/api/v1/orders")
public class OrderController {

    @Operation(summary = "Create a new order")
    @ApiResponse(responseCode = "201", description = "Order created")
    @ApiResponse(responseCode = "422", description = "Validation error")
    @PostMapping
    @ResponseStatus(HttpStatus.CREATED)
    public OrderResponse create(@Valid @RequestBody CreateOrderRequest request) {
        return createOrderUseCase.execute(request.toCommand());
    }

    @Operation(summary = "List orders with pagination")
    @GetMapping
    public PageResponse<OrderResponse> findAll(
        @Parameter(description = "Page number (0-based)")
        @RequestParam(defaultValue = "0") int page,
        @Parameter(description = "Items per page (max 100)")
        @RequestParam(defaultValue = "20") int pageSize
    ) {
        // ...
    }
}
```

## HATEOAS (Hypermedia)

```json
{
  "data": {
    "id": 1,
    "status": "PENDING",
    "_links": {
      "self": { "href": "/api/v1/orders/1" },
      "cancel": { "href": "/api/v1/orders/1/cancel", "method": "POST" },
      "customer": { "href": "/api/v1/customers/5" }
    }
  }
}
```

## Security Headers

```java
// Every response should include:
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 0
Strict-Transport-Security: max-age=31536000; includeSubDomains
Cache-Control: no-store
Content-Security-Policy: default-src 'self'
```

## Common Anti-Patterns (What NOT to Do)

| Anti-pattern | Instead |
|-------------|---------|
| `GET /getOrders` | `GET /api/v1/orders` |
| `POST /updateOrder` | `PUT /api/v1/orders/{id}` |
| `DELETE /deleteOrder?id=1` | `DELETE /api/v1/orders/1` |
| Returning 200 with error in body | Use correct status code (4xx) |
| Returning stack traces to client | Return user-friendly error, log the trace |
| `/api/v1/getAllOrdersAndCustomers` | Separate endpoints |
| Mixing camelCase and snake_case in responses | Pick one (camelCase for JSON) |
| No pagination on list endpoints | Always paginate list endpoints |
