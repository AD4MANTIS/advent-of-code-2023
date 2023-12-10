lib::day!(02, part1, test => 8);

#[allow(unused_variables)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(Game::parse)
        .filter(|game| {
            // only 12 red cubes, 13 green cubes, and 14 blue cubes?
            game.draws
                .iter()
                .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
        })
        .map(|game| game.id)
        .sum()
}

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub draws: Vec<Draw>,
}

impl Game {
    pub fn parse(str: &str) -> Self {
        let collect = &str.split(&[':', ';']).collect::<Vec<_>>();
        let game_id = &collect.first().unwrap()[5..].parse::<u32>().unwrap();
        let mut game = Self::new(*game_id);

        for raw_draw in collect.iter().skip(1) {
            let mut draw = Draw::default();
            for single_draw in raw_draw.split(',') {
                let single_draw = single_draw.trim().split(' ').collect::<Vec<_>>();
                let color_count = single_draw[0].parse::<u32>().unwrap();

                match single_draw[1] {
                    "blue" => {
                        draw.blue += color_count;
                    }
                    "red" => {
                        draw.red += color_count;
                    }
                    "green" => {
                        draw.green += color_count;
                    }
                    _ => (),
                };
            }

            game.draws.push(draw);
        }

        game
    }
}

impl Game {
    pub const fn new(id: u32) -> Self {
        Self { id, draws: vec![] }
    }
}

#[derive(Default, Debug)]
struct Draw {
    pub blue: u32,
    pub green: u32,
    pub red: u32,
}
