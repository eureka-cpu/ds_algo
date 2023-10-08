//! Given a string s containing just the characters '(', ')', '{', '}', '[' and ']',
//! determine if the input string is valid.
//!
//! An input string is valid if:
//! - Open brackets are closed by the same type of brackets.
//! - Open brackets are closed in the correct order.

fn all_delims_match(delims: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for delim in delims.chars() {
        match delim {
            '(' | '[' | '{' => stack.push(delim),
            _ => {
                if let Some(open_delim) = stack.pop() {
                    if !delims_match(open_delim, delim) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}

fn delims_match(open_delim: char, delim: char) -> bool {
    match open_delim {
        '(' => delim == ')',
        '[' => delim == ']',
        '{' => delim == '}',
        _ => false,
    }
}

#[test]
fn test_delims_match() {
    assert!(all_delims_match("()"));
    assert!(all_delims_match("()[]{}"));
    assert!(all_delims_match("{[()]}"));
}

#[test]
#[should_panic]
fn test_delims_do_not_match() {
    assert!(all_delims_match("([)}"));
}
