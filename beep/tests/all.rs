extern crate beep;

#[test]
fn create() {
    let beeper = beep::Beeper::new();
    beeper.beep().finalize();
}