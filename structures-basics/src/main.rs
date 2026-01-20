#![allow(dead_code)]

// Structures
struct WallClock {
    hour: u8,
    minute: u8,
}

impl WallClock {
    fn new(hour: u8, minute: u8) -> Self { // "static" method, meaning we do not need an instance to call it
        Self { hour, minute }
    }

    fn add_minutes(&mut self, minutes: u8) { // instance method, meaning we need an instance to call it
        let total_minutes = self.minute as u16 + minutes as u16;
        self.hour = (self.hour + (total_minutes / 60) as u8) % 24;
        self.minute = (total_minutes % 60) as u8;
    }

    fn add_minutes_into_new(&self, minutes: u8) -> Self { // instance method that returns a new instance
        let total_minutes = self.minute as u16 + minutes as u16;
        let new_hour = (self.hour + (total_minutes / 60) as u8) % 24;
        let new_minute = (total_minutes % 60) as u8;
        Self { hour: new_hour, minute: new_minute }
    }

    fn get_minutes(&self) -> u8 { // instance method to get minutes
        self.minute
    }
}

mod clocks;

fn main() {
    let clock = WallClock { hour: 10, minute: 30 }; // clock lives on the stack
    println!("WallClock - Hour: {}, Minute: {}", clock.hour, clock.minute);

    // print minutes
    println!("Minutes: {}", clock.get_minutes());

    // Add minutes into new clock
    let clock = clock.add_minutes_into_new(90);
    println!("After adding 90 minutes - Hour: {}, Minute: {}", clock.hour, clock.minute);

    let heap_clock = Box::new(WallClock { hour: 12, minute: 45 }); // heap_clock lives on the heap
    println!("WallClock (Heap) - Hour: {}, Minute: {}", heap_clock.hour, heap_clock.minute);

    // Use the "constructor"
    let mut constructed_clock = clocks::WallClock::new(8, 20);
    constructed_clock.add_minutes(50);
}
