use std::cmp::Ordering;

use misc::load_text;


fn compare_card (a: char, b: char) -> Ordering {
    let compare = "23456789TJQKA";
    return compare.find(a).unwrap().cmp(&compare.find(b).unwrap());
}

fn generate_histo(hand: &str) -> Vec<u32> {
    let mut vec_chars: Vec<char> = hand.chars().collect();
    
    vec_chars.sort();
    vec_chars.dedup();

    let mut histo: Vec<u32> = Vec::new();
    for c in vec_chars {
        histo.push(hand.matches(c).count() as u32);
    }
    histo.sort_by(|a, b| b.cmp(a));

    return histo;
}


fn compare_hands(a: &(&str, u32), b: &(&str, u32)) -> Ordering {
    let histo_a = generate_histo(a.0);
    let histo_b = generate_histo(b.0);
    println!("{}:", a.0);
    histo_a.iter().for_each(|v| print!("{}", v));
    println!();

    println!("{}:", b.0);
    histo_b.iter().for_each(|v| print!("{}", v));
    println!();


    let mut ordering = Ordering::Equal;
    let mut index_a = 0;
    let mut index_b = 0;
    while index_a < histo_a.len() && index_b < histo_b.len() && ordering == Ordering::Equal {
        ordering = histo_a[index_a].cmp(&histo_b[index_b]);
        index_a += 1;
        index_b += 1;
    }

    if ordering == Ordering::Equal {
        let mut index = 0;
        while index < a.0.len() && ordering == Ordering::Equal {
            ordering = compare_card(a.0.chars().nth(index).unwrap(), b.0.chars().nth(index).unwrap());
            index += 1;
        }
    }

    return ordering;
}

fn camel_cards(text: &str) -> u32 {
    let mut res = 0;

    // Parse input
    let mut vec: Vec<(&str, u32)> = text.split('\n').map(|s| {
        let (hand, bid) = s.split_once(" ").unwrap();
        return (hand, bid.parse::<u32>().unwrap());
    }).collect();
    vec.sort_by(compare_hands);

    for (i, (hand, bid)) in vec.iter().enumerate() {
        println!("{}: {} - {}", i + 1, hand, bid);
        res = res + (bid * (i + 1) as u32);
    }
    return res;
}

fn main() {
    let test_text: String = load_text(String::from("test.txt"));
    let test_value = camel_cards(test_text.as_str());
    println!("Test value: {}", test_value);
    assert!(test_value == 6440);
    
    let res_text: String = load_text(String::from("input.txt"));
    let res = camel_cards(res_text.as_str());
    println!("Final: {}", res)
}