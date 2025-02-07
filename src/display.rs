use std::{cell::RefCell, rc::Rc};

use crate::map::{Map, Room};

pub fn render(map: &Map) {
    for room in map.rooms.as_slice() {
        draw_room(room);
    }
}

fn draw_room(room: &Rc<RefCell<Room>>) {}
