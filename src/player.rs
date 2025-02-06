use crate::map::Room;
use std::any::Any;
use std::{cell::RefCell, ops::RangeInclusive, rc::Rc};

#[derive(Debug)]
pub struct Player {
    health: i32,
    level: i32,
    current_room: Option<Rc<Room>>,
    equipped_weapon: Option<Rc<Weapon>>,
    inventory: Vec<Box<dyn InventoryItem>>,
}

#[derive(Debug)]
pub struct Weapon {
    dmg_range: RangeInclusive<i32>,
}

pub trait InventoryItem: IIToAny {
    fn get_type(&self) -> InvType;
}

pub trait IIToAny: 'static + std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
}

impl<T: InventoryItem> IIToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub enum InvType {
    Weapon,
    Item,
}

impl Player {
    pub fn new() -> Player {
        Player {
            health: 0,
            level: 1,
            current_room: None,
            equipped_weapon: None,
            inventory: vec![],
        }
    }
}

impl Weapon {
    pub fn new() -> Weapon {
        Weapon { dmg_range: 1..=4 }
    }
}

impl InventoryItem for Weapon {
    fn get_type(&self) -> InvType {
        InvType::Weapon
    }
}
