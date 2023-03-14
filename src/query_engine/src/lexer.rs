use core::fmt;
use core::str::Chars;
use core::iter::Peekable;
use crate::keywords::{KeyWord, ALL_KEYWORDS, ALL_KEYWORDS_INDEX};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token{
    /// End-Of-File
    EOF,
    KeyWord(Word),

    /// unsigned numeric literal
    Number(String, bool),

    /// a character
    Char(char),

    /// White Space
    Whitespace(Whitespace),

    /// hexadecimal string literal
    HexStringLiteral(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result{
        match self {
            Token::EOF => f.write_str("EOF"),
            Token::KeyWord(ref w) => write!(f, "{w}"),
            Token::Number(ref n, l ) => write!(f, "{}{long}", n, long = if *l{ "L" }else { "" }),
            Token::Char(ref c) => write!(f, "{c}"),
            Token::Whitespace(ref ws) => write!(f, "{ws}"),
            Token::HexStringLiteral(ref hx) => write!(f, "{hx}"),
        }
    }
}

impl Token{
    pub fn make_keyword(keyword : &str) -> Self {
        Token::make_word(keyword, None)
    }

    pub fn make_word(word : &str, quote_style : Option<char>) -> Self {
        let word_uppercase = word.to_uppercase();
        Token::KeyWord(
            Word { value: word.to_string(),
                   quote_style, 
                   keyword : if quote_style.is_none(){
                    let keyword = ALL_KEYWORDS.binary_search(&word_uppercase.as_str());
                    keyword.map_or(KeyWord::NoKeyWord, |x| ALL_KEYWORDS_INDEX[x])
                   } else {
                        KeyWord::NoKeyWord
                   },
                })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word{
    pub value : String,

    pub quote_style : Option<char>,

    pub keyword : KeyWord,
}

impl fmt::Display for Word{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.quote_style{
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Location{

    /// Line of parse, starting from 1.
    pub line : u64,

    /// Line column, starting from 1.
    pub column : u64,
}

/// a token but it have location
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TokenWithLocation{
    pub token : Token,
    pub location : Location,
}

impl PartialEq<Token> for TokenWithLocation{
    fn eq(&self, other : &Token) -> bool {
        &self.token == other
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenizerError{
    pub message : String,
    pub line : u64,
    pub col  : u64,
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(
            f,
            "{} at Line : {}, Column {}",
            self.message, self.line, self.col
        )
    }

}

struct State<'a>{
    peekable : Peekable<Chars<'a>>,
    pub line : u64,
    pub col  : u64,
}

impl<'a> State<'a>{
    pub fn next(&mut self) -> Option<char>{
        match self.peekable.next() {
            None => None,
            Some(s) => {
                if s == '\n'{
                    self.line += 1;
                    self.col = 1;
                }else {
                    self.col += 1;
                }
                Some(s)
            }
        }
    }

    /// look next but not consume it.
    pub fn peek(&mut self) -> Option<&char>{
        self.peekable.peek()
    }

    /// return current parse location
    pub fn location(&self) -> Location{
        Location { line: self.line, column : self.col }
    }
}

/// SQL Tokenizer
pub struct Tokenizer<'a>{
    query : &'a str,
}

impl<'a> Tokenizer<'a>{

    /// Create a new SQL Tokenizer for the specified SQL Statement.
    pub fn new(query : &'a str) -> Self{
        Self { query }
    }

    /// Tokenize the statement and produce a vector of tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError>{
        // Calculate the required memory space in advance 
        let twl = self.tokenize_with_location()?;

        let mut tokens : Vec<Token> = vec![];
        tokens.reserve(twl.len());
        for token_with_loction in twl {
            tokens.push(token_with_loction.token);
        }
        Ok(tokens)
    }

    pub fn tokenize_with_location(&mut self) -> Result<Vec<TokenWithLocation>, TokenizerError>{
        let mut state = State{
            peekable : self.query.chars().peekable(),
            line : 1,
            col  : 1,
        };

        let mut tokens : Vec<TokenWithLocation> = vec![];

        let mut location = state.location();
        while let Some(token) = self.next_token(&mut state)?{
            tokens.push(TokenWithLocation{
                token,
                location : location.clone(),
            });

            location = state.location();
        }
        Ok(tokens)
    }

    /// Get Next Token or return None
    fn next_token(&self, chars : &mut State) -> Result<Option<Token>, TokenizerError>{
        match chars.peek() {
            Some(&ch) => match ch {
                ' ' => self.consume_and_return(chars, Token::Whitespace(Whitespace::Space)),         
                '0' ..='9' | '.' => {
                    let  s = peeking_take_while(chars, |ch| ch.is_ascii_digit());
                    
                    if s == "0" && chars.peek() == Some(&'x'){
                        chars.next();
                        let s2 = peeking_take_while(
                            chars, 
                            |ch| matches!(ch, '0' ..= '9' | 'A' ..= 'F' | 'a' ..= 'f'),);
                        return Ok(Some(Token::HexStringLiteral(s2)));
                    }
                    let long = if chars.peek() == Some(&'L'){
                        chars.next();
                        true
                    }else{
                        false
                    };
                    Ok(Some(Token::Number(s, long)))
                },
                other => self.consume_and_return(chars, Token::Char(other)),
            },
            None => Ok(None),
        }
    }


    /// this function will consume a char and return ok.
    #[allow(clippy::unnecessary_wraps)]
    fn consume_and_return(&self, chars : &mut State, t : Token) -> Result<Option<Token>, TokenizerError>{
        chars.next();
        Ok(Some(t))
    }

}

fn peeking_take_while(chars : &mut State, mut predicate : impl FnMut(char) -> bool) -> String{
    let mut s = String::new();
    while let Some(&ch) = chars.peek(){
        if predicate(ch){
            chars.next();
            s.push(ch);
        }else {
            break;
        }
    }
    s
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn tokeniz_select_1(){
        let sql = String::from("SELECT 1");
        let mut tokenizer = Tokenizer::new(&sql);
        let tokens = tokenizer.tokenize().unwrap();
        let expected = vec![
            Token::make_keyword("SELECT"),
            Token::Whitespace(Whitespace::Space),
            Token::Number(String::from("1"), false),
        ];
        compare(expected, tokens);
    }

    fn compare<T : PartialEq + std::fmt::Debug>(expected : Vec<T>, actual : Vec<T>){
        assert_eq!(expected, actual);
    }
}