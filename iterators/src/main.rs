// 1. Read the content of the file "orders.csv"
// 2. Calculate the average revenue after rebate

use std::fs;

fn extract_revenue_after_rebate(line: &str) -> f32 {
    let parts: Vec<&str> = line.split('|').collect();
    let revenue: f32 = parts[3].parse().unwrap();
    let rebate: f32 = parts[4].parse().unwrap();
    revenue * (1.0 - rebate)
}

fn main() {
    let content = fs::read_to_string("orders.csv").unwrap();

    let result = content.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| extract_revenue_after_rebate(line))
        .fold((0.0, 0), |(sum, count), revenue| (sum + revenue, count + 1));

    let average = result.0 / result.1 as f32;
    println!("Average revenue after rebate: {}", average);
}
