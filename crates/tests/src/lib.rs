pub fn add(left: usize, right: usize) -> usize {
    match left.checked_add(right) {
        Some(sum) => {
            sum
        }
        None => {
            usize::MAX
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add_infinity() {
        assert_eq!(add(usize::MAX, 1), usize::MAX)
    }
}

