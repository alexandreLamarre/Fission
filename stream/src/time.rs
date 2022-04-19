#![warn(dead_code)] //FIXME: remove after initial development
use std::ops::Add;
use std::ops::Sub;

/// This is a simple class that represents an absolute instant of time.
/// Internally, it represents time as the difference, measured in milliseconds, between the current
/// time and midnight, January 1, 1970 UTC.
struct Time {
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
    fn test_constructor() {
        assert_eq!(Time { val: 0 }.val, 0);
        assert_eq!(Time::new(0).val, 0);
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
