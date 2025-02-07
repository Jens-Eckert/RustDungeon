pub mod display;
pub mod map;
pub mod player;

use map::Map;

pub fn run() -> Result<(), &'static str> {
    let mut main_map = Map::new();

    let r1 = main_map.add_room((5, 5), (0, 0));
    let r2 = main_map.add_room((7, 5), (10, 0));

    main_map.create_connection(&r1, &r2);

    display::render(&main_map);

    Ok(())
}
