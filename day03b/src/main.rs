use std::fs;

fn _neighbours(text: &String, width: usize, index: &usize) -> String {
    let index_i = (*index as f32 / width as f32).floor() as usize;
    let index_j = index % width;

    let mut res: String = String::from("");
    let min = if index_i > 1 { index_i - 1 } else { 0 };
    let max = if index_i < text.split("\n").count() { index_i } else { index_i + 1 };

    for i in min..max+1 {
        let mut j = 1;
        let (mut left, mut c) = text.char_indices().nth(i * width + index_j - j).unwrap_or((0, '.'));
        while index_j >= j && c.is_alphanumeric() {
            (left, c) = text.char_indices().nth(i * width + index_j - j).unwrap_or((0, '.'));
            j += 1;
        }

        let mut j = 1;
        let (mut right, mut c) = text.char_indices().nth(i * width + index_j + j).unwrap_or((text.split('\n').count(), '.'));
        while index_j + j < width && c.is_alphanumeric() {
            (right, c) = text.char_indices().nth(i * width + index_j + j).unwrap_or((text.split('\n').count(), '.'));
            j += 1;
        }

        let curr = format!("{:.^1}", &text[left..right+1]);
        res.push_str(curr.as_str());
        res.push('\n');
    }
    println!("Value:\n{}", res);
    return res;
}

fn number_in_array(path_file: String) -> u64 {
    let file_path: String = String::from(path_file);
    let contents = fs::read_to_string(file_path);
    
    let mut text: String = String::from("");
    match contents {
        Ok(v) => text = v,
        Err(e) => println!("error parsing header: {e:?}"),
    }

    let line = text.replace(|c: char| c == '\n', "");
    let width = text.split("\n").next().unwrap().len();
    let gears_numbers: Vec<_> = line.char_indices()
        .filter(|(_, c)| c == &'*')
        .map(|(i, _)| i)
        .map(|i: usize| _neighbours(&line, width, &i))
        .collect();

    let mut res: u64 = 0;
    for nb_str in gears_numbers {
        let removed_dot = nb_str.replace(|c: char| c == '.' || c == '*', " ");
        let list_numbers: Vec<_> = removed_dot.split_ascii_whitespace().collect();
        let mut curr: u64 = 1;
        if list_numbers.len() == 2 {
            list_numbers.iter().for_each(|s| curr = curr * s.parse::<u64>().unwrap());
            println!("Curr :\n{}", curr);
            res += curr;
        }
    }
    return res;
}

fn main() {
    let test_value: u64 = number_in_array(String::from("test.txt"));
    println!("Test value: {}", test_value);
    assert!(test_value == 467835);

    let res = number_in_array(String::from("input.txt"));
    println!("Final: {}", res)
}
