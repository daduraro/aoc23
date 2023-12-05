use std::io;
use regex::Regex;

fn parse() -> io::Result<Vec<String>> {
    let mut lines = Vec::new();
    for line in io::stdin().lines() {
        lines.push(line?);
    }
    Ok(lines)
}

fn first_star<T: AsRef<str>>(lines: &[T]) -> io::Result<i32> {
    let first_digit = Regex::new(r"(\d)").unwrap();
    let last_digit = Regex::new(r".*(\d)").unwrap();
    lines.iter().map(|line| {
        let a = first_digit.captures(line.as_ref()).unwrap().get(1).unwrap().as_str();
        let b = last_digit.captures(line.as_ref()).unwrap().get(1).unwrap().as_str();
        a.parse::<i32>().and_then(|a| b.parse::<i32>().map(|b| 10*a + b))
    }).try_fold(0, |acc, x| {
        x.map(|x| acc + x)
    }).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e) )
}

fn second_star<T: AsRef<str>>(lines: &[T]) -> io::Result<i32> {
    let digit = r"(\d|one|two|three|four|five|six|seven|eight|nine)";
    let first_digit = Regex::new(digit).unwrap();
    let last_digit = Regex::new(format!(r".*{}", digit).as_str()).unwrap();
    let parse_number = |s| {
        match s {
            "one" => Ok(1),
            "two" => Ok(2),
            "three" => Ok(3),
            "four" => Ok(4),
            "five" => Ok(5),
            "six" => Ok(6),
            "seven" => Ok(7),
            "eight" => Ok(8),
            "nine" => Ok(9),
            x => x.parse::<i32>()
        }
    };
    lines.iter().map(|line| {
        let a = first_digit.captures(line.as_ref()).unwrap().get(1).unwrap().as_str();
        let b = last_digit.captures(line.as_ref()).unwrap().get(1).unwrap().as_str();
        parse_number(a).and_then(|a| parse_number(b).map(|b| 10*a + b))
    }).try_fold(0, |acc, x| {
        x.map(|x| acc + x)
    }).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e) )
}

fn main() -> io::Result<()> {
    let lines = parse()?;
    println!("First star: {}", first_star(&lines)?);
    println!("Second star: {}", second_star(&lines)?);
    Ok(())
}
