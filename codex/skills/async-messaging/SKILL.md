---
name: async-messaging
description: "Trigger: rabbitmq, kafka, sqs, message broker, queue, pub-sub, event bus, amqp, async messaging, event-driven. Async messaging — RabbitMQ, Kafka, SQS patterns, event-driven architecture, message routing, retries, dead-letter queues."
---

# Async Messaging & Event-Driven Architecture

## RabbitMQ

### Core Concepts
```
Exchange → Binding → Queue → Consumer

Exchange types:
- Direct:    route by exact routing key
- Topic:     route by pattern (orders.*, orders.created)
- Fanout:    broadcast to all bound queues
- Headers:   route by message header values
```

### Spring Boot Configuration
```yaml
spring:
  rabbitmq:
    host: localhost
    port: 5672
    username: guest
    password: guest
    listener:
      simple:
        retry:
          enabled: true
          max-attempts: 3
          initial-interval: 1000
          multiplier: 2
        default-requeue-rejected: false
```

```java
@Configuration
public class RabbitMQConfig {

    // Queue
    @Bean
    public Queue orderCreatedQueue() {
        return QueueBuilder.durable("order.created.queue")
            .deadLetterExchange("order.dlx")
            .deadLetterRoutingKey("order.dead")
            .build();
    }

    // Exchange
    @Bean
    public TopicExchange orderExchange() {
        return new TopicExchange("order.exchange");
    }

    // Binding
    @Bean
    public Binding orderCreatedBinding() {
        return BindingBuilder.bind(orderCreatedQueue())
            .to(orderExchange())
            .with("order.created");
    }

    // Dead Letter Queue
    @Bean
    public Queue deadLetterQueue() {
        return QueueBuilder.durable("order.dead.queue").build();
    }

    @Bean
    public DirectExchange deadLetterExchange() {
        return new DirectExchange("order.dlx");
    }

    @Bean
    public Binding deadLetterBinding() {
        return BindingBuilder.bind(deadLetterQueue())
            .to(deadLetterExchange())
            .with("order.dead");
    }
}
```

### Producer
```java
@Service
public class OrderEventPublisher {

    private final RabbitTemplate rabbitTemplate;

    public OrderEventPublisher(RabbitTemplate rabbitTemplate) {
        this.rabbitTemplate = rabbitTemplate;
    }

    public void orderCreated(Order order) {
        OrderCreatedEvent event = new OrderCreatedEvent(order.getId(), order.getCustomerId());
        rabbitTemplate.convertAndSend("order.exchange", "order.created", event);
    }
}
```

### Consumer
```java
@Component
public class OrderEventConsumer {

    @RabbitListener(queues = "order.created.queue")
    public void handleOrderCreated(OrderCreatedEvent event, Channel channel, @Header(AmqpHeaders.DELIVERY_TAG) long tag) {
        try {
            // Process event
            inventoryService.reserveStock(event.orderId());
            channel.basicAck(tag, false);
        } catch (Exception e) {
            // Reject and send to DLX after retries exhausted
            channel.basicNack(tag, false, false);
        }
    }
}
```

## Apache Kafka

### Core Concepts
```
Topic → Partition → Offset → Consumer Group

- Topic: category of messages
- Partition: ordered, immutable sequence
- Offset: position within a partition
- Consumer Group: load balancing across consumers
- Retention: messages persist (default 7 days) — replayable
```

### Spring Boot Configuration
```yaml
spring:
  kafka:
    bootstrap-servers: localhost:9092
    producer:
      key-serializer: org.apache.kafka.common.serialization.StringSerializer
      value-serializer: org.springframework.kafka.support.serializer.JsonSerializer
      properties:
        enable.idempotence: true
        acks: all
    consumer:
      group-id: order-service
      auto-offset-reset: earliest
      key-deserializer: org.apache.kafka.common.serialization.StringDeserializer
      value-deserializer: org.springframework.kafka.support.serializer.JsonDeserializer
      properties:
        spring.json.trusted.packages: "*"
```

### Producer
```java
@Service
public class OrderEventProducer {

    private static final String TOPIC = "order.events";

    private final KafkaTemplate<String, Object> kafka;

    public OrderEventProducer(KafkaTemplate<String, Object> kafka) {
        this.kafka = kafka;
    }

    public void orderCreated(Order order) {
        OrderCreatedEvent event = new OrderCreatedEvent(order.getId(), order.getCustomerId());
        // Key ensures same orderId goes to same partition (ordering)
        kafka.send(TOPIC, order.getId().toString(), event);
    }
}
```

