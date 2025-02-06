use crate::player::InventoryItem;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Room {
    pub size: (i32, i32),
    pub pos: (i32, i32),
    pub name: String,
    pub connections: Vec<Rc<RoomConnection>>,
    pub items: Vec<Box<dyn InventoryItem>>,
}

#[derive(Debug)]
pub struct RoomConnection {
    pub r1: Rc<Room>,
    pub r2: Rc<Room>,
}

#[derive(Debug)]
pub struct Map {
    rooms: Vec<Rc<Room>>,
    connections: Vec<Rc<RoomConnection>>,
}

impl Map {
    /// Empty new Map object
    pub fn new() -> Map {
        Map {
            rooms: vec![],
            connections: vec![],
        }
    }

    // Creates new room in place and returns a Rc pointer to the new allocation
    pub fn add_room(&mut self, size: (i32, i32), pos: (i32, i32)) -> Rc<Room> {
        self.rooms.push(Rc::new(Room {
            size,
            pos,
            name: "".to_string(),
            connections: vec![],
            items: vec![],
        }));

        Rc::clone(self.rooms.last().unwrap())
    }

    pub fn create_connection(
        &mut self,
        r1: &mut Rc<Room>,
        r2: &mut Rc<Room>,
    ) -> Option<&Rc<RoomConnection>> {
        let con = Rc::new(RoomConnection {
            r1: Rc::clone(r1),
            r2: Rc::clone(r2),
        });

        self.connections.push(Rc::clone(&con));

        Rc::get_mut(r1).unwrap().connections.push(Rc::clone(&con));
        Rc::get_mut(r2).unwrap().connections.push(Rc::clone(&con));

        self.connections.last()
    }
}

impl Room {
    pub fn add_item<T: InventoryItem>(&mut self, item: T) {
        self.items.push(Box::new(item));
    }
}
