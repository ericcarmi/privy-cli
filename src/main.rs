use clap;
use clap::Parser;
use std::{ffi::OsStr, fs::metadata, time::Duration};
mod qdrant;
use qdrant::*;
mod embeddings;
use embeddings::*;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
// use std::io::{self, Write};
// use tokio::time::sleep;

/// yarrrrrrgs
#[derive(Parser, Debug, Clone)]
struct Args {
    /// path of the folder or file
    #[clap(short = 'f', long)]
    fpath: Option<String>,

    /// collection name to search through
    #[arg(short = 'c', long)]
    collection: Option<String>,

    /// delete collection
    #[arg(short = 'd', long)]
    delete: Option<String>,

    /// list collections
    #[clap(long, short = 'l', default_value = None, default_missing_value = None)]
    list: Option<String>,

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
        // println!("collection name: {:?}", collection_name);
        let _r = check_collection(collection_name.as_str()).await;
        // println!("{:?}", r);

        if let Ok(md) = metadata(OsStr::new(&file_path)) {
            if md.is_dir() {
                let files = get_files(&file_path);
                // println!("{:?}", files);

                let progress_bar = ProgressBar::new(files.len() as u64);
                progress_bar.set_draw_target(ProgressDrawTarget::stdout());

                for file in files {
                    let (embeds, strings) = file_embeddings(file.clone());
                    let _r = upsert(embeds, strings, collection_name.as_str(), file.as_str()).await;
                    progress_bar.inc(1);

                    progress_bar.println(format!("adding {} to the collection", file.clone()));
                }
                progress_bar.finish_with_message("done");
            } else {
                let (embeds, strings) = file_embeddings(file_path.clone());
                let _r = upsert(
                    embeds,
                    strings,
                    collection_name.as_str(),
                    file_path.as_str(),
                )
                .await;
                // println!("{:?}", r);
            }
        }
    } else {
        if let Some(collection) = args.collection {
            if !rest.is_empty() {
                let _r = search(rest.as_str(), Some(&collection)).await;
                // println!("{:?}", r);
            } else {
                // must enter a query along with collection name
            }
        } else {
            if !rest.is_empty() {
                let _r = search(rest.as_str(), None).await;
                // println!("{:?}", r);
            } else {
                // must enter a query along with collection name
            }
        }
    }
    if let Some(collection) = args.delete {
        let r = delete_collection(collection.as_str()).await;
        println!("{:?}", r);
    }

    if let Some(collection) = args.list {
        let r = show_collection_info(&collection).await;
        println!("{:?}", r);
    } // if r.is_ok() {
      // sleep(Duration::from_millis(1000)).await;
      //     let x = r.unwrap().kill();
      //     // println!("{:?}", x);
      // }
}
