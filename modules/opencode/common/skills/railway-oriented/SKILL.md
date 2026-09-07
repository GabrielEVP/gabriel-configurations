---
name: railway-oriented
description: Railway-Oriented Programming for error handling. Result types, Either monad patterns, functional error handling without exceptions for expected failures.
license: MIT
compatibility: opencode
---

# Railway-Oriented Programming

## Overview

Railway-Oriented Programming (ROP) treats operations as railroad tracks with two paths: the happy path (success track) and the failure path (error track). Once you're on the failure track, subsequent operations are skipped — the error just flows through.

**Analogy**: Imagine a train on two tracks. The train starts on the success track. Every operation is a switch. If the switch detects a problem, it moves the train to the failure track. From that point, all remaining switches are bypassed — the train goes straight to the end of the failure track.

---

## When to Use ROP vs Exceptions

| Scenario | Use |
|----------|-----|
| Expected business failures (not found, validation, duplicates) | ROP / Result type |
| Unexpected failures (DB down, null pointer, network error) | Exceptions |
| Domain invariant violations | Exceptions (in domain constructors) |
| Use case business logic failures | ROP |

---

## Result Type in Java

```java
// Generic Result type
public sealed interface Result<T> permits Result.Success, Result.Failure {

    record Success<T>(T value) implements Result<T> {}
    record Failure<T>(AppError error) implements Result<T> {}

    static <T> Result<T> success(T value) {
        return new Success<>(value);
    }

    static <T> Result<T> failure(AppError error) {
        return new Failure<>(error);
    }

    static <T> Result<T> failure(String code, String message) {
        return new Failure<>(new AppError(code, message));
    }

    boolean isSuccess();
    boolean isFailure();

    default T getValue() {
        return switch (this) {
            case Success<T> s -> s.value();
            case Failure<T> f -> throw new IllegalStateException("Result is a failure: " + f.error());
        };
    }

    default AppError getError() {
        return switch (this) {
            case Success<T> s -> throw new IllegalStateException("Result is a success");
            case Failure<T> f -> f.error();
        };
    }

    default <U> Result<U> map(Function<T, U> mapper) {
        return switch (this) {
            case Success<T> s -> Result.success(mapper.apply(s.value()));
            case Failure<T> f -> Result.failure(f.error());
        };
    }

    default <U> Result<U> flatMap(Function<T, Result<U>> mapper) {
        return switch (this) {
            case Success<T> s -> mapper.apply(s.value());
            case Failure<T> f -> Result.failure(f.error());
        };
    }

    default Result<T> onSuccess(Consumer<T> action) {
        if (this instanceof Success<T> s) action.accept(s.value());
        return this;
    }

    default Result<T> onFailure(Consumer<AppError> action) {
        if (this instanceof Failure<T> f) action.accept(f.error());
        return this;
    }
}

public record AppError(String code, String message) {
    public static AppError notFound(String entity, Object id) {
        return new AppError(entity.toUpperCase() + "_NOT_FOUND",
            entity + " not found with ID: " + id);
    }
    public static AppError duplicate(String field) {
        return new AppError("DUPLICATE_" + field.toUpperCase(),
            field + " already exists");
    }
    public static AppError validation(String message) {
        return new AppError("VALIDATION_ERROR", message);
    }
}
```

---

## Use Case with ROP

```java
public class RegisterCustomerUseCase {
    private final CustomerRepository customerRepository;

    public RegisterCustomerUseCase(CustomerRepository customerRepository) {
        this.customerRepository = customerRepository;
    }

    @Transactional
    public Result<CustomerResponse> execute(RegisterCustomerCommand command) {
        return validateEmail(command.email())
            .flatMap(email -> checkEmailNotDuplicate(email))
            .flatMap(email -> createCustomer(command))
            .map(CustomerResponse::from);
    }

    private Result<String> validateEmail(String email) {
        if (email == null || !email.contains("@")) {
            return Result.failure(AppError.validation("Invalid email format"));
        }
        return Result.success(email.toLowerCase().trim());
    }

    private Result<String> checkEmailNotDuplicate(String email) {
        if (customerRepository.existsByEmail(Email.of(email))) {
            return Result.failure(AppError.duplicate("email"));
        }
        return Result.success(email);
    }

    private Result<Customer> createCustomer(RegisterCustomerCommand command) {
        try {
            Customer customer = Customer.register(
                CustomerName.of(command.name()),
                Email.of(command.email())
            );
            return Result.success(customerRepository.save(customer));
        } catch (IllegalArgumentException e) {
            return Result.failure(AppError.validation(e.getMessage()));
        }
    }
}
```

---

## Controller Handling Result

```java
@PostMapping
public ResponseEntity<CustomerResponse> register(
        @Valid @RequestBody RegisterCustomerRequest request) {

    Result<CustomerResponse> result = registerCustomerUseCase.execute(request.toCommand());

    return switch (result) {
        case Result.Success<CustomerResponse> s ->
            ResponseEntity.status(HttpStatus.CREATED).body(s.value());
        case Result.Failure<CustomerResponse> f ->
            mapErrorToResponse(f.error());
    };
}

private ResponseEntity<CustomerResponse> mapErrorToResponse(AppError error) {
    HttpStatus status = switch (error.code()) {
        case "CUSTOMER_NOT_FOUND" -> HttpStatus.NOT_FOUND;
        case "DUPLICATE_EMAIL" -> HttpStatus.CONFLICT;
        case "VALIDATION_ERROR" -> HttpStatus.UNPROCESSABLE_ENTITY;
        default -> HttpStatus.INTERNAL_SERVER_ERROR;
    };
    return ResponseEntity.status(status).build();
}
```

---

## ROP in Angular (TypeScript)

```typescript
// Result type for frontend
type Result<T, E = AppError> =
  | { ok: true; value: T }
  | { ok: false; error: E };

interface AppError {
  code: string;
  message: string;
}

function ok<T>(value: T): Result<T> {
  return { ok: true, value };
}

function err<T>(error: AppError): Result<T> {
  return { ok: false, error };
}

// Service method returning Result
async createOrder(command: CreateOrderCommand): Promise<Result<OrderResponse>> {
  try {
    const response = await firstValueFrom(
      this.http.post<OrderResponse>('/api/v1/orders', command)
    );
    return ok(response);
  } catch (error) {
    if (error instanceof HttpErrorResponse) {
      return err({ code: error.error.code, message: error.error.message });
    }
    return err({ code: 'NETWORK_ERROR', message: 'Network request failed' });
  }
}

// Component using Result
async submitOrder(): Promise<void> {
  const result = await this.orderService.createOrder(this.buildCommand());

  if (result.ok) {
    this.router.navigate(['/orders', result.value.id]);
  } else {
    this.errorMessage.set(result.error.message);
  }
}
```

---

## When NOT to Use ROP

- **Domain constructor validation** — throw exceptions for invariant violations (fail fast)
- **Infrastructure errors** — let DB exceptions propagate naturally; handle at the top
- **Programming errors** — null pointer, wrong types, etc. should always throw
- **Simple CRUD** — if there's no business validation to fail, ROP adds overhead without value

Use ROP for **expected business failures** that are part of the domain language.
