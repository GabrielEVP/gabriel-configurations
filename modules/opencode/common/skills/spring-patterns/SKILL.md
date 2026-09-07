---
name: spring-patterns
description: Spring Boot 3.5.5 patterns — Constructor injection, @Transactional use cases, Jakarta validation, @EntityGraph N+1 prevention, Flyway migrations, configuration classes.
license: MIT
compatibility: opencode
---

# Spring Boot 3.5.5 Patterns

## Constructor Injection (ALWAYS)

```java
// CORRECT
@Service
public class CustomerService {
    private final CustomerRepository customerRepository;
    private final EmailService emailService;

    public CustomerService(CustomerRepository customerRepository,
                           EmailService emailService) {
        this.customerRepository = customerRepository;
        this.emailService = emailService;
    }
}

// FORBIDDEN
@Service
public class CustomerService {
    @Autowired  // NO — field injection is forbidden
    private CustomerRepository customerRepository;
}
```

**Why**: Field injection hides dependencies, makes testing harder, and can cause null pointer exceptions at runtime.

---

## @Transactional on Use Cases

```java
public class CreateOrderUseCase {
    private final OrderRepository orderRepository;
    private final InventoryService inventoryService;

    public CreateOrderUseCase(OrderRepository orderRepository,
                               InventoryService inventoryService) {
        this.orderRepository = orderRepository;
        this.inventoryService = inventoryService;
    }

    @Transactional  // Applied at the use case level, not the repository
    public OrderResponse execute(CreateOrderCommand command) {
        // Both operations are in the same transaction
        inventoryService.reserveStock(command.lines());
        Order order = Order.create(command.customerId(), command.lines());
        return OrderResponse.from(orderRepository.save(order));
    }
}
```

### @Transactional Rules
- Apply on use case `execute()` methods when multiple writes are involved
- Use `@Transactional(readOnly = true)` for query-only use cases (optimization)
- Never apply on domain objects or repository interfaces
- Understand that `@Transactional` only works on Spring-managed beans (proxy limitation)

---

## Jakarta Validation

```java
// Request DTO with validation
public record CreateProductRequest(
    @NotBlank(message = "Product name is required")
    @Size(min = 2, max = 200, message = "Name must be between 2 and 200 characters")
    String name,

    @NotBlank(message = "SKU is required")
    @Pattern(regexp = "^[A-Z]{2,4}-[0-9]{4,8}$", message = "SKU must match format: XX-0000")
    String sku,

    @NotNull(message = "Price is required")
    @DecimalMin(value = "0.01", message = "Price must be greater than zero")
    @Digits(integer = 10, fraction = 4, message = "Price cannot exceed 10 integer digits and 4 decimal places")
    BigDecimal price,

    @NotNull(message = "Category is required")
    UUID categoryId,

    @Valid  // Triggers validation of nested object
    @NotNull
    StockRequest stock
) {
    public record StockRequest(
        @Min(value = 0, message = "Initial stock cannot be negative")
        int initialQuantity,

        @Min(value = 0, message = "Minimum stock cannot be negative")
        int minimumQuantity
    ) {}
}

// Controller uses @Valid
@PostMapping
@ResponseStatus(HttpStatus.CREATED)
public ProductResponse create(@Valid @RequestBody CreateProductRequest request) {
    return createProductUseCase.execute(request.toCommand());
}
```

### Custom Validators
```java
@Constraint(validatedBy = UniqueEmailValidator.class)
@Target({ElementType.FIELD})
@Retention(RetentionPolicy.RUNTIME)
public @interface UniqueEmail {
    String message() default "Email already registered";
    Class<?>[] groups() default {};
    Class<? extends Payload>[] payload() default {};
}

@Component
public class UniqueEmailValidator implements ConstraintValidator<UniqueEmail, String> {
    private final CustomerRepository customerRepository;

    public UniqueEmailValidator(CustomerRepository customerRepository) {
        this.customerRepository = customerRepository;
    }

    @Override
    public boolean isValid(String email, ConstraintValidatorContext context) {
        if (email == null) return true;  // Let @NotNull handle null
        return !customerRepository.existsByEmail(Email.of(email));
    }
}
```

---

## @EntityGraph — N+1 Prevention

```java
public interface OrderJpaRepository extends JpaRepository<OrderJpaEntity, UUID> {

    // Fetch order with all its lines in a single JOIN query
    @EntityGraph(attributePaths = {"lines"})
    Optional<OrderJpaEntity> findWithLinesById(UUID id);

    // Fetch order with lines and each line's product details
    @EntityGraph(attributePaths = {"lines", "lines.product", "lines.product.category"})
    Optional<OrderJpaEntity> findWithFullDetailsById(UUID id);

    // For lists: use projection or explicit JOIN FETCH to avoid cartesian product
    @Query("""
        SELECT DISTINCT o FROM OrderJpaEntity o
        JOIN FETCH o.lines
        WHERE o.customerId = :customerId
        ORDER BY o.createdAt DESC
        """)
    List<OrderJpaEntity> findByCustomerIdWithLines(@Param("customerId") UUID customerId);
}
```

