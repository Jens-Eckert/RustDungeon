pub mod map;
pub mod player;

use map::{Map, Room, RoomConnection};
use player::{Player, Weapon};

use std::{cell::RefCell, rc::Rc};

pub fn run() -> Result<(), &'static str> {
    let mut main_map = Map::new();

    let mut r1 = main_map.add_room((5, 5), (0, 0));
    let mut r2 = main_map.add_room((5, 5), (10, 0));

    let o = main_map.create_connection(&mut r1, &mut r2).unwrap();

    println!("{main_map:?}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_test() {}
}
