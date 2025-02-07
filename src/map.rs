use crate::player::InventoryItem;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Room {
    pub size: (i32, i32),
    pub pos: (i32, i32),
    pub name: String,
    pub connections: Vec<Rc<RefCell<RoomConnection>>>,
    pub items: Vec<Rc<RefCell<dyn InventoryItem>>>,
}

#[derive(Debug)]
pub struct RoomConnection {
    pub r1: Rc<RefCell<Room>>,
    pub r2: Rc<RefCell<Room>>,
}

#[derive(Debug)]
pub struct Map {
    pub rooms: Vec<Rc<RefCell<Room>>>,
    pub connections: Vec<Rc<RefCell<RoomConnection>>>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            rooms: vec![],
            connections: vec![],
        }
    }

    pub fn add_room(&mut self, size: (i32, i32), pos: (i32, i32)) -> Rc<RefCell<Room>> {
        let room = Rc::new(RefCell::new(Room::new(size, pos)));

        self.rooms.push(room);

        Rc::clone(self.rooms.last().unwrap())
    }

    pub fn create_connection(
        &mut self,
        r1: &Rc<RefCell<Room>>,
        r2: &Rc<RefCell<Room>>,
    ) -> Rc<RefCell<RoomConnection>> {
        let con = RoomConnection::build(r1, r2);

        self.connections.push(Rc::new(RefCell::new(con)));

        r1.borrow_mut()
            .connections
            .push(Rc::clone(self.connections.last().unwrap()));
        r2.borrow_mut()
            .connections
            .push(Rc::clone(self.connections.last().unwrap()));

        Rc::clone(self.connections.last().unwrap())
    }
}

impl Room {
    pub fn new(size: (i32, i32), pos: (i32, i32)) -> Room {
        Room {
            size,
            pos,
            name: String::new(),
            connections: vec![],
            items: vec![],
        }
    }
}

impl RoomConnection {
    pub fn build(r1: &Rc<RefCell<Room>>, r2: &Rc<RefCell<Room>>) -> RoomConnection {
        RoomConnection {
            r1: Rc::clone(r1),
            r2: Rc::clone(r2),
        }
    }
}
