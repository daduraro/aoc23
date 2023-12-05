
type Range = (usize, usize);

#[derive(Debug)]
struct MapRange {
    src: Range,
    dst: usize,
}

trait Converter {
    fn convert(&self, x: usize) -> Option<usize>;
    fn convert_range(&self, r: &Range) ->(Vec<Range>, Option<Range>);
}

impl Converter for MapRange {
    fn convert(&self, x: usize) -> Option<usize> {
        if x < self.src.0 || x >= self.src.0 + self.src.1 {
            None
        }
        else {
            Some(self.dst + x - self.src.0)
        }
    }
    fn convert_range(&self, r: &Range) -> (Vec<Range>, Option<Range>) {
        let src_a = self.src.0;
        let src_b = self.src.0 + self.src.1;

        let r_a = r.0;
        let r_b = r.0 + r.1;

        if src_a >= r_b || r_a >= src_b {
            (vec![*r], None)
        }
        else {
            let mut unchanged = Vec::new();
            if r_a < src_a {
                unchanged.push((r_a, src_a - r_a));
            }
            if r_b > src_b {
                unchanged.push((src_b, r_b - src_b));
            }
            let a = std::cmp::max(r_a, src_a);
            let b = std::cmp::min(r_b, src_b);
            (unchanged, Some((a - src_a + self.dst, b - a)))
        }
    }
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    ranges: Vec<MapRange>,
}

type Input = (Vec<usize>, Vec<Map>);
fn parse() -> Input {
    let lines = std::io::stdin().lines().collect::<std::io::Result<Vec<String>>>().unwrap();

    let (seeds, body) = lines.split_first().unwrap();

    let seeds = seeds.strip_prefix("seeds: ").unwrap().split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let parse_header = |s: &str| {
        s.strip_suffix(" map:").map(|h| {
            let parts = h.split("-to-").collect::<Vec<_>>();
            (parts[0].to_string(), parts[1].to_string())
        })
    };

    let parse_mapping = |s: &str| {
        let parts = s.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        if parts.len() == 3 {
            Some(MapRange{ dst: parts[0], src: (parts[1], parts[2]) })
        }
        else {
            None
        }
    };

    let mut maps = Vec::new();
    let mut map = None;
    for line in body {
        if let Some((from, to)) = parse_header(line) {
            map.replace(Map{ from, to, ranges: vec![]}).map(|m| maps.push(m));
        }
        else if let Some(range) = parse_mapping(line) {
            if let Some(m) = &mut map {
                m.ranges.push(range);
            }
            else {
                panic!("no header found");
            }
        }
    }
    map.take().map(|m| maps.push(m));

    (seeds, maps)
}

fn first_star(input: &Input) -> usize {
    let (seeds, maps) = input;
    seeds.iter().map(|s| {
        maps.iter().fold(*s, |s, Map{ ranges, ..}| {
            ranges.iter().find_map(|r| r.convert(s)).unwrap_or(s)
        })
    }).min().unwrap()
}

fn second_star(input: &Input) -> usize {
    let (seeds, maps) = input;
    let seed_ranges = seeds.chunks(2).map(|seed_range| (seed_range[0], seed_range[1])).collect::<Vec<_>>();
    maps.iter().fold(seed_ranges, |seeds, Map{ ranges, ..}| {
        let mut processed = Vec::<Range>::new();
        let mut unprocessed = ranges.iter().fold(seeds, |unprocessed, range| {
            let mut to_be_processed = Vec::<Range>::new();
            for x in unprocessed {
                let (mut u, p) = range.convert_range(&x);
                p.map(|converted| processed.push(converted));
                to_be_processed.append(&mut u)
            }
            to_be_processed
        });
        // println!("processed: {:?}; unprocessed: {:?}", &processed, &unprocessed);
        processed.append(&mut unprocessed); // identity for all those values that no range include
        processed
    }).iter().map(|(a,_)| *a).min().unwrap()
}

fn main() {
    let input = parse();
    println!("1st: {}", first_star(&input));
    println!("2nd: {}", second_star(&input));
}
