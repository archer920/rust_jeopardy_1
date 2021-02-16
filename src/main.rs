#[macro_use]
extern crate clap;

#[macro_use]
extern crate icecream;

use clap::App;

fn main() {
    let yaml = load_yaml!("../res/cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let myfile = matches.value_of("game_board").expect("Game file is required");
    let debug = matches.value_of("debug").unwrap_or("none");

    ic!(myfile);
    ic!(debug);
}