### When to Use What
| Scenario | Approach |
|----------|----------|
| Single entity with nested collections | `@EntityGraph` |
| List with nested collections | `JOIN FETCH` + `DISTINCT` |
| List needing only summary fields | Projection interface |
| Counting or aggregation | `@Query` with JPQL |
| Complex filtering | `@Query` with JPQL or Specification |

---

## Flyway Migrations

### Naming Convention
```
V{major}.{minor}__{description}.sql
V1__create_customers_table.sql
V2__create_products_table.sql
V2.1__add_product_sku_column.sql
V3__create_orders_table.sql
```

### Sample Migration
```sql
-- V1__create_customers_table.sql
create table customers (
    id          uuid        primary key,
    name        varchar(200) not null,
    email       varchar(320) not null,
    phone       varchar(50),
    status      varchar(50) not null default 'ACTIVE',
    created_at  timestamptz  not null,
    updated_at  timestamptz  not null
);

create unique index idx_customers_email on customers(email);
create index idx_customers_status on customers(status);
```

### CRITICAL Rules
1. **Never modify a committed migration** — checksums will fail for everyone
2. **Never delete a migration file** — Flyway will complain about missing files
3. **Use `V{version}__` prefix** — double underscore separates version from description
4. **Lowercase SQL** — more readable, works across case-sensitive systems
5. **Always include indexes** — don't leave foreign keys or search columns unindexed
6. **Test rollback** — always have a repair strategy

---

## Configuration Classes

```java
@Configuration
public class ApplicationConfig {

    // Wire use cases — don't use @Service on use case classes
    @Bean
    public CreateCustomerUseCase createCustomerUseCase(
            CustomerRepository customerRepository,
            PasswordEncoder passwordEncoder) {
        return new CreateCustomerUseCase(customerRepository, passwordEncoder);
    }

    @Bean
    public FindCustomerUseCase findCustomerUseCase(CustomerRepository customerRepository) {
        return new FindCustomerUseCase(customerRepository);
    }
}

// Module-specific config for complex modules
@Configuration
public class OrderConfig {

    @Bean
    public CreateOrderUseCase createOrderUseCase(
            OrderRepository orderRepository,
            CustomerRepository customerRepository,
            ProductRepository productRepository,
            OrderPricingService pricingService) {
        return new CreateOrderUseCase(
            orderRepository,
            customerRepository,
            productRepository,
            pricingService
        );
    }
}
```

---

## Exception Handling

```java
// Domain exceptions
public class CustomerNotFoundException extends RuntimeException {
    public CustomerNotFoundException(CustomerId id) {
        super("Customer not found with ID: " + id.value());
    }
}

public class DuplicateEmailException extends RuntimeException {
    public DuplicateEmailException(String email) {
        super("Email already registered: " + email);
    }
}

// Global handler
@RestControllerAdvice
public class GlobalExceptionHandler {

    @ExceptionHandler(CustomerNotFoundException.class)
    @ResponseStatus(HttpStatus.NOT_FOUND)
    public ErrorResponse handleNotFound(CustomerNotFoundException ex) {
        return new ErrorResponse("CUSTOMER_NOT_FOUND", ex.getMessage());
    }

    @ExceptionHandler(DuplicateEmailException.class)
    @ResponseStatus(HttpStatus.CONFLICT)
    public ErrorResponse handleDuplicate(DuplicateEmailException ex) {
        return new ErrorResponse("DUPLICATE_EMAIL", ex.getMessage());
    }

    @ExceptionHandler(MethodArgumentNotValidException.class)
    @ResponseStatus(HttpStatus.UNPROCESSABLE_ENTITY)
    public ErrorResponse handleValidation(MethodArgumentNotValidException ex) {
        String details = ex.getBindingResult().getFieldErrors().stream()
            .map(e -> e.getField() + ": " + e.getDefaultMessage())
            .sorted()
            .collect(Collectors.joining("; "));
        return new ErrorResponse("VALIDATION_ERROR", details);
    }

    @ExceptionHandler(Exception.class)
    @ResponseStatus(HttpStatus.INTERNAL_SERVER_ERROR)
    public ErrorResponse handleUnexpected(Exception ex) {
        log.error("Unexpected error", ex);
        return new ErrorResponse("INTERNAL_ERROR", "An unexpected error occurred");
    }
}

public record ErrorResponse(String code, String message) {}
```

---

## Application Properties Structure

```yaml
# application.yml
spring:
  application:
    name: parsun-api
  datasource:
    url: ${DB_URL:jdbc:postgresql://localhost:5432/parsundb}
    username: ${DB_USER:postgres}
    password: ${DB_PASSWORD:postgres}
  jpa:
    open-in-view: false  # ALWAYS disable this
    hibernate:
      ddl-auto: validate  # Flyway manages schema
    properties:
      hibernate:
        default_batch_fetch_size: 20
        format_sql: true
  flyway:
    enabled: true
    locations: classpath:db/migration

app:
  jwt:
    secret: ${JWT_SECRET}
  cors:
    allowed-origins: ${CORS_ORIGINS:http://localhost:4200}
```

**Critical settings:**
- `open-in-view: false` — prevents lazy loading issues in controllers
- `ddl-auto: validate` — Flyway owns the schema, Hibernate just validates
- `default_batch_fetch_size` — reduces N+1 for lazily loaded collections
