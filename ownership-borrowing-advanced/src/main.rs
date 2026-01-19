use std::vec;

fn main() {
    let customer = "Alice".to_string();
    let another_customer = "Eve".to_string();

    let mut customers = vec![customer, another_customer]; // Transfer of ownership
    println!("Customers: {:?}", customers);
    
    let _alice = customers[0].clone();

    let last_customer = &customers.pop().unwrap();
    println!("Last customer served: {}", last_customer);

    let mut customers = vec!["Bob".to_string(), "Charlie".to_string()];
    for customer in &mut customers { // Loop over read-only borrows
        customer.push('!');
        println!("Customer: {}", customer);
    }
    for customer in customers { // Consumes the vector
        println!("Customer: {}", customer);
    }
}

//fn try_something(v: &mut Vec<String>) -> &String {
//    let customer = &v.pop().unwrap();
//    customer
//}
