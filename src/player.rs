use crate::map::Room;
use std::any::Any;
use std::{ops::RangeInclusive, rc::Rc};

#[derive(Debug)]
pub struct Player {
    pub health: i32,
    pub level: i32,
    pub current_room: Option<Rc<Room>>,
    pub equipped_weapon: Option<Rc<Weapon>>,
    pub inventory: Vec<Box<dyn InventoryItem>>,
}

#[derive(Debug)]
pub struct Weapon {
    pub dmg_range: RangeInclusive<i32>,
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
