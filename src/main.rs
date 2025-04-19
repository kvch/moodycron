use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use chrono::Utc;
use cron::Schedule;

mod cron_parser;
mod stats;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let personality = stats::Personality::from_str(&args[1])?;
    let stats = stats::get_from_personality(personality);

    let mut cron_jobs: HashMap<String, String> = HashMap::new();
    if let Ok(cron_lines) = cron_parser::read_crontab() {
        for cron_line in cron_lines.map_while(Result::ok) {
            let (schedule_expression, cmd_expression) = cron_parser::parse_line(cron_line);
            cron_jobs.insert(schedule_expression, cmd_expression);
        }
    }

    let handle = thread::spawn(move || {
        println!("start");
        loop {
            if stats.is_exhausted() {
                break;
            }
            thread::sleep(Duration::from_secs(10));
        }
        println!("stop");
    });

    for (schedule_expression, cmd_expression) in &cron_jobs {
        let sch = schedule_expression.clone();
        let cmd = cmd_expression.clone();
        thread::spawn(move || {
            let schedule = Schedule::from_str(&sch).unwrap();
            for datetime in schedule.upcoming(Utc) {
                let now = Utc::now();
                let until = datetime - now;
                thread::sleep(until.to_std().unwrap());
                let cron_cmd_output = Command::new("sh")
                    .arg("-c")
                    .arg(&cmd)
                    .output()
                    .unwrap()
                    .stdout;
                println!("{}", String::from_utf8(cron_cmd_output).unwrap());
            }
        });
    }
    handle.join().unwrap();
    Ok(())
}
