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

    /// collection name to search through
    #[arg(short = 'c', long)]
    collection: Option<String>,

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
        let cname = file_path.clone();
        let mut collection_name = cname.as_str().replace('/', "-").replace(' ', "_");
        collection_name.remove(0);
        collection_name.remove(collection_name.len() - 1);
        // println!("collection name: {:?}", collection_name);
        let r = check_collection(collection_name.as_str()).await;
        // println!("{:?}", r);

        if let Ok(md) = metadata(OsStr::new(&file_path)) {
            if md.is_dir() {
                let files = get_files(&file_path);
                // println!("{:?}", files);

                for file in files {
                    let (embeds, strings) = file_embeddings(file);
                    let r = upsert(embeds, strings, collection_name.as_str()).await;
                    println!("wtf is this {:?}", r);
                }
            } else {
                let (embeds, strings) = file_embeddings(file_path);
                let r = upsert(embeds, strings, collection_name.as_str()).await;
                // println!("{:?}", r);
            }
        }
    } else {
        if let Some(collection) = args.collection {
            if !rest.is_empty() {
                let r = search(rest.as_str(), &collection).await;
                println!("{:?}", r);
            } else {
                // must enter a query along with collection name
            }
        }
    }
    // if r.is_ok() {
    // sleep(Duration::from_millis(1000)).await;
    //     let x = r.unwrap().kill();
    //     // println!("{:?}", x);
    // }
}
