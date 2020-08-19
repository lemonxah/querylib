use bson::Document;
use crate::query::*;

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

#[cfg(test)]
mod test {
  use crate::mongo::{self, *};

  #[test]
  fn test_bson() {
    let q = query!("deleted" == false && "b" == 5);
    let q2 = query!(..q.clone(); || "c" == 7);
    let q3 = query!(..q.clone(); && ("a" == 5 || "b" < 5));
    let q_r = doc!("$and" : [ doc!("deleted": doc!("$eq": false)) , doc!("b": doc!("$eq": 5)) ]);
    let q2_r = doc!("$or" : [ doc!("$and" : [ doc!("deleted": doc!("$eq": false)) , doc!("b": doc!("$eq": 5)) ]) , doc!("c": doc!("$eq": 7)) ]);
    let q3_r = doc!("$and" : [ doc!("$and" : [ doc!("deleted": doc!("$eq": false)) , doc!("b": doc!("$eq": 5)) ]) , doc!("$or" : [ doc!("a": doc!("$eq": 5)) , doc!("b": doc!("$lt": 5)) ]) ]);
    assert_eq!(mongo::to_bson(q), q_r);
    assert_eq!(mongo::to_bson(q2), q2_r);
    assert_eq!(mongo::to_bson(q3), q3_r);
  }
}