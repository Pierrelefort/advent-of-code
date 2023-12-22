use misc::load_text;

fn _mirage_maintenance(line: &str) -> i64 {
    println!("Line: {}", line);
    let curr_vec: Vec<i64> = line.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect();
    let first = *curr_vec.first().unwrap();

    if !curr_vec.iter().all(|&i| i == first) {
        // Create new line of difference
        let mut str = String::from("");
        for i in 0..curr_vec.len() - 1 {
            let value = (curr_vec[i + 1] - curr_vec[i]).to_string();
            str = str + " " + value.as_str();
        }

        return first - _mirage_maintenance(str.as_str());
    }
    return first;
}

fn mirage_maintenance(text: &str) -> i64 {
    let lines = text.split("\n");
    let mut res = 0;

    for line in lines {
        res = res +  _mirage_maintenance(line)
    }


    return res;
}

fn main() {
    let test_text: String = load_text(String::from("test.txt"));
    let test_value = mirage_maintenance(test_text.as_str());
    println!("Test value: {}", test_value);
    assert!(test_value == 2);
    
    let res_text: String = load_text(String::from("input.txt"));
    let res = mirage_maintenance(res_text.as_str());
    println!("Final: {}", res)
}
