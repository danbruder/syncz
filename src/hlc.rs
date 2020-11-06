use chrono::{TimeZone, Utc};
use std::cmp::max;

struct Timestamp {
    millis: i64,
    counter: i64,
    node: i64,
}

impl Timestamp {
    pub fn new(millis: i64, counter: i64, node: i64) -> Self {
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

    pub fn send(&mut self) -> Result<(), String> {
        let phys = Utc::now().timestamp_millis();

        let l_old = self.millis;
        let c_old = self.counter;

        let l_new = max(l_old, phys);
        let c_new = if l_old == l_new { c_old + 1 } else { 0 };

        if l_new - phys > 60000 {
            return Err("Clockdrift!".to_string());
        }

        if c_new > 65535 {
            return Err("overflow".to_string());
        }

        self.millis = l_new;
        self.counter = c_new;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let ts = Timestamp::new(0, 0, 0);
        let got = ts.to_string();
        let want = "1970-01-01T00:00:00+00:00-0000-0000000000000000";
        assert_eq!(got, want);
    }
}
