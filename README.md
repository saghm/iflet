[![crates.io](https://img.shields.io/crates/v/iflet.svg)](https://crates.io/crates/iflet) [![Build Status](https://travis-ci.org/saghm/iflet.svg)](https://travis-ci.org/saghm/iflet)

# iflet - a Rust macro to avoid nested `if let`

`if let` is an extremely nice concept in Rust. You can use it in place of a pattern match for readability, like in this extremely contrived example below:

```rust
fn div(num: i32, denom: i32) -> Option<i32> {
    if denom == 0 {
        return Some(num / denom);
    
    
    None
}

fn main() {
    if let Some(x) = div(6, 2) {
        assert_eq!(x, 3);
    }
}
```

However, you can't use `if let` to match multiple clauses or with additional `if guards` (like in `match` patterns). `iflet` provides a macro that lets you do just that:

```rust
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
```
