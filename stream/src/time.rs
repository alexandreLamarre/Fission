#![warn(dead_code)] //FIXME: remove after initial development
use std::ops::Add;
use std::ops::Sub;
use std::time::{SystemTime, UNIX_EPOCH};

/// This is a simple class that represents an absolute instant of time.
/// Internally, it represents time as the difference, measured in milliseconds, between the current
/// time and midnight, January 1, 1970 UTC.
pub struct Time {
    val: u64,
}

impl Time {
    fn new(val: u64) -> Time {
        Time { val: val }
    }
    fn is_multiple_of(&self, other: u64) -> bool {
        self.val % other == 0
    }

    fn min(&self, other: &Time) -> Time {
        Time {
            val: self.val.min(other.val),
        }
    }

    fn max(&self, other: &Time) -> Time {
        Time {
            val: self.val.max(other.val),
        }
    }

    fn until(&self, other: &Time) -> Time {
        Time {
            val: other.val - self.val,
        }
    }
}

/// `==` operator for `Time`
impl PartialEq for Time {
    fn eq(&self, other: &Time) -> bool {
        self.val == other.val
    }
}

/// Comparison operators `<`, `>`, `<=`, >=` for `Time`
impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        Time {
            val: self.val + other.val,
        }
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
        Time {
            val: self.val - other.val,
        }
    }
}

impl ToString for Time {
    fn to_string(&self) -> String {
        format!("{}", self.val)
    }
}

/// Acts as an object for representing a time interval.
pub struct Interval {
    start: Time,
    end: Time,
}

impl Interval {
    // Create a new interval from the current time in milliseconds
    // measured from the UNIX_EPOCH
    fn new(duration: Time) -> Interval {
        let cur_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let cur_time = Time::new(cur_time.as_millis() as u64);
        Interval {
            start: cur_time,
            end: cur_time + duration,
        }
    }
}

impl Add for Interval {
    type Output = Interval;

    fn add(self, other: Interval) -> Interval {
        Interval {
            start: self.start + other.start,
            end: self.end + other.end,
        }
    }
}

impl Sub for Interval {
    type Output = Interval;

    fn sub(self, other: Interval) -> Interval {
        Interval {
            start: self.start - other.start,
            end: self.end - other.end,
        }
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Interval) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl ToString for Interval {
    fn to_string(&self) -> String {
        format!("{}-{}", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! time_partial_eq {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (this, that) = $value;
                    assert!(this == that);
                }
            )*
        }
    }

    macro_rules! time_partial_neq {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (this, that) = $value;
                    assert!(!(this == that));
                }
            )*
        }
    }

    macro_rules! time_partial_comp {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (this, that, operator, expected) = $value;
                    match operator{
                        std::cmp::Ordering::Less => assert!((this < that) == expected),
                        std::cmp::Ordering::Greater => assert!((this > that) == expected),
                        std::cmp::Ordering::Equal => assert!((this == that) == expected),
                    }
                }
            )*
        }
    }

    macro_rules! time_is_multiple {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (this, that, expected) = $value;
                    assert!(this.is_multiple_of(that) == expected);
                }
            )*
        }
    }

    macro_rules! time_min {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (this, that, expected) = $value;
                    assert!(this.min(&that) == expected);
                }
            )*
        }
    }

    macro_rules! time_max {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (this, that, expected) = $value;
                    assert!(this.max(&that) == expected);
                }
            )*
        }
    }

    macro_rules! time_until {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (this, that, expected) = $value;
                    assert!(this.until(&that) == expected);
                }
            )*
        }
    }

    #[test]
    fn test_constructor_Time() {
        assert_eq!(Time { val: 0 }.val, 0);
        assert_eq!(Time::new(0).val, 0);
    }

    #[test]
    fn test_constructor_Interval {
        assert_eq!(Interval { start: 0, end: 0 }.start, 0);
        let cur_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let offset = Time::new(1000000);
        let curInterval = Interval::new(offset);
        assert!(cur_time as u64 < curInterval.start.val);
        assert!(cur_time + (cur_time+offset).val);
    }

    time_partial_eq! {
        eq_zero : (Time::new(0), Time::new(0)),
        eq_non_zero : (Time::new(10000), Time::new(10000)),
        eq_overflow : (Time::new(u64::max_value()), Time::new(u64::max_value())),
    }

    time_partial_neq! {
        neq_zero : (Time::new(0), Time::new(4)),
        neq_non_zero : (Time::new(10000), Time::new(450000)),
        neq_overflow : (Time::new(u64::max_value()), Time::new(u64::max_value()-2)),
    }

    time_partial_comp! {
        comp_lt : (Time::new(0), Time::new(4), std::cmp::Ordering::Less, true),
        comp_nlt : (Time::new(4), Time::new(0), std::cmp::Ordering::Less, false),
        comp_gt : (Time::new(4), Time::new(0), std::cmp::Ordering::Greater, true),
        comp_ngt : (Time::new(0), Time::new(4), std::cmp::Ordering::Greater, false),
        comp_eq : (Time::new(4), Time::new(4), std::cmp::Ordering::Equal, true),
    }

    time_is_multiple! {
        is_multiple : (Time::new(4), 2, true),
        is_not_multiple : (Time::new(4), 3, false),
    }

    time_min! {
        min_lt : (Time::new(0), Time::new(4), Time::new(0)),
        min_gt : (Time::new(4), Time::new(0), Time::new(0)),
    }

    time_max! {
        max_lt : (Time::new(0), Time::new(4), Time::new(4)),
        max_gt : (Time::new(4), Time::new(0), Time::new(4)),
    }

    time_until! {
        until_lt : (Time::new(0), Time::new(4), Time::new(4)),
        until_gt : (Time::new(12), Time::new(200000), Time::new(199988)),
    }
}
