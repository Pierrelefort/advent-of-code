use misc::load_text;

fn wait_for_it(text: &str) -> u64 {
    // Parse input
    let (time_str, distance_str) = text.split_once('\n').unwrap();

    // "Distance:  ".len() = 10
    let time: u64= time_str.split_at("Time:      ".len()).1.replace(" ", "").parse::<u64>().unwrap();
    let distance: u64= distance_str.split_at("Distance:  ".len()).1.replace(" ", "").parse::<u64>().unwrap();

    let mut curr_speed = 0;
    let mut fast_enough = false;
    while !fast_enough {
        curr_speed += 1;
        fast_enough = curr_speed * (time - curr_speed) > distance;
    }
    let min_time = curr_speed;

    curr_speed = time - 1;
    fast_enough = curr_speed * (time - curr_speed) > distance;
    while !fast_enough {
        curr_speed -= 1;
        fast_enough = curr_speed * (time - curr_speed) > distance;
    }
    let max_time = curr_speed;

    println!("{}", max_time - min_time + 1);
    return  max_time - min_time + 1;
}


fn main() {
    let test_text: String = load_text(String::from("test.txt"));
    let test_value = wait_for_it(test_text.as_str());
    println!("Test value: {}", test_value);
    assert!(test_value == 71503);
    
    let res_text: String = load_text(String::from("input.txt"));
    let res = wait_for_it(res_text.as_str());
    println!("Final: {}", res)
}