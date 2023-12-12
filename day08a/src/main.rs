use misc::load_text;

fn haunted_wasteland(text: &str) -> u32 {
    return 0;
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
