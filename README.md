## Enum Variants

Easily create an array of all variants of an Enum, preserving their order.

(only works with unit-like variants)

```rust
#[derive(EnumVariants)]
enum Color {
    Red,
    Blue,
    Green,
    Purple
}

fn main() {
    // Produces &[Red, Blue, Green, Purple]
    let variants = Color::variants();
}

```
