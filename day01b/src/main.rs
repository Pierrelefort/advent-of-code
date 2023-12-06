use std::fs;

fn get_first_number_string(str: String) -> char {
    let digits = [
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
        String::from("five"),
        String::from("six"),
        String::from("seven"),
        String::from("eight"),
        String::from("nine"),
    ];

    let mut found = false;
    let mut res: char = '0';
    for (i, c) in str.chars().enumerate() {
        if c.is_numeric() {
            res = c;
            break;
        }
        for (index, digit) in digits.iter().enumerate() {
            if str.get(i..(i+digit.len())).unwrap_or("") == digit {
                res = char::from_digit((index + 1) as u32, 10).unwrap_or(' ');
                found = true;
            }
        }

        if found {
            break;
        }
    }
    
    return res;
}

fn get_last_number_string(str: String) -> char {
    let digits = [
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
        String::from("five"),
        String::from("six"),
        String::from("seven"),
        String::from("eight"),
        String::from("nine"),
    ];

    let mut found = false;
    let mut res: char = '0';
    for (i, c) in str.chars().rev().enumerate() {
        if c.is_numeric() {
            res = c;
            break;
        }
        for (index, digit) in digits.iter().enumerate() {
            if str.get(str.len() - 1 - i..str.len() - 1 - i + digit.len()).unwrap_or("") == digit {
                res = char::from_digit((index + 1) as u32, 10).unwrap_or(' ');
                found = true;
            }
        }

        if found {
            break;
        }
    }
    
    return res;
}

fn trebuchet(str: String) -> u32 {
    let number1_char: char = get_first_number_string(str.clone());
    let number2_char: char = get_last_number_string(str.clone());

    let mut res_string: String = String::from(number1_char);
    res_string.push(number2_char);
    
    let mut res: u32 = 0;
    match res_string.parse::<u32>() {
        Ok(n) => res = n,
        Err(e) => println!("Error: {}", e),
    }

    return res;
}


fn file_to_value(path: String) -> u32 {
    let file_path: String = String::from(path);
    let contents = fs::read_to_string(file_path);

    let mut contents_str: String = String::from("");
    match contents {
        Ok(v) => contents_str = v,
        Err(e) => println!("error parsing header: {e:?}"),
    }

    let lines = contents_str.split("\n");

    let mut res: u32 = 0;
    for line in lines {
        let curr: u32 = trebuchet(line.to_string());
        res += curr;
    }

    return res;
}


fn main() {
    let test_value = file_to_value(String::from("test.txt"));
    assert!(test_value == 281);

    let res = file_to_value(String::from("input.txt"));
    println!("Final: {}", res)
}
