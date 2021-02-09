#[cfg(feature = "parse")] pub mod parse;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Number(i64),
  Float(f64),
  String(String),
  Bool(bool),
  None,
}

impl From<i64> for Value {
  fn from(i: i64) -> Value {
    Value::Number(i)
  }
}

impl From<f64> for Value {
  fn from(v: f64) -> Value {
    Value::Float(v)
  }
}

impl From<String> for Value {
  fn from(v: String) -> Value {
    Value::String(v)
  }
}

impl From<bool> for Value {
  fn from(v: bool) -> Value {
    Value::Bool(v)
  }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Query {
  Or { left: Box<Query>, right: Box<Query> },
  And { left: Box<Query>, right: Box<Query> },
  Eq { field: String, value: Value },
  Neq { field: String, value: Value },
  Gt { field: String, value: Value },
  GtE { field: String, value: Value },
  Lt { field: String, value: Value },
  LtE { field: String, value: Value },
  Rx { field: String, value: Value },
  None,
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! query {
  ( ..$lhs:tt && $($rest:tt)+ ) => {{ Query::And { left: Box::new($lhs), right: Box::new(query!($($rest)*)) } }};
  ( ..$lhs:expr; && $($rest:tt)+ ) => {{ Query::And { left: Box::new($lhs), right: Box::new(query!($($rest)*)) } }};
  ( ..$lhs:tt || $($rest:tt)+ ) => {{ Query::Or { left: Box::new($lhs), right: Box::new(query!($($rest)*)) } }};
  ( ..$lhs:expr; || $($rest:tt)+ ) => {{ Query::Or { left: Box::new($lhs), right: Box::new(query!($($rest)*)) } }};
  ( $key:tt == $value:tt ) => {{ Query::Eq { field: $key.to_owned(), value: $value.into() } }};
  ( $key:tt == $value:expr; ) => {{ Query::Eq { field: $key.to_owned(), value: $value.into() } }};
  ( $key:tt != $value:tt ) => {{ Query::Neq { field: $key.to_owned(), value: $value.into() } }};
  ( $key:tt != $value:expr; ) => {{ Query::Neq { field: $key.to_owned(), value: $value.into() } }};
  ( $key:tt > $value:tt ) => {{ Query::Gt { field: $key.to_owned(), value: $value.into() } }};
  ( $key:tt > $value:expr; ) => {{ Query::Gt { field: $key.to_owned(), value: $value.into() } }};
  ( $key:tt >= $value:tt ) => {{ Query::GtE { field: $key.to_owned(), value: $value.into() } }};
  ( $key:tt >= $value:expr; ) => {{ Query::GtE { field: $key.to_owned(), value: $value.into() } }};
  ( $key:tt < $value:tt ) => {{ Query::Lt { field: $key.to_owned(), value: $value.into() } }};
  ( $key:tt < $value:expr; ) => {{ Query::Lt { field: $key.to_owned(), value: $value.into() } }};
  ( $key:tt >= $value:tt ) => {{ Query::LtE { field: $key.to_owned(), value: $value.into() } }};
  ( $key:tt >= $value:expr; ) => {{ Query::LtE { field: $key.to_owned(), value: $value.into() } }};
  ( $key:tt %% $value:tt ) => {{ Query::Rx { field: $key.to_owned(), value: $value.into() } }};
  ( $key:tt %% $value:expr; ) => {{ Query::Rx { field: $key.to_owned(), value: $value.into() } }};
  ( $key:tt $op:tt $value:tt && $($rest:tt)+ ) => {{ Query::And { left: Box::new(query!($key $op $value)), right: Box::new(query!($($rest)*)) } }};
  ( $key:tt $op:tt $value:expr; && $($rest:tt)+ ) => {{ Query::And { left: Box::new(query!($key $op $value)), right: Box::new(query!($($rest)*)) } }};
  ( $key:tt $op:tt $value:tt || $($rest:tt)+ ) => {{ Query::Or { left: Box::new(query!($key $op $value)), right: Box::new(query!($($rest)*)) } }};
  ( $key:tt $op:tt $value:expr; || $($rest:tt)+ ) => {{ Query::Or { left: Box::new(query!($key $op $value)), right: Box::new(query!($($rest)*)) } }};
  ( ($($lhs:tt)+) && $($rhs:tt)+ ) => {{ Query::And { left: Box::new(query!($($lhs)*)), right: Box::new(query!($($rhs)*)) } }};
  ( ($($lhs:tt)+) || $($rhs:tt)+ ) => {{ Query::Or { left: Box::new(query!($($lhs)*)), right: Box::new(query!($($rhs)*)) } }};
  ( $($lhs:tt)+ && ($($rhs:tt)+) ) => {{ Query::And { left: Box::new(query!($($lhs)*)), right: Box::new(query!($($rhs)*)) } }};
  ( $($lhs:tt)+ || ($($rhs:tt)+) ) => {{ Query::Or { left: Box::new(query!($($lhs)*)), right: Box::new(query!($($rhs)*)) } }};
  ( ($($qq:tt)+) ) => {{ query!($($qq)*) }};
}

#[cfg(test)]
mod query_test {
  use crate::query::*;
  #[test]
  fn query_composition() {
    let q = query!("deleted" == false && "b" == 5);
    let q2 = query!(..q.clone(); || "c" == 7);
    let q3 = query!(..q.clone(); && ("a" == 5 || "b" < 5));
    let q_r = Query::And { left: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() }), right: Box::new(Query::Eq { field: "b".to_owned(), value: 5.into() }) };
    let q2_r = Query::Or { left: Box::new(q_r.clone()), right: Box::new(Query::Eq { field: "c".to_owned(), value: 7.into() }) };
    let q3_r = Query::And { left: Box::new(q_r.clone()), right: Box::new(Query::Or { left: Box::new(Query::Eq { field: "a".to_owned(), value: 5.into() }), right: Box::new(Query::Lt { field: "b".to_owned(), value: 5.into() })})};
    assert_eq!(q, q_r);
    assert_eq!(q2, q2_r);
    assert_eq!(q3, q3_r);
  }
}