use chrono::{Datelike, Duration, Utc, Weekday};
use std::ops::Add;

static USAGE: &'static str = "usage: thirtyday [-s DAYS] MESSAGE...";

static USAGE_LONG: &'static str = r#"thirtyday - generate a 'checkbox' for a 30-day challenge

usage: thirtyday [-s DAYS] MESSAGE...

Options:
    -s DAYS    Shift the start by DAYS days
    -d         Prefix message with the start and end date

This will start from the current day and generate a checkbox for the next
30 days, with the initial weekday letter, e.g.:

    [WTFSS MTWTFSS MTWTFSS MTWTFSS MTWT]

You can copy this template into a plaintext file for keeping a log on your
challenge, replacing each day's letter with an '.' or 'x' to represent fail
or success (or use whatever symbols you like). At the end of the month, you
can see something like...

    [xxxxx xxxx..x xx.x..x xxxx.xx xxxx]

...and add in any notes, such as 'sick in week 2'.
"#;

fn weekday_letter(w: Weekday) -> char {
    match w {
        Weekday::Mon => 'M',
        Weekday::Tue => 'T',
        Weekday::Wed => 'W',
        Weekday::Thu => 'T',
        Weekday::Fri => 'F',
        Weekday::Sat => 'S',
        Weekday::Sun => 'S',
    }
}

#[derive(PartialEq)]
enum DigitParsing {
    NoDigit,
    WaitingForDigit,
    GotDigit,
}

fn parse_args() -> Result<(String, u8), Box<dyn ::std::error::Error>> {
    let mut args = Vec::new();
    let mut shift_state = DigitParsing::NoDigit;
    let mut shift = 0;
    let mut with_date = false;
    for arg in std::env::args().skip(1) {
        if arg == "-h" {
            println!("{}", USAGE);
            std::process::exit(0);
        } else if arg == "--help" {
            println!("{}", USAGE_LONG);
            std::process::exit(0);
        } else if arg == "-d" {
            with_date = true
        } else if arg == "-s" {
            shift_state = DigitParsing::WaitingForDigit;
            continue;
        } else if shift_state == DigitParsing::WaitingForDigit {
            shift = arg.parse()?;
            shift_state = DigitParsing::GotDigit;
        } else {
            args.push(arg);
        }
    }
    if shift_state == DigitParsing::WaitingForDigit {
        Err("Flag -d requires a digit afterwards, for number of days to shift".into())
    } else {
        let message = if with_date {
            let date_start = Utc::now().add(Duration::days(shift));
            let date_end = date_start.add(Duration::days(30));
            let dates = format!(
                "{} to {}",
                date_start.format("%Y-%m-%d"),
                date_end.format("%Y-%m-%d")
            );
            if args.is_empty() {
                dates
            } else {
                format!("{} -- {}", dates, args.join(" "))
            }
        } else {
            args.join(" ")
        };
        Ok((message, shift as u8))
    }
}

fn main() {
    let (message, shift) = match parse_args() {
        Ok((m, s)) => (m, s),
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("{}", USAGE);
            return;
        }
    };
    let mut t = Utc::now().weekday();
    for _ in 0..shift {
        t = t.succ();
    }
    let mut output = String::with_capacity(35);
    for i in 0..30 {
        output.push(weekday_letter(t));
        if t == Weekday::Sun && i != 29 {
            output.push(' ');
        }
        t = t.succ();
    }
    if !message.is_empty() {
        println!("{}", message);
    }
    println!("[{}]", output);
}
