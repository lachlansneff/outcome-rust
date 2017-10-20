extern crate outcome;
use outcome::*;

#[test]
fn is_success_failure() {
    assert!(Success.is_success());
    assert!(Failure.is_failure());
}

#[test]
fn wrap_in_option() {
    assert_eq!(Success.or_none("test"), Some("test"));
    assert_eq!(Failure.or_none("test"), None);
}

#[test]
fn map_to_closure() {
    assert_eq!(Success.and_then(|| Success), Success);
    assert_eq!(Failure.or_then(|| Success), Success);
}

#[test]
fn binary_ops() {
    assert_eq!(Success.and(Success), Success);
    assert_eq!(Success.and(Failure), Failure);

    assert_eq!(Success.or(Failure), Success);
    assert_eq!(Failure.or(Failure), Failure);
}

#[test]
#[should_panic]
fn test_or_panic() {
    assert_eq!(Failure.or_panic(42), 42);
}