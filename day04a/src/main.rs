use std::fs;


fn scratchcards(text: String) -> u32{
    let mut res = 0;

    for game in text.split('\n') {
        let (_, card_numbers_str) = game.split_once(": ").unwrap();
        let (numbers_str, winning_numbers_str) = card_numbers_str.split_once(" | ").unwrap();
        let numbers: Vec<_> = numbers_str.split_ascii_whitespace().collect();
        let winning_numbers: Vec<_> = winning_numbers_str.split_ascii_whitespace().collect();

        let mut value: u32 = 0;
        winning_numbers.iter().for_each(|w_nb| {
            if numbers.contains(w_nb) {
                value = value * 2;

                if value == 0 {
                    value = 1;
                }
            }
        });

        res += value;
    }
    return res;
}


fn load_text(path_file: String) -> String {
    let file_path: String = String::from(path_file);
    let contents = fs::read_to_string(file_path);
    
    let mut text: String = String::from("");
    match contents {
        Ok(v) => text = v,
        Err(e) => println!("error parsing header: {e:?}"),
    }

    return text;
}

fn main() {
    let test_text: String = load_text(String::from("test.txt"));
    let test_value = scratchcards(test_text);
    println!("Test value: {}", test_value);
    assert!(test_value == 13);
    
    let res_text = load_text(String::from("input.txt"));
    let res = scratchcards(res_text);
    println!("Final: {}", res)
}
