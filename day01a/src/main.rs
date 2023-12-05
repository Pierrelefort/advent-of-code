use std::fs;

fn get_first_number_string(str: String) -> char {
    let mut res: char = ' ';
    for c in str.chars() {
        if c.is_numeric() {
            res = c;
            break;
        }
    }
    
    return res;
}

fn get_last_number_string(str: String) -> char {
    let mut res: char = ' ';
    for c in str.chars().rev() {
        if c.is_numeric() {
            res = c;
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

fn main() {
    let file_path: String = String::from("input.txt");
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
        println!("{}", curr);
        res += curr;
    }

    println!("Final: {}", res);
}
