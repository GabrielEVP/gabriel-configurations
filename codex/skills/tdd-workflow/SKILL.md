---
name: tdd-workflow
description: Test-Driven Development workflow — Red/Green/Refactor cycle, unit tests for domain, integration tests for infrastructure, test structure, naming conventions, Mockito, Jest.
---

# TDD Workflow — Red / Green / Refactor

## The Cycle

```
1. RED    — Write a failing test for the behavior you want
2. GREEN  — Write the minimum code to make the test pass
3. REFACTOR — Clean up code without breaking the test
```

**Why this order matters**: Writing tests first forces you to think about the interface before the implementation. You design from the consumer's perspective.

---

## Backend TDD — Java / JUnit 5 / Mockito

### Test Structure by Layer

```
src/test/java/
└─ com/company/app/
   └─ orders/
      ├─ domain/
      │  ├─ OrderTest.java               // Pure unit — no Spring, no mocks needed
      │  ├─ OrderLineTest.java
      │  └─ MoneyTest.java
      ├─ application/
      │  ├─ CreateOrderUseCaseTest.java  // Unit — mock repositories
      │  ├─ CancelOrderUseCaseTest.java
      │  └─ FindOrderUseCaseTest.java
      └─ infrastructure/
         ├─ persistence/
         │  └─ OrderRepositoryAdapterTest.java  // @DataJpaTest
         └─ web/
            └─ OrderControllerTest.java         // @WebMvcTest
```

---

### Domain Layer Tests (Pure Unit — Fastest)

```java
class OrderTest {

    @Test
    void should_create_order_with_pending_status() {
        // Given
        CustomerId customerId = CustomerId.generate();
        List<OrderLine> lines = List.of(
            new OrderLine(ProductId.generate(), 2, Money.of(100, "USD"))
        );

        // When
        Order order = Order.create(customerId, lines);

        // Then
        assertThat(order.getId()).isNotNull();
        assertThat(order.getStatus()).isEqualTo(OrderStatus.PENDING);
        assertThat(order.getCustomerId()).isEqualTo(customerId);
        assertThat(order.getLines()).hasSize(1);
        assertThat(order.getCreatedAt()).isNotNull();
    }

    @Test
    void should_calculate_total_amount_correctly() {
        // Given
        List<OrderLine> lines = List.of(
            new OrderLine(ProductId.generate(), 2, Money.of(100, "USD")),  // 200
            new OrderLine(ProductId.generate(), 3, Money.of(50, "USD"))    // 150
        );
        Order order = Order.create(CustomerId.generate(), lines);

        // When
        Money total = order.totalAmount();

        // Then
        assertThat(total.amount()).isEqualByComparingTo(new BigDecimal("350.0000"));
    }

    @Test
    void should_cancel_pending_order() {
        // Given
        Order order = createTestOrder(OrderStatus.PENDING);

        // When
        order.cancel(CancellationReason.CUSTOMER_REQUEST);

        // Then
        assertThat(order.getStatus()).isEqualTo(OrderStatus.CANCELLED);
    }

    @Test
    void should_throw_when_cancelling_shipped_order() {
        // Given
        Order order = createTestOrder(OrderStatus.SHIPPED);

        // When / Then
        assertThatThrownBy(() -> order.cancel(CancellationReason.CUSTOMER_REQUEST))
            .isInstanceOf(OrderStateException.class)
            .hasMessageContaining("SHIPPED");
    }

    @Test
    void should_reject_order_creation_with_empty_lines() {
        assertThatThrownBy(() -> Order.create(CustomerId.generate(), List.of()))
            .isInstanceOf(IllegalArgumentException.class)
            .hasMessage("Order must have at least one line");
    }

    // Helper — reduces boilerplate
    private Order createTestOrder(OrderStatus status) {
        Order order = Order.create(
            CustomerId.generate(),
            List.of(new OrderLine(ProductId.generate(), 1, Money.of(100, "USD")))
        );
        // Reflection or package-private method to set status for testing
        return order;
    }
}
```

---

### Application Layer Tests (Unit with Mocks)

```java
@ExtendWith(MockitoExtension.class)
class CreateOrderUseCaseTest {

    @Mock
    private OrderRepository orderRepository;

    @Mock
    private CustomerRepository customerRepository;

    @InjectMocks
    private CreateOrderUseCase useCase;

    @Test
    void should_create_order_when_customer_exists() {
        // Given
        UUID customerId = UUID.randomUUID();
        Customer customer = CustomerTestFactory.create(customerId);

        CreateOrderCommand command = new CreateOrderCommand(
            customerId,
            List.of(new CreateOrderCommand.OrderLineCommand(UUID.randomUUID(), 2))
        );

        when(customerRepository.findById(CustomerId.of(customerId)))
            .thenReturn(Optional.of(customer));
        when(orderRepository.save(any(Order.class)))
            .thenAnswer(invocation -> invocation.getArgument(0));

        // When
        OrderResponse response = useCase.execute(command);

        // Then
        assertThat(response.customerId()).isEqualTo(customerId);
        assertThat(response.status()).isEqualTo("PENDING");
        assertThat(response.lines()).hasSize(1);

        verify(orderRepository).save(any(Order.class));
    }

    @Test
    void should_throw_when_customer_not_found() {
        // Given
        CreateOrderCommand command = new CreateOrderCommand(
            UUID.randomUUID(),
            List.of(new CreateOrderCommand.OrderLineCommand(UUID.randomUUID(), 1))
        );

        when(customerRepository.findById(any())).thenReturn(Optional.empty());

        // When / Then
        assertThatThrownBy(() -> useCase.execute(command))
            .isInstanceOf(CustomerNotFoundException.class);

        verifyNoInteractions(orderRepository);
    }
}
```

