use bson::Document;

#[derive(Debug, Clone)]
pub enum Operator<A, B> {
  Or { left: A, right: B },
  And { left: A, right: B }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
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
macro_rules! query {
  ( $key:tt == $value:tt ) => {{ Operand::Eq { field: $key.to_owned(), value: $value } }};
  ( $key:tt != $value:tt ) => {{ Operand::Neq { field: $key.to_owned(), value: $value } }};
  ( $key:tt > $value:tt ) => {{ Operand::Gt { field: $key.to_owned(), value: $value } }};
  ( $key:tt >= $value:tt ) => {{ Operand::GtE { field: $key.to_owned(), value: $value } }};
  ( $key:tt < $value:tt ) => {{ Operand::Lt { field: $key.to_owned(), value: $value } }};
  ( $key:tt >= $value:tt ) => {{ Operand::LtE { field: $key.to_owned(), value: $value } }};
  ( $key:tt %% $value:tt ) => {{ Operand::Rx { field: $key.to_owned(), value: $value } }};
  ( $key:tt $op:tt $value:tt && $($rest:tt)+ ) => {{ Operator::And { left: query!($key $op $value), right: query!($($rest)*) } }};
  ( $key:tt $op:tt $value:tt || $($rest:tt)+ ) => {{ Operator::Or { left: query!($key $op $value), right: query!($($rest)*) } }};
  ( ...$lhs:tt && $($rest:tt)+ ) => {{ Operator::And { left: $lhs, right: query!($($rest)*) } }};
  ( ...$lhs:tt || $($rest:tt)+ ) => {{ Operator::Or { left: $lhs, right: query!($($rest)*) } }};
  ( ($($lhs:tt)+) && $($rhs:tt)+ ) => {{ Operator::And { left: query!($($lhs)*), right: query!($($rhs)*) } }};
  ( ($($lhs:tt)+) || $($rhs:tt)+ ) => {{ Operator::Or { left: query!($($lhs)*), right: query!($($rhs)*) } }};
  ( $($lhs:tt)+ && ($($rhs:tt)+) ) => {{ Operator::And { left: query!($($lhs)*), right: query!($($rhs)*) } }};
  ( $($lhs:tt)+ || ($($rhs:tt)+) ) => {{ Operator::Or { left: query!($($lhs)*), right: query!($($rhs)*) } }};
  ( ($($qq:tt)+) ) => {{ query!($($qq)*) }};
}


pub trait FromOperand {
  fn from_op(self: &Self, op: &str) -> Document;
}
impl FromOperand for i32 {
  fn from_op(self: &Self, op: &str) -> Document { doc!(op: self) }
}
impl FromOperand for String {
  fn from_op(self: &Self, op: &str) -> Document { doc!(op: self) }
}
impl FromOperand for &str {
  fn from_op(self: &Self, op: &str) -> Document { doc!(op: self) }
}
impl FromOperand for bool {
  fn from_op(self: &Self, op: &str) -> Document { doc!(op: self) }
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

pub trait ToBson {
  fn to_bson(self) -> Document;
}
impl <A> ToBson for Operand<A> where A: FromOperand {
  fn to_bson(self) -> Document {
    match self {
      Operand::Eq { field, value } => doc!( field : value.from_op("$eq")),
      Operand::Neq { field, value } => doc!( field : value.from_op("$neq")),
      Operand::Gt { field, value } => doc!( field : value.from_op("$gt")),
      Operand::GtE { field, value } => doc!( field : value.from_op("$gte")),
      Operand::Lt { field, value } => doc!( field : value.from_op("$lt")),
      Operand::LtE { field, value } => doc!( field : value.from_op("$lte")),
      Operand::Rx { field, value } => doc!( field : value.from_op("$regex")),
    }  
  }
}

impl <A, B> ToBson for Operator<A, B> where A: ToBson, B: ToBson {
  fn to_bson(self) -> Document {
    match self {
      Operator::And { left, right } => doc!("$and": [ left.to_bson() , right.to_bson() ]),
      Operator::Or { left, right } => doc!("$or": [ left.to_bson() , right.to_bson() ]),
    }
  }
}

pub fn to_bson<Q>(query: Q) -> Document where Q: ToBson {
  query.to_bson()
}
