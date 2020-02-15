pub trait DurationSinceEpoch {
    fn as_secs(&self) -> u64;

    fn as_millis(&self) -> u64;
}
