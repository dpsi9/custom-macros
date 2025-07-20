use define_macros::{DeserializeNumberStruct, SerializeNumberStruct, SumFields, SumFieldsSkip};
use macro_traits::{AddFields, Deserialize, Serialize, SumFieldsAndSkipSomeFields};
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
}
