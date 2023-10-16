use std::fs;
pub fn get_embeds(file_path: String) -> Vec<f32> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.split('\n');
    for line in lines {
        if !line.is_empty() {
            // println!("{:?}", line);
            let mut cmd =
                std::process::Command::new("/Users/eric/Downloads/bert.cpp/build/bin/main");
            let r = cmd.args([
                "-m",
                "/Users/eric/Downloads/bert.cpp/models/all-MiniLM-L6-v2/ggml-model-f16.bin",
                "-p",
                line,
            ]);
            if let Ok(x) = r.output() {
                // println!("{:?}", x.stdout);
                let mut y = String::from_utf8(x.stdout).unwrap();
                let mut embeds = vec![];
                for num in y.split(',') {
                    if let Ok(f) = num.trim().parse::<f32>() {
                        embeds.push(f);
                    }
                }
                if !embeds.is_empty() {
                    //upsert
                    return embeds;
                }
            }
        }
    }
    return vec![];
    // let point = PointStruct {
    //     id: Some(PointId::from(42)), // unique u64 or String
    //     vectors: Some(vec![0.0_f32; 512].into()),
    //     payload: std::collections::HashMap::from([
    //         ("great".into(), Value::from(true)),
    //         ("level".into(), Value::from(9000)),
    //         ("text".into(), Value::from("Hi Qdrant!")),
    //         ("list".into(), Value::from(vec![1.234, 0.815])),
    //     ]),
    // };
    //
    // let response = qdrant_client
    //     .upsert_points("my_collection", vec![point], None)
    //     .await?;
    //# Ok(())
}

pub fn string_embeddings(string: &str) -> Vec<f32> {
    let mut cmd = std::process::Command::new("/Users/eric/Downloads/bert.cpp/build/bin/main");
    let r = cmd.args([
        "-m",
        "/Users/eric/Downloads/bert.cpp/models/all-MiniLM-L6-v2/ggml-model-f16.bin",
        "-p",
        string,
    ]);
    if let Ok(x) = r.output() {
        let mut y = String::from_utf8(x.stdout).unwrap();
        let mut embeds = vec![];
        for num in y.split(',') {
            if let Ok(f) = num.trim().parse::<f32>() {
                embeds.push(f);
            }
        }
        if !embeds.is_empty() {
            return embeds;
        } else {
            return vec![];
        }
    } else {
        return vec![];
    }
}
