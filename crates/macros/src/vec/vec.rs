#[macro_export]
macro_rules! my_vec {
	// $() enclose a repetition pattern
	// pattern is <expr>, 0 - any times.
    ($($x:expr), *) => {
	    // this {} delimit the expansion code
	    {
				let mut v = Vec::new();
				// enclose the repetition pattern
				$(
					v.push($x);
				)*
				v
			}
	};
}

#[cfg(test)]
mod test {
    #[test]
    fn test_my_vec() {
        let new_vec: Vec<u32> = my_vec![1, 2, 3];
        assert_eq!(new_vec.len(), 3);
    }
}
