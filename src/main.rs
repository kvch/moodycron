use log::{error, info};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use sd_notify;
use signal_hook::{consts::SIGHUP, iterator::Signals};
use std::env;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use std::sync::mpsc;
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
    let mut scheduler = load_scheduler();

    env_logger::init();
    info!("hallo, hallo");

    let (file_watcher_tx, scheduler_rx) = mpsc::channel();
    let signal_tx = file_watcher_tx.clone();

    thread::spawn(move || {
        let (tx, rx) = mpsc::channel();

        let mut watcher = match RecommendedWatcher::new(tx, Config::default()) {
            Ok(watcher) => watcher,
            Err(_) => return,
        };

        watcher
            .watch(Path::new("cron"), RecursiveMode::Recursive)
            .unwrap();

        for res in rx {
            match res {
                Ok(event) => match event.kind {
                    EventKind::Modify(_) | EventKind::Create(_) => {
                        info!("reloading triggered");
                        _ = file_watcher_tx.send("reload");
                    }
                    _ => (),
                },
                Err(error) => error!("Error: {error:?}"),
            }
        }
    });

    let mut signals = Signals::new([SIGHUP])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            info!("Received signal {:?}", sig);
            _ = signal_tx.send("reload");
        }
    });

    loop {
        if stats.is_exhausted() {
            info!("tired, bye");
            break;
        }
        match scheduler_rx.try_recv() {
            Ok(_) => {
                scheduler = load_scheduler();
                info!("reloaded");
            }
            _ => (),
        };
        let start_at = match scheduler.next_time() {
            Some(start_at) => start_at,
            None => break,
        };

        let cmds = scheduler.get_next_cmd(start_at);
        let until = start_at - Utc::now();
        match until.to_std() {
            Ok(sleep_until) => thread::sleep(sleep_until),
            Err(_) => (),
        };

        for cmd in cmds.iter() {
            thread::scope(|s| {
                s.spawn(|| {
                    thread::sleep(Duration::new(stats.reaction_time().into(), 0));
                    let cmd = cmd.clone();
                    for _ in 0..stats.tries() {
                        if stats.is_exhausted() {
                            return;
                        }
                        let cron_cmd_output = Command::new("sh").arg("-c").arg(&cmd).output();
                        info!("{:?}", cron_cmd_output);
                        stats.complete_task();
                    }
                });
            });
        }
    }
    Ok(())
}

fn load_scheduler() -> scheduler::Scheduler {
    let _ = sd_notify::notify(true, &[sd_notify::NotifyState::Reloading]);

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

    let _ = sd_notify::notify(true, &[sd_notify::NotifyState::Ready]);

    return scheduler;
}
