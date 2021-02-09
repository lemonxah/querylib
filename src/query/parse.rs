use std::collections::LinkedList;
use std::fmt::{self, Write};
use lexer::*;

use crate::query::*;

#[derive(Debug, Clone,  PartialEq)]
pub enum Operator {
  And,
  Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
  Eq,
  Neq,
  Gt,
  GtE,
  Lt,
  LtE,
  Rx,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
  Number(i64),
  Float(f64),
  String(String),
  Bool(bool),

  Operand(Operand),
  Operator(Operator),
  Identifier(String),
  Grouped(LinkedList<Token>),
}

impl From<TokenValue> for Value {
  fn from(v: TokenValue) -> Value {
    match v {
      TokenValue::Number(n) => Value::Number(n),
      TokenValue::Float(f) => Value::Float(f),
      TokenValue::String(s) => Value::String(s),
      TokenValue::Bool(b) => Value::Bool(b),
      none => {
        dbg!("none?",none);
        Value::None
      }
    }
  }
}

impl fmt::Display for TokenValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      &TokenValue::Number(ref n) => write!(f, "{}", n),
      &TokenValue::String(ref s) => write!(f, "{:?}", s),
      &TokenValue::Bool(ref b) => write!(f, "{}", b),
      &TokenValue::Float(ref fv) => write!(f, "{:?}", fv),
      &TokenValue::Operand(ref s) => write!(f, ":{:?}", s),
      &TokenValue::Operator(ref s) => write!(f, ":{:?}", s),
      &TokenValue::Identifier(ref s) => write!(f, "{}", s),
      &TokenValue::Grouped(ref grouped) => {
        f.write_char('(')?;
        let mut index = 0;
        for token in grouped {
          write!(f, "{}", token.value())?;
          index += 1;
          if index < grouped.len() {
            f.write_str(", ")?;
          }
        }
        f.write_char(')')
      }
    }
  }
}

pub type Token = lexer::Token<TokenValue>;
pub type TokenError = lexer::TokenError<&'static str>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WhitespaceReader;

