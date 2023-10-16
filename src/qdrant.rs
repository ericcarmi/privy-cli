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
    // Example of top level client
    // You may also use tonic-generated client from `src/qdrant.rs`
    let client = QdrantClient::from_url("http://localhost:6334").build()?;

    let collections_list = client.list_collections().await?;
    // dbg!(collections_list);
    // collections_list = ListCollectionsResponse {
    //     collections: [
    //         CollectionDescription {
    //             name: "test",
    //         },
    //     ],
    //     time: 1.78e-6,
    // }

    // client.delete_collection(collection_name).await?;

    if let Err(collection_info) = client.collection_info(collection_name).await {
        println!("\ncollection does not exist, creating it\n");

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

    let mut points = vec![];
    for i in 0..embeds.len() {
        let v = embeds[i].clone();
        let payload: Payload = json!(
            {
                "text": strings[i],
                "line number": i,
            }
        )
        .try_into()
        .unwrap();
        points.push(PointStruct {
            id: Some((i as u64).into()),
            vectors: Some(v.into()),
            payload: payload.into(),
        })
    }

    client
        .upsert_points_blocking(collection_name, points, None)
        .await?;

    Ok(())
}

pub async fn search(query: &str) -> Result<()> {
    let client = QdrantClient::from_url("http://localhost:6334").build()?;
    let embeds = string_embeddings(query);
    let collection_name = "test";
    let search_result = client
        .search_points(&SearchPoints {
            collection_name: collection_name.into(),
            vector: embeds,
            // filter: Some(Filter::all([Condition::matches("what", 12)])),
            limit: 10,
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
