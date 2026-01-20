#![allow(dead_code, unused_variables)]

struct Customer {
    id: u32,
    name: String,
}

struct Order<'a> {
    order_id: u32,
    customer: &'a Customer,
    receiver_of_goods: &'a Customer,
}

struct Line {
    start: (f32, f32),
    end: (f32, f32),
}

impl Line {
    fn length(&self) -> f32 {
        let dx = self.end.0 - self.start.0;
        let dy = self.end.1 - self.start.1;
        (dx * dx + dy * dy).sqrt()
    }
}

fn get_longer_line<'a>(line1: &'a Line, line2: &'a Line) -> &'a Line {
    if line1.length() >= line2.length() {
        line1
    } else {
        line2
    }
}

fn get_longer_lines<'a, 'b>(
    line1: &'a Line,
    line2: &'a Line,
    line3: &'b Line,
    line4: &'b Line,
) -> (&'a Line, &'b Line) {
    let longer_line1;
    if line1.length() >= line2.length() {
        longer_line1 = line1;
    } else {
        longer_line1 = line2;
    }

    let longer_line2;
    if line3.length() >= line4.length() {
        longer_line2 = line3;
    } else {
        longer_line2 = line4;
    }

    (longer_line1, longer_line2)
}

fn main() {
    let order;

    {
        let customer = Customer {
            id: 1,
            name: String::from("Alice"),
        };

        order = Order {
            order_id: 1001,
            customer: &customer,
            receiver_of_goods: &customer,
        };

        // Print the order
        println!(
            "Order ID: {}, Customer Name: {}",
            order.order_id, order.customer.name
        );
    }

    let line1 = Line {
        start: (0.0, 0.0),
        end: (3.0, 4.0),
    };
    let longer_line;

    let line2;
    {
        line2 = Line {
            start: (1.0, 1.0),
            end: (40.0, 50.0),
        };

        longer_line = get_longer_line(&line1, &line2);
    }
    println!(
        "Longer line length: {}",
        longer_line.length()
    );
}
