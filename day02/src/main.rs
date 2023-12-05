use regex::Regex;
use std::io;
use std::cmp::max;

#[derive(Debug, Default)]
struct CubeCount {
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}

#[derive(Debug)]
struct Game {
    game_id: u32,
    rounds: Vec<CubeCount>,
}

trait Queryable {
    fn query(&self, query: &CubeCount) -> bool;
}

impl Queryable for Game {
    fn query(&self, query: &CubeCount) -> bool {
        self.rounds.iter().all(|v| {
            v.red_count <= query.red_count && v.green_count <= query.green_count && v.blue_count <= query.blue_count
        })
    }
}

fn parse() -> io::Result<Vec<Game>> {
    let game_re = Regex::new(r"Game (\d+)").unwrap();
    let cube_re = Regex::new(r"(\d+) (\w+)").unwrap();

    io::stdin().lines().map(|line| {
        let line = line?;
        let mut it = line.split(':');
        let header = it.next().ok_or(io::ErrorKind::InvalidData)?;
        let body = it.next().ok_or(io::ErrorKind::InvalidData)?;
        if it.next().is_some() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many colons"));
        }

        let game_id = game_re.captures(header).ok_or(io::ErrorKind::InvalidData)?.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let rounds = body.split(';').map(|round| {
            let cubes: io::Result<CubeCount> = round.split(',').map(|cube| -> io::Result<(String, u32)> {
                let cube = cube_re.captures(cube).ok_or(io::ErrorKind::InvalidData)?;
                let count = cube.get(1).ok_or(io::ErrorKind::InvalidData)?.as_str().parse::<u32>().unwrap();
                let color = cube.get(2).ok_or(io::ErrorKind::InvalidData)?.as_str().to_string();
                Ok((color, count))
            }).try_fold(CubeCount{ ..Default::default() }, |mut acc, x| {
                x.map(|(color, count)| {
                    match color.as_str() {
                        "red" => acc.red_count = count,
                        "green" => acc.green_count = count,
                        "blue" => acc.blue_count = count,
                        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid color"))
                    }
                    Ok(acc)
                })?
            });
            cubes
        }).collect::<io::Result<Vec<CubeCount>>>()?;
        Ok(Game {
            game_id,
            rounds,
        })
    }).collect::<io::Result<Vec<Game>>>()
}

fn first_star(games: &[Game]) -> u32 {
    let query = CubeCount {
        red_count: 12,
        green_count: 13,
        blue_count: 14,
    };
    games.iter().filter(|game| {
        game.query(&query)
    }).map(|g| g.game_id).sum()
}

fn second_star(games: &[Game]) -> u32 {
    games.iter().map(|game| {
        let CubeCount{ red_count, blue_count, green_count } = game.rounds.iter().fold(CubeCount{ ..Default::default() }, |mut acc, round| {
            acc.red_count = max(acc.red_count, round.red_count);
            acc.green_count = max(acc.green_count, round.green_count);
            acc.blue_count = max(acc.blue_count, round.blue_count);
            acc
        });
        red_count * green_count * blue_count
    }).sum()
}


fn main() -> io::Result<()> {
    let data = parse()?;
    println!("First star: {}", first_star(&data));
    println!("Second star: {}", second_star(&data));
    Ok(())
}
