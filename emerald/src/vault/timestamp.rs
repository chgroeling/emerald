pub struct Timestamp(pub i64);

impl Timestamp {
    pub fn get_raw_value(&self) -> i64 {
        self.0
    }
}
