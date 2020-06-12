use std::str::FromStr;

use super::error::ChannelError;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Channel {
	// Event Channel
	BGM,
	Length,
	BPM,
	ExBPM,
	Stop,
	// Object Channel
	FirstPlayerVisible(u8),
	SecondPlayerVisible(u8),
	FirstPlayerInvisible(u8),
	SecondPlayerInvisible(u8),
}

impl FromStr for Channel {
	type Err = ChannelError;

	fn from_str(value: &str) -> Result<Channel, Self::Err> {
		if value.len() != 2 {
			return Err(ChannelError::InvalidChannel);
		}

		let mut target = value.chars();
		let category = target.next().unwrap();
		let number = target.next().unwrap();

		if category == '0' {
			// Event Channel
			match number {
				'1' => Ok(Channel::BGM),
				'2' => Ok(Channel::Length),
				'3' => Ok(Channel::BPM),
				'8' => Ok(Channel::ExBPM),
				'9' => Ok(Channel::Stop),
				_ => Err(ChannelError::UnsupportedChannel),
			}
		} else {
			// Object Channel
			if let Ok(alphanum) = u8::from_str_radix(&number.to_string(), 36) {
				match category {
					'1' => Ok(Channel::FirstPlayerVisible(alphanum)),
					'2' => Ok(Channel::SecondPlayerVisible(alphanum)),
					'3' => Ok(Channel::FirstPlayerInvisible(alphanum)),
					'4' => Ok(Channel::SecondPlayerInvisible(alphanum)),
					_ => Err(ChannelError::UnsupportedChannel)
				}
			} else {
				Err(ChannelError::InvalidChannel)
			}
		}
	}
}

