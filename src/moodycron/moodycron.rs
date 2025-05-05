use log::{error, info};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use sd_notify;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use tokio::signal::unix::{SignalKind, signal};

use chrono::Utc;
use cron::Schedule;

use crate::moodycron::cron_parser;
use crate::moodycron::scheduler;
use crate::moodycron::stats;

struct ReloadSchedulerMessage {}

pub struct App {
    stats: stats::CronStats,
    scheduler: scheduler::Scheduler,

    file_watcher_tx: Sender<ReloadSchedulerMessage>,
    signal_tx: Sender<ReloadSchedulerMessage>,
    scheduler_reload_trigger_rx: Receiver<ReloadSchedulerMessage>,
}

unsafe impl Sync for App {}

impl App {
    pub fn new(personality: stats::Personality) -> App {
        let (file_watcher_tx, scheduler_rx) = mpsc::channel();
        App {
            stats: stats::get_from_personality(personality),
            scheduler: Self::load_scheduler(),
            signal_tx: file_watcher_tx.clone(),
            file_watcher_tx: file_watcher_tx,
            scheduler_reload_trigger_rx: scheduler_rx,
        }
    }

    fn load_scheduler() -> scheduler::Scheduler {
        let _ = sd_notify::notify(true, &[sd_notify::NotifyState::Reloading]);

        let mut scheduler = scheduler::Scheduler::default();
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

    pub async fn run(mut self) {
        info!("hallo, hallo");

        tokio::task::spawn(async move {
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
                            _ = self.file_watcher_tx.send(ReloadSchedulerMessage {});
                        }
                        _ => (),
                    },
                    Err(error) => error!("Error: {error:?}"),
                }
            }
        });

        tokio::task::spawn(async move {
            let mut stream = signal(SignalKind::hangup()).unwrap();
            loop {
                stream.recv().await;
                info!("Received signal SIGHUP");
                _ = self.signal_tx.send(ReloadSchedulerMessage {});
            }
        });

        loop {
            if self.stats.is_exhausted() {
                info!("tired, bye");
                break;
            }
            match self.scheduler_reload_trigger_rx.try_recv() {
                Ok(_) => {
                    self.scheduler = Self::load_scheduler();
                    info!("reloaded");
                }
                _ => (),
            };
            let start_at = match self.scheduler.next_time() {
                Some(start_at) => start_at,
                None => break,
            };

            let cmds = self.scheduler.get_next_cmd(start_at);
            let until = start_at - Utc::now();
            match until.to_std() {
                Ok(sleep_until) => thread::sleep(sleep_until),
                Err(_) => (),
            };

            for cmd in cmds.iter() {
                thread::scope(|s| {
                    s.spawn(|| {
                        thread::sleep(Duration::new(self.stats.reaction_time().into(), 0));
                        let cmd = cmd.clone();
                        for _ in 0..self.stats.tries() {
                            if self.stats.is_exhausted() {
                                return;
                            }
                            let cron_cmd_output = Command::new("sh").arg("-c").arg(&cmd).output();
                            info!("{:?}", cron_cmd_output);
                            self.stats.complete_task();
                        }
                    });
                });
            }
        }
    }
}
