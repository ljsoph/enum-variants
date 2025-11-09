## Enum Variants

Easily create an array of all variants of an enum, preserving their order.

If using an unnamed or named variant, all fields must implement `Default` or have an associated function with the same signature.

If you wish to use `EnumVariants` with Named and Unnamed variants, but would still like easy access
via discriminants, you can also derive `EnumDiscriminants`. This will generate a new enum with data-less variants of the derived enum as well implement the `From` trait.
All generated enums derive `Clone`, `Copy`, `Debug`, `Eq`, and `PartialEq` by default.

---

### Example

**Unit Types**
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

**Unnamed & Named Types**

Not quite as useful but it's there :)

```rust
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(EnumVariants)]
enum Color {
    Red,
    Green(Rgb),
    Blue { r: u8, g: u8, b: u8 },
}

fn main() {
    // Produces [Red, Green(0, 0, 0), Blue { r: 0, g: 0, b: 0 }]
    let variants = Color::variants();
}
```

**Discriminants**
```rust
#[derive(EnumDiscriminants)]
enum Foo {
    Bar,
    Baz(String),
    Qux { a: i32, b: i32 },
}

// Generates
enum FooDiscriminant {
    Bar,
    Baz,
    Qux,
}

impl From<Foo> for FooDiscriminant {
    fn from(value: Foo) -> Self {
        match value {
            Foo::Bar => FooDiscriminant::Bar,
            Foo::Baz(_) => FooDiscriminant::Baz,
            Foo::Qux { .. } => FooDiscriminant::Qux,
        }
    }
}

```
