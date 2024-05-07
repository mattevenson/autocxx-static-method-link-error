use autocxx::prelude::*;

include_cpp! {
    #include "toy.h"
    generate!("Toy")
    safety!(unsafe)
}

fn main() {
    // let _toy = ffi::Toy::new(); // Builds fine
    let _result = ffi::Toy::add_one(1); // Fails with link error
    println!("Hello, world!");
}
