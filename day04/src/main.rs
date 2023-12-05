
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
        let id = parts[0].strip_prefix("Card ").unwrap().parse::<u32>().unwrap();

        let number_parts = parts[1].split("|").collect::<Vec<_>>();

        println!("number_parts: {:?}", number_parts);

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

fn main() {
    let input = parse();
    println!("1st: {}", first_star(&input));
}
