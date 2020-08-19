#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Operator<A, B> {
  Or { left: A, right: B },
  And { left: A, right: B }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Operand<A> {
  Eq { field: String, value: A },
  Neq { field: String, value: A },
  Gt { field: String, value: A },
  GtE { field: String, value: A },
  Lt { field: String, value: A },
  LtE { field: String, value: A },
  Rx { field: String, value: A },
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! query {
  ( ..$lhs:tt && $($rest:tt)+ ) => {{ Operator::And { left: $lhs, right: query!($($rest)*) } }};
  ( ..$lhs:expr; && $($rest:tt)+ ) => {{ Operator::And { left: $lhs, right: query!($($rest)*) } }};
  ( ..$lhs:tt || $($rest:tt)+ ) => {{ Operator::Or { left: $lhs, right: query!($($rest)*) } }};
  ( ..$lhs:expr; || $($rest:tt)+ ) => {{ Operator::Or { left: $lhs, right: query!($($rest)*) } }};
  ( $key:tt == $value:tt ) => {{ Operand::Eq { field: $key.to_owned(), value: $value } }};
  ( $key:tt == $value:expr; ) => {{ Operand::Eq { field: $key.to_owned(), value: $value } }};
  ( $key:tt != $value:tt ) => {{ Operand::Neq { field: $key.to_owned(), value: $value } }};
  ( $key:tt != $value:expr; ) => {{ Operand::Neq { field: $key.to_owned(), value: $value } }};
  ( $key:tt > $value:tt ) => {{ Operand::Gt { field: $key.to_owned(), value: $value } }};
  ( $key:tt > $value:expr; ) => {{ Operand::Gt { field: $key.to_owned(), value: $value } }};
  ( $key:tt >= $value:tt ) => {{ Operand::GtE { field: $key.to_owned(), value: $value } }};
  ( $key:tt >= $value:expr; ) => {{ Operand::GtE { field: $key.to_owned(), value: $value } }};
  ( $key:tt < $value:tt ) => {{ Operand::Lt { field: $key.to_owned(), value: $value } }};
  ( $key:tt < $value:expr; ) => {{ Operand::Lt { field: $key.to_owned(), value: $value } }};
  ( $key:tt >= $value:tt ) => {{ Operand::LtE { field: $key.to_owned(), value: $value } }};
  ( $key:tt >= $value:expr; ) => {{ Operand::LtE { field: $key.to_owned(), value: $value } }};
  ( $key:tt %% $value:tt ) => {{ Operand::Rx { field: $key.to_owned(), value: $value } }};
  ( $key:tt %% $value:expr; ) => {{ Operand::Rx { field: $key.to_owned(), value: $value } }};
  ( $key:tt $op:tt $value:tt && $($rest:tt)+ ) => {{ Operator::And { left: query!($key $op $value), right: query!($($rest)*) } }};
  ( $key:tt $op:tt $value:expr; && $($rest:tt)+ ) => {{ Operator::And { left: query!($key $op $value), right: query!($($rest)*) } }};
  ( $key:tt $op:tt $value:tt || $($rest:tt)+ ) => {{ Operator::Or { left: query!($key $op $value), right: query!($($rest)*) } }};
  ( $key:tt $op:tt $value:expr; || $($rest:tt)+ ) => {{ Operator::Or { left: query!($key $op $value), right: query!($($rest)*) } }};
  ( ($($lhs:tt)+) && $($rhs:tt)+ ) => {{ Operator::And { left: query!($($lhs)*), right: query!($($rhs)*) } }};
  ( ($($lhs:tt)+) || $($rhs:tt)+ ) => {{ Operator::Or { left: query!($($lhs)*), right: query!($($rhs)*) } }};
  ( $($lhs:tt)+ && ($($rhs:tt)+) ) => {{ Operator::And { left: query!($($lhs)*), right: query!($($rhs)*) } }};
  ( $($lhs:tt)+ || ($($rhs:tt)+) ) => {{ Operator::Or { left: query!($($lhs)*), right: query!($($rhs)*) } }};
  ( ($($qq:tt)+) ) => {{ query!($($qq)*) }};
}

pub enum QueryType {
  Operand,
  Operator
}

pub trait Query<A, B> {
  type R;
  fn value(self) -> Self::R;
  fn query_type(self) -> QueryType;
}

impl <A, B> Query<A, B> for Operand<A> {
  type R = Operand<A>;
  fn value(self) -> Operand<A> { self }
  fn query_type(self) -> QueryType { QueryType::Operand }
}

impl <A, B> Query<A, B> for Operator<A, B> {
  type R = Operator<A, B>;
  fn value(self) -> Operator<A, B> { self }
  fn query_type(self) -> QueryType { QueryType::Operator }
}

#[cfg(test)]
mod test {
  use crate::query::*;
  #[test]
  fn still_works() {
    let q = query!("deleted" == false && "b" == 5);
    let q2 = query!(..q.clone(); || "c" == 7);
    let q3 = query!(..q.clone(); && ("a" == 5 || "b" < 5));
    let q_r = Operator::And { left: Operand::Eq { field: "deleted".to_owned(), value: false }, right: Operand::Eq { field: "b".to_owned(), value: 5 } };
    let q2_r = Operator::Or { left: q_r.clone(), right: Operand::Eq { field: "c".to_owned(), value: 7 }};
    let q3_r = Operator::And { left: q_r.clone(), right: Operator::Or { left: Operand::Eq { field: "a".to_owned(), value: 5 }, right: Operand::Lt { field: "b".to_owned(), value: 5 }}};
    assert_eq!(q, q_r);
    assert_eq!(q2, q2_r);
    assert_eq!(q3, q3_r);
  }
}