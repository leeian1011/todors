pub const RED_TEXT: &'static str = "\u{001b}[31m";
pub const GREEN_TEXT: &'static str = "\u{001b}[32m";
pub const BLUE_TEXT: &'static str = "\u{001b}[34m";
pub const RED_BG: &'static str = "\u{001b}[41m";
pub const GREEN_BG: &'static str = "\u{001b}[42m";
pub const BLUE_BG: &'static str = "\u{001b}[44m";
pub const RESET: &'static str = "\u{001b}[0m";

const LINECOLOURS: [Colour;1] = [Colour::BlueText];

#[allow(dead_code)]
pub enum Colour {
    RedText,
    GreenText,
    BlueText,
    RedBg,
    GreenBg,
    BlueBg,
}

fn get_colour_code(colour: &Colour) -> &'static str {
    match colour {
        Colour::RedText => RED_TEXT,
        Colour::GreenText => GREEN_TEXT,
        Colour::BlueText => BLUE_TEXT,
        Colour::RedBg => RED_BG,
        Colour::GreenBg => GREEN_BG,
        Colour::BlueBg => BLUE_BG,
    }
}

pub struct Printer;

impl Printer {
    pub fn println_colour(subject: &str, colours: &[Colour]) {
        let codes = colours
            .iter()
            .map(|colour| get_colour_code(colour))
            .collect::<Vec<_>>();
        let mut outcome = String::new();
        for code in codes {
            outcome.push_str(code);
        }

        outcome += &subject.to_string();
        outcome.push_str(RESET);
        println!("{}", outcome);
    }

    pub fn print_colour(subject: &str, colours: &[Colour]) {
        let codes = colours
            .iter()
            .map(|colour| get_colour_code(colour))
            .collect::<Vec<_>>();
        let mut outcome = String::new();
        for code in codes {
            outcome.push_str(code);
        }

        outcome += &subject.to_string();
        outcome.push_str(RESET);
        print!("{}", outcome);
    }

    pub fn print_colour_no_reset(subject: &str, colours: &[Colour]) {
        let codes = colours
            .iter()
            .map(|colour| get_colour_code(colour))
            .collect::<Vec<_>>();
        let mut outcome = String::new();
        for code in codes {
            outcome.push_str(code);
        }

        outcome += &subject.to_string();
        print!("{}", outcome);
    }

    pub fn println_colour_no_reset(subject: &str, colours: &[Colour]) {
        let codes = colours
            .iter()
            .map(|colour| get_colour_code(colour))
            .collect::<Vec<_>>();
        let mut outcome = String::new();
        for code in codes {
            outcome.push_str(code);
        }

        outcome += &subject.to_string();
        println!("{}", outcome);
    }
    fn gen_line(length: usize) {
        Self::print_colour("+", &LINECOLOURS);
        for _ in 0..length {
            Self::print_colour("=", &LINECOLOURS);
        }
        Self::println_colour("+", &LINECOLOURS);
    }

    fn gen_segment(subject: &str, colours: &[Colour]) {
        Self::print_colour("|", &LINECOLOURS);
        Self::print_colour(subject, colours);
    }

    fn add_padding(max_length: usize, text_length: usize) {
        let length = max_length - text_length;
        for _ in 0..length {
            print!(" ");
        }
        Self::println_colour("|", &LINECOLOURS);
    }

    pub fn box_print(list: &[&str], colours: &[Colour]) {
        let mut longest = 0;
        for item in list {
            if item.len() > longest {
                longest = item.len();
            }
        }
        Self::gen_line(longest);
        for item in list {
            Printer::gen_segment(item, colours);
            Printer::add_padding(longest, item.len());
        }
        Self::gen_line(longest);
    }
}


