use dfa2reg::scanner::{validate_alphabet, Scanner};

#[test]
fn first_case() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn alphabet_has_valid_chars() {
    let org_alphabet = ["a", "b", "c", "d", "!"].map(String::from).to_vec();

    assert!(
        validate_alphabet(&org_alphabet),
        "Alphabet does not contain valid characters"
    );
}

#[test]
fn alphabet_contains_numbers() {
    let org_alphabet = ["1", "b", "c", "d", "!"].map(String::from).to_vec();

    assert_eq!(
        true,
        validate_alphabet(&org_alphabet),
        "Alphabet does not contain valid characters"
    );
}

#[test]
fn alphabet_has_invalid_chars() {
    let org_alphabet = ["#", "b", "c", "d", "!"].map(String::from).to_vec();

    assert_eq!(
        false,
        validate_alphabet(&org_alphabet),
        "Alphabet does not contain valid characters"
    );
}

#[test]
fn alphabet_has_symbol_greater_than_one() {
    let org_alphabet = ["a", "b", "ca", "d", "!"].map(String::from).to_vec();

    assert_eq!(
        false,
        validate_alphabet(&org_alphabet),
        "Alphabet does not contain valid characters"
    );
}

#[test]
fn gnfa_algo_1() {
    let valid_regex =
        String::from("(a U (! U (b U c)(a U b)*a)a)*((b U c)(a U b)* U (! U (b U c)(a U b)*a))");
    let scanner = Scanner::new();
    match scanner.run_file("src/inputs/ex_input1.txt".to_string()) {
        Ok(regex) => assert_eq!(regex, valid_regex),
        Err(msg) => {
            eprintln!("{}", msg);
        }
    }
}

#[test]
fn gnfa_algo_2() {
    let valid_regex = String::from("aa*b");
    let scanner = Scanner::new();
    match scanner.run_file("src/inputs/ex_input2.txt".to_string()) {
        Ok(regex) => assert_eq!(regex, valid_regex),
        Err(msg) => {
            eprintln!("{}", msg);
        }
    }
}
