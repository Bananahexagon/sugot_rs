
#[derive(Debug, Clone)]
pub struct Location {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub val: String,
    pub location: Location,
}