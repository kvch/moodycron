// TODO run as daemon
// TODO look for update of cron tab file
// TODO track programs to run
// TODO run programs
// TODO log moods, stamina
use chrono::Utc;
use cron::Schedule;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::str::FromStr;
use std::thread;

struct CronStats {
    stamina: u8,
    reflexes: u8,
    dexterity: u8,
}

fn parse_cron_line(cron_line: String) -> (String, String) {
    let mut cron_expression: Vec<&str> = (&cron_line).splitn(7, " ").collect();
    let cmd_expression = cron_expression.pop().unwrap();
    let schedule_expression = cron_expression.into_iter().collect::<String>();
    return (schedule_expression, cmd_expression.to_string());
}

fn read_crontab() -> Option<String> {
    let mut file = File::open("cron").ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    return Some(contents);
}

fn main() {
    let contents = match read_crontab() {
        Some(contents) => contents,
        None => return,
    };
    let (schedule_expression, cmd_expression) = parse_cron_line(contents);
    let schedule = Schedule::from_str(&schedule_expression).unwrap();
    for datetime in schedule.upcoming(Utc).take(1) {
        let now = Utc::now();
        let until = datetime - now;
        thread::sleep(until.to_std().unwrap());
        let cron_cmd_output = Command::new("sh")
            .arg("-c")
            .arg(&cmd_expression)
            .output()
            .unwrap()
            .stdout;
        println!("{}", String::from_utf8(cron_cmd_output).unwrap());
    }
}
