use core::fmt;

use crate::keywords::{KeyWord};

#[derive(Debug, Clone)]
pub enum Token{
    /// End-Of-File
    EOF,
    KeyWord(Word),

    /// unsigned numeric literal
    Number(String, bool),
}

impl fmt::Display for Token {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result{
        match self {
            Token::EOF => f.write_str("EOF"),
            Token::KeyWord(ref w) => write!(f, "{w}"),
            Token::Number(ref n, l ) => write!(f, "{}{long}", n, long = if *l{ "L" }else { "" }),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Word{
    pub value : String,

    pub quoto_style : Option<char>,

    pub keyword : KeyWord,
}

impl fmt::Display for Word{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.quoto_style{
            Some(s) if s == '"' || s == '[' || s == '`' => {
                write!(f, "{}{}{}", s, self.value, Word::matching_end_quote(s))
            }
            None => f.write_str(&self.value),
            _    => panic!("Unexpected quote_style!"),
        }
    }
}

impl Word{
    fn matching_end_quote(ch : char) -> char{
        match ch{
            '"' => '"',
            '[' => ']',
            '`' => '`',
            _   => panic!("Unexpected quoting style!"),
        }
    }
}


pub enum Whitespace{
    // a whitespace
    Space,

    NewLine,

    Tab,

    SingleLineComment{ comment : String, prefix : String},
    
    MultiLineComment(String), 
}

impl fmt::Display for Whitespace{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Whitespace::Space => f.write_str(" "),
            Whitespace::NewLine => f.write_str("\n"),
            Whitespace::Tab => f.write_str("\t"),
            Whitespace::SingleLineComment { comment, prefix } => write!(f, "{prefix}{comment}"),
            Whitespace::MultiLineComment(s) => write!(f, "/*{s}*/"),
        }
    }
}

pub struct Location{
    pub line : u64,

    pub column : u64,
}

/// SQL Tokenizer
pub struct Tokenizer<'a>{
    query : &'a str,
}

impl<'a > Tokenizer<'a>{
    
}