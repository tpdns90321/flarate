use std::str::FromStr;

use num_traits::Num;
use num_traits::identities::zero;

use super::error::MelodyError;

#[derive(Debug, PartialEq)]
pub enum Melody {
	Alphanumeric(Vec<u16>),
	Hex(Vec<u8>),
	Number(u16),
}

// split by 2(length), and parsing radix
fn parse_by_radix<R: Num>(value: &str, radix: u32) -> Vec<R> {
	value.chars()
		.fold((vec![], String::new()), |(mut result, mut temp), text| {
			// when temp length is 0, just push to temp
			if temp.len() == 0 {
				temp.push(text);
			} else {
				// when temp length is 1, just push to temp, and parsing temp!
				temp.push(text);
				// extracting alphanumeric from temp
				if let Ok(number) = R::from_str_radix(&temp, radix) {
					result.push(number);
				} else {
					// when occured parsing error, store 0
					result.push(zero());
				}
				// clear temp
				temp = String::new();
			}
			(result, temp)
		}).0
}

impl Melody {
	pub fn parse_alphanumeric(value: &str) -> Result<Melody, MelodyError> {
		if value.len() % 2 != 0 {
			return Err(MelodyError::OddLength)
		}

		Ok(Melody::Alphanumeric(parse_by_radix(value, 36)))
	}

	pub fn parse_hex(value: &str) -> Result<Melody, MelodyError> {
		if value.len() % 2 != 0 {
			return Err(MelodyError::OddLength)
		}

		Ok(Melody::Hex(parse_by_radix(value, 16)))
	}

	pub fn parse_number(value: &str) -> Result<Melody, MelodyError> {
		let num = u16::from_str(value);
		if let Ok(num) = num {
			Ok(Melody::Number(num))
		} else {
			Err(MelodyError::NotNumber)
		}
	}
}

