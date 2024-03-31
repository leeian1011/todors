pub const RED_TEXT: &'static str = "\u{001b}[31m";
pub const GREEN_TEXT: &'static str = "\u{001b}[32m";
pub const BLUE_TEXT: &'static str = "\u{001b}[34m";
pub const MAGENTA_TEXT: &'static str = "\u{001b}[35m";
pub const CYAN_TEXT: &'static str = "\u{001b}[36m";
pub const RESET: &'static str = "\u{001b}[0m";
// pub const RED_BG: &'static str = "\u{001b}[41m";
// pub const GREEN_BG: &'static str = "\u{001b}[42m";
// pub const BLUE_BG: &'static str = "\u{001b}[44m";

const LINECOLOURS: Colour = Colour::BlueText;

#[derive(serde::Deserialize, Clone, Copy)]
pub enum Colour {
    RedText,
    GreenText,
    BlueText,
    MagentaText,
    CyanText,
}

fn get_colour_code(colour: &Colour) -> &'static str {
    match colour {
        Colour::RedText => RED_TEXT,
        Colour::GreenText => GREEN_TEXT,
        Colour::BlueText => BLUE_TEXT,
        Colour::MagentaText => MAGENTA_TEXT,
        Colour::CyanText => CYAN_TEXT,
    }
}

pub struct Printer;

impl Printer {
    pub fn println_colour(subject: &str, colour: &Colour) {
        let mut outcome = String::new();
        outcome.push_str(get_colour_code(colour));

        outcome += &subject.to_string();
        outcome.push_str(RESET);
        println!("{}", outcome);
    }

    pub fn print_colour(subject: &str, colour: &Colour) {
        let mut outcome = String::new();
        outcome.push_str(get_colour_code(colour));

        outcome += &subject.to_string();
        outcome.push_str(RESET);
        print!("{}", outcome);
    }

    pub fn cursor() {
        print!("|> ");
    }

    fn gen_line(length: usize) {
        Self::print_colour("+", &LINECOLOURS);
        for _ in 0..length {
            Self::print_colour("=", &LINECOLOURS);
        }
        Self::println_colour("+", &LINECOLOURS);
    }

    fn start_line(length: usize) {
        Self::print_colour("+", &LINECOLOURS);
        for _ in 0..length {
            Self::print_colour("=", &LINECOLOURS);
        }
        Self::print_colour("+", &LINECOLOURS);
    }

    fn continue_line(length: usize) {
        for _ in 0..length {
            Self::print_colour("=", &LINECOLOURS);
        }
        Self::print_colour("+", &LINECOLOURS);
    }

    fn close() {
        print!("\n");
    }

    fn gen_segment(subject: &str, colour: &Colour) {
        Self::print_colour("|", &LINECOLOURS);
        Self::print_colour(subject, colour);
    }

    fn add_padding(max_length: usize, text_length: usize) {
        let length = max_length - text_length;
        for _ in 0..length {
            print!(" ");
        }
        Self::print_colour("|", &LINECOLOURS);
    }

    pub fn box_print(list: &[&str], colours: &Colour) {
        let mut longest = 0;
        for item in list {
            if item.len() > longest {
                longest = item.len();
            }
        }
        Self::gen_line(longest);
        for item in list {
            Self::gen_segment(item, colours);
            Self::add_padding(longest, item.len());
            Self::close();
        }
        Self::gen_line(longest);
    }

    pub fn table_print(keys: &[Vec<&str>], colours: &[Colour]) -> Result<(), PrinterError> {
        if keys.len() < 2 {
            return Err(PrinterError {
                error: String::from("Can't produce table of one key"),
            });
        }

        let mut max_key_length: Vec<usize> = vec![];
        for key in keys {
            let mut key_longest = 0;
            for subject in key {
                if subject.len() > key_longest {
                    key_longest = subject.len();
                }
            }
            max_key_length.push(key_longest);
        }

        for i in 0..max_key_length.len() {
            if max_key_length.len() == 1 {
                Self::gen_line(max_key_length[i]);
                break;
            }
            if i == 0 {
                Self::start_line(max_key_length[i]);
                continue;
            }

            Self::continue_line(max_key_length[i]);
        }
        Self::close();

        for i in 0..keys[0].len() {
            for j in 0..keys.len() {
                if j == 0 {
                    Self::gen_segment(keys[j][i], &colours[i]);
                    Self::add_padding(max_key_length[j], keys[j][i].len());
                    continue;
                }
                Self::print_colour(keys[j][i], &colours[i]);
                Self::add_padding(max_key_length[j], keys[j][i].len());
            }
            Self::close();
            for i in 0..max_key_length.len() {
                if max_key_length.len() == 1 {
                    Self::gen_line(max_key_length[i]);
                    break;
                }
                if i == 0 {
                    Self::start_line(max_key_length[i]);
                    continue;
                }

                Self::continue_line(max_key_length[i]);
            }
            Self::close();
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct PrinterError {
    error: String,
}

impl std::fmt::Display for PrinterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pritner err")
    }
}

impl std::error::Error for PrinterError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        "print error"
    }
}
