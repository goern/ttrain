pub mod locations;
pub mod vehicles;

fn main() {
    stderrlog::new().module(module_path!()).init().unwrap();
}
