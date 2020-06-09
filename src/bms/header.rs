use std::str::FromStr;

use getset::{Getters, MutGetters};
use regex::Regex;

lazy_static! {
	// #XXX... YYY...
	static ref HEADER_PARSER: Regex = Regex::new(r"#(\w+)\s+([\S ]+)").unwrap();
	// XXX...: header name
	// YYY...: header data
}

// BMS file header
#[derive(Getters, MutGetters)]
pub struct Header {
	// empty title is impossible, but bms's folder name can replace it.
	#[getset(get="pub", set, get_mut)]
	title:		String,
	#[getset(get="pub", get_mut)]
	artist:		String,
	// unique level naming
	#[getset(get="pub", get_mut)]
	playlevel:	String,
	#[getset(get="pub", get_mut)]
	bpm:		f32,
	#[getset(get="pub", get_mut)]
	total:		u16,
	#[getset(get="pub", get_mut)]
	level:		Option<u16>,
	// maybe more attribute
}

impl Default for Header {
	// bms header's default value allocation
	fn default() -> Header {
		Header {
			title:		String::new(),
			artist:		String::new(),
			playlevel:	String::new(),
			bpm:		130.0,
			total:		160,
			level:		None,
		}
	}
}

// before applying, check error
#[inline(always)]
fn check_and_apply<F: FromStr>(target: &mut F, value: &str) {
	if let Ok(v) = F::from_str(value) {
		*target = v;
	}
}

impl Header {
	pub fn parse(plain: &str) -> Header {
		// set default value or None!
		let mut header = Header::default();
	
		// search header and dispense result
		for cap in HEADER_PARSER.captures_iter(plain) {
			let value = &cap[2];
			match &cap[1] {
				"TITLE"		=>	{ *header.title_mut() = String::from(value) },
				"ARTIST"	=>	{ *header.artist_mut() = String::from(value) },
				"PLAYLEVEL"	=>	{ *header.playlevel_mut() = String::from(value) },
				"BPM"		=>	{ check_and_apply(header.bpm_mut(), value); }
				"TOTAL"		=>	{ check_and_apply(header.total_mut(), value); }
				"LEVEL"		=>	{ *header.level_mut() = value.parse().ok() },
				_ => {}
			}
		}
	
		return header;
	}
}
