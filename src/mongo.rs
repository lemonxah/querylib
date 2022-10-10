use bson::Bson::Null;
use bson::Document;
use crate::query::*;

pub trait FromOperand {
  fn from_op(self: &Self, op: &str) -> Document;
}
impl FromOperand for Value {
  fn from_op(&self, op: &str) -> Document { 
    match self {
      Value::Uuid(u) => doc!(op: u.hyphenated().to_string()),
      Value::String(s) => doc!(op: s),
      Value::Number(n) => doc!(op: n),
      Value::Float(f) => doc!(op: f),
      Value::Bool(b) => doc!(op: b),
      Value::Array(arr) => {
        match &arr.clone().into_iter().next() {
          Some(Value::String(_)) => {
            doc!(op: arr.clone().into_iter().map(|v| v.get_string()).flatten().collect::<Vec<String>>())
          },
          Some(Value::Number(_)) => {
            doc!(op: arr.clone().into_iter().map(|v| v.get_number()).flatten().collect::<Vec<i64>>())
          },
          Some(Value::Float(_)) => {
            doc!(op: arr.clone().into_iter().map(|v| v.get_float()).flatten().collect::<Vec<f64>>())
          },
          _ => doc!(op: Null)
        }
      },
      Value::None => doc!(op: Null),
    }
  }
}
pub trait ToBson {
  fn to_bson(&self) -> Document;
}
impl ToBson for Query {
  fn to_bson(&self) -> Document {
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
      Query::In { field, value } => doc!( field : value.from_op("$in")),
      Query::Contains { field, value } => doc!( field : value.from_op("$elemMatch")),
      Query::None => doc!(),
    }
  }
}

pub fn to_bson(query: &dyn ToBson) -> Document {
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
    assert_eq!(mongo::to_bson(&q), q_r);
    assert_eq!(mongo::to_bson(&q2), q2_r);
    assert_eq!(mongo::to_bson(&q3), q3_r);
  }

  #[test]
  fn query_in_bson_string() {
    let q = query!("deleted" == false && "b" in ["5","6","7"]);
    let q_r = doc!("$and" : [ doc!("deleted": doc!("$eq": false)) , doc!("b": doc!("$in": vec!["5","6","7"])) ]);
    assert_eq!(mongo::to_bson(&q), q_r);
  }

  #[test]
  fn query_in_bson_uuid() {
    let uuid = uuid::Uuid::new_v4();
    let uuid_string = uuid.hyphenated().to_string();
    let q = query!("deleted" == false && "b" == uuid);
    let q_r = doc!("$and" : [ doc!("deleted": doc!("$eq": false)) , doc!("b": doc!("$eq": uuid_string)) ]);
    assert_eq!(mongo::to_bson(&q), q_r);
  }

  #[test]
  fn query_in_bson_i32() {
    let q = query!("deleted" == false && "b" in [5,6,7]);
    let q_r = doc!("$and" : [ doc!("deleted": doc!("$eq": false)) , doc!("b": doc!("$in": vec![5i64,6i64,7i64])) ]);
    assert_eq!(mongo::to_bson(&q), q_r);
  }

  #[test]
  fn query_in_bson_i64() {
    let q = query!("deleted" == false && "b" in [5i64,6i64,7i64]);
    let q_r = doc!("$and" : [ doc!("deleted": doc!("$eq": false)) , doc!("b": doc!("$in": vec![5i64,6i64,7i64])) ]);
    assert_eq!(mongo::to_bson(&q), q_r);
  }

  #[test]
  fn query_in_bson_f64() {
    let q = query!("deleted" == false && "b" in [5.5f64,6.3f64,7f64]);
    let q_r = doc!("$and" : [ doc!("deleted": doc!("$eq": false)) , doc!("b": doc!("$in": vec![5.5f64,6.3f64,7f64])) ]);
    assert_eq!(mongo::to_bson(&q), q_r);
  }

  #[test]
  fn query_contains_string() {
    let q = query!("deleted" == false && "b" contains "hi");
    let q_r = doc!("$and" : [ doc!("deleted": doc!("$eq": false)) , doc!("b": doc!("$elemMatch": "hi".to_owned())) ]);
    assert_eq!(mongo::to_bson(&q), q_r);
  }

  #[test]
  fn query_contains_integer() {
    let q = query!("deleted" == false && "b" contains 6);
    let q_r = doc!("$and" : [ doc!("deleted": doc!("$eq": false)) , doc!("b": doc!("$elemMatch": 6i64)) ]);
    assert_eq!(mongo::to_bson(&q), q_r);
  }

  #[test]
  fn query_contains_float() {
    let q = query!("deleted" == false && "b" contains 123.43f64);
    let q_r = doc!("$and" : [ doc!("deleted": doc!("$eq": false)) , doc!("b": doc!("$elemMatch": 123.43f64)) ]);
    assert_eq!(mongo::to_bson(&q), q_r);
  }
}