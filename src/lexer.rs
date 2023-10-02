use crate::types::general::Location;

pub fn main(code: String) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut start_location = (0, 0);
    let mut end_location = (0, 0);
    let mut current_ptr = 0;
    let mut current_token = String::new();
    let code_len = code.chars().count();
    while current_ptr < code_len {
        let c = code.chars().nth(current_ptr).unwrap();
        if c == '\n' {
            end_location.0 += 1;
            end_location.1 = 0;
        } else {
            end_location.1 += 1;
        }
        if c != '\n' && c != ' ' {
            current_token.push(c);
        } else if !current_token.is_empty() {
            result.push(Token {
                val: current_token,
                location: Location {
                    start_line: start_location.0,
                    start_column: start_location.1,
                    end_line: end_location.0,
                    end_column: end_location.1,
                },
            });
            current_token = String::new();
            start_location = end_location;
        }
        current_ptr += 1;
    }
    return result;
}
#[derive(Debug, Clone)]
pub struct Token {
    val: String,
    location: Location,
}
