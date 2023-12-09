use std::fs;
use std::collections::HashMap;

fn _scratchcards(text: &str, card_number: u32) -> u32{
    let line = text.split('\n').nth(card_number as usize - 1).unwrap();
    let (numbers_str, winning_numbers_str) = line.split_once(" | ").unwrap();
    let numbers: Vec<_> = numbers_str.split_ascii_whitespace().collect();
    let winning_numbers: Vec<_> = winning_numbers_str.split_ascii_whitespace().collect();

    let mut value: u32 = 0;
    winning_numbers.iter().for_each(|w_nb| {
        if numbers.contains(w_nb) {
            value = value + 1;
        }
    });

    return value;
}

fn scratchcards(text: &str) -> u32{
    let mut res = 0;
    let mut map_copies: HashMap<u32, u32> = HashMap::new();
    let nb_cards = text.split('\n').count() as u32;
    (1..nb_cards + 1).for_each(|k: u32| {
        map_copies.insert(k, 0);
    });
    
    for card_number in 1..nb_cards + 1 {
        map_copies.insert(card_number, map_copies[&card_number] + 1);

        let value = _scratchcards(&text, card_number);
        if value != 0 {
            let mut max =  card_number + value;
            if card_number + value > nb_cards {
                max = nb_cards;
            }
            for _ in 0..map_copies[&card_number] {
                (card_number+1..max+1).for_each(|k: u32| { 
                    map_copies.insert(k, map_copies[&k] + 1); 
                });
            }

        }
    } 
    
    map_copies.iter().for_each(|(k, v)| {
        println!("{} {}", k, v);
        res += v;
    });
    
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
    let test_value = scratchcards(test_text.as_str());
    println!("Test value: {}", test_value);
    assert!(test_value == 30);
    
    let res_text: String = load_text(String::from("input.txt"));
    let res = scratchcards(res_text.as_str());
    println!("Final: {}", res)
}
