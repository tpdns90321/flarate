use std::str::FromStr;
use super::channel::Channel;

#[test]
fn test_channel_parse() {
	let event_channel = Channel::from_str("01");
	let object_channel = Channel::from_str("13");
	let unsupported_channel = Channel::from_str("82");
	let invalid_channel = Channel::from_str("1");

	assert_eq!(event_channel.ok().unwrap(), Channel::BGM);
	assert_eq!(
		object_channel.ok().unwrap(),
		Channel::FirstPlayerVisible(3));
	// error check
	assert!(unsupported_channel.is_err());
	assert!(invalid_channel.is_err());
}
