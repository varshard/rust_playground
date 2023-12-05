use std::collections::HashMap;

fn main() {
    let line = ".123.";
    println!("{}", line[1..4].to_string().parse::<i32>().unwrap());

    let mut map = HashMap::new();
    map.insert(1..4, 123);

    println!("{}", map.get(&(1..4)).unwrap());
}
