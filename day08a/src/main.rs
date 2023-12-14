use std::collections::HashMap;

use misc::load_text;

fn haunted_wasteland(text: &str) -> u32 {
    // Parse input
    let instructions: &str = text.split_once("\n").unwrap().0;

    let mut hashmap: HashMap<&str, (&str, &str)> = HashMap::new();
    let lines =  text.split_once("\n\n").unwrap().1.split("\n");
    for line in lines {
        let (key, mut keys) = line.split_once(" = ").unwrap();
        keys = &keys[1..keys.len() - 1];

        let (key_left, key_right) = keys.split_once(", ").unwrap();
        hashmap.insert(key, (key_left, key_right));
    }

    let mut index = "AAA";
    let mut steps = 0;
    let stop_key = "ZZZ";
    while index != stop_key {
        for instruction in instructions.chars() {
            if instruction == 'L' {
                index = hashmap.get_key_value(index).unwrap().1.0;
                steps += 1;
            }
            else if instruction == 'R' {
                index = hashmap.get_key_value(index).unwrap().1.1;
                steps += 1;
            }

            if index == stop_key {
                return steps;
            }
        }
    }

    return steps;
}

fn main() {
    let test_text: String = load_text(String::from("test.txt"));
    let test_value = haunted_wasteland(test_text.as_str());
    println!("Test value: {}", test_value);
    assert!(test_value == 6);
    
    let res_text: String = load_text(String::from("input.txt"));
    let res = haunted_wasteland(res_text.as_str());
    println!("Final: {}", res)
}
