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
            color: "#66ff66",
            head: "silly",
            tail: "small-rattle",
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
    height: u32,
    width: u32,
    food: Vec<Coord>,
    hazards: Vec<Coord>,
    snakes: Vec<Battlesnake>,
}

#[derive(Clone, Default, Deserialize, Eq, Hash, PartialEq)]
pub struct Coord {
    x: u32,
    y: u32,
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

#[derive(Default, Deserialize)]
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

pub fn next_move(game_state: &GameState) -> Direction {
    let head = &game_state.you.head;
    let target = &game_state.board.food[0];
    let dirs = [Up, Down, Left, Right];
    let mut allowed: HashSet<_> = dirs.iter().cloned().collect();
    let body: HashSet<_> = game_state.you.body.iter().cloned().collect();
    for ref dir in dirs {
        if body.contains(&head.go(dir)) {
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
        allowed.iter().next().unwrap_or(&Up).to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_towards_food() {
        let mut game_state: GameState = Default::default();
        game_state.board.food = vec![Coord { x: 5, y: 5 }];
        game_state.you.head = Coord { x: 5, y: 3 };
        assert_eq!(next_move(&game_state), Up);
        game_state.you.head = Coord { x: 5, y: 7 };
        assert_eq!(next_move(&game_state), Down);
        game_state.you.head = Coord { x: 7, y: 5 };
        assert_eq!(next_move(&game_state), Left);
        game_state.you.head = Coord { x: 3, y: 5 };
        assert_eq!(next_move(&game_state), Right);
    }

    #[test]
    fn avoids_self() {
        let mut game_state: GameState = Default::default();
        game_state.board.food = vec![Coord { x: 5, y: 5 }];
        game_state.you.head = Coord { x: 5, y: 3 };
        game_state.you.body = vec![Coord { x: 5, y: 3 }, Coord { x: 5, y: 4 }];
        assert_ne!(next_move(&game_state), Up);
        game_state.you.head = Coord { x: 5, y: 7 };
        game_state.you.body = vec![Coord { x: 5, y: 7 }, Coord { x: 5, y: 6 }];
        assert_ne!(next_move(&game_state), Down);
        game_state.you.head = Coord { x: 7, y: 5 };
        game_state.you.body = vec![Coord { x: 7, y: 5 }, Coord { x: 6, y: 5 }];
        assert_ne!(next_move(&game_state), Left);
        game_state.you.head = Coord { x: 3, y: 5 };
        game_state.you.body = vec![Coord { x: 3, y: 5 }, Coord { x: 4, y: 5 }];
        assert_ne!(next_move(&game_state), Right);
    }
}
