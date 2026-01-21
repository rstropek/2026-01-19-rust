struct Counter {
    count: usize,
    current: i32,
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.count as i32 {
            let result = self.current;
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter {
        count: 20,
        current: 1,
    };

    let mut sum = 0;
    //for number in counter.map(|x| x * 2).filter(|&x| x < 10) {
    for number in counter.map(double).filter(|&x| x < 10) {
        sum += number;
    }

    println!("Sum: {}", sum);

    fn double(x: i32) -> i32 {
        x * 2
    }
}
