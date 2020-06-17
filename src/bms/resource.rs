use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;
use getset::Getters;

lazy_static! {
	static ref RESOURCE_PARSER: Regex = Regex::new(
		// xxxyy zzzzzz...
		r"#(WAV|BPM|STOP)([\da-zA-Z]{2})\s+(\S+)"
		// xxx: file type, string
		// yy: index number, alphanumeric
		// zzzzzz...: file name or data
	).unwrap();
}

#[derive(Debug, Default, Getters)]
pub struct Resource {
	#[getset(get="pub")]
	audio: HashMap<u16, String>,
	#[getset(get="pub")]
	bpm: HashMap<u16, f32>,
	#[getset(get="pub")]
	stop: HashMap<u16, u32>
}

#[inline(always)]
fn check_insert<V>(map: &mut HashMap<u16, V>, num: u16, value: Option<V>) {
	match value {
		Some(v) => { map.insert(num, v); },
		None => { }
	}
}

impl Resource {
	pub fn parse(plain: &str) -> Self {
		// default hashmap initialize
		let mut resource = Self::default();
		// regex parser iterate
		for cap in RESOURCE_PARSER.captures_iter(plain) {
			// variable alias
			let name = &cap[1];
			let number = u16::from_str_radix(&cap[2], 36).unwrap();
			let data = &cap[3];

			// resource type dispense
			match name {
				"WAV" => {
					resource.audio.insert(number, String::from(data));
				},
				"BPM" => {
					check_insert(&mut resource.bpm, number, f32::from_str(data).ok())
				},
				"STOP" => {
					check_insert(&mut resource.stop, number, u32::from_str(data).ok())
				}
				_ => { }
			}
		}

		return resource;
	}
}
