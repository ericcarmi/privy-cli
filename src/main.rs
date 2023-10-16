use std::{
    ffi::{OsStr, OsString},
    fs::metadata,
    process::Command,
    time::Duration,
};

use clap::Parser;
mod qdrant;
use qdrant::*;
mod embeddings;
use embeddings::*;
use tokio::time::sleep;

/// yarrrrrrgs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path of the folder or file
    #[arg(short = 'f', long)]
    fpath: Option<String>,

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
    let rest = args.get_joined(" ");
    // let mut cmd = Command::new("/Users/eric/Downloads/qdrant");
    // cmd.args(["--config-path", "/Users/eric/qdrant/config.yaml"]);
    // let r = cmd.spawn();

    // println!("{:?}", r);

    if let Some(file_path) = args.fpath {
        if let Ok(md) = metadata(OsStr::new(&file_path)) {
            if md.is_dir() {
                let files = get_files("/Users/eric/notes/signalcomputer");
                for file in files {
                    println!("{:?}", file);
                    let (embeds, strings) = file_embeddings(file);
                    let r = upsert(embeds, strings, "test").await;
                }
            } else {
                let (embeds, strings) = file_embeddings(file_path);
                let r = upsert(embeds, strings, "test").await;
                // println!("{:?}", r);
            }
        }
    } else {
        if !rest.is_empty() {
            let r = search(rest.as_str()).await;
            println!("{:?}", r);
        }
    }
    // if r.is_ok() {
    // sleep(Duration::from_millis(1000)).await;
    //     let x = r.unwrap().kill();
    //     // println!("{:?}", x);
    // }
}
