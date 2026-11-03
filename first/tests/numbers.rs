use first::numbers::is_even;
use first::numbers::is_a_valid_number;

#[test]
fn two_is_even() {
    assert!(is_even(2));
}

#[test]
fn array_is_even() {
    let evens = [0, 4, 16, 10, -4];
    
    for n in evens {
        assert!(is_even(n), "{n} should be even!");
    }
}

#[test]
fn array_is_not_even() {
    let odds = [1, 5, 21, -23];

    for n in odds {
        assert!(!is_even(n), "{n} shouldn't be even!");
    }
}

#[test]
fn string_is_a_valid_number() {
    assert!(is_a_valid_number("5.23"));
}

#[test]
fn strings_are_valid_numbers() {
    for s in ["5.12", "0", "-34", "1e4"] {
        assert!(is_a_valid_number(s), "{s} should be a valid number!");
    }
}

#[test]
fn strings_arent_valid_numbers() {
    for s in ["a", "1b", "0.4a", "3.3.2"] {
        assert!(!is_a_valid_number(s), "{s} shouldn't be a valid number!")
    }
}