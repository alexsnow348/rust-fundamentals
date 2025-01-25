
fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    // This will not compile!
    let result = parts.get(field);
    result.expect("something went wrong").to_string()
}

fn main() {
    let chunk = split_string("hello,world".to_string(), ',', 0);
    println!("Split string: {}", chunk);
}
