use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scheduler {
    starts: Vec<DateTime<Utc>>,
    jobs: HashMap<DateTime<Utc>, Vec<Job>>,
}

#[derive(Debug)]
struct Job {
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
        self.schedule_job(Job {
            schedule: schedule,
            cmd: cmd,
        });
    }

    pub fn next_time(self: &mut Scheduler) -> Option<DateTime<Utc>> {
        if self.starts.len() == 0 {
            return None;
        }
        return Some(self.starts.remove(0));
    }

    pub fn get_next_cmd(self: &mut Scheduler, time: DateTime<Utc>) -> Vec<String> {
        let jobs = self.jobs.remove(&time).unwrap();
        let mut cmds = Vec::<String>::new();
        for job in jobs.into_iter() {
            cmds.push(job.cmd.clone());
            self.schedule_job(job);
        }
        return cmds;
    }

    fn schedule_job(self: &mut Scheduler, job: Job) {
        let next_time = job.schedule.upcoming(Utc).take(1).next().unwrap();
        self.starts.push(next_time);
        self.starts.sort();
        self.starts.dedup();
        if self.jobs.contains_key(&next_time) {
            let mut jobs_next_time = self.jobs.remove(&next_time).unwrap();
            jobs_next_time.push(job);
            self.jobs.insert(next_time.clone(), jobs_next_time);
        } else {
            self.jobs.insert(next_time.clone(), vec![job]);
        }
    }
}
