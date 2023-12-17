pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
	let mut p = 0;
	let length = nums.len();
	while p < length {
		let x = nums[p];
		let index = p + 1;
		if x < 1 || x >= length as i32 || x == index as i32{
			p += 1;
		} else {
			let next = (x-1) as usize;
			nums[p] = nums[next];
			nums[next] = x;
			if nums[p] == nums[next] {
				p += 1
			}
		}
	}


	for i in 1..=p {
		if nums[i - 1] != i as i32 {
			return i as i32
		}
	}

	(p + 1) as i32
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_112() {
		assert_eq!(3, first_missing_positive(vec![1,2,0]));
	}

	#[test]
	fn test() {
		assert_eq!(3, first_missing_positive(vec![-1,4,2,1,9,10]));
	}
}