use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub struct Scheduler {
    starts: Vec<DateTime<Utc>>,
    jobs: HashMap<DateTime<Utc>, Vec<Job>>,
}

pub struct Job {
    schedule: cron::Schedule,
    cmd: String,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        return Scheduler {
            starts: Vec::<DateTime<Utc>>::new(),
            jobs: HashMap::new(),
        };
    }

    pub fn add_job(self: &mut Scheduler, schedule: cron::Schedule, cmd: String) {
        let start_time = schedule.upcoming(Utc).take(1).next().unwrap();
        self.starts.push(start_time);
        self.starts.sort();
        self.starts.dedup();
        if self.jobs.contains_key(&start_time) {
            let mut jobs = self.jobs.remove(&start_time).unwrap();
            jobs.push(Job {
                schedule: schedule,
                cmd: cmd,
            });
            self.jobs.insert(start_time.clone(), jobs);
        } else {
            self.jobs.insert(
                start_time.clone(),
                vec![Job {
                    schedule: schedule,
                    cmd: cmd,
                }],
            );
        }
    }
    pub fn next_time(self: &mut Scheduler) -> Option<DateTime<Utc>> {
        if self.starts.len() == 0 {
            return None;
        }
        return Some(self.starts.remove(0));
    }
    pub fn get_next_job(self: &mut Scheduler, time: DateTime<Utc>) -> Vec<String> {
        let jobs = self.jobs.remove(&time).unwrap();
        let mut cmds = Vec::<String>::new();
        for job in jobs.into_iter() {
            let next_time = job.schedule.upcoming(Utc).take(1).next().unwrap();
            self.starts.push(next_time);
            self.starts.sort();
            self.starts.dedup();
            cmds.push(job.cmd.clone());
            if self.jobs.contains_key(&next_time) {
                let mut jobs_next_time = self.jobs.remove(&next_time).unwrap();
                jobs_next_time.push(job);
                self.jobs.insert(next_time.clone(), jobs_next_time);
            } else {
                self.jobs.insert(next_time.clone(), vec![job]);
            }
        }
        return cmds;
    }
}
