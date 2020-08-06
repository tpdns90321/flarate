use std::str::FromStr;
use std::collections::HashMap;

use getset::Getters;
use num_traits::Num;
use num_traits::identities::zero;

use super::channel::Channel;
use super::error::MelodyError;

type Hex = Vec<u8>;
type Alphanum = Vec<u16>;
type Float = f32;

#[derive(Debug, Getters)]
pub struct Melody {
	#[getset(get="pub")]
	bgm: Alphanum,
	#[getset(get="pub")]
	length: Float,
	#[getset(get="pub")]
	bpm: Hex,
	#[getset(get="pub")]
	ex_bpm: Alphanum,
	#[getset(get="pub")]
	stop: Hex,
	#[getset(get="pub")]
	first_player_visible: HashMap<u8, Alphanum>,
	#[getset(get="pub")]
	second_player_visible: HashMap<u8, Alphanum>,
	#[getset(get="pub")]
	first_player_invisible: HashMap<u8, Alphanum>,
	#[getset(get="pub")]
	second_player_invisible: HashMap<u8, Alphanum>,
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

fn parse_alphanum(value: &str) -> Result<Alphanum, MelodyError> {
	if value.len() % 2 != 0 {
		return Err(MelodyError::OddLength)
	}

	Ok(parse_by_radix(value, 36))
}

fn parse_hex(value: &str) -> Result<Hex, MelodyError> {
	if value.len() % 2 != 0 {
		return Err(MelodyError::OddLength)
	}

	Ok(parse_by_radix(value, 16))
}

fn parse_float(value: &str) -> Result<Float, MelodyError> {
	let num = Float::from_str(value);
	if let Ok(num) = num {
		Ok(num)
	} else {
		Err(MelodyError::NotNumber)
	}
}

#[inline(always)]
fn check_apply<V>(target: &mut V, value: Result<V, MelodyError>) {
	match value {
		Ok(v) => { *target = v; },
		Err(_err) => { },
	};
}

#[inline(always)]
fn check_apply_object(target: &mut HashMap<u8,Vec<u16>>, key: u8, value: &str) {
	match parse_alphanum(value) {
		Ok(v) => { target.insert(key, v); },
		Err(_err) => { },
	};
}

impl Melody {
	pub fn apply(&mut self, channel: Channel, value: &str) {
		match channel {
			Channel::BGM => check_apply(&mut self.bgm, parse_alphanum(value)),
			Channel::Length => check_apply(&mut self.length, parse_float(value)),
			Channel::BPM => check_apply(&mut self.bpm, parse_hex(value)),
			Channel::ExBPM => check_apply(&mut self.ex_bpm, parse_alphanum(value)),
			Channel::Stop => check_apply(&mut self.stop, parse_hex(value)),
			Channel::FirstPlayerVisible(k) => check_apply_object(&mut self.first_player_visible, k, value),
			Channel::SecondPlayerVisible(k) => check_apply_object(&mut self.second_player_visible, k, value),
			Channel::FirstPlayerInvisible(k) => check_apply_object(&mut self.first_player_invisible, k, value),
			Channel::SecondPlayerInvisible(k) => check_apply_object(&mut self.second_player_invisible, k, value),
		};
	}
}

impl Default for Melody {
	fn default() -> Self {
		Self {
			bgm: vec![],
			length: 1.0,
			bpm: vec![],
			ex_bpm: vec![],
			stop: vec![],
			first_player_visible: HashMap::new(),
			second_player_visible: HashMap::new(),
			first_player_invisible: HashMap::new(),
			second_player_invisible: HashMap::new(),
		}
	}
}
