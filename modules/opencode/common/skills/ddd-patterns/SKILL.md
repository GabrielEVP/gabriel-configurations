---
name: ddd-patterns
description: Domain-Driven Design tactical patterns — Aggregates, Entities, Value Objects, Domain Services, Domain Events, Repositories. Java implementation examples.
license: MIT
compatibility: opencode
---

# Domain-Driven Design — Tactical Patterns

## Overview

DDD tactical patterns give you the building blocks for a rich domain model. These patterns solve the problem of anemic domain models where business logic leaks into services and controllers.

**Analogy**: Think of your domain as a real business. An `Order` (Aggregate) knows its own rules, like "you can't cancel a shipped order." It doesn't need a manager (service) to tell it that.

---

## Aggregate Root

An Aggregate Root is the entry point to a cluster of domain objects that must be consistent together. All modifications go through the root.

```java
public class Order {
    private final OrderId id;
    private CustomerId customerId;
    private OrderStatus status;
    private final List<OrderLine> lines;
    private Instant createdAt;
    private Instant updatedAt;

    private Order(OrderId id, CustomerId customerId, List<OrderLine> lines) {
        this.id = id;
        this.customerId = customerId;
        this.status = OrderStatus.PENDING;
        this.lines = new ArrayList<>(lines);
        this.createdAt = Instant.now();
        this.updatedAt = Instant.now();
    }

    // Factory method — controls creation invariants
    public static Order create(CustomerId customerId, List<OrderLine> lines) {
        Objects.requireNonNull(customerId, "CustomerId is required");
        if (lines == null || lines.isEmpty()) {
            throw new IllegalArgumentException("Order must have at least one line");
        }
        return new Order(OrderId.generate(), customerId, lines);
    }

    // Business methods — enforce invariants
    public void confirm() {
        if (status != OrderStatus.PENDING) {
            throw new OrderStateException("Only PENDING orders can be confirmed, current: " + status);
        }
        this.status = OrderStatus.CONFIRMED;
        this.updatedAt = Instant.now();
    }

    public void cancel(CancellationReason reason) {
        if (status == OrderStatus.SHIPPED || status == OrderStatus.DELIVERED) {
            throw new OrderStateException("Cannot cancel order in status: " + status);
        }
        this.status = OrderStatus.CANCELLED;
        this.updatedAt = Instant.now();
    }

    public Money totalAmount() {
        return lines.stream()
            .map(OrderLine::subtotal)
            .reduce(Money.ZERO, Money::add);
    }

    // Read-only access to collection
    public List<OrderLine> getLines() {
        return Collections.unmodifiableList(lines);
    }

    // Getters — no setters on aggregate root
    public OrderId getId() { return id; }
    public CustomerId getCustomerId() { return customerId; }
    public OrderStatus getStatus() { return status; }
    public Instant getCreatedAt() { return createdAt; }
}
```

### Rules for Aggregate Roots
- One repository per aggregate root
- Only reference other aggregates by ID (not by object reference)
- Keep aggregates small — don't grow them endlessly
- All business invariants must hold within a single aggregate

---

## Value Objects

Value Objects are immutable objects defined by their attributes, not by identity. Two Money objects with the same amount and currency are equal.

```java
// Using Java records for simple value objects
public record Money(BigDecimal amount, Currency currency) {
    public static final Money ZERO = new Money(BigDecimal.ZERO, Currency.getInstance("USD"));

    public Money {
        Objects.requireNonNull(amount, "Amount cannot be null");
        Objects.requireNonNull(currency, "Currency cannot be null");
        if (amount.scale() > 4) {
            throw new IllegalArgumentException("Amount cannot have more than 4 decimal places");
        }
        if (amount.compareTo(BigDecimal.ZERO) < 0) {
            throw new IllegalArgumentException("Amount cannot be negative");
        }
        amount = amount.setScale(4, RoundingMode.HALF_UP);
    }

    public Money add(Money other) {
        requireSameCurrency(other);
        return new Money(this.amount.add(other.amount), this.currency);
    }

    public Money multiply(int factor) {
        return new Money(this.amount.multiply(BigDecimal.valueOf(factor)), this.currency);
    }

    public boolean isGreaterThan(Money other) {
        requireSameCurrency(other);
        return this.amount.compareTo(other.amount) > 0;
    }

    private void requireSameCurrency(Money other) {
        if (!this.currency.equals(other.currency)) {
            throw new IllegalArgumentException(
                "Cannot operate on different currencies: " + this.currency + " vs " + other.currency
            );
        }
    }
}

// ID value objects
public record CustomerId(UUID value) {
    public CustomerId {
        Objects.requireNonNull(value, "CustomerId cannot be null");
    }
    public static CustomerId of(UUID value) { return new CustomerId(value); }
    public static CustomerId of(String value) { return new CustomerId(UUID.fromString(value)); }
    public static CustomerId generate() { return new CustomerId(UUID.randomUUID()); }
}

// Domain-specific value objects with validation
public record Email(String value) {
    private static final Pattern EMAIL_PATTERN =
        Pattern.compile("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$");

    public Email {
        Objects.requireNonNull(value, "Email cannot be null");
        if (!EMAIL_PATTERN.matcher(value).matches()) {
            throw new IllegalArgumentException("Invalid email format: " + value);
        }
        value = value.toLowerCase().trim();
    }
}
```

