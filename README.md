# typed_id2

## Usage

```rust
use typed_id::Id;

struct Customer {
    name: String,
}
type CustomerId = Id<i32, Customer>;

let customer_id = CustomerId::new(1);
```