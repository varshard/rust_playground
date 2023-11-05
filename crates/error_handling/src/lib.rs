// PartialEq is required for comparison
#[derive(Debug, Clone, PartialEq)]
pub enum DoubleError {
  NotANumber,
  None
}

pub fn doubler_first(vec: Vec<&str>) -> Result<i32, DoubleError> {
    vec.first()
      .ok_or(DoubleError::None)
      .and_then(|s| {
          s.parse::<i32>()
            .map_err(|_| DoubleError::NotANumber)
            .map(|i| 2*i)
      })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
      assert_eq!(doubler_first(vec!["2", "3"]), Ok(4));
      assert_eq!(doubler_first(vec![]), Err(DoubleError::None));
      assert_eq!(doubler_first(vec!["x"]), Err(DoubleError::NotANumber));
    }
}
