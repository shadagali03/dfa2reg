use dfa2reg::scanner::validate_alphabet;

#[test]
fn first_case() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn alphabet_has_valid_chars() {
    let alphabet = vec!['a', 'b', 'c', 'd', '!'];
    let org_alphabet = vec!["a", "b", "c", "d", "!"];

    assert!(validate_alphabet(alphabet, org_alphabet), "Alphabet does not contain valid characters");
}

#[test]
fn alphabet_has_invalid_chars() {
    let alphabet = vec!['1', 'b', 'c', 'd', '!'];
    let org_alphabet = vec!["1", "b", "c", "d", "!"];

    assert_eq!(false, validate_alphabet(alphabet, org_alphabet), "Alphabet does not contain valid characters");
}

#[test]
fn alphabet_has_diff_length() {
    let alphabet = vec!['a', 'b', 'c', 'd', '!', 'a'];
    let org_alphabet = vec!["a", "b", "c", "d", "!"];

    assert_eq!(false, validate_alphabet(alphabet, org_alphabet), "Alphabet does not contain valid characters");
}
