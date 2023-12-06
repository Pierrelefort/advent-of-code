use std::cmp::Ordering;
use std::fs;
use std::collections::HashMap;

fn _is_possible_game(str: &str, max_cube: HashMap<&str, u32>) -> bool {
    let mut invalid = false;
    let (_, games_str) = str.split_once(": ").unwrap_or(("", ""));
    let game_iter = games_str.split("; ");

    for game in game_iter {
        let mut curr_max_cube : HashMap<&str, u32> = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        let cube_info_iter = game.split(", ");
        for cube_info in cube_info_iter {
            let (cube_number_str, cube_color) = cube_info.split_once(" ").unwrap_or(("0", ""));
            let cube_number = cube_number_str.parse::<u32>().unwrap();
            if !cube_color.is_empty() {
                let curr_max_color = curr_max_cube.get_mut(cube_color).unwrap();
                if cube_number.cmp(curr_max_color) == Ordering::Greater {
                    curr_max_cube.insert(cube_color, cube_number);
                }
            }
        }

        for cube_color in max_cube.keys() {
            if curr_max_cube.get_key_value(cube_color) > max_cube.get_key_value(cube_color) {
                // Invalid game: too many cube of this color
                invalid = true;
            }
        }
    }

    return invalid
}

fn is_possible_game(path_file: String, max_cube: HashMap<&str, u32>) -> u32 {
    let file_path: String = String::from(path_file);
    let contents = fs::read_to_string(file_path);

    let mut contents_str: String = String::from("");
    match contents {
        Ok(v) => contents_str = v,
        Err(e) => println!("error parsing header: {e:?}"),
    }

    let lines = contents_str.split("\n");

    let mut res: u32 = 0;
    for (i, line) in lines.enumerate() {
        let invalid = _is_possible_game(line, max_cube.clone());
        if invalid {
            println!("Skiped Game {}", i + 1);
        }
        else {
            res += i as u32 + 1;
        }
    }
    return res;
}

fn main() {
    let max_hash: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let test_value = is_possible_game(String::from("test.txt"), max_hash);
    assert!(test_value == 8);

    let max_hash: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let res = is_possible_game(String::from("input.txt"), max_hash);
    println!("Final: {}", res)
}
