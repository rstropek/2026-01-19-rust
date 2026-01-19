#[allow(unused_variables)]

fn add(mut x: i32, y: i32) -> i32 {
    x += 1;
    x + y
}

fn main() {
    let mut x = 42.0;
    x += 1.0;

    let mut numbers = vec![1, 2, 3];
    //numbers = vec![4, 5, 6];
    numbers.push(4);

    let x = 1;
    println!("{}", add(x, 2));

    let user_input = "42";
    let user_input: i32 = user_input.parse().unwrap();

    let my_var = 42;
    let mut my_var = my_var; // "unfreezing"
    my_var += 1;
    let my_var = my_var; // "freeze"

    let my_var = 42;
    {
        let mut my_var = my_var; // "unfreezing"
        my_var += 1;
        println!("{}", my_var);
    }
    // my_var is automatically "frozen" again
    println!("{}", my_var);

    let mut user_input = "42".to_string();
    user_input.push('!');
    let user_input = "43";

}
