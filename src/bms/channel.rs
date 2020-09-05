use std::str::FromStr;

use super::error::ChannelError;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Channel {
    // Event Channel
    Event(Event),
    // Object Channel
    Object(Object),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Event {
    BGM,
    Length,
    BPM,
    ExBPM,
    Stop,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Object {
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
                '1' => Ok(Channel::Event(Event::BGM)),
                '2' => Ok(Channel::Event(Event::Length)),
                '3' => Ok(Channel::Event(Event::BPM)),
                '8' => Ok(Channel::Event(Event::ExBPM)),
                '9' => Ok(Channel::Event(Event::Stop)),
                _ => Err(ChannelError::UnsupportedChannel),
            }
        } else {
            // Object Channel
            if let Ok(alphanum) = u8::from_str_radix(&number.to_string(), 36) {
                match category {
                    '1' => Ok(Channel::Object(Object::FirstPlayerVisible(alphanum))),
                    '2' => Ok(Channel::Object(Object::SecondPlayerVisible(alphanum))),
                    '3' => Ok(Channel::Object(Object::FirstPlayerInvisible(alphanum))),
                    '4' => Ok(Channel::Object(Object::SecondPlayerInvisible(alphanum))),
                    _ => Err(ChannelError::UnsupportedChannel),
                }
            } else {
                Err(ChannelError::InvalidChannel)
            }
        }
    }
}
