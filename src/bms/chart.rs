use std::collections::HashMap;
use std::str::FromStr;

use getset::Getters;
use regex::Regex;

use super::channel::Channel;
use super::error::ChartError;
use super::melody::Melody;

lazy_static! {
    static ref ALPHANUMERIC_MELODY_PARSER: Regex = Regex::new(
        // #xxxyy:zzzzzz...
        r"#([\dA-Za-z]{3})([\dA-Za-z]{2}):([\dA-Za-z\.]+)"
        // xxx: bar number, length is 3,
        // yy: channel code, length is 2,
        // zzzz...: bar's melody, length is always even,
        // but melody's number type is free length
    ).unwrap();
    static ref HEXDECIMAL_MELODY_PARSER: Regex = Regex::new(
        // #xxxyy:zzzzzz...
        r"#([\dA-Fa-f]{3})([\dA-Fa-f]{2}):([\dA-Za-z\.]+)"
        // xxx: bar number, length is 3, but hexdecimal
        // rest are same
    ).unwrap();
}

#[derive(Debug, Getters)]
pub struct Chart {
    #[getset(get = "pub")]
    melodies: HashMap<u16, Melody>,
}

pub enum ChartType {
    Hex,
    Alphanum,
}

impl Chart {
    pub fn parse(chart_type: ChartType, value: &str) -> Result<Self, ChartError> {
        // check and set radix, regex parser
        let (radix, parser): (_, &Regex) = if let ChartType::Alphanum = chart_type {
            (36, &ALPHANUMERIC_MELODY_PARSER)
        } else {
            (16, &HEXDECIMAL_MELODY_PARSER)
        };

        let mut chart = Self {
            melodies: HashMap::new(),
        };
        for capture in parser.captures_iter(value) {
            let num = u16::from_str_radix(&capture[1], radix).unwrap();
            let channel = Channel::from_str(&capture[2]);
            let data = &capture[3];

            match channel {
                Ok(channel) => {
                    if let Some(melody) = chart.melodies.get_mut(&num) {
                        melody.apply(channel, data);
                    } else {
                        let mut melody = Melody::default();
                        melody.apply(channel, data);
                        chart.melodies.insert(num, melody);
                    }
                }
                // when implemented logging module, implement error logging!
                Err(_err) => {}
            }
        }

        // can't run empty chart
        if chart.melodies.len() == 0 {
            return Err(ChartError::EmptyChart);
        }

        Ok(chart)
    }
}
