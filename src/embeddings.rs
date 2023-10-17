use std::fs;
use std::fs::metadata;

pub fn get_files(folder_path: &str) -> Vec<String> {
    let paths = fs::read_dir(folder_path).unwrap();
    let md = metadata(folder_path).unwrap();
    if !md.is_dir() {
        println!("needs to be folder");
        return vec![];
    }

    let mut files = vec![];
    for path in paths {
        let md = metadata(path.as_ref().unwrap().path()).unwrap();
        if md.is_dir() {
            // recursion? got a stack overflow...
            files.append(&mut get_files(
                path.as_ref().unwrap().path().to_str().unwrap(),
            ));
        }
        if md.is_file() {
            if let Some(ext) = path.as_ref().unwrap().path().extension() {
                if ext == "md" || ext == "txt" {
                    // println!("{:?}", path.unwrap().path().display());
                    files.push(path.unwrap().path().to_str().unwrap().to_string());
                }
            }
        }
    }
    files
}

pub fn file_embeddings(file_path: String) -> (Vec<Vec<f32>>, Vec<String>) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.split('\n');
    let mut embeds = vec![];
    let mut strings = vec![];
    for line in lines {
        if !line.is_empty() && !line.replace('-', "").is_empty() && !line.trim().is_empty() {
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
