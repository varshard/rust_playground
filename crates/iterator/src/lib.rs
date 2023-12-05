pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn fold_map() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(String::from("foo"), String::from("bar"));
        map.insert(String::from("a"), String::from("bar"));
        map.insert(String::from("b"), String::from("baz"));

        assert_eq!(map.iter().filter(|(k, v)| **v == String::from("bar"))
          .count(), 2)
    }
}
