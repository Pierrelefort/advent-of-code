use misc::load_text;

fn wait_for_it(text: &str) -> u32 {
    let mut res = 1;
    // Parse input
    let (time_str, distance_str) = text.split_once('\n').unwrap();

    // "Distance:  ".len() = 10
    let time_vec: Vec<u32>= time_str.split_at("Time:      ".len()).1.split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
    let distance_vec: Vec<u32>= distance_str.split_at("Distance:  ".len()).1.split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();

    let mut count_ways: Vec<u32> = Vec::new();

    for i in 0..distance_vec.len() {
        let mut curr_speed = 0;
        let mut fast_enough = false;
        while !fast_enough {
            curr_speed += 1;
            fast_enough = curr_speed * (time_vec[i] - curr_speed) > distance_vec[i];
        }
        let min_time = curr_speed;

        curr_speed = time_vec[i] - 1;
        fast_enough = curr_speed * (time_vec[i] - curr_speed) > distance_vec[i];
        while !fast_enough {
            curr_speed -= 1;
            fast_enough = curr_speed * (time_vec[i] - curr_speed) > distance_vec[i];
        }
        let max_time = curr_speed;

        println!("{} {}", min_time, max_time);
        count_ways.push(max_time - min_time + 1);
    }

    count_ways.iter().for_each(|v| {
        res = res * v;
        println!("{}", v);
    });
    return  res;
}


fn main() {
    let test_text: String = load_text(String::from("test.txt"));
    let test_value = wait_for_it(test_text.as_str());
    println!("Test value: {}", test_value);
    assert!(test_value == 288);
    
    let res_text: String = load_text(String::from("input.txt"));
    let res = wait_for_it(res_text.as_str());
    println!("Final: {}", res)
}