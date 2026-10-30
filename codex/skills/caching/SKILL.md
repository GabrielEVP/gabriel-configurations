---
name: caching
description: "Trigger: cache, redis, caffeine, redis cache, caching strategy, cache invalidation, distributed cache, ttl, redis stack, cache pattern. Caching strategies — Redis, Caffeine, Spring Cache, cache-aside, write-through, TTL, invalidation, distributed caching, session store."
---

# Caching

## Core Concepts

### When to Cache
```
✅ Cache when:
- Data is read frequently, written rarely
- Data is expensive to compute or fetch
- Slight staleness is acceptable
- Data is the same for multiple users

❌ DON'T cache when:
- Data changes every request
- Stale data causes incorrect behavior
- Data is user-specific (unless partitioned)
- Cache hit rate would be low
```

### Cache-Aside Pattern
```
1. Read: check cache → miss → load from DB → store in cache → return
2. Write: write to DB → invalidate cache (not update!)
```

```java
public Order findById(OrderId id) {
    String key = "order:" + id.value();

    // 1. Try cache
    Order cached = cache.get(key);
    if (cached != null) return cached;

    // 2. Cache miss — load from DB
    Order order = repository.findById(id)
        .orElseThrow(() -> new OrderNotFoundException(id));

    // 3. Store in cache
    cache.put(key, order, Duration.ofMinutes(15));

    return order;
}

public void cancel(OrderId id) {
    repository.cancel(id);

    // Invalidate cache (don't update — let next read populate)
    cache.evict("order:" + id.value());
}
```

## Spring Cache with Caffeine

### Configuration
```java
@Configuration
@EnableCaching
public class CacheConfig {

    @Bean
    public CacheManager cacheManager() {
        var caffeine = Caffeine.newBuilder()
            .maximumSize(10_000)
            .expireAfterWrite(Duration.ofMinutes(15))
            .recordStats();

        var manager = new CaffeineCacheManager();
        manager.setCaffeine(caffeine);
        manager.setCacheNames(List.of("orders", "products", "customers"));
        return manager;
    }
}
```

```yaml
# Per-cache configuration
spring:
  cache:
    caffeine:
      spec: maximumSize=10000,expireAfterWrite=15m
    cache-names:
      - orders
      - products
      - customers
```

### Usage
```java
@Service
public class OrderService {

    @Cacheable(value = "orders", key = "#id", unless = "#result == null")
    public Order findById(UUID id) {
        return repository.findById(id).orElse(null);
    }

    @Cacheable(value = "orders", key = "'all'", condition = "#page == 0")
    public List<Order> findAll(int page, int size) {
        return repository.findAll();
    }

    @CacheEvict(value = "orders", key = "#orderId")
    public void cancel(UUID orderId) {
        repository.cancel(orderId);
    }

    @CacheEvict(value = "orders", allEntries = true)
    public void clearCache() {
        // admin operation
    }

    @CachePut(value = "orders", key = "#order.id")
    public Order update(Order order) {
        return repository.save(order);
    }

    @Caching(evict = {
        @CacheEvict(value = "orders", key = "#id"),
        @CacheEvict(value = "order-summaries", key = "#id")
    })
    public void delete(UUID id) {
        repository.deleteById(id);
    }
}
```

## Redis

### Spring Boot Configuration
```yaml
spring:
  data:
    redis:
      host: localhost
      port: 6379
      timeout: 2000ms
      lettuce:
        pool:
          max-active: 16
          max-idle: 8
          min-idle: 4

  cache:
    type: redis
    redis:
      time-to-live: 15m
      cache-null-values: false
      use-key-prefix: true
      key-prefix: "myapp:"
```

```java
@Configuration
public class RedisConfig {

    @Bean
    public RedisTemplate<String, Object> redisTemplate(RedisConnectionFactory factory) {
        var template = new RedisTemplate<String, Object>();
        template.setConnectionFactory(factory);
        template.setKeySerializer(new StringRedisSerializer());
        template.setValueSerializer(new Jackson2JsonRedisSerializer<>(Object.class));
        template.setHashKeySerializer(new StringRedisSerializer());
        template.setHashValueSerializer(new Jackson2JsonRedisSerializer<>(Object.class));
        return template;
    }

    @Bean
    public CacheManager cacheManager(RedisConnectionFactory factory) {
        var config = RedisCacheConfiguration.defaultCacheConfig()
            .entryTtl(Duration.ofMinutes(15))
            .prefixName("myapp:")
            .serializeKeysWith(
                RedisSerializationContext.SerializationPair.fromSerializer(new StringRedisSerializer()))
            .serializeValuesWith(
                RedisSerializationContext.SerializationPair.fromSerializer(
                    new Jackson2JsonRedisSerializer<>(Object.class)));

        return RedisCacheManager.builder(factory)
            .cacheDefaults(config)
            .withCacheConfiguration("orders", config.entryTtl(Duration.ofMinutes(5)))
            .withCacheConfiguration("products", config.entryTtl(Duration.ofHours(1)))
            .build();
    }
}
```

