use chrono::{TimeZone, Utc};
use std::str::FromStr;

use std::cmp::max;
use std::hash::{Hash, Hasher};

use fasthash::{murmur, FastHasher, MurmurHasher};

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

    pub fn recv(&mut self, msg: Self) -> Result<(), String> {
        let phys = Utc::now().timestamp_millis();

        let l_msg = msg.millis();
        let c_msg = msg.counter();

        if msg.node() == self.node() {
            return Err("Duplicate node error".into());
        }

        if l_msg - phys > 60000 {
            return Err("Clockdrift!".to_string());
        }

        let l_old = self.millis();
        let c_old = self.counter();

        let l_new = std::cmp::max(std::cmp::max(l_old, phys), l_msg);
        let c_new = if l_new == l_old && l_new == l_msg {
            std::cmp::max(c_old, c_msg) + 1
        } else if l_new == l_old {
            c_old + 1
        } else if l_new == l_msg {
            c_msg + 1
        } else {
            0
        };

        if l_new - phys > 60000 {
            return Err("Clockdrift!".to_string());
        }
        if c_new > 65535 {
            return Err("Overflow!".to_string());
        }

        self.set_millis(l_new);
        self.set_counter(c_new);

        Ok(())
    }

    pub fn millis(&self) -> i64 {
        self.millis
    }

    pub fn counter(&self) -> i64 {
        self.counter
    }

    pub fn node(&self) -> i64 {
        self.node
    }

    pub fn hash(&self) -> u32 {
        murmur::hash32(self.to_string())
    }

    pub fn set_millis(&mut self, millis: i64) {
        self.millis = millis;
    }

    pub fn set_counter(&mut self, counter: i64) {
        self.counter = counter;
    }

    pub fn set_node(&mut self, node: i64) {
        self.node = node;
    }
}

impl FromStr for Timestamp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('-').collect::<Vec<_>>();

        if parts.len() == 5 {}
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
