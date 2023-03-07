#[macro_use]
extern crate diesel;
extern crate dotenv;

mod args;
mod ops;
mod schema;
mod db;
mod models;

use ops::user_ops::handle_user_command;
use ops::video_ops::handle_video_command;
use ops::view_ops::handle_view_command;
use args::EntityType;
use args::RuflixArgs;
use clap::Parser;

fn main() {
    let args = RuflixArgs::parse();

    match args.entity_type {
        EntityType::User(user) => handle_user_command(user),
        EntityType::Video(video) => handle_video_command(video),
        EntityType::View(view) => handle_view_command(view),
    };
}