impl Reader<Token, TokenError> for WhitespaceReader {
  fn read(&self, _reader: &Readers<Token, TokenError>, input: &mut dyn Input, _current: &State, next: &mut State) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => {
        if ch.is_whitespace() || ch == ',' {
          while let Some(ch) = input.peek(next, 0) {
            if ch.is_whitespace() || ch == ',' {
              input.read(next);
            } else {
              break;
            }
          }
          ReaderResult::Empty
        } else {
          ReaderResult::None
        }
      },
      None => ReaderResult::None,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct NumberReader;

impl Reader<Token, TokenError> for NumberReader {
  fn read(&self, _reader: &Readers<Token, TokenError>, input: &mut dyn Input, current: &State, next: &mut State) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => {
        if ch.is_numeric() {
          let mut string = String::new();
          string.push(ch);
          while let Some(ch) = input.peek(next, 0) {
            if ch.is_numeric() || ch == '.' {
              input.read(next);
              string.push(ch);
            } else {
              break;
            }
          }
          if string.contains(".") {
            ReaderResult::Some(Token::new(
              TokenMeta::new_state_meta(current, next),
              TokenValue::Float(string.parse().unwrap()),
            ))  
          } else {
            ReaderResult::Some(Token::new(
              TokenMeta::new_state_meta(current, next),
              TokenValue::Number(string.parse().unwrap()),
            ))
          }
        } else {
          ReaderResult::None
        }
      },
      None => ReaderResult::None,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StringReader;

impl Reader<Token, TokenError> for StringReader {
  fn read(&self, _reader: &Readers<Token, TokenError>, input: &mut dyn Input, current: &State, next: &mut State) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => {
        if ch == '\'' {
          let mut string = String::new();
          while let Some(ch) = input.read(next) {
            if ch == '\'' {
              break;
            } else {
              string.push(ch);
            }
          }
          ReaderResult::Some(Token::new(
            TokenMeta::new_state_meta(current, next),
            TokenValue::String(string),
          ))
        } else {
          ReaderResult::None
        }
      },
      None => ReaderResult::None,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct OperandReader;

impl Reader<Token, TokenError> for OperandReader {
  fn read(&self, _reader: &Readers<Token, TokenError>, input: &mut dyn Input, current: &State, next: &mut State) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => {
        match ch {
          '=' => {
            match input.peek(next, 0) {
              Some(ch) => {
                if ch == '=' {
                  input.read(next);
                  ReaderResult::Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenValue::Operand(Operand::Eq))
                  )
                } else {
                  ReaderResult::None
                }
              },
              None => ReaderResult::None,
            }
          },
          '!' => {
            match input.peek(next, 0) {
              Some(ch) => {
                if ch == '=' {
                  input.read(next);
                  ReaderResult::Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenValue::Operand(Operand::Neq))
                  )
                } else {
                  ReaderResult::None
                }
              },
              None => ReaderResult::None,
            }
          },
          '#' => {
            match input.peek(next, 0) {
              Some(ch) => {
                if ch == '=' {
                  input.read(next);
                  ReaderResult::Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenValue::Operand(Operand::Rx))
                  )
                } else {
                  ReaderResult::None
                }
              },
              None => ReaderResult::None,
            }
          },
          '<' => {
            match input.peek(next, 0) {
              Some(ch) => {
                if ch == '=' {
                  input.read(next);
                  ReaderResult::Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenValue::Operand(Operand::LtE))
                  )
                } else {
                  ReaderResult::Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenValue::Operand(Operand::Lt))
                  )
                }
              },
              None => ReaderResult::Some(Token::new(
                TokenMeta::new_state_meta(current, next),
                TokenValue::Operand(Operand::Lt))
              ),
            }
          },
          '>' => {
            match input.peek(next, 0) {
              Some(ch) => {
                if ch == '=' {
                  input.read(next);
                  ReaderResult::Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenValue::Operand(Operand::GtE))
                  )
                } else {
                  ReaderResult::Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenValue::Operand(Operand::Gt))
                  )
                }
              },
              None => ReaderResult::Some(Token::new(
                TokenMeta::new_state_meta(current, next),
                TokenValue::Operand(Operand::Gt))
              ),
            }
          },
          _ => ReaderResult::None
        }
      },
      None => ReaderResult::None,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct OperatorReader;

impl Reader<Token, TokenError> for OperatorReader {
  fn read(&self, _reader: &Readers<Token, TokenError>, input: &mut dyn Input, current: &State, next: &mut State) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => {
        match ch {
          '&' => {
            match input.peek(next, 0) {
              Some(ch) => {
                if ch == '&' {
                  input.read(next);
                  ReaderResult::Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenValue::Operator(Operator::And))
                  )
                } else {
                  ReaderResult::None
                }
              },
              None => ReaderResult::None,
            }
          },
          '|' => {
            match input.peek(next, 0) {
              Some(ch) => {
                if ch == '|' {
                  input.read(next);
                  ReaderResult::Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenValue::Operator(Operator::Or))
                  )
                } else {
                  ReaderResult::None
                }
              },
              None => ReaderResult::None
            }
          },
          _ => ReaderResult::None
        }
      },
      None => ReaderResult::None,
    }
  }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentifierReader;

