use game::{next_move, Direction, GameState, SnakeInfo};
use serde::Serialize;
use serde_json::to_string;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;

mod game;

#[derive(Serialize)]
struct Message<'a> {
    message: &'a str,
}

impl<'a> Message<'a> {
    fn new(message: &'a str) -> Self {
        Self { message }
    }
}

#[derive(Serialize)]
struct MoveResponse {
    r#move: Direction,
}

impl MoveResponse {
    fn new(r#move: Direction) -> Self {
        Self { r#move }
    }
}

#[wasm_bindgen]
pub async fn run(request: Request) -> String {
    match do_run(request).await {
        Ok(v) => v,
        Err(e) => format!("oopsie: {}", e),
    }
}

async fn do_run(request: Request) -> Result<String> {
    let url = request.url();
    // I didn't want to install the url crate.
    let path_start = url.match_indices("/").nth(2).unwrap().0;
    let path = &url[path_start..];
    let method = request.method();
    match (method.as_str(), path) {
        ("GET", "/") => info(),
        ("POST", "/start") => start(get_game_state(request).await?),
        ("POST", "/end") => end(get_game_state(request).await?),
        ("POST", "/move") => do_move(get_game_state(request).await?),
        _ => dunno(),
    }
}

async fn get_game_state(request: Request) -> Result<GameState> {
    JsFuture::from(request.json().unwrap())
        .await
        .map_err(|a| a.as_string().unwrap())?
        .into_serde()
        .map_err(From::from)
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

fn dunno() -> Result<String> {
    to_string(&Message::new("dunno")).map_err(From::from)
}

fn info() -> Result<String> {
    let snake_info: SnakeInfo = Default::default();
    to_string(&snake_info).map_err(From::from)
}

fn start(_game_state: GameState) -> Result<String> {
    to_string(&Message::new("start")).map_err(From::from)
}

fn end(_game_state: GameState) -> Result<String> {
    to_string(&Message::new("end")).map_err(From::from)
}

fn do_move(game_state: GameState) -> Result<String> {
    to_string(&MoveResponse::new(next_move(game_state))).map_err(From::from)
}
