use rand::Rng;
use std::{thread, time};

#[derive(Clone)]
enum State {
    Alive,
    Dead,
}

pub struct GameSize {
    pub width: i32,
    pub height: i32,
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

pub struct Game {
    size: GameSize,
    world: Vec<Vec<State>>,
    delay: i32,
}

impl Game {
    pub fn new(size: GameSize, delay: i32) -> Self {
        let mut world: Vec<Vec<State>> = Vec::new();
        for _ in 0..size.height {
            let mut row = Vec::new();
            for _ in 0..size.width {
                row.push(Game::random_state());
            }
            world.push(row);
        }
        Game { size, world, delay }
    }

    fn random_state() -> State {
        let mut rng = rand::thread_rng();
        let random: bool = rng.gen();
        if random {
            State::Alive
        } else {
            State::Dead
        }
    }

    fn get_point_at(&self, point: &Point) -> &State {
        &self.world[point.y as usize][point.x as usize]
    }

    fn get_alive_neighbor_count_for(&self, point: &Point) -> usize {
        vec![
            Point::new(point.x, point.y + 1),     // N
            Point::new(point.x + 1, point.y + 1), // NE
            Point::new(point.x + 1, point.y),     // E
            Point::new(point.x + 1, point.y - 1), // SE
            Point::new(point.x, point.y - 1),     // S
            Point::new(point.x - 1, point.y - 1), // SW
            Point::new(point.x - 1, point.y),     // W
            Point::new(point.x - 1, point.y + 1), // NW
        ]
        .iter()
        .filter(|n| {
            n.x >= 0
                && n.x < self.size.width as i32
                && n.y >= 0
                && n.y < self.size.height as i32
                && matches!(self.get_point_at(n), State::Alive)
        })
        .map(|n| self.get_point_at(n))
        .count()
    }

    fn evaluate_point(&self, point: &Point) -> State {
        let neighbors = self.get_alive_neighbor_count_for(point);
        match self.get_point_at(point) {
            State::Alive => {
                if neighbors < 2 || neighbors > 3 {
                    State::Dead
                } else {
                    State::Alive
                }
            }
            State::Dead => {
                if neighbors == 3 {
                    State::Alive
                } else {
                    State::Dead
                }
            }
        }
    }

    fn to_string_vec(&self) -> Vec<String> {
        let mut world_string: Vec<String> = Vec::new();
        for row in self.world.iter() {
            world_string.push(
                row.iter()
                    .map(|p| match p {
                        State::Alive => "+",
                        State::Dead => " ",
                    })
                    .collect::<Vec<&str>>()
                    .join(""),
            );
        }
        world_string
    }

    fn draw(&self) {
        print!("\x1B[2J\x1B[1;1H");
        for line in self.to_string_vec() {
            println!("{}", line);
        }
    }

    pub fn run(&mut self) {
        loop {
            self.draw();
            let sec = time::Duration::from_millis(self.delay as u64);
            thread::sleep(sec);

            let mut next_generation =
                vec![vec![State::Dead; self.size.width as usize]; self.size.height as usize];
            for (y, row) in self.world.iter().enumerate() {
                for (x, _) in row.iter().enumerate() {
                    let p = Point::new(x as i32, y as i32);
                    next_generation[y][x] = self.evaluate_point(&p)
                }
            }
            self.world = next_generation;
        }
    }
}

impl std::string::ToString for Game {
    fn to_string(&self) -> String {
        self.to_string_vec().join("\n")
    }
}
