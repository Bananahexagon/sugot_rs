use crate::types::general::{Location, Token};

pub fn main(code: String) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut start_location = (0, 0);
    let mut end_location = (0, 0);
    let mut current_ptr = 0;
    let mut current_token = String::new();
    let code_len = code.chars().count();
    macro_rules! push {
        () => {
            result.push(Token {
                val: current_token,
                location: Location {
                    start_line: start_location.0,
                    start_column: start_location.1,
                    end_line: end_location.0,
                    end_column: end_location.1,
                },
            });
            #[allow(unused_assignments)]
            {
                current_token = String::new();
            }
            start_location = end_location;
        };
    }
    while current_ptr < code_len {
        let c = code.chars().nth(current_ptr).unwrap();
        if c == '\n' {
            end_location.0 += 1;
            end_location.1 = 0;
        } else {
            end_location.1 += 1;
        }
        if let Some(lv) = match_string_open(&code[current_ptr..]) {
            if !current_token.is_empty() {
                push!();
            }
            current_token = "\"".to_string();
            current_ptr += (lv + 1) as usize;
            end_location.1 += (lv + 1) as usize;
            let mut q_count = 0;
            for c in (code[current_ptr..]).chars() {
                if c == '\n' {
                    end_location.0 += 1;
                    end_location.1 = 0;
                } else {
                    end_location.1 += 1;
                }
                if q_count == 0 && c == '"' {
                    q_count = 1;
                } else if 0 < q_count && c == '#' {
                    q_count += 1;
                } else {
                    if 0 < q_count {
                        current_token.push('"')
                    };
                    let mut tmp = 0;
                    while tmp + 1 < q_count {
                        current_token.push('#');
                        tmp += 1;
                    }
                    if c == '"' {
                        q_count = 1;
                    } else {
                        current_token.push(c);
                        q_count = 0;
                    }
                }
                current_ptr += 1;
                if lv < q_count {
                    break;
                }
            }
            current_token += "\"";
            push!();
            current_ptr -= 1;
        }
        if matches!(c, ' ' | '\n' | '\t' | '\r') {
            if !current_token.is_empty() {
                push!();
            }
        } else if c == '/' && code.chars().nth(current_ptr + 1).unwrap_or('_') == '/' {
            while current_ptr < code_len && code.chars().nth(current_ptr).unwrap() != '\n' {
                current_ptr += 1;
            }
        } else if match_long(&code[current_ptr..], &["(", ")", "{", "}"]).0 {
            if !current_token.is_empty() {
                push!();
            }
            let matched = match_long(&code[current_ptr..], &["(", ")", "{", "}"]);
            current_ptr += matched.1.as_ref().unwrap().len() - 1;
            current_token = matched.1.unwrap();
            push!();
        } else {
            current_token.push(c);
        }

        current_ptr += 1;
    }
    return result;
}

fn match_long(left: &str, rights: &[&str]) -> (bool, Option<String>) {
    for right in rights {
        let len = right.len();
        if left.len() < len {
            continue;
        }
        let r = &right[0..len];
        let l = &left[0..len];
        if r == l {
            return (true, Some(right.to_string()));
        }
    }
    (false, None)
}

fn match_string_open(left: &str) -> Option<u32> {
    let mut result = 0;
    for c in left.chars() {
        if c == '#' {
            result += 1;
        } else if c == '"' {
            return Some(result);
        } else {
            return None;
        }
    }
    return None;
}
