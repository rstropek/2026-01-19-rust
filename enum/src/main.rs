#![allow(dead_code, unused_variables)]

#[derive(PartialEq, Eq)]
struct MaintenanceData {
    reason: String,
    days_remaining: u8,
}

#[derive(PartialEq, Eq)]
enum HotelRoomState {
    Vacant,
    Occupied(String /* name of guest */),
    UnderMaintenance(MaintenanceData),
}

impl HotelRoomState {
    fn new_vacant() -> Self {
        HotelRoomState::Vacant
    }
}

#[derive(PartialEq, Eq)]
enum Colors {
    Red,
    Green,
    Blue,
}

fn main() {
    let room = HotelRoomState::UnderMaintenance(MaintenanceData {
        reason: String::from("Plumbing issues"),
        days_remaining: 3,
    });

    if room == HotelRoomState::Vacant {
        println!("The room is vacant.");
    }

    if let HotelRoomState::Occupied(guest_name) = &room {
        println!("The room is occupied by {}.", guest_name);
    }
    
    match &room {
        HotelRoomState::Vacant => {
            println!("The room is vacant.");
        }
        HotelRoomState::Occupied(guest_name) => {
            println!("The room is occupied by {}.", guest_name);
        }
        HotelRoomState::UnderMaintenance(data) => {
            println!(
                "The room is under maintenance for {} days due to: {}",
                data.days_remaining, data.reason
            );
        }
    }


    let my_color = Colors::Green;
    if my_color == Colors::Green {
        println!("The color is green!");
    }
}
