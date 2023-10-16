use clap::Parser;
mod qdrant;
use qdrant::*;
mod embeddings;
use embeddings::*;
use std::env;

/// yarrrrrrgs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path of the file
    #[arg(short = 'f', long)]
    file_path: Option<String>,

    // #[arg(short = 'q', long)]
    // query: Option<String>,
    #[structopt(name = "ARGUMENTS")]
    rest: Vec<String>,
}

impl Args {
    fn get_joined(&self, separator: &str) -> String {
        self.rest.join(separator)
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("{}", args.get_joined(" "));

    if let Some(file_path) = args.file_path {
        let (embeds, strings) = file_embeddings(file_path);
        let r = upsert(embeds, strings, "poop2").await;
        // println!("{:?}", r);
    }
    // if let Some(query) = args.query {
    // let r = search(query.as_str()).await;
    // println!("{:?}", r);
    // }
}
