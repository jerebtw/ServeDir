use std::{net::SocketAddr, path::Path};

use anyhow::Result;
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

    axum::Server::bind(&url)
        .serve(tower::make::Shared::new(
            ServeDir::new(dir).append_index_html_on_directories(true),
        ))
        .await?;

    Ok(())
}
