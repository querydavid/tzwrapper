use chrono::prelude::*;
use chrono_tz::{Tz, TZ_VARIANTS};
use clap::Parser;
use std::collections::HashMap;

#[derive(Debug, Parser)]
struct Cli {
    //time: String, eventually we want to be able to manipulate the time.
    //for now, will just work with "right now"
    #[arg(value_delimiter = ',')]
    timezone: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    //println!("{:?}", args); for debugging. YES I KNOW i should use proper logging TODO etc.
    let mut zonesofinterest: Vec<Tz> = Vec::new();
    for zone in args.timezone {
        let mut possiblezones = parsezone(zone);
        zonesofinterest.append(&mut possiblezones);
    }
    //let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();

    println!(
        "{:<20}{:02}:{:02}",
        //local.timezone(), TODO: fix later. The chrono impl doesn't pretty print
        "System Time",
        local.hour(),
        local.minute()
    );

    for z in zonesofinterest {
        let timez = local.with_timezone(&z);

        //formatters don't work as expected so I am formatting as a string
        //Claude came up with that fix. sigh.
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

fn parsezone(input: String) -> Vec<Tz> {
    //this is very bad - need to implement fuzzy matching.
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
    possiblezones
}

//in the future we may want to create an option to print all valid zones.
//I forgot what I was doing here and broke it.... fix later
fn printzones() {
    let utc_now: DateTime<Utc> = Utc::now();
    for tz in TZ_VARIANTS {
        let local_time = utc_now.with_timezone(&tz);
        let offset = local_time.offset().fix().local_minus_utc();
        //println!("UTC {:02}:{:02}, {}", hours, minutes, tz.name());
    }
}
