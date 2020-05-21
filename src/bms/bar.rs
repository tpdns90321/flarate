use std::str::FromStr;
use std::collections::BTreeMap;

use regex::Regex;

use super::channel::Channel;
use super::melody::Melody;
use super::error::BarError;

lazy_static! {
	static ref BAR_PARSER: Regex = Regex::new(
		// #xxxyy:zzzzzz...
		r"#([^_\W]{3})([^_\W]{2}):([^_\W]+)"
		// xxx: bar number, length is 3,
		// yy: channel code, length is 2,
		// zzzz...: bar's melody, length is always even,
		// but melody's number type is free length
	).unwrap();
}

#[derive(Default)]
pub struct Bar {
	melody: BTreeMap<(Channel, u16), Melody>
}

pub enum BarType {
	Hex,
	Alphanumeric,
}

impl Bar {
	pub fn parse(value: &str, bar_type: BarType) -> Result<Self, BarError> {
		let radix = if let BarType::Alphanumeric = bar_type {
			36
		} else {
			16
		};

		let mut bar = Self::default();
		for capture in BAR_PARSER.captures_iter(value) {
			let num = u16::from_str_radix(&capture[1], radix);
			let channel = Channel::from_str(&capture[2]);
			let data = &capture[3];

			match (num, channel) {
				(Ok(num), Ok(channel)) => {
					if let Ok(melody) = channel.parse_melody(data) {
						bar.melody.insert((channel, num), melody);
					}
				},
				_ => {
					// when implemented logging module, implement error logging!
				}
			}
		}

		if bar.melody.len() == 0 {
			return Err(BarError::EmptyBar);
		}

		Ok(bar)
	}
}
