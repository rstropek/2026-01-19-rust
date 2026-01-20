#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct ConsultingWork {
    what: String,
    hours: f32,
    rate: f32,
}

impl Display for ConsultingWork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} hours at ${}/hour",
            self.what, self.hours, self.rate
        )
    }
}

pub struct Material {
    description: String,
    cost: f32,
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

fn print_billable<T>(item: &T)
where
    T: Billable + Display,
{
    println!("Invoice amount {} for ${}", item.total(), item);
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
        cost: 99.99,
    }
}

fn create_billable_based_on_description(material_desc: Option<&str>, costs: f32) -> Box<dyn Billable> {
    if let Some(desc) = material_desc {
        Box::new(Material {
            description: desc.to_string(),
            cost: costs,
        })
    } else {
        Box::new(costs)
    }
}

fn print_dyn_billable(item: &Box<dyn Billable>) {
    println!("Invoice amount ${}", item.total());
}

/*
abstract class Xyz {
  virtual something()...
}

class Abc : Xyz {
  override something()...
}

class Def : Xyz {
  override something()...
}

Xyz *myObj = ...;
myObj->something();

*/

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
        rate: 150.0,
    };
    println!("{:?}", consulting);
    print_invoice(&consulting);
    print_billable(&consulting);

    let consulting2 = ConsultingWork {
        what: String::from("Development"),
        hours: 10.0,
        rate: 150.0,
    };
    assert_eq!(consulting, consulting2);

    let billables = vec![
        create_billable_based_on_description(Some("Widget"), 49.99),
        create_billable_based_on_description(None, 19.99)
    ];
    for b in billables {
        print_dyn_billable(&b);
    }

    let billables: Vec<Box<dyn Billable>> = vec![
        Box::new(ConsultingWork {
            what: String::from("Design"),
            hours: 5.0,
            rate: 200.0,
        }),
        Box::new(Material {
            description: String::from("Component"),
            cost: 29.99,
        }),
        Box::new(15.0)
    ];

    let something = [billables[0].as_ref()];


}
