use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Deserialize)]
pub struct GameState {
    turn: u32,
    game: Game,
    board: Board,
    you: Battlesnake,
}

#[derive(Deserialize)]
pub struct Game {
    id: String,
    ruleset: HashMap<String, String>,
    timeout: u32,
}

#[derive(Deserialize)]
pub struct Board {
    height: u32,
    width: u32,
    food: Vec<Coord>,
    hazards: Vec<Coord>,
    snakes: Vec<Battlesnake>,
}

#[derive(Deserialize)]
pub struct Coord {
    x: u32,
    y: u32,
}

#[derive(Deserialize)]
pub struct Battlesnake {
    id: String,
    name: String,
    health: u32,
    body: Vec<Coord>,
    head: Coord,
    length: u32,
    latency: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn next_move(game_state: GameState) -> Direction {
    let width = game_state.board.width;
    let height = game_state.board.height;
    let head = game_state.you.head;
    let target = &game_state.board.food[0];
    if target.x > head.x {
        Direction::Right
    } else if target.x < head.x {
        Direction::Left
    } else if target.y > head.y {
        Direction::Up
    } else {
        Direction::Down
    }
}