---

## Entities

Entities have identity and lifecycle. Two customers with the same name but different IDs are different entities.

```java
public class OrderLine {  // Entity within Order aggregate
    private final OrderLineId id;
    private final ProductId productId;
    private int quantity;
    private Money unitPrice;

    public OrderLine(ProductId productId, int quantity, Money unitPrice) {
        this.id = OrderLineId.generate();
        this.productId = productId;
        setQuantity(quantity);
        this.unitPrice = unitPrice;
    }

    public void updateQuantity(int newQuantity) {
        setQuantity(newQuantity);
    }

    private void setQuantity(int quantity) {
        if (quantity <= 0) {
            throw new IllegalArgumentException("Quantity must be positive, got: " + quantity);
        }
        this.quantity = quantity;
    }

    public Money subtotal() {
        return unitPrice.multiply(quantity);
    }
}
```

---

## Repository Interface (Domain Port)

```java
// Lives in domain layer — no Spring, no JPA
public interface OrderRepository {
    Order save(Order order);
    Optional<Order> findById(OrderId id);
    List<Order> findAll();
    List<Order> findByCustomerId(CustomerId customerId);
    List<Order> findByStatus(OrderStatus status);
    void delete(OrderId id);
    boolean existsById(OrderId id);
}
```

---

## Domain Service

Use when logic naturally belongs to multiple aggregates or doesn't fit in one.

```java
// Domain service — no Spring annotations, injected via constructor
public class OrderPricingService {
    private final ProductRepository productRepository;
    private final DiscountRepository discountRepository;

    public OrderPricingService(ProductRepository productRepository,
                                DiscountRepository discountRepository) {
        this.productRepository = productRepository;
        this.discountRepository = discountRepository;
    }

    public PricedOrderLines calculatePrices(CustomerId customerId,
                                             List<OrderLineRequest> requestedLines) {
        Optional<Discount> discount = discountRepository.findActiveForCustomer(customerId);

        List<PricedLine> pricedLines = requestedLines.stream()
            .map(line -> {
                Product product = productRepository.findById(line.productId())
                    .orElseThrow(() -> new ProductNotFoundException(line.productId()));
                Money price = discount
                    .map(d -> d.apply(product.price()))
                    .orElse(product.price());
                return new PricedLine(product, line.quantity(), price);
            })
            .toList();

        return new PricedOrderLines(pricedLines);
    }
}
```

---

## Domain Events (Ask User Before Implementing)

```java
// Event — immutable record
public record OrderCreatedEvent(
    OrderId orderId,
    CustomerId customerId,
    Money totalAmount,
    Instant occurredAt
) implements DomainEvent {
    public OrderCreatedEvent(OrderId orderId, CustomerId customerId, Money totalAmount) {
        this(orderId, customerId, totalAmount, Instant.now());
    }
}

// Aggregate raises events
public class Order {
    private final List<DomainEvent> domainEvents = new ArrayList<>();

    public static Order create(CustomerId customerId, List<OrderLine> lines) {
        Order order = new Order(OrderId.generate(), customerId, lines);
        order.domainEvents.add(new OrderCreatedEvent(
            order.id, customerId, order.totalAmount()
        ));
        return order;
    }

    public List<DomainEvent> pullDomainEvents() {
        List<DomainEvent> events = new ArrayList<>(domainEvents);
        domainEvents.clear();
        return events;
    }
}
```

---

## When to Use Each Pattern

| Pattern | When to Use |
|---------|-------------|
| Aggregate Root | A cluster of objects with a transactional boundary |
| Value Object | Immutable concepts defined by attributes (money, email, address) |
| Entity | Objects with identity that change over time |
| Domain Service | Logic spanning multiple aggregates |
| Repository | Persistence abstraction for aggregate roots |
| Domain Event | Side effects that should happen AFTER a state change |
