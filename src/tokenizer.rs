use std::fmt;
use std::iter::{Iterator, Peekable};

use super::Result;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier(String),
    String(String),
    Number(f64),
    Comment(String),

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::Minus => write!(f, "-"),
            Token::Plus => write!(f, "+"),
            Token::Semicolon => write!(f, ";"),
            Token::Slash => write!(f, "/"),
            Token::Star => write!(f, "*"),

            Token::Bang => write!(f, "!"),
            Token::BangEqual => write!(f, "!="),
            Token::Equal => write!(f, "="),
            Token::EqualEqual => write!(f, "=="),
            Token::Greater => write!(f, ">"),
            Token::GreaterEqual => write!(f, ">="),
            Token::Less => write!(f, "<"),
            Token::LessEqual => write!(f, "<="),

            Token::Identifier(ref s) | Token::String(ref s) => write!(f, "{}", s),
            Token::Number(n) => write!(f, "{}", n),
            Token::Comment(ref s) => write!(f, "// {}", s),

            Token::And => write!(f, "and"),
            Token::Class => write!(f, "class"),
            Token::Else => write!(f, "else"),
            Token::False => write!(f, "false"),
            Token::Fun => write!(f, "fun"),
            Token::For => write!(f, "for"),
            Token::If => write!(f, "if"),
            Token::Nil => write!(f, "nil"),
            Token::Or => write!(f, "or"),
            Token::Print => write!(f, "print"),
            Token::Return => write!(f, "return"),
            Token::Super => write!(f, "super"),
            Token::This => write!(f, "this"),
            Token::True => write!(f, "true"),
            Token::Var => write!(f, "var"),
            Token::While => write!(f, "while"),

            Token::Eof => write!(f, "EOF"),
        }
    }
}

fn match_keyword(s: &str) -> Option<Token> {
    let s: &str = &s;
    match s {
        "and" => Some(Token::And),
        "class" => Some(Token::Class),
        "else" => Some(Token::Else),
        "false" => Some(Token::False),
        "fun" => Some(Token::Fun),
        "for" => Some(Token::For),
        "if" => Some(Token::If),
        "nil" => Some(Token::Nil),
        "or" => Some(Token::Or),
        "print" => Some(Token::Print),
        "return" => Some(Token::Return),
        "super" => Some(Token::Super),
        "this" => Some(Token::This),
        "true" => Some(Token::True),
        "var" => Some(Token::Var),
        "while" => Some(Token::While),
        _ => None,
    }
}

fn consume_line<I: Iterator<Item = char>>(src: &mut Peekable<I>) -> String {
    let mut l = String::new();
    for  c in src {
        match c {
            '\n' => break,
            _ => l.push(c),
        }
    }
    l
}

fn consume_match_one_two<I, A, B, C>(src: &mut Peekable<I>, next: char, a: A, b: B) -> C
where
    I : Iterator<Item = char>,
    A: Fn() -> C,
    B: Fn() -> C,
{
    if consume_match_next(src, next) {
        b()
    } else {
        a()
    }
}

fn consume_match_next<I: Iterator<Item = char>>(src: &mut Peekable<I>, next: char) -> bool {
    let next_match = match src.peek() {
        Some(&c) => c == next,
        None => false,
    };
    if next_match {
        src.next();
    }
    next_match
}

fn consume_number<I: Iterator<Item = char>>(src: &mut Peekable<I>, c: char) -> Result<f64> {
    let mut num_str = String::new();
    num_str.push(c);
    while let Some(&next_ch) = src.peek() {
        if next_ch.is_numeric() || next_ch == '.' {
            num_str.push(src.next().unwrap());
        } else {
            break;
        }
    }
    Ok(num_str.parse()?)
}

fn consume_identifer_or_keyword<I: Iterator<Item = char>>(src: &mut Peekable<I>, c: char) -> Result<String> {
    let mut s = String::new();
    s.push(c);
    while let Some(&c) = src.peek() {
        if c.is_alphanumeric() || c == '_' {
            s.push(c);
            src.next();
        } else {
            break;
        }
    }
    Ok(s)
}

fn eat_whitespace<I: Iterator<Item=char>>(src: &mut Peekable<I>) {
    while let Some(c) = src.peek() {
        if c.is_whitespace() {
            src.next();
        } else {
            break;
        }
    }
}

fn next_token<I: Iterator<Item=char>>(src: &mut Peekable<I>) -> Result<Token> {
    eat_whitespace(src);

    let ch = match src.next() {
        Some(ch) => ch,
        None => return Ok(Token::Eof),
    };

    let token = match ch {
        '(' => Token::LeftParen,
        ')' => Token::RightParen,
        '{' => Token::LeftBrace,
        '}' => Token::RightBrace,
        ',' => Token::Comma,
        '.' => Token::Dot,
        '-' => Token::Minus,
        '+' => Token::Plus,
        ';' => Token::Semicolon,
        '/' => {
            let is_comment = consume_match_one_two(src, '/',
                || true,
                || false);
            if is_comment {
                Token::Comment(consume_line(src))
            } else {
                Token::Slash
            }
        },
        '*' => Token::Star,
        '!' => consume_match_one_two(src, '=',
            || Token::Bang,
            || Token::BangEqual),
        '=' => consume_match_one_two(src, '=',
            || Token::Equal,
            || Token::EqualEqual),
        '>' => consume_match_one_two(src, '=',
            || Token::Greater,
            || Token::GreaterEqual),
        '<' => consume_match_one_two(src, '=',
            || Token::Less,
            || Token::LessEqual),
        _ => {
            if ch.is_numeric() {
                let number: f64 = consume_number(src, ch).unwrap();
                Token::Number(number)
            } else if ch.is_alphanumeric() {
                let s = consume_identifer_or_keyword(src, ch).unwrap();
                if let Some(t) = match_keyword(&s) {
                    t
                } else {
                    Token::Identifier(s)
                }
            } else {
                Token::Eof
            }
        },
    };
    Ok(token)
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut src = s.chars().peekable();

    loop {
        match next_token(&mut src) {
            Ok(token) => {
                let eof = token == Token::Eof;
                tokens.push(token);
                if eof {
                    break;
                }
            },
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }
    tokens
}
