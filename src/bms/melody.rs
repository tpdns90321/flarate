use std::str::FromStr;
use std::collections::HashMap;

use num_traits::Num;
use num_traits::identities::zero;

use super::channel::Channel;
use super::error::MelodyError;

type Hex = Vec<u8>;
type Alphanum = Vec<u16>;
type RealNum = f32;

#[derive(Debug, PartialEq)]
pub struct Melody {
	bgm: Alphanum,
	length: RealNum,
	bpm: Hex,
	ex_bpm: Alphanum,
	stop: Hex,
	first_player_visible: HashMap<u8, Alphanum>,
	second_player_visible: HashMap<u8, Alphanum>,
	first_player_invisible: HashMap<u8, Alphanum>,
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

pub(super) fn parse_alphanum(value: &str) -> Result<Alphanum, MelodyError> {
	if value.len() % 2 != 0 {
		return Err(MelodyError::OddLength)
	}

	Ok(parse_by_radix(value, 36))
}

pub(super) fn parse_hex(value: &str) -> Result<Hex, MelodyError> {
	if value.len() % 2 != 0 {
		return Err(MelodyError::OddLength)
	}

	Ok(parse_by_radix(value, 16))
}

pub(super) fn parse_num(value: &str) -> Result<RealNum, MelodyError> {
	let num = RealNum::from_str(value);
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
fn check_apply_map<V>(target: &mut HashMap<u8,V>, key: u8, value: Result<V, MelodyError>) {
	match value {
		Ok(v) => { target.insert(key, v); },
		Err(_err) => { },
	};
}

impl Melody {
	pub fn apply(&mut self, channel: Channel, value: &str) {
		match channel {
			Channel::BGM => check_apply(&mut self.bgm, parse_alphanum(value)),
			Channel::Length => check_apply(&mut self.length, parse_num(value)),
			Channel::BPM => check_apply(&mut self.bpm, parse_hex(value)),
			Channel::ExBPM => check_apply(&mut self.ex_bpm, parse_alphanum(value)),
			Channel::Stop => check_apply(&mut self.stop, parse_hex(value)),
			Channel::FirstPlayerVisible(k) => check_apply_map(&mut self.first_player_visible, k,  parse_alphanum(value)),
			Channel::SecondPlayerVisible(k) => check_apply_map(&mut self.second_player_visible, k, parse_alphanum(value)),
			Channel::FirstPlayerInvisible(k) => check_apply_map(&mut self.first_player_invisible, k, parse_alphanum(value)),
			Channel::SecondPlayerInvisible(k) => check_apply_map(&mut self.second_player_invisible, k, parse_alphanum(value)),
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

