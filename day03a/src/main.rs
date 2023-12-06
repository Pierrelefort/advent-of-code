use std::fs;

fn _neighbours(width: usize, i: usize, j: usize) -> [usize; 8] {
    return [
        (i - 1) * width + j - 1,
        i * width + j - 1,
        (i + 1) * width + j - 1,
        (i - 1) * width + j,
        (i + 1) * width + j,
        (i - 1) * width + j + 1,
        i * width + j + 1,
        (i + 1) * width + j + 1,
    ];
}
fn number_in_array(path_file: String) -> u32 {
    let file_path: String = String::from(path_file);
    let contents = fs::read_to_string(file_path);
    
    let mut text: String = String::from("");
    match contents {
        Ok(v) => text = v,
        Err(e) => println!("error parsing header: {e:?}"),
    }

    let mut next_symbol = text.replace(|c: char| !c.is_ascii_whitespace(), ".");

    let str = text.split("\n");
    for (i, line) in str.enumerate() {
        let width = line.len() + 1;
        for (j, c) in line.chars().enumerate() {
            if !c.is_alphanumeric() && !(c == '.') {
                let neighbours_index = _neighbours(width, i, j);
                for neighbour in neighbours_index {
                    if text.chars().nth(neighbour).unwrap_or(' ').is_alphanumeric() {
                        let curr: &str = &String::from(text.chars().nth(neighbour).unwrap());
                        next_symbol.replace_range(neighbour..neighbour + 1, curr);
                    }
                }
            }
        }
    }

    let next_symbol_index_list: Vec<usize> = next_symbol.char_indices()
        .filter(|(_, c)| c.is_alphanumeric())
        .map(|(i, _)| i)
        .collect();

    println!("Text:\n{}", text);
    println!("Next Symbol:\n{}", next_symbol);

    let res: u32 = 0;
    return res;
}

fn main() {
    let test_value = number_in_array(String::from("test.txt"));
    println!("Test value: {}", test_value);
    assert!(test_value == 4361);

    let res = number_in_array(String::from("input.txt"));
    println!("Final: {}", res)
}
