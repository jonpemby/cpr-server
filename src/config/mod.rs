use std::path::Path;
use std::collections::HashMap;

mod token;
mod file;
mod parser;

use self::file::File;
use self::parser::Parser;
use self::token::Token;

//

#[cfg(test)]
mod tests {
//    use super::parse;
//
//    #[test]
//    fn test_stuff() {
//        parse("./fixtures/PortConfig.yaml");
//
//        assert_eq!(true, false);
//    }
}