mod todoer;
use std::io::{BufRead, Write};

use chrono::Local;
use todoer::print::{println_colour, Colour, print_colour_no_reset, Printer};

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
    print_colour_no_reset("|> ", &[Colour::BlueText]);
    let _ = std::io::stdout().flush();

    let mut stdin_buffer = String::new();

    let mut stdin_handle = std::io::stdin().lock();  
    while stdin_handle.read_line(&mut stdin_buffer).unwrap() > 0 {
        match stdin_buffer.trim_end() {
            "help" => {
                println_colour("todors: You may run the following commands", &TODORSCOLOURS);
                Printer::box_print(&HELP_LIST, &TODORSCOLOURS);
            }
            "exit" => {
                println_colour("todors: goodbye!", &TODORSCOLOURS);
                panic!();
            } ,
            _ => println_colour("todors: I didn't quite get that", &TODORSCOLOURS),
        }
        stdin_buffer.clear();
        print_colour_no_reset("|> ", &[Colour::BlueText]);
        let _ = std::io::stdout().flush();
    }
    
    let date = Local::now().to_rfc3339();

    println_colour(&date, &[Colour::BlueText]);
}
