use std::env;
use std::process::Command;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use chrono::Utc;
use cron::Schedule;

mod cron_parser;
mod scheduler;
mod stats;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let personality = stats::Personality::from_str(&args[1])?;
    let mut stats = stats::get_from_personality(personality);
    let mut scheduler = scheduler::Scheduler::new();

    if let Ok(cron_lines) = cron_parser::read_crontab() {
        for cron_line in cron_lines.map_while(Result::ok) {
            let (schedule_expression, cmd_expression) = cron_parser::parse_line(cron_line);
            scheduler.add_job(
                Schedule::from_str(&schedule_expression).unwrap(),
                cmd_expression,
            );
        }
    }

    loop {
        if stats.is_exhausted() {
            println!("tired, bye");
            break;
        }
        let start_at = match scheduler.next_time() {
            Some(start_at) => start_at,
            None => break,
        };
        let until = start_at - Utc::now();
        let sleep_until = match until.to_std() {
            Ok(sleep_until) => sleep_until,
            Err(_) => Duration::new(0, 0),
        };
        let cmds = scheduler.get_next_job(start_at);

        thread::sleep(sleep_until);
        for cmd in cmds.iter() {
            thread::scope(|s| {
                s.spawn(|| {
                    thread::sleep(Duration::new(stats.reaction_time().into(), 0));
                    let cmd = cmd.clone();
                    for _ in 0..stats.tries() {
                        let cron_cmd_output = Command::new("sh")
                            .arg("-c")
                            .arg(&cmd)
                            .output()
                            .unwrap()
                            .stdout;
                        println!("{}", String::from_utf8(cron_cmd_output).unwrap());
                        stats.complete_task();
                    }
                });
            });
        }
    }
    Ok(())
}
