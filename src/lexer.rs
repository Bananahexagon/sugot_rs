pub fn main(code: String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut current_token = String::new();
    let mut i: usize = 0;
    let code_len = code.chars().count();
    while i < code_len {
        let c = code.chars().nth(i).unwrap();
        if let Some(lv) = match_string_open(&code[i..]) {
            if !current_token.is_empty() {
                tokens.push(current_token);
            }
            current_token = "\"".to_string();
            i += (lv + 1) as usize;
            let mut q_count = 0;
            for c in (code[i..]).chars() {
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
                i += 1;
                if lv < q_count {
                    break;
                }
            }
            current_token += "\"";
            tokens.push(current_token);
            i -= 1;
            current_token = String::new();
        } else if c == '/' && code.chars().nth(i + 1).unwrap_or('_') == '/' {
            while i < code_len && code.chars().nth(i).unwrap() != '\n' {
                i += 1;
            }
        } else if matches!(c, ' ' | '\n' | '\t' | '\r') {
            if !current_token.is_empty() {
                tokens.push(current_token);
                current_token = String::new();
            }
        } else if match_long(
            &code[i..],
            &[
                "&&", "+=", "-=", "||", ">", "<", ">=", "<=", "==", "!=", "->", "(", ")", ";", ":",
                ",", "=", "+", "-", "*", "/", "%","[","]","{","}",
            ],
        )
        .0
        {
            if !current_token.is_empty() {
                tokens.push(current_token);
                current_token = String::new();
            }
            let matched = match_long(
                &code[i..],
                &[
                    "&&", "+=", "-=", "||", ">", "<", ">=", "<=", "==", "!=", "->", "(", ")", ";",
                    ":", ",", "=", "+", "-", "*", "/", "%","[","]","{","}",
                ],
            );
            let matched_str = matched.1.unwrap();
            i += &matched_str.len() - 1;

            tokens.push(matched_str);
        } else {
            current_token.push(c);
        }
        i += 1;
    }
    return tokens;
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
