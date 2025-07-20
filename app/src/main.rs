use define_macros::{DeserializeNumberStruct, SerializeNumberStruct, SumFields, SumFieldsSkip, MathOps};
use macro_traits::{AddFields, Deserialize, Serialize, SumFieldsAndSkipSomeFields, MathOps};
use std::fmt::Error;

#[derive(SerializeNumberStruct, DeserializeNumberStruct, SumFields)]
struct Swap {
    qty_1: i32,
    qty_2: i32,
    qty_3: i32,
}

#[derive(SumFieldsSkip)]
struct Stats {
    kills: i32,
    assists: i32,
    #[skip_sum]
    deaths: i32,
}

#[derive(MathOps)]
struct Numbers {
    num1: i32,
    num2: i32,
    num3: i32,
    num4: i32,
}

fn main() {
    println!("Hello, world!");
    let s = Swap {
        qty_1: 1,
        qty_2: 2,
        qty_3: 1000,
    };
    // Serialize the struct
    let bytes = s.serialize();
    println!("Serialized bytes: {:?}", bytes);

    // Deserialize back to struct
    match Swap::deserialize(&bytes) {
        Ok(deserialized) => {
            println!(
                "Deserialized: qty_1={}, qty_2={}, qty_3={}",
                deserialized.qty_1, deserialized.qty_2, deserialized.qty_3
            );
        }
        Err(_) => {
            println!("Failed to deserialize");
        }
    }

    let ans = s.add_fields();
    println!("{}", ans);

    let stats = Stats {
        kills: 23,
        assists: 42,
        deaths: 30,
    };

    let stats_sum = stats.add_fields();
    println!("stats sum = {}", stats_sum);

    // Test MathOps macro
    let numbers = Numbers {
        num1: 2,
        num2: 3,
        num3: 4,
        num4: 5,
    };

    println!("\nTesting MathOps:");
    println!("Sum: {}", numbers.sum());
    println!("Product: {}", numbers.product());
    println!("Average: {:.2}", numbers.average());
}
