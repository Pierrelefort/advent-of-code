use misc::load_text;


fn if_you_give_a_seed_a_fertilizer(text: &str) -> u64 {
    let (mut seeds, unparse_text) = text.split_once("\n\n").unwrap();

    seeds = seeds.split_once("seeds: ").unwrap().1;
    // println!("seeds: {}", seeds);
    let mut vec: Vec<u64> = Vec::new();

    let list_map_str: Vec<&str> = unparse_text.split("\n\n").collect();

    for seed in seeds.split_ascii_whitespace() {
        let mut curr = seed.parse::<u64>().unwrap();
        for map_str in list_map_str.iter() {
            let (name, values_str) = map_str.split_once(" map:\n").unwrap_or(("", ""));
            // println!("{} map:\n{}", name, values_str);
            let values = values_str.split('\n');
            for value in values {
                let mut destination_source_length = value.splitn(3, ' ');
                let destination = destination_source_length.next().unwrap().parse::<u64>().unwrap();
                let source = destination_source_length.next().unwrap().parse::<u64>().unwrap();
                let length = destination_source_length.next().unwrap().parse::<u64>().unwrap();

                let curr_destination;
                if curr >= source && curr < source + length {
                    curr_destination = curr + destination - source; 
                    println!("Map: {} Source: {} Destination : {}", name, curr, curr_destination);
                    curr = curr_destination;
                    break;
                }
            }
        }

        println!("Location: {}", curr);
        vec.push(curr);
    }
    return *vec.iter().min().unwrap();
}

fn main() {
    let test_text: String = load_text(String::from("test.txt"));
    let test_value = if_you_give_a_seed_a_fertilizer(test_text.as_str());
    println!("Test value: {}", test_value);
    assert!(test_value == 35);
    
    let res_text: String = load_text(String::from("input.txt"));
    let res = if_you_give_a_seed_a_fertilizer(res_text.as_str());
    println!("Final: {}", res)
}
