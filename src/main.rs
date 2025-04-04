use chrono::prelude::*;
use chrono_tz::{Tz, TZ_VARIANTS};
use clap::Parser;
use colored::*;
use crossterm::execute;
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode, ClearType};
use std::io::{stdout, Write};
use std::{fs, io::Error}; //I need to fix error handling. This was auto imported somehow.

#[derive(Debug, Parser)]
struct Cli {
    //eventually we want to be able to pass in hypothetical times.
    //for now, will just work with "right now"
    ///Timezone names. Avoid names with spaces until I fix the parsing.
    ///If blank, retrieves last results saved with -s.
    #[arg(value_delimiter = ',')]
    //couldn't get the delimiters to work, just use spaces for now
    timezone: Vec<String>,

    ///Saves the results. This overwrites previous saves.
    #[arg(short = 's', long = "save")]
    save: bool,
}

fn main() {
    let args = Cli::parse();
    //println!("{:?}", args); //for debugging. YES I KNOW i should use proper logging TODO etc.
    let mut zonesofinterest: Vec<Tz> = Vec::new();
    for zone in &args.timezone {
        let mut possiblezones = parsezone(zone);
        zonesofinterest.append(&mut possiblezones);
    }
    //let utc: DateTime<Utc> = Utc::now();
    //zonesofinterest.sort(); Tz does not implement Ord. I wonder if there is a PartialOrd way
    let local: DateTime<Local> = Local::now();
    let _ = execute!(stdout(), terminal::Clear(ClearType::All));
    println!(
        "{:<20}{:02}{}{}",
        //local.timezone(), TODO: fix later. Chrono doesn't pretty print, but tz_chrono does
        "System Time".green().bold().underline(),
        local.hour().to_string().green().bold().underline(),
        ":".green().bold().underline(), //just mind boggling workaround to allow continuous underline
        format!("{:02}", local.minute()).green().bold().underline() //Claude saved me here - the formatter doesn't work on strings, only numbers, but the ANSI formatting doesn't work on numbers, only strings.
    );

    if args.save == true {
        cachezones(&zonesofinterest); //TODO unused result, fix later
    }

    if args.timezone.is_empty() {
        zonesofinterest.append(&mut retrievezones().unwrap()); //this is clearly wrong its not a reference
                                                               //why does this work???
    }

    for z in zonesofinterest {
        let timez = local.with_timezone(&z);

        //formatters don't work as expected so I am formatting as a string
        //Claude came up with that fix. sigh. thanks Claude, my hero
        let timezstring = format!("{}", timez.timezone());
        println!(
            "{:<20}{:02}:{:02}",
            timezstring,
            timez.hour(),
            timez.minute()
        );
    }
    //printzones();
}

fn cachezones(zones: &Vec<Tz>) -> Result<(), Error> {
    //this is probably not idiomatic - something about the weird vec is striking to me
    let mut towrite = Vec::new();
    for z in zones {
        towrite.push(z.to_string());
    }
    fs::write("tzcache.txt", towrite.join("\n"))?;
    Ok(())
}

fn retrievezones() -> Result<Vec<Tz>, Error> {
    let mut zones: Vec<Tz> = Vec::new();
    let toread = fs::read_to_string("tzcache.txt")?; //TODO need to do proper open/close in case file does not exist
    let toconvert: Vec<&str> = toread.lines().collect();
    for z in toconvert {
        let tz: Tz = z.parse().unwrap(); //TODO this is highly likely to throw parse err in prod
        zones.push(tz);
    }
    Ok(zones)
}

fn parsezone(input: &String) -> Vec<Tz> {
    //this is very bad - need to implement fuzzy matching or an interactive menu
    //I suppose the user has specific timezones in mind.
    //they aren't... trying to figure out what time zone they want.
    let mut possiblezones: Vec<Tz> = Vec::new();
    for tz in TZ_VARIANTS {
        if tz
            .name()
            .to_lowercase()
            .contains(&input.as_str().to_lowercase())
        {
            possiblezones.push(tz);
        }
    }
    possiblezones //interesting. if empty its an error of sorts but not a programmatic one...
}

//in the future we may want to create an option to print all valid zones.
//I forgot what I was doing here and broke it.... fix later
//would need to curate down the full list or maybe display slow chyrons for each UTC offset
fn printzones() {
    todo!("implement a print zone function");
    let utc_now: DateTime<Utc> = Utc::now();
    for tz in TZ_VARIANTS {
        let local_time = utc_now.with_timezone(&tz);
        let offset = local_time.offset().fix().local_minus_utc();
    }
}
