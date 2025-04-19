use std::fs::File;
use std::io::{self, BufRead};

pub fn read_crontab() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("cron")?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_line(cron_line: String) -> (String, String) {
    let mut cron_expression: Vec<&str> = (&cron_line).splitn(7, " ").collect();
    let cmd_expression = cron_expression.pop().unwrap();
    let schedule_expression = cron_expression.into_iter().collect::<String>();
    return (schedule_expression, cmd_expression.to_string());
}
