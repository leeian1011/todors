mod todoer;
use std::io::{BufRead, Write};

use chrono::Local;
use todoer::print::{Colour, Printer};

const TODORSCOLOURS: [Colour;1] = [Colour::GreenText];
const HELP_LIST: [&'static str; 5] = [
    "list",
    "add [task]",
    "remove [task]",
    "complete [task]",
    "filter [priorities]"
];

fn main() {
    Printer::box_print(&["todors"], &TODORSCOLOURS);
    Printer::print_colour_no_reset("|> ", &[Colour::BlueText]);
    let _ = std::io::stdout().flush();

    let mut stdin_buffer = String::new();

    let mut stdin_handle = std::io::stdin().lock();  
    while stdin_handle.read_line(&mut stdin_buffer).unwrap() > 0 {
        let split_buffer = stdin_buffer.split(' ').collect::<Vec<_>>();
        match split_buffer.first().unwrap().trim() {
            "list" => {
                
            }
            "help" => {
                if split_buffer.len() == 1 {
                    Printer::println_colour("todors: You may run the following commands", &TODORSCOLOURS);
                    Printer::box_print(&HELP_LIST, &TODORSCOLOURS);
                } else if split_buffer.len() == 2 {
                    match split_buffer[1].trim() {
                        "list" => {
                            todo!("how ironic");
                        },
                        "add" => {
                            todo!("how ironic");
                        },
                        "remove" => {
                            todo!("how ironic");
                        },
                        "complete" => {
                            todo!("how ironic");
                        }
                        _ => {
                            Printer::println_colour("todors: I didn't quite get that", &TODORSCOLOURS);
                            Printer::println_colour("todors: You may run the following commands", &TODORSCOLOURS);
                            Printer::box_print(&HELP_LIST, &TODORSCOLOURS);
                        }
                    }
                } else {
                    Printer::println_colour("todors: I didn't quite get that", &TODORSCOLOURS);
                    Printer::println_colour("todors: You may run the following commands", &TODORSCOLOURS);
                    Printer::box_print(&HELP_LIST, &TODORSCOLOURS);
                }
            }
            "exit" => {
                Printer::println_colour("todors: goodbye!", &TODORSCOLOURS);
                panic!();
            } ,
            _ => Printer::println_colour("todors: I didn't quite get that", &TODORSCOLOURS),
        }
        stdin_buffer.clear();
        Printer::print_colour_no_reset("|> ", &[Colour::BlueText]);
        let _ = std::io::stdout().flush();
    }
    
    let date = Local::now().to_rfc3339();

    Printer::println_colour(&date, &[Colour::BlueText]);
}
