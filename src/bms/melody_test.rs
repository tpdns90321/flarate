use super::melody::{parse_alphanum, parse_hex, parse_num};

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
fn test_melody_parse_num() {
	assert_eq!(parse_num("0.5").unwrap(), 0.5);
	//num not support hex or alphanum!
	assert!(parse_num("ef").is_err());
}