---

### Infrastructure Layer Tests (@DataJpaTest)

```java
@DataJpaTest
@AutoConfigureTestDatabase(replace = AutoConfigureTestDatabase.Replace.NONE)
@Testcontainers
class OrderRepositoryAdapterTest {

    @Container
    static PostgreSQLContainer<?> postgres = new PostgreSQLContainer<>("postgres:16")
        .withDatabaseName("testdb")
        .withUsername("test")
        .withPassword("test");

    @DynamicPropertySource
    static void configureProperties(DynamicPropertyRegistry registry) {
        registry.add("spring.datasource.url", postgres::getJdbcUrl);
        registry.add("spring.datasource.username", postgres::getUsername);
        registry.add("spring.datasource.password", postgres::getPassword);
    }

    @Autowired
    private OrderJpaRepository jpaRepository;

    @Autowired
    private TestEntityManager entityManager;

    private OrderRepositoryAdapter adapter;

    @BeforeEach
    void setUp() {
        adapter = new OrderRepositoryAdapter(jpaRepository, new OrderJpaMapper());
    }

    @Test
    void should_save_and_retrieve_order() {
        // Given
        Order order = OrderTestFactory.createPending();

        // When
        Order saved = adapter.save(order);
        entityManager.flush();
        entityManager.clear();

        // Then
        Optional<Order> found = adapter.findById(saved.getId());
        assertThat(found).isPresent();
        assertThat(found.get().getStatus()).isEqualTo(OrderStatus.PENDING);
        assertThat(found.get().getLines()).hasSize(order.getLines().size());
    }
}
```

---

### Controller Layer Tests (@WebMvcTest)

```java
@WebMvcTest(OrderController.class)
class OrderControllerTest {

    @Autowired
    private MockMvc mockMvc;

    @Autowired
    private ObjectMapper objectMapper;

    @MockBean
    private CreateOrderUseCase createOrderUseCase;

    @MockBean
    private FindOrderUseCase findOrderUseCase;

    @Test
    void should_return_201_when_order_created_successfully() throws Exception {
        // Given
        OrderResponse expectedResponse = OrderResponseTestFactory.create();
        when(createOrderUseCase.execute(any())).thenReturn(expectedResponse);

        CreateOrderRequest request = new CreateOrderRequest(
            UUID.randomUUID(),
            List.of(new CreateOrderRequest.OrderLineRequest(UUID.randomUUID(), 2))
        );

        // When / Then
        mockMvc.perform(post("/api/v1/orders")
                .contentType(MediaType.APPLICATION_JSON)
                .content(objectMapper.writeValueAsString(request)))
            .andExpect(status().isCreated())
            .andExpect(jsonPath("$.id").value(expectedResponse.id().toString()))
            .andExpect(jsonPath("$.status").value("PENDING"));
    }

    @Test
    void should_return_422_when_request_is_invalid() throws Exception {
        // Given — missing required fields
        String invalidRequest = "{}";

        // When / Then
        mockMvc.perform(post("/api/v1/orders")
                .contentType(MediaType.APPLICATION_JSON)
                .content(invalidRequest))
            .andExpect(status().isUnprocessableEntity())
            .andExpect(jsonPath("$.code").value("VALIDATION_ERROR"));
    }
}
```

---

## Frontend TDD — Jest + Angular Testing Library

### Component Test
```typescript
describe('OrderListComponent', () => {
  let fixture: ComponentFixture<OrderListComponent>;
  let component: OrderListComponent;

  const mockOrders: OrderResponse[] = [
    { id: '1', customerName: 'Acme Corp', status: 'PENDING', totalAmount: 500, createdAt: new Date().toISOString() }
  ];

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [OrderListComponent],
      providers: [
        { provide: OrderService, useValue: { findAll: () => of(mockOrders) } }
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(OrderListComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should display orders', () => {
    const rows = fixture.debugElement.queryAll(By.css('[data-testid="order-row"]'));
    expect(rows.length).toBe(1);
  });

  it('should filter by search term', () => {
    component.searchTerm.set('nonexistent');
    fixture.detectChanges();

    const rows = fixture.debugElement.queryAll(By.css('[data-testid="order-row"]'));
    expect(rows.length).toBe(0);
  });

  it('should navigate to detail on row click', () => {
    const router = TestBed.inject(Router);
    jest.spyOn(router, 'navigate');

    const row = fixture.debugElement.query(By.css('[data-testid="order-row"]'));
    row.triggerEventHandler('click', null);

    expect(router.navigate).toHaveBeenCalledWith(['/orders', '1']);
  });
});
```

---

## Test Naming Conventions

```
should_{what}_{when}

Examples:
should_create_order_with_pending_status
should_throw_when_customer_not_found
should_calculate_total_correctly
should_return_201_when_order_created_successfully
should_return_422_when_request_is_invalid
should_filter_orders_by_search_term
```

---

## Test Factories

```java
// Avoid duplicating test data setup
public class OrderTestFactory {
    public static Order createPending() {
        return Order.create(
            CustomerId.generate(),
            List.of(new OrderLine(ProductId.generate(), 1, Money.of(100, "USD")))
        );
    }

    public static Order createConfirmed() {
        Order order = createPending();
        order.confirm();
        return order;
    }
}
```
