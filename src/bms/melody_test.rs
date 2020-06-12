use super::melody::{parse_alphanum, parse_hex, parse_float, Melody};
use super::channel::Channel;

#[test]
fn test_melody_parse_alphanum() {
	// normal alphanum
	assert_eq!(
		parse_alphanum("00ZZ09").unwrap(),
		vec![0,1295,9]
	);
	// some error in the here
	assert_eq!(
		parse_alphanum("00Aㅋ0z").unwrap(),
		vec![0,0,35]
	);
	// odd length error
	assert!(parse_alphanum("00A").is_err());
}

#[test]
fn test_melody_parse_hex() {
	// normal hex
	assert_eq!(
		parse_hex("00FF09").unwrap(),
		vec![0,255,9]
	);
	// some error in the here
	assert_eq!(
		parse_hex("00Aㅋ0a").unwrap(),
		vec![0,0,10]
	);

	// alphanum range error
	assert_eq!(
		parse_hex("ZA").unwrap(),
		vec![0]
	);

	// odd length error
	assert!(parse_hex("00A").is_err());
}

#[test]
fn test_melody_parse_float() {
	assert_eq!(parse_float("0.5").unwrap(), 0.5);
	//num not support hex or alphanum!
	assert!(parse_float("ef").is_err());
}

#[test]
fn test_melody_struct() {
	// melody initialize
	let mut melody = Melody::default();

	// Float value check
	melody.apply(Channel::Length, "0.5");
	assert_eq!(*melody.length(), 0.5);

	// Alphanum array value check
	melody.apply(Channel::ExBPM, "10Aㅋ0A");
	assert_eq!(*melody.ex_bpm(), vec![36, 0, 10]);

	// Hex array value check
	melody.apply(Channel::BPM, "10Za0A");
	assert_eq!(*melody.bpm(), vec![16, 0, 10]);
}
