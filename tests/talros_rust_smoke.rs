#[test]
fn recognises_the_last_fixture_sample() {
    let samples = [1, 2, 3];
    assert_eq!(samples.last(), Some(&2));
}
