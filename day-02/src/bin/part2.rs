lib::day!("02", part2, test => 2286);

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(Game::parse)
        .map(|game| game.draws.into_iter().reduce(Draw::max).unwrap().power())
        .sum()
}

#[derive(Debug)]
struct Game {
    #[allow(unused)]
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
    pub fn new(id: u32) -> Self {
        Self { id, draws: vec![] }
    }
}

#[derive(Default, Debug)]
struct Draw {
    pub blue: u32,
    pub green: u32,
    pub red: u32,
}

impl Draw {
    pub fn max(self, other_draw: Draw) -> Draw {
        Draw {
            blue: self.blue.max(other_draw.blue),
            green: self.green.max(other_draw.green),
            red: self.red.max(other_draw.red),
        }
    }

    pub fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}
