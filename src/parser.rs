peg::parser! {
    pub grammar parser() for str {
        pub rule calc() -> f64
            = n:number() { n }

        pub rule number() -> f64
            = n:$(['0'..='9']+) {?
                n.parse().or(Err("Can't parse a number"))
            }
    }
}
