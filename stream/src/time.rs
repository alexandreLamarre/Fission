#![warn(dead_code)] //FIXME: remove after initial development
use std::ops::Add;
use std::ops::Sub;
use std::time::{SystemTime, UNIX_EPOCH};

/// This is a simple class that represents an absolute instant of time.
/// Internally, it represents time as the difference, measured in milliseconds, between the current
/// time and midnight, January 1, 1970 UTC.
#[derive(Debug, Clone, Copy)]
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

impl Add<Time> for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        Time {
            val: self.val + other.val,
        }
    }
}

impl Add<u64> for Time {
    type Output = Time;

    fn add(self, other: u64) -> Time {
        Time {
            val: self.val + other,
        }
    }
}

impl Sub<Time> for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
        Time {
            val: self.val - other.val,
        }
    }
}

impl Sub<u64> for Time {
    type Output = Time;

    fn sub(self, other: u64) -> Time {
        Time {
            val: self.val - other,
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
    fn new(duration: u64) -> Interval {
        let cur_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let cur_time = Time::new(cur_time.as_millis() as u64);
        Interval {
            start: cur_time.clone(),
            end: cur_time + Time::new(duration),
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

impl PartialOrd for Interval {
    // FIXME: not happy with this implementation
    fn partial_cmp(&self, other: &Interval) -> Option<std::cmp::Ordering> {
        (self.end - self.start).partial_cmp(&(other.end - other.start))
    }
}

impl ToString for Interval {
    fn to_string(&self) -> String {
        format!("{:}-{:}", self.start.to_string(), self.end.to_string()) // Note:might want to convert ms since UNIX EPOCH to human readable time
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

    macro_rules! time_add {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (this, that, expected) = $value;
                    assert!(this + that == expected);
                }
            )*
        };
    }

    macro_rules! time_sub {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (this, that, expected) = $value;
                    assert!(this - that == expected);
                }
            )*
        };
    }

    macro_rules! time_to_string {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (this, expected) = $value;
                    assert!(this.to_string() == expected);
                }
            )*
        };
    }

    // FIXME: this should try and test intervals using ::new()
    macro_rules! interval_add {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name(){
                    let (this, that, expected) = $value;
                    assert!(this + that == expected);
                }
            )*
        };
    }

    //FIXME: this should try and test intervals using ::new()
    macro_rules! interval_sub {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name(){
                    let (this, that, expected) = $value;
                    assert!(this - that == expected);
                }
            )*
        };
    }

    #[test]
    fn test_constructor_time() {
        assert_eq!(Time { val: 0 }.val, 0);
        assert_eq!(Time::new(0).val, 0);
    }

    #[test]
    fn test_constructor_interval() {
        assert_eq!(
            Interval {
                start: Time::new(0),
                end: Time::new(0)
            }
            .start,
            Time::new(0)
        );
        let cur_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let offset = 1000000;
        let cur_interval = Interval::new(offset);
        println!("{:?}", cur_time.as_millis() as u64);
        println!("{:?}", cur_interval.start);
        assert!((cur_time.as_millis() as u64) <= cur_interval.start.val);
        assert!(cur_interval.end >= Time::new(cur_time.as_millis() as u64) + offset);
    }

    time_to_string! {
        test_to_string_time_zero: (Time::new(0), "0"),
        test_to_string_time_one: (Time::new(1), "1"),
        test_to_string_time_max: (Time::new(std::u64::MAX), "18446744073709551615"),
    }

    time_to_string! {
        it_to_string_interval : (Interval{start: Time::new(0), end: Time::new(4)}, "0-4"),
    }

    time_partial_eq! {
        eq_zero : (Time::new(0), Time::new(0)),
        eq_non_zero : (Time::new(10000), Time::new(10000)),
        eq_overflow : (Time::new(u64::max_value()), Time::new(u64::max_value())),
    }

    time_partial_eq! {
        it_eq_zero : (Interval::new(0), Interval::new(0)),
        it_eq_non_zero : (Interval::new(10000), Interval::new(10000)),
    }

    time_partial_neq! {
        neq_zero : (Time::new(0), Time::new(4)),
        neq_non_zero : (Time::new(10000), Time::new(450000)),
        neq_overflow : (Time::new(u64::max_value()), Time::new(u64::max_value()-2)),
    }

    time_partial_neq! {
        it_neq_zero : (Interval::new(0), Interval::new(4)),
        it_neq_non_zero : (Interval::new(10000), Interval::new(450000)),
    }

    time_partial_comp! {
        comp_lt : (Time::new(0), Time::new(4), std::cmp::Ordering::Less, true),
        comp_nlt : (Time::new(4), Time::new(0), std::cmp::Ordering::Less, false),
        comp_gt : (Time::new(4), Time::new(0), std::cmp::Ordering::Greater, true),
        comp_ngt : (Time::new(0), Time::new(4), std::cmp::Ordering::Greater, false),
        comp_eq : (Time::new(4), Time::new(4), std::cmp::Ordering::Equal, true),
    }

    time_partial_comp! {
        it_comp_lt : (Interval::new(0), Interval::new(4), std::cmp::Ordering::Less, true),
        it_comp_nlt : (Interval::new(4), Interval::new(0), std::cmp::Ordering::Less, false),
        it_comp_gt : (Interval::new(4), Interval::new(0), std::cmp::Ordering::Greater, true),
        it_comp_ngt : (Interval::new(0), Interval::new(4), std::cmp::Ordering::Greater, false),
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

    time_add! {
        add_int : (Time::new(4), 2, Time::new(6)),
        add_time : (Time::new(4), Time::new(2), Time::new(6)),
    }

    time_sub! {
        sub_int : (Time::new(4), 2, Time::new(2)),
        sub_time : (Time::new(4), Time::new(2), Time::new(2)),
    }

    interval_add! {
        it_add : (Interval{start : Time::new(0), end : Time::new(4)}, Interval{start : Time::new(0), end : Time::new(2)}, Interval{start: Time::new(0), end : Time::new(6)}),
    }

    interval_sub! {
        it_sub : (Interval{start : Time::new(0), end : Time::new(4)}, Interval{start : Time::new(0), end : Time::new(2)}, Interval{start: Time::new(0), end : Time::new(2)}),
    }
}
