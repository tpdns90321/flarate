use super::chart::{Chart, ChartType};

const TEST_CHART: &'static str = "
#00003:DE
#00002:0.5
#0A011:00zz0002
#ZZZ11:00100000";

#[test]
fn test_chart_hex_struct() {
    let chart = Chart::parse(ChartType::Hex, TEST_CHART).unwrap();

    assert_eq!(*chart.melodies().get(&0).unwrap().length(), 0.5);
    // FREEDOM DIVE BPM: 222
    assert_eq!(*chart.melodies().get(&0).unwrap().bpm(), vec![222]);
    // Object Channel Chart
    assert_eq!(
        *chart
            .melodies()
            .get(&160)
            .unwrap()
            .first_player_visible()
            .get(&1)
            .unwrap(),
        vec![0, 1295, 0, 02]
    );
    // out of range test
    // alphanum maximum range 46655
    assert!(chart.melodies().get(&46655).is_none());
}

#[test]
fn test_chart_alphanum_struct() {
    let chart = Chart::parse(ChartType::Alphanum, TEST_CHART).unwrap();

    assert_eq!(*chart.melodies().get(&0).unwrap().length(), 0.5);
    // FREEDOM DIVE BPM: 222
    assert_eq!(*chart.melodies().get(&0).unwrap().bpm(), vec![222]);
    // Object Channel Chart
    assert_eq!(
        *chart
            .melodies()
            .get(&360)
            .unwrap()
            .first_player_visible()
            .get(&1)
            .unwrap(),
        vec![0, 1295, 0, 02]
    );
    // alphanum range test
    // alphanum maximum range 46655
    assert!(chart.melodies().get(&46655).is_some());
}
