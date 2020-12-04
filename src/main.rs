//extern crate ncurses;
//use ncurses::*;

//fn main() {
    //println!("Hello, world!");
    //initscr();
    //printw("Hello");
    //refresh();
    //getch();
    //endwin();
//}

// (Full example with detailed comments in examples/01a_quick_example.rs)
//
// This example demonstrates clap's "usage strings" method of creating arguments
// which is less verbose
use clap::{Arg, App};
use std::io::{self, Read};
use edit;

fn main() {
    let matches = App::new("DotRush")
        .version("1.0")
        .author("PerseoGI. <perseo.gi98@gmail.com>")
        .about("Manage dotfiles on your git repository")
        .subcommand(App::new("init"))
            .about("start dotfile configuration process")
            .version("1.0")

        .get_matches();


    if let Some(ref matches) = matches.subcommand_matches("init") {
        println!("Starting dotrush configuration");

        println!("Type your Git");
        let mut username = String::new();
        match io::stdin().read_line(&mut username){
            Ok(n) => {
                println!("Okey {}", username);
            }
            Err(error) => println!("error {}", error)
        }

        let template = "Fill in the blank: Hello, _____!";
        let edited = edit::edit(template);
        println!("after editing: '{:?}'", edited);

        // "$ myapp test" was run
        if matches.is_present("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
        }
    }
}