### Redis Template Usage
```java
@Service
public class OrderCacheService {

    private static final Duration TTL = Duration.ofMinutes(15);

    private final RedisTemplate<String, Object> redis;

    public OrderCacheService(RedisTemplate<String, Object> redis) {
        this.redis = redis;
    }

    public Optional<Order> findById(UUID id) {
        String key = "order:" + id;
        Order cached = (Order) redis.opsForValue().get(key);
        return Optional.ofNullable(cached);
    }

    public void put(Order order) {
        String key = "order:" + order.getId();
        redis.opsForValue().set(key, order, TTL);
    }

    public void evict(UUID id) {
        redis.delete("order:" + id);
    }

    // Batch operations
    public List<Order> findAll(List<UUID> ids) {
        List<String> keys = ids.stream().map(id -> "order:" + id).toList();
        List<Object> values = redis.opsForValue().multiGet(keys);
        return values.stream()
            .filter(Objects::nonNull)
            .map(v -> (Order) v)
            .toList();
    }
}
```

### Redis Other Uses

#### Rate Limiting
```java
public boolean isRateLimited(String userId, int maxRequests, Duration window) {
    String key = "ratelimit:" + userId;
    Long count = redis.opsForValue().increment(key);
    if (count == 1) {
        redis.expire(key, window);
    }
    return count > maxRequests;
}
```

#### Distributed Locks
```java
public boolean tryLock(String key, Duration timeout) {
    return Boolean.TRUE.equals(
        redis.opsForValue().setIfAbsent("lock:" + key, "locked", timeout)
    );
}

public void unlock(String key) {
    redis.delete("lock:" + key);
}
```

#### Session Store
```yaml
spring:
  session:
    store-type: redis
    timeout: 30m
  redis:
    namespace: myapp:sessions
```

## Caffeine (Local Cache)

```java
@Configuration
public class CaffeineConfig {

    @Bean
    public Cache<UUID, Order> orderCache() {
        return Caffeine.newBuilder()
            .maximumSize(10_000)
            .expireAfterWrite(Duration.ofMinutes(15))
            .expireAfterAccess(Duration.ofHours(1))
            .refreshAfterWrite(Duration.ofMinutes(5))    // async refresh
            .recordStats()
            .build(key -> loadFromDb(key));              // auto-load
    }

    private Order loadFromDb(UUID id) {
        return repository.findById(id).orElse(null);
    }
}
```

### Cache Metrics
```java
// Get statistics
CacheStats stats = orderCache.stats();
double hitRate = stats.hitRate();
double missRate = stats.missRate();
long evictionCount = stats.evictionCount();
```

## Invalidation Strategies

| Strategy | How | When |
|----------|-----|------|
| **TTL** | `expireAfterWrite(15m)` | Simple, works for most cases |
| **Write-through** | Update cache on write | Strong consistency, higher write latency |
| **Write-behind** | Async cache update | Fast writes, eventual consistency |
| **Manual eviction** | Delete cache key on write | Full control, error-prone |
| **Event-driven** | Invalidate via event/message | Distributed, decoupled |

### TTL Guidelines
```
Session data:       15-30 minutes
Product catalog:    1 hour (or until manually invalidated)
User profiles:      5-10 minutes
Configuration:      5 minutes
Reference data:     1-24 hours (currencies, countries)
Aggregated stats:   5-15 minutes
Search results:     Don't cache (or 1 minute max)
```

## Cache Sizing

```java
// Estimate size
// 10,000 orders × ~2KB each = ~20MB

// Caffeine: fixed max
Caffeine.newBuilder()
    .maximumSize(10_000)
    .maximumWeight(50_000_000)  // 50MB
    .weigher((UUID key, Order value) -> value.serializedSize())
    .build();

// Redis: maxmemory policy
// In redis.conf:
// maxmemory 512mb
// maxmemory-policy allkeys-lru
```

## Eviction Policies

| Policy | Description | Use when |
|--------|-------------|----------|
| **LRU** | Least Recently Used | General purpose |
| **LFU** | Least Frequently Used | Some items much more popular |
| **TTL** | Time-based expiry | Data has natural expiry |
| **FIFO** | First In, First Out | Stream/buffer data |

## Anti-Patterns

```
❌ Cache as primary data store (data lives in DB, cache is temporary)
❌ Updating cache instead of invalidating (race conditions)
❌ Caching user-specific data without partitioning (memory waste)
❌ Too long TTL (stale data)
❌ Too short TTL (cache useless)
❌ Caching everything (diminishing returns)
❌ No cache warming after deploy (cold start → all requests hit DB)
❌ Distributed cache for single-instance apps (unnecessary complexity)
```

## Cache Warming

```java
@Component
public class CacheWarmer implements CommandLineRunner {

    private final OrderRepository repository;
    private final CacheManager cacheManager;

    public CacheWarmer(OrderRepository repository, CacheManager cacheManager) {
        this.repository = repository;
        this.cacheManager = cacheManager;
    }

    @Override
    public void run(String... args) {
        // Warm top 100 recent orders
        List<Order> recent = repository.findTop100ByOrderByCreatedAtDesc();
        Cache cache = cacheManager.getCache("orders");
        recent.forEach(order -> cache.put(order.getId(), order));
        log.info("Cache warmed with {} orders", recent.size());
    }
}
```

## Performance Monitoring

```yaml
# Spring Boot Actuator cache metrics
management:
  endpoints:
    web:
      exposure:
        include: caches,metrics
  metrics:
    cache:
      enabled: true

# Prometheus (if micrometer-registry-prometheus on classpath)
# GET /actuator/metrics/cache.gets
# GET /actuator/prometheus → look for cache_gets_total, cache_hits_total
```
