use super::header::*;

#[test]
fn test_header_parse() {
	let plain = "#TITLE HELLO
#PLAYLEVEL WORLD
#BPM 222
#LEVEL 10
Wrong Value Test
#TOTAL 123헬";

	let header = Header::parse(plain);

	assert_eq!(header.title(), "HELLO"); 
	assert_eq!(header.playlevel(), "WORLD");
	// default value
	assert_eq!(*header.total(), 160);
	// changed value
	assert_eq!(*header.bpm(), 222.0);
	// option value
	assert_eq!(header.level().unwrap(), 10);
}
