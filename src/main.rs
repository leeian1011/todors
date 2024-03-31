mod todoer;
use std::io::{BufRead, Write};
use todoer::print::{Colour, Printer};

use crate::todoer::todo;

const TODORSCOLOURS: Colour = Colour::GreenText;
const HELPLIST: [&'static str; 5] = [
    "list",
    "add [task]",
    "remove [task]",
    "complete [task]",
    "filter [priorities]",
];

fn main() {
    let mut todo = todo::Todo::new().unwrap();
    let listhelp: [Vec<&'static str>; 2] = {
        [
            vec!["COMMAND", "list"],
            vec!["DESCRIPTION", "Retrieve all tasks."],
        ]
    };

    let addhelp: [Vec<&'static str>; 2] = {
        [
            vec!["COMMAND", "add [task]"],
            vec!["DESCRIPTION", "Add a new task."],
        ]
    };

    let removehelp: [Vec<&'static str>; 2] = {
        [
            vec!["COMMAND", "remove [task]"],
            vec!["DESCRIPTION", "Remove a task."],
        ]
    };

    let completehelp: [Vec<&'static str>; 2] = {
        [
            vec!["COMMAND", "complete [task]"],
            vec!["DESCRIPTION", "Mark a task as completed."],
        ]
    };

    Printer::box_print(&["todors"], &TODORSCOLOURS);
    print!("|> ");
    let _ = std::io::stdout().flush();

    let mut stdin_buffer = String::new();

    let mut stdin_handle = std::io::stdin().lock();
    while stdin_handle.read_line(&mut stdin_buffer).unwrap() > 0 {
        let split_buffer = stdin_buffer.split(' ').collect::<Vec<_>>();
        match split_buffer.first().unwrap().trim() {
            "debug" => {
                _ = Printer::table_print(
                    &[
                        vec!["Name", "Cybotrade-v1.4.0", "Datasource-v1.7.8"],
                        vec![
                        "Description",
                        "Check if change of function signature will affect anything",
                        "Check for any changes required for cybotrade to introduce new datasource"],
                        vec!["Created", "3d ago", "17m ago"],
                    ],
                    &[
                        Colour::MagentaText,
                        Colour::RedText,
                        Colour::GreenText,
                        Colour::CyanText,
                    ],
                );
            }
            "list" => {
                todo.list();
            }
            "add" => {
                match todo.add("task1".to_string(), Some("Ohboi".to_string()), None) {
                    Err(e) => Printer::box_print(&[e.0.as_str()], &Colour::RedText),
                    Ok(_) => Printer::box_print(&[format!("Successfully added {}!", "task1").as_str()], &TODORSCOLOURS),
                };
            }
            "help" => {
                if split_buffer.len() == 1 {
                    Printer::println_colour(
                        "todors: You may run the following commands",
                        &TODORSCOLOURS,
                    );
                    Printer::box_print(&HELPLIST, &TODORSCOLOURS);
                } else if split_buffer.len() == 2 {
                    match split_buffer[1].trim() {
                        "list" => {
                            todo.list();
                        }
                        "add" => {
                            _ = Printer::table_print(
                                &addhelp,
                                &[Colour::RedText, Colour::GreenText],
                            );
                        }
                        "remove" => {
                            _ = Printer::table_print(
                                &removehelp,
                                &[Colour::RedText, Colour::GreenText],
                            );
                        }
                        "complete" => {
                            _ = Printer::table_print(
                                &completehelp,
                                &[Colour::RedText, Colour::GreenText],
                            );
                        }
                        _ => {
                            Printer::println_colour(
                                "todors: I didn't quite get that",
                                &TODORSCOLOURS,
                            );
                            Printer::println_colour(
                                "todors: You may run the following commands",
                                &TODORSCOLOURS,
                            );
                            Printer::box_print(&HELPLIST, &TODORSCOLOURS);
                        }
                    }
                } else {
                    Printer::println_colour("todors: I didn't quite get that", &TODORSCOLOURS);
                    Printer::println_colour(
                        "todors: You may run the following commands",
                        &TODORSCOLOURS,
                    );
                    Printer::box_print(&HELPLIST, &TODORSCOLOURS);
                }
            }
            "exit" => {
                Printer::println_colour("todors: goodbye!", &TODORSCOLOURS);
                panic!();
            }
            _ => Printer::println_colour("todors: I didn't quite get that", &TODORSCOLOURS),
        }
        stdin_buffer.clear();
        print!("|> ");
        let _ = std::io::stdout().flush();
    }

    // let date = Local::now().to_rfc3339();
    //
    // Printer::println_colour(&date, &[Colour::BlueText]);
}
