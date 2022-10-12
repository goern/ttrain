#[macro_use]
extern crate juniper;

use rusqlite::Connection;

pub mod locations;
pub mod vehicles;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("ttrain/migrations");
}

fn main() {
    stderrlog::new().module(module_path!()).init().unwrap();

    let mut conn = Connection::open_in_memory().unwrap();
    embedded::migrations::runner().run(&mut conn).unwrap();
}
