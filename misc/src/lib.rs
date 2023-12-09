use std::fs;

pub fn load_text(path_file: String) -> String {
    let file_path: String = String::from(path_file);
    let contents = fs::read_to_string(file_path);
    
    let mut text: String = String::from("");
    match contents {
        Ok(v) => text = v,
        Err(e) => println!("error parsing header: {e:?}"),
    }

    return text;
}