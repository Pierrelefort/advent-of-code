use std::collections::HashMap;

use misc::load_text;

fn haunted_wasteland(text: &str) -> u32 {
    // Parse input
    let instructions: &str = text.split_once("\n").unwrap().0;
    let mut start_indexes: Vec<String> = Vec::new();
    let mut vec_end: Vec<bool> = Vec::new();

    let mut hashmap: HashMap<String, (String, String)> = HashMap::new();
    let lines =  text.split_once("\n\n").unwrap().1.split("\n");
    for line in lines {
        let (key, mut keys) = line.split_once(" = ").unwrap();
        if key.ends_with('A') {
            start_indexes.push(String::from(key));
            vec_end.push(false);
        }
        keys = &keys[1..keys.len() - 1];

        let (key_left, key_right) = keys.split_once(", ").unwrap();
        hashmap.insert(String::from(key), (String::from(key_left), String::from(key_right)));
    }
    
    let mut steps = 0;
    while !vec_end.iter().all(|b| *b == true) {
        for instruction in instructions.chars() {
            vec_end = vec_end.iter().map(|v| false).collect();
            
            println!("Step {}:", steps);
            let len_start_indexes = start_indexes.len();
            for (i, index) in start_indexes.iter_mut().enumerate() {
                print!("{}{}", index, if i != len_start_indexes - 1 { "," } else { "\n" } );

                let mut value = "".to_string(); // Assign an initial value to the `value` variable.
                if instruction == 'L' {
                    value = hashmap.get_key_value(index).unwrap().1.0.clone();
                }
                else if instruction == 'R' {
                    value = hashmap.get_key_value(index).unwrap().1.1.clone();
                }
                index.clear();
                index.insert_str(0, &value);

                if index.ends_with("Z") {
                    vec_end[i] = true;
                }
            }
            steps += 1;
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
