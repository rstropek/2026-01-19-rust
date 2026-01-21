use std::{cell::RefCell, fmt::{Display, Formatter}, rc::Rc};

struct MyPreciousRing {
    engraving: String
}

impl MyPreciousRing {
    fn new() -> Self {
        MyPreciousRing {
            engraving: String::from("One Ring to rule them all"),
        }
    }
}

impl Display for MyPreciousRing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "One Ring to rule them all")
    }
}

impl Drop for MyPreciousRing {
    fn drop(&mut self) {
        println!("My precious is gone...");
    }
}

fn main() {
    let saurons_ring = Rc::new(RefCell::new(MyPreciousRing::new()));
    println!("Ref counter = {}", Rc::strong_count(&saurons_ring));
    let gollums_ring = saurons_ring.clone();
    println!("Ref counter = {}", Rc::strong_count(&saurons_ring));

    let samwise_ring = saurons_ring.borrow_mut();
    let frodo_ring = saurons_ring.borrow_mut();
    drop(samwise_ring);
    drop(frodo_ring);

    drop(gollums_ring);
    println!("Ref counter = {}", Rc::strong_count(&saurons_ring));
    drop(saurons_ring);
}
