use bson::Bson::Null;
use bson::Document;
use crate::query::*;

pub trait FromOperand {
  fn from_op(self: &Self, op: &str) -> Document;
}
impl FromOperand for Value {
  fn from_op(&self, op: &str) -> Document { 
    match self {
      Value::String(s) => doc!(op: s),
      Value::Number(n) => doc!(op: n),
      Value::Float(f) => doc!(op: f),
      Value::Bool(b) => doc!(op: b),
      Value::None => doc!(op: Null),
    }
  }
}
pub trait ToBson {
  fn to_bson(self) -> Document;
}
impl ToBson for Query {
  fn to_bson(self) -> Document {
    match self {
      Query::And { left, right } => doc!("$and": [ left.to_bson() , right.to_bson() ]),
      Query::Or { left, right } => doc!("$or": [ left.to_bson() , right.to_bson() ]),
      Query::Eq { field, value } => doc!( field : value.from_op("$eq")),
      Query::Neq { field, value } => doc!( field : value.from_op("$neq")),
      Query::Gt { field, value } => doc!( field : value.from_op("$gt")),
      Query::GtE { field, value } => doc!( field : value.from_op("$gte")),
      Query::Lt { field, value } => doc!( field : value.from_op("$lt")),
      Query::LtE { field, value } => doc!( field : value.from_op("$lte")),
      Query::Rx { field, value } => doc!( field : value.from_op("$regex")),
      Query::None => doc!(),
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
    let q_r = doc!("$and" : [ doc!("deleted": doc!("$eq": false)) , doc!("b": doc!("$eq": 5i64)) ]);
    let q2_r = doc!("$or" : [ doc!("$and" : [ doc!("deleted": doc!("$eq": false)) , doc!("b": doc!("$eq": 5i64)) ]) , doc!("c": doc!("$eq": 7i64)) ]);
    let q3_r = doc!("$and" : [ doc!("$and" : [ doc!("deleted": doc!("$eq": false)) , doc!("b": doc!("$eq": 5i64)) ]) , doc!("$or" : [ doc!("a": doc!("$eq": 5i64)) , doc!("b": doc!("$lt": 5i64)) ]) ]);
    assert_eq!(mongo::to_bson(q), q_r);
    assert_eq!(mongo::to_bson(q2), q2_r);
    assert_eq!(mongo::to_bson(q3), q3_r);
  }
}