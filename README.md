## Enum Variants

Easily create an array of all variants of an Enum, preserving their order.

Currently only support Unit and Unnamed enum variants.

If using an unnamed variant, all fields must implement `Default` or have an associated function with the same signature.

---

**Example**

```rust
#[derive(EnumVariants)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    // Produces [Red, Green, Blue]
    let variants = Color::variants();
}
```
