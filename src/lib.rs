pub mod map;
pub mod player;

use map::{Map, Room, RoomConnection};
use player::{Player, Weapon};

use std::{cell::RefCell, rc::Rc};

pub fn run() -> Result<(), &'static str> {
    let mut main_map = Map::new();

    let r1 = main_map.add_room((5, 5), (0, 0));
    let r2 = main_map.add_room((5, 5), (10, 0));

    r1.borrow_mut().name = String::from("Room 1");
    r2.borrow_mut().name = String::from("Room 2");

    println!("{main_map:?}");

    Ok(())
}
