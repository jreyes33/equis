use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// Types from https://github.com/BattlesnakeOfficial/starter-snake-rust/blob/6c771ce/src/main.rs#L21
#[derive(Serialize)]
pub struct SnakeInfo<'a> {
    apiversion: &'a str,
    author: &'a str,
    color: &'a str,
    head: &'a str,
    tail: &'a str,
    version: &'a str,
}

impl Default for SnakeInfo<'_> {
    fn default() -> Self {
        SnakeInfo {
            apiversion: "1",
            author: "jreyes33",
            color: "#669d83",
            head: "dead",
            tail: "hook",
            version: "0.1.0",
        }
    }
}

#[derive(Default, Deserialize)]
pub struct GameState {
    turn: u32,
    game: Game,
    board: Board,
    you: Battlesnake,
}

#[derive(Default, Deserialize)]
pub struct Game {
    id: String,
    ruleset: HashMap<String, String>,
    timeout: u32,
}

#[derive(Default, Deserialize)]
pub struct Board {
    height: i32,
    width: i32,
    food: Vec<Coord>,
    hazards: Vec<Coord>,
    snakes: Vec<Battlesnake>,
}

#[derive(Clone, Default, Deserialize, Eq, Hash, PartialEq)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn go(&self, dir: &Direction) -> Self {
        let Coord { x, y } = *self;
        match dir {
            Up => Self { x, y: y + 1 },
            Down => Self { x, y: y - 1 },
            Left => Self { x: x - 1, y },
            Right => Self { x: x + 1, y },
        }
    }
}

#[derive(Clone, Default, Deserialize)]
pub struct Battlesnake {
    id: String,
    name: String,
    health: u32,
    body: Vec<Coord>,
    head: Coord,
    length: u32,
    latency: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

const DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];

pub fn next_move(game_state: &GameState) -> Direction {
    let Board {
        height,
        width,
        ref snakes,
        ..
    } = game_state.board;
    let head = &game_state.you.head;
    let target = &game_state.board.food[0];
    let mut allowed: HashSet<_> = DIRECTIONS.iter().collect();
    let blocked: HashSet<_> = snakes.iter().flat_map(|s| &s.body).collect();
    for ref dir in DIRECTIONS {
        let next = head.go(dir);
        let Coord { ref x, ref y } = next;
        if !(0..width).contains(x) || !(0..height).contains(y) || blocked.contains(&next) {
            allowed.remove(dir);
        }
    }
    if target.x > head.x && allowed.contains(&Right) {
        Right
    } else if target.x < head.x && allowed.contains(&Left) {
        Left
    } else if target.y > head.y && allowed.contains(&Up) {
        Up
    } else if allowed.contains(&Down) {
        Down
    } else {
        allowed.into_iter().next().unwrap_or(&Up).to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_towards_food() {
        let mut game_state: GameState = Default::default();
        game_state.board.height = 11;
        game_state.board.width = 11;
        game_state.board.food = vec![Coord { x: 5, y: 5 }];
        game_state.you.head = Coord { x: 5, y: 3 };
        game_state.board.snakes = vec![game_state.you.clone()];
        assert_eq!(next_move(&game_state), Up);
        game_state.you.head = Coord { x: 5, y: 7 };
        game_state.board.snakes = vec![game_state.you.clone()];
        assert_eq!(next_move(&game_state), Down);
        game_state.you.head = Coord { x: 7, y: 5 };
        game_state.board.snakes = vec![game_state.you.clone()];
        assert_eq!(next_move(&game_state), Left);
        game_state.you.head = Coord { x: 3, y: 5 };
        game_state.board.snakes = vec![game_state.you.clone()];
        assert_eq!(next_move(&game_state), Right);
    }

    #[test]
    fn avoids_self() {
        let mut game_state: GameState = Default::default();
        game_state.board.height = 11;
        game_state.board.width = 11;
        game_state.board.food = vec![Coord { x: 5, y: 5 }];
        game_state.you.head = Coord { x: 5, y: 3 };
        game_state.you.body = vec![Coord { x: 5, y: 3 }, Coord { x: 5, y: 4 }];
        game_state.board.snakes = vec![game_state.you.clone()];
        assert_ne!(next_move(&game_state), Up);
        game_state.you.head = Coord { x: 5, y: 7 };
        game_state.you.body = vec![Coord { x: 5, y: 7 }, Coord { x: 5, y: 6 }];
        game_state.board.snakes = vec![game_state.you.clone()];
        assert_ne!(next_move(&game_state), Down);
        game_state.you.head = Coord { x: 7, y: 5 };
        game_state.you.body = vec![Coord { x: 7, y: 5 }, Coord { x: 6, y: 5 }];
        game_state.board.snakes = vec![game_state.you.clone()];
        assert_ne!(next_move(&game_state), Left);
        game_state.you.head = Coord { x: 3, y: 5 };
        game_state.you.body = vec![Coord { x: 3, y: 5 }, Coord { x: 4, y: 5 }];
        game_state.board.snakes = vec![game_state.you.clone()];
        assert_ne!(next_move(&game_state), Right);
    }

    #[test]
    fn avoids_walls() {
        let mut game_state: GameState = Default::default();
        game_state.board.height = 11;
        game_state.board.width = 11;
        game_state.board.food = vec![Coord { x: 5, y: 5 }];
        game_state.you.head = Coord { x: 5, y: 10 };
        game_state.you.body = vec![Coord { x: 5, y: 10 }, Coord { x: 5, y: 9 }];
        game_state.board.snakes = vec![game_state.you.clone()];
        assert_ne!(next_move(&game_state), Up);
        game_state.you.head = Coord { x: 5, y: 0 };
        game_state.you.body = vec![Coord { x: 5, y: 0 }, Coord { x: 5, y: 1 }];
        game_state.board.snakes = vec![game_state.you.clone()];
        assert_ne!(next_move(&game_state), Down);
        game_state.you.head = Coord { x: 0, y: 5 };
        game_state.you.body = vec![Coord { x: 0, y: 5 }, Coord { x: 1, y: 5 }];
        game_state.board.snakes = vec![game_state.you.clone()];
        assert_ne!(next_move(&game_state), Left);
        game_state.you.head = Coord { x: 10, y: 5 };
        game_state.you.body = vec![Coord { x: 10, y: 5 }, Coord { x: 9, y: 5 }];
        game_state.board.snakes = vec![game_state.you.clone()];
        assert_ne!(next_move(&game_state), Right);
    }

    #[test]
    fn avoids_others() {
        let mut game_state: GameState = Default::default();
        game_state.board.height = 11;
        game_state.board.width = 11;
        game_state.board.food = vec![Coord { x: 5, y: 5 }];
        game_state.you.head = Coord { x: 5, y: 3 };
        game_state.you.body = vec![Coord { x: 5, y: 3 }, Coord { x: 5, y: 2 }];
        let other = Battlesnake {
            head: Coord { x: 5, y: 4 },
            body: vec![
                Coord { x: 5, y: 4 },
                Coord { x: 4, y: 4 },
                Coord { x: 4, y: 3 },
            ],
            ..Default::default()
        };
        game_state.board.snakes = vec![game_state.you.clone(), other];
        assert_eq!(next_move(&game_state), Right);
    }
}
