use super::channel::{Channel, Event};
use super::melody::Melody;

#[test]
fn test_melody_struct() {
    // melody initialize
    let mut melody = Melody::default();

    // Float value check
    melody.apply(Channel::Event(Event::Length), "0.5");
    assert_eq!(*melody.length(), 0.5);

    // Alphanum array value check
    melody.apply(Channel::Event(Event::ExBPM), "10Aㅋ0A");
    assert_eq!(*melody.ex_bpm(), vec![36, 0, 10]);

    // Hex array value check
    melody.apply(Channel::Event(Event::BPM), "10Za0A");
    assert_eq!(*melody.bpm(), vec![16, 0, 10]);
}
