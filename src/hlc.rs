use chrono::{TimeZone, Utc};

struct Hlc {
    millis: i64,
    counter: i16,
    node: i16,
}

impl Hlc {
    pub fn new(millis: i64, counter: i16, node: i16) -> Self {
        Self {
            millis,
            counter,
            node,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}-{:04}-{:016}",
            Utc.timestamp_millis(self.millis).to_rfc3339(),
            self.counter,
            self.node
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let ts = Hlc::new(0, 0, 0);
        let got = ts.to_string();
        let want = "1970-01-01T00:00:00+00:00-0000-0000000000000000";
        assert_eq!(got, want);
    }
}
