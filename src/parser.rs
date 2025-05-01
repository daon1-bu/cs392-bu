use std::iter::Peekable;
use crate::lexer::{Lexer, Token};
use crate::utils::*;

#[derive(Debug)]
pub enum Error {
    EndOfFile,
    Lexer(crate::lexer::Error),
    Unexpected(Token),
}

type ParseResult<T> = Result<T, Error>;

pub struct Parser<'a> {
    pub lexer: Peekable<Lexer<'a>>,
    pub fresh: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(input).peekable(),
            fresh: 1,
        }
    }

    fn next_token(&mut self) -> ParseResult<Token> {
        match self.lexer.next() {
            Some(Ok(tok)) => Ok(tok),
            Some(Err(err)) => Err(Error::Lexer(err)),
            None => Err(Error::EndOfFile),
        }
    }

    fn next_token_match(&mut self, expected: Token) -> ParseResult<Token> {
        let tok = self.next_token()?;
        if tok == expected {
            Ok(tok)
        } else {
            Err(Error::Unexpected(tok))
        }
    }

    fn peek_token(&mut self) -> ParseResult<&Token> {
        match self.lexer.peek() {
            Some(Ok(tok)) => Ok(tok),
            Some(Err(_)) | None => Err(Error::EndOfFile),
        }
    }

    fn parse_lval(&mut self) -> ParseResult<Lval> {
        let mut derefs = 0;
        while matches!(self.peek_token()?, Token::Star) {
            self.next_token()?;
            derefs += 1;
        }
        let name = match self.next_token()? {
            Token::Var(s) => s,
            t => return Err(Error::Unexpected(t)),
        };
        Ok(Lval { ident: name, derefs })
    }

    fn parse_expr(&mut self) -> ParseResult<Expr> {
        match self.peek_token()? {
            Token::Int(_) => match self.next_token()? {
                Token::Int(n) => Ok(Expr::Int(n)),
                _ => unreachable!(),
            },
            Token::Var(_) | Token::Star => {
                let lval = self.parse_lval()?;
                Ok(Expr::Lval(lval, false))
            }
            Token::Box => {
                self.next_token()?;
                self.next_token_match(Token::Lparen)?;
                let inner = self.parse_expr()?;
                self.next_token_match(Token::Rparen)?;
                Ok(Expr::Box(Box::new(inner)))
            }
            Token::Ampersand => {
                self.next_token()?;
                let is_mut = matches!(self.peek_token()?, Token::Mut);
                if is_mut {
                    self.next_token()?;
                }
                let lval = self.parse_lval()?;
                Ok(Expr::Borrow(lval, is_mut))
            }
            Token::Lbracket => self.parse_block(),
            Token::AssertEq => {
                self.next_token()?;
                self.next_token_match(Token::Lparen)?;
                let left = self.parse_expr()?;
                self.next_token_match(Token::Comma)?;
                let right = self.parse_expr()?;
                self.next_token_match(Token::Rparen)?;
                Ok(Expr::AssertEq(Box::new(left), Box::new(right)))
            }
            t => Err(Error::Unexpected(t.clone())),
        }
    }

    fn parse_stmt(&mut self) -> ParseResult<Stmt> {
        match self.peek_token()? {
            Token::Let => {
                self.next_token()?;
                self.next_token_match(Token::Mut)?;
                let ident = match self.next_token()? {
                    Token::Var(s) => s,
                    t => return Err(Error::Unexpected(t)),
                };
                self.next_token_match(Token::Eq)?;
                let e = self.parse_expr()?;
                Ok(Stmt::LetMut(ident, e))
            }
            Token::Star | Token::Var(_) => {
                let lval = self.parse_lval()?;
                self.next_token_match(Token::Eq)?;
                let e = self.parse_expr()?;
                Ok(Stmt::Assign(lval, e))
            }
            _ => Ok(Stmt::Expr(self.parse_expr()?)),
        }
    }

    fn parse_block(&mut self) -> ParseResult<Expr> {
        self.next_token_match(Token::Lbracket)?;
        let l = Lifetime(self.fresh);
        self.fresh += 1;
        let mut stmts = vec![];
        while !matches!(self.peek_token()?, Token::Rbracket) {
            let stmt = self.parse_stmt()?;
            self.next_token_match(Token::Semicolon)?;
            stmts.push(stmt);
        }
        self.next_token_match(Token::Rbracket)?;
        Ok(Expr::Block(stmts, Box::new(Expr::Unit), l))
    }

    pub fn parse(&mut self) -> ParseResult<Expr> {
        self.next_token_match(Token::Fn)?;
        match self.next_token()? {
            Token::Var(s) if s == "main" => {
                self.next_token_match(Token::Lparen)?;
                self.next_token_match(Token::Rparen)?;
                self.parse_block()
            }
            t => Err(Error::Unexpected(t)),
        }
    }
}
