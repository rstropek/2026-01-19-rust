use std::vec;

fn main() {
    {
        // Ownership: There is always exactly ONE owner
        // If the owner goes out of scope, the value is dropped
        let numbers = vec![10, 20, 30];
        println!("Before: {:?}", numbers);
        
        let other_numbers = numbers; // Transfer of ownership
        println!("Before: {:?}", other_numbers);
        // println!("Before: {:?}", numbers); // Not possible because numbers is no longer the owner

        drop(other_numbers); // Manually dropping the value
    }

    let i = 42;
    let _j = i; // NOT a transfer of ownership, because i32 implements the Copy trait

    {
        let numbers = create_vector(); // Transfer of ownership
        print_vector(&numbers); // Borrowing
        consume_vector(numbers); // Transfer of ownership
    }

    {
        let numbers = vec![1, 2, 3];
        let borrow1 = &numbers; // Read-only borrow
        let borrow2 = &numbers; // Another read-only borrow
        let borrow3 = &numbers; // Yet another read-only borrow
        println!("Borrows: {:?}, {:?}, {:?}", borrow1, borrow2, borrow3);
    }

    {
        let mut numbers = vec![1, 2, 3];
        let borrow_mut = &mut numbers; // Mutable borrow
        manipulate_vector(borrow_mut);
        println!("After mutation: {:?}", borrow_mut);
        let _other_numbers = numbers;
    }
}

fn create_vector() -> Vec<i32> {
    let temp = vec![1, 2, 3];
    temp
}

fn consume_vector(v: Vec<i32>) {
    println!("Consuming vector: {:?}", v);
}

fn print_vector(v: &Vec<i32>) { // Read-only borrow
    // As many read-only borrows as you want at the same time
    println!("Vector contents: {:?}", v);
}

fn manipulate_vector(v: &mut Vec<i32>) { // Mutable borrow
    // Only one mutable borrow at a time
    v.push(42);
}