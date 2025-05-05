use std::fs::File;
use std::io::{self, BufRead};

pub fn read_crontab() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("cron")?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_line(cron_line: String) -> (String, String) {
    let mut cron_expression: Vec<&str> = (&cron_line).splitn(7, " ").collect();
    let cmd_expression = match cron_expression.pop() {
        Some(cmd_expression) => cmd_expression,
        None => {
            eprintln!("failed to parse line from cron file");
            return (String::new(), String::new());
        }
    };
    let schedule_expression = cron_expression.into_iter().collect::<String>();
    return (schedule_expression, cmd_expression.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_parse_line() {
        let (_, cmd) = parse_line(String::from_str(r#"0/2 * * * * * echo -n "hello""#).unwrap());
        assert_eq!(cmd, r#"echo -n "hello""#);
        let (_, cmd) = parse_line(String::from_str(r#"0/2 * * * * * echo -n "hali""#).unwrap());
        assert_eq!(cmd, r#"echo -n "hali""#);
        let (_, cmd) = parse_line(String::from_str("0/2 * * * * * cargo run eager").unwrap());
        assert_eq!(cmd, "cargo run eager");
    }
}
