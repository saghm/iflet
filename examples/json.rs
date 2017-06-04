#[macro_use]
extern crate iflet;
#[macro_use]
extern crate serde_json;

use serde_json::Value::{Object, Array};

fn main() {
    let value = json!({
        "numbers": [ 1, 2, 4, 9, 16, 25 ]
    });

    if_chain!([let Object(ref map) => value,
               let Some(&Array(ref vec)) if !vec.is_empty() => map.get("numbers")] {
        println!("there are {} numbers stored in the object", vec.len());
    } else {
        println!("there are no numbers stored in the object");
    });
}
