// https://doc.rust-lang.org/rust-by-example/fn.html
use std::str::FromStr;

#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    println!("----------------------------------------");
    fn get_count_item(s: &str) -> (u64, &str) {
        let mut it = s.split(' ');
        let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
            panic!("Can't segment count item pair: '{s}'");
        };
        let Ok(count) = u64::from_str(count_str) else {
            panic!("Can't parse integer: '{count_str}'");
        };
        (count, item)
    }
    assert_eq!(get_count_item("23 23"), (3, "chairs"));
    println!("----------------------------------------");
}