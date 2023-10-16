use clap::Parser;
mod qdrant;
use qdrant::{go, search};
mod embeddings;
use embeddings::*;

/// yarrrrrrgs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path of the file
    #[arg(short = 'f', long)]
    file_path: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let file_path: String = args.file_path;
    let embeds = get_embeds(file_path);
    let r = go(embeds).await;
    search("8266").await;
}
