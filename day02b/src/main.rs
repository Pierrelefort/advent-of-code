use std::cmp::Ordering;
use std::fs;
use std::collections::HashMap;

fn _minimum_cubes(str: &str) -> u32 {
    let mut res = 1;

    let (_, round_str) = str.split_once(": ").unwrap_or(("", ""));
    let round_iter = round_str.split("; ");

    let mut max_cube : HashMap<&str, u32> = HashMap::from([
        ("red", 1),
        ("green", 1),
        ("blue", 1),
    ]);

    for round in round_iter {
        let cube_info_iter = round.split(", ");
        for cube_info in cube_info_iter {
            let (cube_number_str, cube_color) = cube_info.split_once(" ").unwrap();
            let cube_number = cube_number_str.parse::<u32>().unwrap();
            if !cube_color.is_empty() {
                let curr_max_color = max_cube.get_mut(cube_color).unwrap();
                if cube_number.cmp(curr_max_color) == Ordering::Greater {
                    max_cube.insert(cube_color, cube_number);
                }
            }
        }
    }

    for cube_color in max_cube.keys() {
        let value: u32 = max_cube.get(cube_color).unwrap().clone(); 
        res *= value;
    }

    println!("String: {} Value: {}", str, res);
    return res;
}

fn minimum_cubes(path_file: String) -> u32 {
    let file_path: String = String::from(path_file);
    let contents = fs::read_to_string(file_path);

    let mut contents_str: String = String::from("");
    match contents {
        Ok(v) => contents_str = v,
        Err(e) => println!("error parsing header: {e:?}"),
    }

    let lines = contents_str.split("\n");

    let mut res: u32 = 0;
    for (_, line) in lines.enumerate() {
        res += _minimum_cubes(line);
    }
    return res;
}

fn main() {
    let test_value = minimum_cubes(String::from("test.txt"));
    println!("Test value: {}", test_value);
    assert!(test_value == 2286);

    let res = minimum_cubes(String::from("input.txt"));
    println!("Final: {}", res)
}
