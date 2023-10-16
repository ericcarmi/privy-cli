use std::fs;
pub fn file_embeddings(file_path: String) -> (Vec<Vec<f32>>, Vec<String>) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.split('\n');
    let mut embeds = vec![];
    let mut strings = vec![];
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
                let y = String::from_utf8(x.stdout).unwrap();
                strings.push(line.to_string());
                let mut e = vec![];
                for num in y.split(',') {
                    if let Ok(f) = num.trim().parse::<f32>() {
                        e.push(f);
                    }
                }
                embeds.push(e);
            }
        }
    }
    return (embeds, strings);
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
        let y = String::from_utf8(x.stdout).unwrap();
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
