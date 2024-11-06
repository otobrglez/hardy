#[allow(unused_imports, dead_code)]
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use clap::Parser;
use hardy::board::{Board, Move};
use hardy::engine::GameEngine;
use hardy::engine::{GameEngineError, RandomEngine};
use hardy::server::game_query::GameQuery;
use std::convert::Infallible;
use warp::{http::StatusCode, *};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ServerArgs {
    #[arg(short, long, env = "PORT", default_value_t = 8884)]
    port: u16,
}

#[derive(Debug)]
struct InvalidParameter;

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message: String;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".to_string();
    } else if let Some(e) = err.find::<InvalidParameter>() {
        code = StatusCode::BAD_REQUEST;
        message = format!("Invalid parameter: {:?}", e).to_string();
    } else if let Some(e) = err.find::<reject::InvalidQuery>() {
        code = StatusCode::BAD_REQUEST;
        message = format!("Invalid query parameter: {:?}", e).to_string();
    } else {
        eprintln!("Unhandled rejection: {:#?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal server error".to_string();
    }

    Ok(reply::with_status(message, code))
}

fn next_move(game_query: &GameQuery) -> Result<Move, GameEngineError> {
    let game_query_r = game_query.clone();
    let board = Board::try_from(game_query_r).map_err(|e| GameEngineError::LoadingError {
        message: "Problem with board loading.".to_string(),
    })?;
    let mut engine = RandomEngine::load_board(board)?;
    let next_move = engine.next_move(game_query.playing);
    next_move
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let args = ServerArgs::parse();

    let get_move = path!("move")
        .and(warp::query::<GameQuery>())
        .map(|query: GameQuery| next_move(&query))
        .map(|maybe_move| {
            match maybe_move {
                Ok(Move { player, position }) => reply::with_status(format!("Move:{}-{}-{}", player, position.0, position.1).to_string(), StatusCode::BAD_REQUEST),
                Err(err) => reply::with_status(format!("Error:{}", err).to_string(), StatusCode::BAD_REQUEST),
            }
        });

    serve(get_move.recover(handle_rejection))
        .run(([0, 0, 0, 0], args.port))
        .await;
}
