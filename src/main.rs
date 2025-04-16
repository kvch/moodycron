// TODO run as daemon
// TODO open crontab
// TODO look for update of cron tab file
// TODO track programs to run
// TODO run programs
// TODO log moods, stamina
use cron::Schedule;
//use crono::Utc;
use std::fs::File;
use std::io::prelude::*;
use std::splitn;
use std::str::FromStr;

// Cron with stamina
// Cron with moods

//fn read_crontab() -> Option<String> {
//    let mut file = match File::open("cron") {
//        Ok(file) => file,
//        Err(_) => return None,
//    };
//    let mut contents = String::new();
//    match file.read_to_string(&mut contents) {
//        Ok(_) => return Some(contents),
//        Err(_) => return None,
//    };
//}
fn read_crontab() -> Option<String> {
    let mut file = File::open("cron").ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    return Some(contents);
}

fn main() {
    println!("Hello, world!");
    let contents = match read_crontab() {
        Some(contents) => contents,
        None => String::new(),
    };
    println!("AFDAFS {}", contents);
    let cron_expression = splitn(&contents.to_owned(), " ", 6);
    println!("{}", cron_expression);
}
