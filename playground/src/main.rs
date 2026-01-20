#![allow(dead_code)]

use std::fmt::Debug;

pub struct ConsultingWork {
    what: String,
    hours: f32,
    rate: f32
}

pub struct Material {
    description: String,
    cost: f32
}

trait Billable {
    fn total(&self) -> f32;
}

impl Billable for ConsultingWork {
    fn total(&self) -> f32 {
        self.hours * self.rate
    }
}

impl Billable for Material {
    fn total(&self) -> f32 {
        self.cost
    }
}

trait Rebatable {
    fn apply_rebate(&mut self);
}

impl Rebatable for Material {
    fn apply_rebate(&mut self) {
        self.cost *= 0.9; // Apply a 10% rebate
    }
}

impl Rebatable for ConsultingWork {
    fn apply_rebate(&mut self) {
        self.rate *= 0.95; // Apply a 5% rebate
    }
}

fn print_invoice(item: &impl Billable) {
    println!("Invoice amount ${}", item.total());
}

fn apply_rebate_to_item(item: &mut impl Rebatable) {
    item.apply_rebate();
}

impl Billable for f32 {
    fn total(&self) -> f32 {
        *self
    }
}

impl Rebatable for f32 {
    fn apply_rebate(&mut self) {
        *self *= 0.9; // Apply a 10% rebate
    }
}

impl Billable for () {
    fn total(&self) -> f32 {
        0.0
    }
}

fn create_billable() -> impl Billable {
    Material {
        description: String::from("Gadget"),
        cost: 99.99
    }
}

fn main() {
    let mut work = 42.0;

    print_invoice(&work);
    apply_rebate_to_item(&mut work);
    print_invoice(&work);

    let item = create_billable();
    print_invoice(&item);

    let consulting = ConsultingWork {
        what: String::from("Development"),
        hours: 10.0,
        rate: 150.0
    };
    print_invoice(&consulting);
}
