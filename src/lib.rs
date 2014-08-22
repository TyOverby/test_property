extern crate debug;

use std::rand::Rand;
use std::rand::Rng;
use std::rand::task_rng;

fn property<T: Rand + Clone>(name: &str, property: |&T| -> bool) {
    let mut rng = task_rng();
    for _ in range(0u, 100) {
       let item: T = rng.gen();
       if !property(&item) {
            fail!("{} did not hold for {:?}", name, item);
       }
    }
}

#[test]
fn test_property() {
    property("addition associativity", |&(a, b): &(u32, u32)| {
        a + b == b + a
    });

    /* This will fail
    property("subtraction associativity", |(a, b): (u32, u32)| {
        a - b == b - a // this is entirely wrong
    });
    */
}
