use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Hello, rust!");
    println!("Did you date match?{}", re.is_match("2022-02-03"));
}
