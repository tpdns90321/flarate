use super::melody::Melody;
use super::channel::Channel;

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
