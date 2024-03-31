mod todoer;
use std::io::{BufRead, Write};
use todoer::print::{Colour, Printer};

use crate::todoer::todo;

const TODORS_COLOURS: Colour = Colour::GreenText;
const TODORS_TABLE_COLOURS: [Colour; 2] = [Colour::RedText, Colour::GreenText];
const HELP_LIST: [&'static str; 5] = [
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

    Printer::box_print(&["todors"], &TODORS_COLOURS);
    Printer::cursor();
    _ = std::io::stdout().flush();

    let mut stdin_buffer = String::new();

    let mut stdin_handle = std::io::stdin().lock();
    'main: while stdin_handle.read_line(&mut stdin_buffer).unwrap() > 0 {
        let stdin_buffer_two = stdin_buffer.clone();
        let mut split_buffer = stdin_buffer_two.split(' ').collect::<Vec<_>>();
        match split_buffer.first().expect("split returned len 0").trim() {
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
                split_buffer.swap_remove(0);
                if split_buffer.len() < 1 {
                    Printer::box_print(&["`add` command used incorrectly."], &Colour::RedText);
                    Printer::cursor();
                    _ = std::io::stdout().flush();
                    stdin_buffer.clear();
                    continue 'main;
                }

                let mut name = None;
                let mut desc = None;
                let mut prio = None;

                for item in split_buffer {
                    let key_value = item.split("=").collect::<Vec<_>>();
                    if key_value.len() != 2 {
                        Printer::box_print(&["`add` command used incorrectly."], &Colour::RedText);
                        Printer::cursor();
                        _ = std::io::stdout().flush();
                        stdin_buffer.clear();
                        continue 'main;
                    }
                    match key_value[0] {
                        "name" | "n" => {
                            name = Some(key_value[1].trim().to_string());
                        }
                        "description" | "d" => {
                            desc = Some(key_value[1].trim().to_string());
                        }
                        "priority" | "p" => {
                            prio = match key_value[1].trim().to_lowercase().as_str() {
                                "high" => Some(Colour::RedText),
                                "medium" => Some(Colour::MagentaText),
                                "low" => Some(Colour::CyanText),
                                _ => {
                                    Printer::box_print(&["Provided `priority` value was not recognized. Defaulting to 'low'"], &TODORS_COLOURS);
                                    None
                                }
                            };
                        }
                        _ => {
                            Printer::box_print(
                                &[format!("Invalid key input '{}'", key_value[0]).as_str()],
                                &Colour::RedText,
                            );
                            Printer::cursor();
                            _ = std::io::stdout().flush();
                            stdin_buffer.clear();
                            continue 'main;
                        }
                    }
                }

                let name = match name {
                    None => {
                        Printer::box_print(&["The `name` key input was not given!"], &Colour::RedText);
                        Printer::cursor();
                        _ = std::io::stdout().flush();
                        stdin_buffer.clear();
                        continue 'main;
                    }
                    Some(name) => name 
                };

                match todo.add(name.clone(), desc, prio) {
                    Err(e) => Printer::box_print(&[e.0.as_str()], &Colour::RedText),
                    Ok(_) => Printer::box_print(
                        &[format!("Successfully added {}!", name).as_str()],
                        &TODORS_COLOURS,
                    ),
                };
            }
            "help" => {
                if split_buffer.len() == 1 {
                    Printer::box_print(&["You may run the following commands"], &TODORS_COLOURS);
                    Printer::box_print(&HELP_LIST, &TODORS_COLOURS);
                } else if split_buffer.len() == 2 {
                    match split_buffer[1].trim() {
                        "list" => {
                            _ = Printer::table_print(&listhelp, &TODORS_TABLE_COLOURS);
                        }
                        "add" => {
                            _ = Printer::table_print(&addhelp, &TODORS_TABLE_COLOURS);
                        }
                        "remove" => {
                            _ = Printer::table_print(&removehelp, &TODORS_TABLE_COLOURS);
                        }
                        "complete" => {
                            _ = Printer::table_print(&completehelp, &TODORS_TABLE_COLOURS);
                        }
                        _ => {
                            Printer::box_print(&["Unrecognized command"], &TODORS_COLOURS);
                            Printer::box_print(&["You may run the following commands"], &TODORS_COLOURS);
                            Printer::box_print(&HELP_LIST, &TODORS_COLOURS);
                        }
                    }
                } else {
                    Printer::box_print(&["Unrecognized command"], &TODORS_COLOURS);
                    Printer::println_colour(
                        "todors: You may run the following commands",
                        &TODORS_COLOURS,
                    );
                    Printer::box_print(&HELP_LIST, &TODORS_COLOURS);
                }
            }
            "exit" => {
                Printer::box_print(&["Goodbye!"], &TODORS_COLOURS);
                panic!();
            }
            _ => Printer::box_print(&["Unrecognized command"], &TODORS_COLOURS),
        }
        stdin_buffer.clear();
        Printer::cursor();
        let _ = std::io::stdout().flush();
    }
}
