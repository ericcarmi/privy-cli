use anyhow::Result;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    Condition, CreateCollection, Filter, PointId, SearchPoints, Vector, VectorParams, VectorsConfig,
};
use serde_json::json;

use crate::embeddings::string_embeddings;

pub async fn upsert(
    embeds: Vec<Vec<f32>>,
    strings: Vec<String>,
    collection_name: &str,
) -> Result<()> {
    let client = QdrantClient::from_url("http://localhost:6334").build()?;

    let collection_info = client.collection_info(collection_name).await?;
    println!("{:?}", collection_info.result);

    let num_points = collection_info.result.expect("").points_count;
    println!("{:?}", num_points);

    let mut points = vec![];
    for i in 0..embeds.len() {
        let v = embeds[i].clone();
        let payload: Payload = json!(
            {
                "text": strings[i],
                "line_number": i,
                "file_name": collection_name
            }
        )
        .try_into()
        .unwrap();
        points.push(PointStruct {
            id: Some((i as u64 + num_points).into()),
            vectors: Some(v.into()),
            payload: payload.into(),
        })
    }

    client
        .upsert_points_blocking(collection_name, points, None)
        .await?;

    Ok(())
}

pub async fn search(query: &str, collection_name: &str) -> Result<()> {
    let client = QdrantClient::from_url("http://localhost:6334").build()?;
    let embeds = string_embeddings(query);
    let search_result = client
        .search_points(&SearchPoints {
            collection_name: collection_name.into(),
            vector: embeds,
            // filter: Some(Filter::all([Condition::matches("what", 12)])),
            limit: 4,
            with_payload: Some(true.into()),
            ..Default::default()
        })
        .await?;

    for (i, point) in search_result.result.into_iter().enumerate() {
        let mut payload = point.payload;
        let baz_payload = payload.remove("text").unwrap().into_json();
        println!("\nresult {}: \n{}\n", i + 1, baz_payload);
    }

    Ok(())
}

pub async fn check_collection(collection_name: &str) -> Result<()> {
    let client = QdrantClient::from_url("http://localhost:6334").build()?;

    // right now, creates or deletes then creates...will change later
    if let Err(_collection_info) = client.collection_info(collection_name).await {
        println!(
            "\ncollection does not exist, creating collection {}\n",
            collection_name
        );

        client
            .create_collection(&CreateCollection {
                collection_name: collection_name.into(),
                vectors_config: Some(VectorsConfig {
                    config: Some(Config::Params(VectorParams {
                        size: 383, // magic number come from BERT model
                        distance: Distance::Cosine.into(),
                        ..Default::default()
                    })),
                }),
                ..Default::default()
            })
            .await?;
    } else {
        // for now, just delete
        // in the future, would like to check what exists, only update stuff...but that is complicated
        client.delete_collection(collection_name).await?;
        client
            .create_collection(&CreateCollection {
                collection_name: collection_name.into(),
                vectors_config: Some(VectorsConfig {
                    config: Some(Config::Params(VectorParams {
                        size: 383, // magic number come from BERT model
                        distance: Distance::Cosine.into(),
                        ..Default::default()
                    })),
                }),
                ..Default::default()
            })
            .await?;
    }
    Ok(())
}
