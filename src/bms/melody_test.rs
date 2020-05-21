use super::melody::Melody;

#[test]
fn test_melody_parse_alphanumeric() {
	// normal alphanumeric
	assert_eq!(
		Melody::parse_alphanumeric("00ZZ09").unwrap(),
		Melody::Alphanumeric(vec![0,1295,9])
	);
	// some error in the here
	assert_eq!(
		Melody::parse_alphanumeric("00Aㅋ0z").unwrap(),
		Melody::Alphanumeric(vec![0,0,35])
	);
	// odd length error
	assert!(Melody::parse_alphanumeric("00A").is_err());
}

#[test]
fn test_melody_parse_hex() {
	// normal hex
	assert_eq!(
		Melody::parse_hex("00FF09").unwrap(),
		Melody::Hex(vec![0,255,9])
	);
	// some error in the here
	assert_eq!(
		Melody::parse_hex("00Aㅋ0a").unwrap(),
		Melody::Hex(vec![0,0,10])
	);
	// odd length error
	assert!(Melody::parse_hex("00A").is_err());
}

#[test]
fn test_melody_parse_number() {
	assert_eq!(Melody::parse_number("100").unwrap(), Melody::Number(100));
	//number not support hex or alphanumeric!
	assert!(Melody::parse_number("ef").is_err());
}
