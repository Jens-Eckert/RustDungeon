use std::io::{self, prelude::*};
use std::{cell::RefCell, rc::Rc};

use crate::map::{Map, Room};

pub fn render(map: &Map) {
    let test = map.rooms.iter();

    for room in test {
        draw_room(room);
    }
}

fn draw_room(room: &Rc<RefCell<Room>>) {
    let (w, h) = room.borrow().size;

    for i in 0..h {
        for _ in 0..w {
            if i == 0 || i == h - 1 {
                print!("* ");
                continue;
            } else {
                print!("*");
                for _ in 0..(w - 2) {
                    print!("  ");
                }
                print!(" *");
                break;
            }
        }
        println!();
    }
}
