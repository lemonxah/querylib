use std::fmt::Display;

use crate::query::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Param { value: Value }
impl Param {
    pub fn from_value(v: Value) -> Param {
        Param { value: v }
    }
}
pub trait ToParam {
  fn to_param(self: Self, params: &mut Vec<Param>) -> String;
}

impl ToParam for Value {
  fn to_param(self, params: &mut Vec<Param>) -> String { 
    params.push(Param::from_value(self));
    format!("${}", params.len())
  }
}

#[allow(dead_code)]
pub struct Where { 
    where_clause: String,
    params: Vec<Param>,
}

impl Display for Where {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.where_clause)
    }
}

impl Where {
    fn from(wc: String, params: Vec<Param>) -> Where {
        Where { where_clause: wc, params }
    }
}
pub trait ToWhere {
    fn to_where(&self) -> Where {
        self.to_where_with_params(&mut vec![])
    }
    fn to_where_with_params(&self, params: &mut Vec<Param>) -> Where;
}

impl ToWhere for Query {
  fn to_where_with_params(&self, params: &mut Vec<Param>) -> Where {
    match self {
      Query::And { left, right } => Where::from(format!("({} AND {})", left.to_where_with_params(params), right.to_where_with_params(params)), params.clone()),
      Query::Or { left, right } => Where::from(format!("({} OR {})", left.to_where_with_params(params), right.to_where_with_params(params)), params.clone()),
      Query::Eq { field, value } => Where::from(format!("{field} = {value}", field = field, value = value.clone().to_param(params)), params.clone()),
      Query::Neq { field, value } => Where::from(format!("{field} != {value}", field = field, value = value.clone().to_param(params)), params.clone()),
      Query::Gt { field, value } => Where::from(format!("{field} > {value}", field = field, value = value.clone().to_param(params)), params.clone()),
      Query::GtE { field, value } => Where::from(format!("{field} >= {value}", field = field, value = value.clone().to_param(params)), params.clone()),
      Query::Lt { field, value } => Where::from(format!("{field} < {value}", field = field, value = value.clone().to_param(params)), params.clone()),
      Query::LtE { field, value } => Where::from(format!("{field} <= {value}", field = field, value = value.clone().to_param(params)), params.clone()),
      Query::Rx { field, value } => Where::from(format!("{field} LIKE {value}", field = field, value = value.clone().to_param(params)), params.clone()),
      Query::In { field, value } => Where::from(format!("{field} IN {value}", field = field, value = value.clone().to_param(params)), params.clone()),
      Query::Contains { field, value } => Where::from(format!("{field} CONTAINS {value}", field = field, value = value.clone().to_param(params)), params.clone()),
      Query::None => Where::from("".to_owned(), params.clone()),
    }
  }
}

pub fn to_where(query: &dyn ToWhere) -> Where {
    query.to_where()
}  

#[cfg(test)]
mod test {
  use crate::postgres::{self, *};

  #[test]
  fn test_postgress() {
    let q = query!("deleted" == false && "b" == 5);
    let q2 = query!(..q.clone(); || "c" == 7);
    let q3 = query!(..q.clone(); && ("a" == 5 || "b" < 5));
    let q_r = "(deleted = $1 AND b = $2)".to_owned();
    let q2_r = format!("({} OR c = $3)", q_r);
    let q3_r = format!("((deleted = $1 AND b = $2) AND (a = $3 OR b < $4))");
    let result = postgres::to_where(&q);
    assert_eq!(result.where_clause, q_r);
    assert_eq!(result.params, vec![Param::from_value(Value::from(false)), Param::from_value(Value::from(5))]);
    assert_eq!(postgres::to_where(&q2).where_clause, q2_r);
    assert_eq!(postgres::to_where(&q3).where_clause, q3_r);
  }

  #[test]
  fn query_in_bson_string() {
    let q = query!("deleted" == false && "b" in ["5","6","7"]);
    let q_r = "(deleted = $1 AND b IN $2)".to_owned();
    let result = postgres::to_where(&q);
    assert_eq!(result.where_clause, q_r);
    assert_eq!(result.params, vec![Param::from_value(Value::from(false)), Param::from_value(Value::from(vec!["5","6","7"]))])
  }

  #[test]
  fn query_in_bson_i32() {
    let q = query!("deleted" == false && "b" in [5,6,7]);
    let q_r = "(deleted = $1 AND b IN $2)".to_owned();
    let result = postgres::to_where(&q);
    assert_eq!(result.where_clause, q_r);
    assert_eq!(result.params, vec![Param::from_value(Value::from(false)), Param::from_value(Value::from(vec![5,6,7]))])
  }

  #[test]
  fn query_in_bson_i64() {
    let q = query!("deleted" == false && "b" in [5i64,6i64,7i64]);
    let q_r = "(deleted = $1 AND b IN $2)".to_owned();
    let result = postgres::to_where(&q);
    assert_eq!(result.where_clause, q_r);
    assert_eq!(result.params, vec![Param::from_value(Value::from(false)), Param::from_value(Value::from(vec![5i64,6i64,7i64]))])
  }

  #[test]
  fn query_in_bson_f64() {
    let q = query!("deleted" == false && "b" in [5.5f64,6.3f64,7f64]);
    let q_r = "(deleted = $1 AND b IN $2)".to_owned();
    let result = postgres::to_where(&q);
    assert_eq!(result.where_clause, q_r);
    assert_eq!(result.params, vec![Param::from_value(Value::from(false)), Param::from_value(Value::from(vec![5.5f64,6.3f64,7f64]))])
  }

  #[test]
  fn query_contains_string() {
    let q = query!("deleted" == false && "b" contains "hi");
    let q_r = "(deleted = $1 AND b CONTAINS $2)".to_owned();
    let result = postgres::to_where(&q);
    assert_eq!(result.where_clause, q_r);
    assert_eq!(result.params, vec![Param::from_value(Value::from(false)), Param::from_value(Value::from("hi"))])
  }

  #[test]
  fn query_contains_integer() {
    let q = query!("deleted" == false && "b" contains 6);
    let q_r = "(deleted = $1 AND b CONTAINS $2)".to_owned();
    let result = postgres::to_where(&q);
    assert_eq!(result.where_clause, q_r);
    assert_eq!(result.params, vec![Param::from_value(Value::from(false)), Param::from_value(Value::from(6))])
  }

  #[test]
  fn query_contains_float() {
    let q = query!("deleted" == false && "b" contains 123.43f64);
    let q_r = "(deleted = $1 AND b CONTAINS $2)".to_owned();
    let result = postgres::to_where(&q);
    assert_eq!(result.where_clause, q_r);
    assert_eq!(result.params, vec![Param::from_value(Value::from(false)), Param::from_value(Value::from(123.43f64))])
  }
}