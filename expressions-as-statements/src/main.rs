#![allow(unused_variables)]

fn add(x: f32, y: f32) -> f32 {
    if x == 42.0 { -1.0 } else { x + y }
}

fn _do_something() -> i32 {
    let _temp = loop {
        break 42;
    };
    _temp
}

fn main() {
    let input1 = 21.0;
    let input2 = 21.0;

    let result = add(input1, input2);
}
