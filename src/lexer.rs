use std::str::Lines;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Lparen,
    Rparen,
    Lbracket,
    Rbracket,
    Eq,
    Ampersand,
    Star,
    Comma,
    Semicolon,
    Fn,
    Let,
    Mut,
    Box,
    AssertEq,
    Int(i32),
    Var(String),
}

const LEXEMES: [(&str, Token); 14] = [
    ("(", Token::Lparen),
    (")", Token::Rparen),
    ("{", Token::Lbracket),
    ("}", Token::Rbracket),
    ("=", Token::Eq),
    ("&", Token::Ampersand),
    ("*", Token::Star),
    (",", Token::Comma),
    (";", Token::Semicolon),
    ("fn", Token::Fn),
    ("let", Token::Let),
    ("mut", Token::Mut),
    ("Box::new", Token::Box),
    ("assert_eq!", Token::AssertEq),
    ];

#[derive(Debug)]
pub enum Error {
    Unknown(usize, usize),
}

type LexResult = Result<Token, Error>;

pub struct Lexer<'a> {
    contents: Lines<'a>,
    curr_line_num: usize,
    curr_col_num: usize,
    pub curr_line: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lines = input.lines();
        let first_line = lines.next().unwrap_or("");
        Lexer {
            contents: lines,
            curr_line_num: 1,
            curr_col_num: 0,
            curr_line: first_line,
        }
    }

    fn unknown(&self) -> Error {
        Error::Unknown(self.curr_line_num, self.curr_col_num)
    }

    fn consume(&mut self, n: usize) {
        self.curr_col_num += n;
        self.curr_line = &self.curr_line[n..];
        while self.curr_line.trim().is_empty() {
            if let Some(next_line) = self.contents.next() {
                self.curr_line_num += 1;
                self.curr_col_num = 0;
                self.curr_line = next_line;
            } else {
                break;
            }
        }
        self.curr_line = self.curr_line.trim_start();
    }

    fn symbol_or_keyword(&mut self) -> Option<LexResult> {
        for (lexeme, token) in LEXEMES.iter() {
            if self.curr_line.starts_with(lexeme) {
                self.consume(lexeme.len());
                return Some(Ok(token.clone()));
            }
        }
        None
    }

    fn variable(&mut self) -> Option<LexResult> {
        let chars: Vec<char> = self.curr_line.chars().collect();
        if chars.is_empty() || !chars[0].is_ascii_alphabetic() {
            return None;
        }
        let mut end = 1;
        while end < chars.len() && chars[end].is_ascii_alphanumeric() {
            end += 1;
        }
        let ident = &self.curr_line[..end];
        self.consume(end);
        Some(Ok(Token::Var(ident.to_string())))
    }

    fn int(&mut self) -> Option<LexResult> {
        let chars: Vec<char> = self.curr_line.chars().collect();
        let mut end = 0;
        while end < chars.len() && chars[end].is_ascii_digit() {
            end += 1;
        }
        if end == 0 {
            return None;
        }
        let number: i32 = self.curr_line[..end].parse().unwrap();
        self.consume(end);
        Some(Ok(Token::Int(number)))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexResult;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr_line = self.curr_line.trim_start();
        if self.curr_line.is_empty() {
            if let Some(next_line) = self.contents.next() {
                self.curr_line_num += 1;
                self.curr_col_num = 0;
                self.curr_line = next_line;
                return self.next();
            } else {
                return None;
            }
        }

        if let Some(tok) = self.symbol_or_keyword() {
            return Some(tok);
        }

        if let Some(tok) = self.int() {
            return Some(tok);
        }

        if let Some(tok) = self.variable() {
            return Some(tok);
        }

        Some(Err(self.unknown()))
    }
}
