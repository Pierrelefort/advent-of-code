use misc::load_text;
use std::cmp::{max, min};

fn if_you_give_a_seed_a_fertilizer(text: &str) -> u32 {
    let seeds_str = text.split_once("\n\n").unwrap().0.split_once("seeds: ").unwrap().1;
    let mut seeds: Vec<u32> = seeds_str.split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
    seeds.sort();

    let maps_str = text.split_once("\n\n").unwrap().1;
    let mut maps: Vec<&str> = maps_str.split("\n\n").collect();
    let mut maps_values: Vec<&str> = Vec::new();
    maps.iter().for_each(|s| {
        maps_values.push(s.split_once(" map:\n").unwrap().1);
    });

    while seeds.len() > 0 {
        let seed = seeds.pop().unwrap();

        for map_values in &maps_values {
            for value in map_values.split('\n') {
                let mut value_splited = value.splitn(3, ' ');
                let destination = value_splited.next().unwrap();
                let source = value_splited.next().unwrap();
                let length = value_splited.next().unwrap();

                
            }
        }
    }
    return 0
}

fn main() {
    let test_text: String = load_text(String::from("test.txt"));
    let test_value = if_you_give_a_seed_a_fertilizer(test_text.as_str());
    println!("Test value: {}", test_value);
    assert!(test_value == 46);
    
    let res_text: String = load_text(String::from("input.txt"));
    let res = if_you_give_a_seed_a_fertilizer(res_text.as_str());
    println!("Final: {}", res)
}
