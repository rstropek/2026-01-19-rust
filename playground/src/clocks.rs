pub struct WallClock {
    hour: u8,
    minute: u8,
}

impl WallClock {
    pub fn new(hour: u8, minute: u8) -> Self {
        // "static" method, meaning we do not need an instance to call it
        Self { hour, minute }
    }

    pub fn add_minutes(&mut self, minutes: u8) {
        // instance method, meaning we need an instance to call it
        let total_minutes = self.minute as u16 + minutes as u16;
        self.hour = (self.hour + (total_minutes / 60) as u8) % 24;
        self.minute = (total_minutes % 60) as u8;
    }

    pub fn add_minutes_into_new(&self, minutes: u8) -> Self {
        // instance method that returns a new instance
        let total_minutes = self.minute as u16 + minutes as u16;
        let new_hour = (self.hour + (total_minutes / 60) as u8) % 24;
        let new_minute = (total_minutes % 60) as u8;
        Self {
            hour: new_hour,
            minute: new_minute,
        }
    }

    pub fn get_minutes(&self) -> u8 {
        // instance method to get minutes
        self.minute
    }
}
