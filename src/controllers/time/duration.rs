use std::time::Duration;

pub trait Seconds {
    fn as_seconds(&self) -> f32;
}

impl Seconds for Duration {
    fn as_seconds(&self) -> f32 {
        self.as_secs() as f32 + self.subsec_nanos() as f32 * 1e-9
    }
}

#[test]
fn test_duration_as_seconds() {
    let d = Duration::from_millis(1500);
    assert_eq!(1.5, d.as_seconds());
}
