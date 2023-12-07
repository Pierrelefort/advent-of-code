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

    let mut str = text.split("\n");
    let width = str.next().unwrap().len();
    let gears_index: Vec<_> = text.char_indices()
        .filter(|(_, c)| c == &'*')
        .map(|(i, _)| (i / width, i % width))
        .collect();

    for (i, j) in gears_index {
        let neighbours_index = _neighbours(width, i, j).iter()
            .map(|i:&usize| text.char_indices().nth(*i).unwrap_or((0, '.')))
            .filter(|(_, c)| c.is_alphanumeric());
    }


             

    let next_symbol_index_list: Vec<usize> = next_symbol.char_indices()
        .filter(|(_, c)| c.is_alphanumeric() || c == &'*')
        .map(|(i, _)| i)
        .collect();

    println!("Text:\n{}", text);
    println!("Next Symbol:\n{}", next_symbol);

    // Find adjacent number
    for next_symbol_index in next_symbol_index_list {
        let mut i = 1;
        while next_symbol_index + i < text.len() && text.chars().nth(next_symbol_index + i).unwrap_or(' ').is_alphanumeric() {
            i += 1;
        }
        if !next_symbol_index + i < text.len() {
            i -= 1;
        }
        next_symbol.replace_range(next_symbol_index..next_symbol_index + i, &text[next_symbol_index..next_symbol_index + i]);

        i = 1;
        while next_symbol_index >= i && text.chars().nth(next_symbol_index - i).unwrap_or(' ').is_alphanumeric() {
            i += 1;
        }
        if !next_symbol_index >= i {
            i -= 1;
        }
        next_symbol.replace_range(next_symbol_index - i..next_symbol_index, &text[next_symbol_index - i..next_symbol_index]);
    }

    println!("Text:\n{}", text);
    println!("Next Symbol:\n{}", next_symbol);

    // Return sum of numbers
    let list_numbers = next_symbol.split(|c: char| c == '.' || c == '\n');
    let mut res: u32 = 0;
    list_numbers.for_each(|s:&str| res += s.parse::<u32>().unwrap_or(0));
    return res;
}

fn main() {
    let test_value = number_in_array(String::from("test.txt"));
    println!("Test value: {}", test_value);
    assert!(test_value == 467835);

    let res = number_in_array(String::from("input.txt"));
    println!("Final: {}", res)
}
