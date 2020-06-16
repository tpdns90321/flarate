use super::resource::Resource;

const TEST_RESOURCE: &'static str = "
#WAV01 HELLO.wav
#WAV02 HELLO.wav
#WAV02 HELLO2.wav
#BPM01 222.22
";

#[test]
fn test_resource_parse() {
	let resource = Resource::parse(TEST_RESOURCE);
	// generally read
	assert_eq!(resource.audio().get(&1).unwrap(), "HELLO.wav");
	// overwrite test
	assert_eq!(resource.audio().get(&2).unwrap(), "HELLO2.wav");
	// FREEDOM DIVE's BPM is 222.22
	assert_eq!(*resource.bpm().get(&1).unwrap(), 222.22);
}

