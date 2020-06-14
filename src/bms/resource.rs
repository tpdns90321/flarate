use std::collections::HashMap;

use regex::Regex;

lazy_static! {
	static ref RESOURCE_PARSER: Regex = Regex::new(
		// xxxyy zzzzzz...
		r"#(WAV|OGG|BPM)([\da-zA-Z]{2})\s+(\S+)"
		// xxx: file type, string
		// yy: index number, alphanumeric
		// zzzzzz...: file name or data
	).unwrap();
}

#[derive(Debug, Default)]
pub struct Resource {
	audio: HashMap<u16, String>,
	bpm: HashMap<u16, f32>,
}