### Consumer
```java
@Component
public class OrderEventConsumer {

    @KafkaListener(topics = "order.events", groupId = "order-service")
    public void handleOrderCreated(OrderCreatedEvent event) {
        inventoryService.reserveStock(event.orderId());
    }

    @KafkaListener(topics = "order.events", groupId = "notification-service")
    public void handleForNotifications(OrderCreatedEvent event) {
        emailService.sendConfirmation(event.orderId());
    }
}

// Retry configuration
@Configuration
public class KafkaRetryConfig {

    @Bean
    public ConcurrentKafkaListenerContainerFactory<String, Object> retryFactory(
            ConsumerFactory<String, Object> consumerFactory) {
        var factory = new ConcurrentKafkaListenerContainerFactory<String, Object>();
        factory.setConsumerFactory(consumerFactory);
        factory.setCommonErrorHandler(new DefaultErrorHandler(
            new FixedBackOff(1000L, 3)     // 3 retries, 1s apart
        ));
        return factory;
    }
}
```

## AWS SQS

```yaml
spring:
  cloud:
    aws:
      region:
        static: us-east-1
      credentials:
        access-key: ${AWS_ACCESS_KEY_ID}
        secret-key: ${AWS_SECRET_ACCESS_KEY}
    function:
      definition: orderCreatedHandler
    stream:
      bindings:
        orderCreatedHandler-in-0:
          destination: order-queue
        orderCreated-out-0:
          destination: order-queue
```

```java
// Using Spring Cloud AWS
@Service
public class OrderQueueService {

    private final SqsTemplate sqsTemplate;

    public OrderQueueService(SqsTemplate sqsTemplate) {
        this.sqsTemplate = sqsTemplate;
    }

    public void send(OrderCreatedEvent event) {
        sqsTemplate.send("order-queue", event);
    }
}

// Polling consumer
@Component
public class OrderQueueConsumer {

    @SqsListener(value = "order-queue", deletionPolicy = SqsMessageDeletionPolicy.ON_SUCCESS)
    public void handle(OrderCreatedEvent event) {
        inventoryService.reserveStock(event.orderId());
    }
}
```

## Event-Driven Patterns

### Event Sourcing
```
Store sequence of events instead of current state.
Current state = replay all events.

Pros: full audit trail, temporal queries, rebuild any state
Cons: complexity, event schema evolution, querying current state requires snapshot
```

### CQRS (Command Query Responsibility Segregation)
```
Commands (writes) → separate model
Queries (reads) → separate model (can be denormalized)

Event Store ──→ Event Handler ──→ Read Model (materialized view)
      │
      └──→ Command Handler ──→ Write Model (aggregates)
```

### Outbox Pattern
```
Atomic write to DB + outbox table, then publish from outbox.

1. Service writes Order + OrderOutboxEvent in same DB transaction
2. Outbox publisher polls outbox table
3. Publishes to message broker
4. Marks outbox event as sent

Prevents: message sent but DB transaction rolled back
```

```java
// Outbox event
@Entity
@Table(name = "outbox_events")
public class OutboxEvent {
    @Id private UUID id;
    private String aggregateType;      // "order"
    private String aggregateId;        // order UUID
    private String eventType;          // "OrderCreated"
    private String payload;            // JSON
    private Instant createdAt;
    private Instant sentAt;            // null until sent
}
```

### Idempotent Consumer
```java
@Component
public class IdempotentConsumer {

    private final ProcessedEventRepository processedRepo;

    @RabbitListener(queues = "order.queue")
    public void handle(OrderCreatedEvent event, @Header("messageId") String messageId) {
        // Check if already processed
        if (processedRepo.existsById(messageId)) {
            return;  // Already processed — skip
        }

        // Process
        inventoryService.reserveStock(event.orderId());

        // Mark as processed
        processedRepo.save(new ProcessedEvent(messageId, Instant.now()));
    }
}
```

## Retry & Dead Letter Patterns

### Retry Strategies
```
Immediate retry:      for transient network errors (3 attempts)
Exponential backoff:  for rate limiting (1s, 2s, 4s, 8s...)
Circuit breaker:      for downstream failures (stop after N failures, retry after delay)
Dead letter queue:    after all retries exhausted
```

```yaml
# RabbitMQ retry with DLQ
spring:
  rabbitmq:
    listener:
      simple:
        retry:
          enabled: true
          max-attempts: 3
          initial-interval: 1000
          multiplier: 2.0
          max-interval: 10000
        default-requeue-rejected: false  # send to DLX after retries
```

### Message Structure
```json
{
  "eventId": "uuid",
  "eventType": "OrderCreated",
  "version": 1,
  "timestamp": "2024-06-15T10:30:00Z",
  "source": "order-service",
  "data": {
    "orderId": "uuid",
    "customerId": "uuid"
  },
  "metadata": {
    "correlationId": "uuid",
    "causationId": "prev-event-id"
  }
}
```

## Which Broker to Use

| Need | Choose |
|------|--------|
| Exactly-once delivery, ordering per key | Kafka |
| Complex routing (topic/direct/fanout) | RabbitMQ |
| Simple queue, minimal ops | SQS |
| Stream processing, replay | Kafka |
| Low latency, high throughput | Kafka |
| Dead simple, fully managed | SQS |
| Need DLQ out of the box | RabbitMQ / SQS |
