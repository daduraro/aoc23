
#[derive(Debug)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
}


fn parse() -> Vec<Card> {
    std::io::stdin().lines().map(|line| {
        let line = line.unwrap();

        let parts = line.split(":").collect::<Vec<_>>();
        let id = parts[0].strip_prefix("Card").unwrap().trim().parse::<u32>().unwrap();

        let number_parts = parts[1].split("|").collect::<Vec<_>>();

        let winning = number_parts[0].split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap() ).collect::<Vec<_>>();
        let numbers = number_parts[1].split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap() ).collect::<Vec<_>>();

        Card { id, winning, numbers }
    }).collect::<Vec<_>>()
}

fn first_star(cards: &Vec<Card>) -> u32 {
    cards.iter().map(|card| {
        let mut winning = 0;
        for number in &card.winning {
            if card.numbers.contains(number) {
                winning += 1;
            }
        }
        if winning > 0 {
            1 << (winning-1)
        }
        else {
            0
        }
    }).sum()
}

fn second_star(cards: &Vec<Card>) -> u32 {
    let winning = cards.iter().map(|card| {
        let mut winning = 0;
        for number in &card.winning {
            if card.numbers.contains(number) {
                winning += 1;
            }
        }
        winning
    }).collect::<Vec<_>>();

    let n = cards.len();
    let mut copies = std::iter::repeat(1).take(n).collect::<Vec<u32>>();
    for (idx, wins) in winning.iter().enumerate() {

        for i in idx+1..std::cmp::min(idx+wins+1, n) {
            copies[i] += copies[idx];
        }
    }

    copies.iter().sum()
}

fn main() {
    let input = parse();
    println!("1st: {}", first_star(&input));
    println!("2nd: {}", second_star(&input));
}
