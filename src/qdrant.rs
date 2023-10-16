use anyhow::Result;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    Condition, CreateCollection, Filter, SearchPoints, VectorParams, VectorsConfig,
};
use serde_json::json;

use crate::embeddings::string_embeddings;

pub async fn go(embeds: Vec<f32>) -> Result<()> {
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

    let collection_name = "test";
    client.delete_collection(collection_name).await?;

    client
        .create_collection(&CreateCollection {
            collection_name: collection_name.into(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: embeds.len() as u64,
                    distance: Distance::Cosine.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        })
        .await?;

    let collection_info = client.collection_info(collection_name).await?;
    // dbg!(collection_info);

    let payload: Payload = json!(
        {
            "foo": "this is just metadata",
            "line number": 12,
            "baz": {
                "qux": "quux"
            }
        }
    )
    .try_into()
    .unwrap();

    let points = vec![PointStruct::new(0, embeds, payload)];
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
    dbg!(&search_result);

    let found_point = search_result.result.into_iter().next().unwrap();
    let mut payload = found_point.payload;
    let baz_payload = payload.remove("baz").unwrap().into_json();
    // println!("payload: {:?}", payload);
    // baz: {"qux":"quux"}

    Ok(())
}
