# Custom Rust Macros

A collection of custom procedural derive macros for Rust that provide various functionality for structs.

## Available Macros

### 🔢 MathOps
Provides mathematical operations on struct fields.

```rust
#[derive(MathOps)]
struct Numbers {
    a: i32,
    b: i32,
    c: i32,
}

let nums = Numbers { a: 2, b: 3, c: 4 };
println!("Sum: {}", nums.sum());        // Sum: 9
println!("Product: {}", nums.product()); // Product: 24
println!("Average: {:.2}", nums.average()); // Average: 3.00
```

### ➕ SumFields
Adds all numeric fields together.

```rust
#[derive(SumFields)]
struct Data {
    x: i32,
    y: i32,
}

let data = Data { x: 10, y: 20 };
println!("Total: {}", data.add_fields()); // Total: 30
```

### ⏭️ SumFieldsSkip
Adds fields together while skipping marked fields.

```rust
#[derive(SumFieldsSkip)]
struct Stats {
    kills: i32,
    assists: i32,
    #[skip_sum]
    deaths: i32,
}

let stats = Stats { kills: 10, assists: 5, deaths: 3 };
println!("Score: {}", stats.add_fields()); // Score: 15 (deaths skipped)
```

### 📦 SerializeNumberStruct / DeserializeNumberStruct
Binary serialization for structs with i32 fields.

```rust
#[derive(SerializeNumberStruct, DeserializeNumberStruct)]
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 100, y: 200 };
let bytes = point.serialize();
let restored = Point::deserialize(&bytes).unwrap();
```

## Project Structure

```
my-macros/
├── macro_traits/     # Trait definitions
├── define_macros/    # Procedural macro implementations
└── app/             # Example usage
```

## Usage

1. Add dependencies to your `Cargo.toml`:
```toml
[dependencies]
macro_traits = { path = "macro_traits" }
define_macros = { path = "define_macros" }
```

2. Import and use:
```rust
use define_macros::{MathOps, SumFields};
use macro_traits::{MathOps as MathOpsTrait, AddFields};

#[derive(MathOps)]
struct MyStruct {
    field1: i32,
    field2: i32,
}
```

## Running Examples

```bash
cd app
cargo run
```

## Features

- ✅ Mathematical operations (sum, product, average)
- ✅ Selective field summation with skip attributes
- ✅ Binary serialization/deserialization
- ✅ Works with named struct fields
- ✅ Type-safe procedural macros

Built with Rust 🦀
