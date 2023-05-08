use std::{net::SocketAddr, path::Path};

use anyhow::Result;
use axum::{routing::get, Router};
use clap::Parser;
use tower_http::services::ServeDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// The IP to bind to
  #[arg(short, long)]
  url: String,

  /// The directory to serve
  #[arg(short, long)]
  dir: String,
}

#[tokio::main]
async fn main() -> Result<()> {
  setup_api(Args::parse()).await?;
  Ok(())
}

async fn setup_api(args: Args) -> Result<()> {
  let url = args.url.parse::<SocketAddr>()?;
  let dir = Path::new(&args.dir).canonicalize()?;

  println!("Listening on {url}");

  let router = Router::new()
    .nest_service(
      "/",
      ServeDir::new(dir).append_index_html_on_directories(true),
    )
    .route("/ping", get(ping));
  axum::Server::bind(&url)
    .serve(router.into_make_service_with_connect_info::<SocketAddr>())
    .await?;

  Ok(())
}

async fn ping() -> String {
  "Pong".to_string()
}
