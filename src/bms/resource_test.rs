use super::resource::Resource;

const TEST_RESOURCE: &'static str = "
#WAV01 HELLO.wav
#OGG02 HELLO.ogg
#WAV02 HELLO2.wav
#BPM01 222.22
";

#[test]
fn test_resource_parse() {
	let resource = Resource::parse(TEST_RESOURCE);
	assert_eq!(resource.audio().get(&1).unwrap(), "HELLO.wav");
	assert_eq!(resource.audio().get(&2).unwrap(), "HELLO2.wav");
	assert_eq!(*resource.bpm().get(&1).unwrap(), 222.22);
}

