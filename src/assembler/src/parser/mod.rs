pub struct Parser {
    source: String,
    current: u8,
}

impl Parser {
    pub fn new(source: String) -> Parser {
        let current = 0;

        Parser { source, current }
    }
}
