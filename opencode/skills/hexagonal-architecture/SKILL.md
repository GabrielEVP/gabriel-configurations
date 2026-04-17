---
name: hexagonal-architecture
description: Hexagonal Architecture (Ports & Adapters) patterns for Spring Boot. Domain layer, Application layer, Infrastructure layer separation. Repository ports, use cases, adapters.
license: MIT
compatibility: opencode
---

# Hexagonal Architecture (Ports & Adapters)

## Overview

Hexagonal Architecture (also called Ports & Adapters) isolates the domain and application logic from external concerns (HTTP, databases, messaging). The core idea: your business logic has NO knowledge of how it's delivered or persisted.

**Analogy**: Think of your domain as a USB device. It doesn't care if it's plugged into a laptop, a desktop, or a power bank — it just works. The ports are the USB connectors; the adapters are the specific plugs.

---

## Layer Hierarchy

```
┌─────────────────────────────────────────────────────┐
│                   Infrastructure                     │
│  (HTTP Controllers, JPA Repositories, Config, etc.)  │
│                                                       │
│   ┌───────────────────────────────────────────────┐  │
│   │                 Application                    │  │
│   │    (Use Cases, Port Interfaces, DTOs)          │  │
│   │                                                │  │
│   │   ┌─────────────────────────────────────────┐ │  │
│   │   │               Domain                    │ │  │
│   │   │  (Entities, Value Objects, Aggregates,  │ │  │
│   │   │   Repository Interfaces, Domain Services)│ │  │
│   │   └─────────────────────────────────────────┘ │  │
│   └───────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
```

## Dependency Direction (CRITICAL)

Dependencies point **inward only**:
- `Infrastructure` → `Application` → `Domain`
- `Domain` imports NOTHING from Application or Infrastructure
- `Application` imports Domain only — never Spring, JPA, HTTP
- `Infrastructure` imports everything but is the only layer that can

---

## Domain Layer

### What goes here
- Aggregate Roots and Entities
- Value Objects
- Repository interfaces (Ports OUT)
- Domain Services (logic spanning multiple aggregates)
- Domain Events (when explicitly needed)
- Custom exceptions

### What NEVER goes here
- Spring annotations (no @Component, @Service, @Repository)
- JPA annotations (@Entity, @Table, @Column)
- HTTP anything
- Database-specific types

### Example: Repository Port
```java
// Domain defines WHAT it needs — not HOW it's implemented
public interface CustomerRepository {
    Customer save(Customer customer);
    Optional<Customer> findById(CustomerId id);
    Optional<Customer> findByEmail(Email email);
    List<Customer> findAll();
    boolean existsByEmail(Email email);
}
```

---

## Application Layer

### What goes here
- Use Cases (one public method: `execute()`)
- Input/Output DTOs (Java records)
- Port interfaces the application exposes (Ports IN, also called Use Case interfaces)
- Application-specific exceptions

### What NEVER goes here
- JPA entities
- HTTP request/response objects
- Spring annotations (except @Transactional, which is acceptable)

### Use Case Pattern
```java
public class RegisterCustomerUseCase {
    private final CustomerRepository customerRepository;  // Domain port

    // Constructor injection — always
    public RegisterCustomerUseCase(CustomerRepository customerRepository) {
        this.customerRepository = customerRepository;
    }

    @Transactional
    public CustomerResponse execute(RegisterCustomerCommand command) {
        if (customerRepository.existsByEmail(Email.of(command.email()))) {
            throw new DuplicateEmailException(command.email());
        }

        Customer customer = Customer.register(
            CustomerName.of(command.name()),
            Email.of(command.email())
        );

        Customer saved = customerRepository.save(customer);
        return CustomerResponse.from(saved);
    }
}
```

---

## Infrastructure Layer

### What goes here
- JPA Entities and Spring Data Repositories
- Repository Adapters (implement domain port interfaces)
- JPA Mappers (domain ↔ JPA entity conversion)
- REST Controllers
- Config classes (wire use cases with adapters)
- External service clients

### Repository Adapter Pattern
```java
@Repository
public class CustomerRepositoryAdapter implements CustomerRepository {
    private final CustomerJpaRepository jpaRepository;
    private final CustomerJpaMapper mapper;

    public CustomerRepositoryAdapter(CustomerJpaRepository jpaRepository,
                                      CustomerJpaMapper mapper) {
        this.jpaRepository = jpaRepository;
        this.mapper = mapper;
    }

    @Override
    public Customer save(Customer customer) {
        CustomerJpaEntity entity = mapper.toJpa(customer);
        return mapper.toDomain(jpaRepository.save(entity));
    }

    @Override
    public Optional<Customer> findById(CustomerId id) {
        return jpaRepository.findById(id.value()).map(mapper::toDomain);
    }

    @Override
    public Optional<Customer> findByEmail(Email email) {
        return jpaRepository.findByEmail(email.value()).map(mapper::toDomain);
    }
}
```

### Config Class Pattern
```java
@Configuration
public class CustomerConfig {
    @Bean
    public RegisterCustomerUseCase registerCustomerUseCase(CustomerRepository customerRepository) {
        return new RegisterCustomerUseCase(customerRepository);
    }

    @Bean
    public FindCustomerUseCase findCustomerUseCase(CustomerRepository customerRepository) {
        return new FindCustomerUseCase(customerRepository);
    }
}
```

---

## Benefits of This Architecture

1. **Testability**: Domain and Application layers test without Spring, JPA, or HTTP
2. **Replaceability**: Swap PostgreSQL for MongoDB by replacing the adapter only
3. **Independence**: Business rules survive framework upgrades
4. **Clarity**: Reading a Use Case tells you exactly what the business operation does

## Common Mistakes to Avoid

- Injecting JPA repositories directly into use cases (breaks the port pattern)
- Returning JPA entities from use cases (leaks infrastructure into application)
- Annotating domain entities with @Entity (couples domain to JPA)
- Using Spring's ApplicationContext in domain or application layers
