use std::process;

fn main() {
    if let Err(e) = dungeon_crawler::run() {
        eprintln!("Error running application: {e}");
        process::exit(-1);
    }
}