impl Reader<Token, TokenError> for IdentifierReader {
  fn read(&self, _reader: &Readers<Token, TokenError>, input: &mut dyn Input, current: &State, next: &mut State) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => {
        if ch.is_alphabetic() {
          let mut string = String::new();
          string.push(ch);
          while let Some(ch) = input.peek(next, 0) {
            if ch.is_alphanumeric() || ch == '.' || ch == '_' {
              input.read(next);
              string.push(ch);
            } else {
              break;
            }
          }
          match string.as_str() {
            "true" => ReaderResult::Some(Token::new(
              TokenMeta::new_state_meta(current, next),
              TokenValue::Bool(true),
            )),
            "false" => ReaderResult::Some(Token::new(
              TokenMeta::new_state_meta(current, next),
              TokenValue::Bool(false),
            )),
            val => ReaderResult::Some(Token::new(
              TokenMeta::new_state_meta(current, next),
              TokenValue::Identifier(string),
            ))
          }
        } else {
          ReaderResult::None
        }
      },
      None => ReaderResult::None,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GroupedReader;

impl Reader<Token, TokenError> for GroupedReader {
  fn read(&self, readers: &Readers<Token, TokenError>, input: &mut dyn Input, current: &State, next: &mut State) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => {
        if ch == '(' {
          let mut group = LinkedList::new();

          while let Some(ch) = input.peek(next, 0) {
            if ch == ')' {
              input.read(next);
              break;
            } else {
              match lexer::next(readers, input, next) {
                Some(Ok(token)) => {
                  group.push_back(token);
                },
                Some(Err(error)) => {
                  return ReaderResult::Err(error);
                },
                _ => {
                  break;
                }
              }
            }
          }

          ReaderResult::Some(Token::new(
            TokenMeta::new_state_meta(current, next),
            TokenValue::Grouped(group),
          ))
        } else {
          ReaderResult::None
        }
      },
      None => ReaderResult::None,
    }
  }
}

fn parse(tokens: LinkedList<Token>, state: Query) -> Query {
  let mut list = tokens.clone();
  match list.pop_front() {
    None => state,
    Some(head) => {
      match head.value() {
        TokenValue::Grouped(group) => {
          let q = parse(group.clone(), Query::None);
          parse(list, q)
        }
        TokenValue::Operator(Operator::And) => {
          Query::And {left: Box::new(state), right: Box::new(parse(list.clone(), Query::None))}
        },
        TokenValue::Operator(Operator::Or) => {
          Query::Or {left: Box::new(state), right: Box::new(parse(list, Query::None))}
        },
        TokenValue::Identifier(ident) => {
          let op = list.pop_front().unwrap();
          let val = list.pop_front().unwrap();
          let query = match op.value() {
            TokenValue::Operand(Operand::Eq) => Query::Eq { field: ident.clone(), value: val.value().clone().into() },
            TokenValue::Operand(Operand::Neq) => Query::Neq { field: ident.clone(), value: val.value().clone().into() },
            TokenValue::Operand(Operand::Gt) => Query::Gt { field: ident.clone(), value: val.value().clone().into() },
            TokenValue::Operand(Operand::GtE) => Query::GtE { field: ident.clone(), value: val.value().clone().into() },
            TokenValue::Operand(Operand::Lt) => Query::Lt { field: ident.clone(), value: val.value().clone().into() },
            TokenValue::Operand(Operand::LtE) => Query::LtE { field: ident.clone(), value: val.value().clone().into() },
            TokenValue::Operand(Operand::Rx) => Query::Rx { field: ident.clone(), value: val.value().clone().into() },
            _ => Query::None
          };
          parse(list, query)
        },
        _ => Query::None,
      } 
    }
  }
}

pub fn from_str(s: &str) -> Query {
  let readers = ReadersBuilder::new()
    .add(WhitespaceReader)
    .add(NumberReader)
    .add(StringReader)
    .add(OperandReader)
    .add(OperatorReader)
    .add(IdentifierReader)
    .add(GroupedReader)
    .build();
  let _lexer = readers.lexer(s.chars());
  let tokens: LinkedList<Token> = _lexer.map(Result::unwrap).collect();
  parse(tokens, Query::None)
}

#[cfg(test)]
mod parse_test {
  use crate::query::*;
  #[test]
  fn lexer_works() {
    let squery = "deleted == false && b == 5 && (a == 5 || b < 5)";
    let query = parse::from_str(squery);
    dbg!(&query);
    let q_r = Query::And { 
      left: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() }),
      right: Box::new(Query::And { 
        left: Box::new(Query::Eq { field: "b".to_owned(), value: 5.into() }), 
        right: Box::new(Query::Or { 
          left: Box::new(Query::Eq { field: "a".to_owned(), value: 5.into() }), 
          right: Box::new(Query::Lt { field: "b".to_owned(), value: 5.into() })
        })
      })
    };
    dbg!(&query, &q_r);
    assert_eq!(query, q_r);
  }
}