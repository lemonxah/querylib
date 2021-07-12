#[cfg(feature = "parse")] pub mod parse;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Number(i64),
  Float(f64),
  String(String),
  Bool(bool),
  Array(Box<Vec<Value>>),
  None,
}

impl Value {
  pub fn get_number(&self) -> Option<i64> {
    match self {
      Value::Number(n) => Some(*n),
      _ => None,
    }
  }
  pub fn get_string(&self) -> Option<String> {
    match self {
      Value::String(s) => Some(s.clone()),
      _ => None,
    }
  }
  pub fn get_float(&self) -> Option<f64> {
    match self {
      Value::Float(f) => Some(*f),
      _ => None,
    }
  }
}

macro_rules! from_vec_value {
  ($t:ident) => {
    impl From<Vec<$t>> for Value {
      fn from(a: Vec<$t>) -> Value {
        Value::Array(Box::new(a.into_iter().map(|v| v.into()).collect::<Vec<Value>>()))
      }
    }
  };
}

from_vec_value!(i64);
from_vec_value!(f64);
from_vec_value!(String);

impl From<Vec<&str>> for Value {
  fn from(a: Vec<&str>) -> Value {
    Value::Array(Box::new(a.into_iter().map(|v| v.into()).collect::<Vec<Value>>()))
  }
}

impl From<isize> for Value {
  fn from(i: isize) -> Value {
    Value::Number(i as i64)
  }
}

impl From<usize> for Value {
  fn from(i: usize) -> Value {
    Value::Number(i as i64)
  }
}

impl From<i32> for Value {
  fn from(i: i32) -> Value {
    Value::Number(i as i64)
  }
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

impl From<&str> for Value {
  fn from(v: &str) -> Value {
    Value::String(v.to_owned())
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
  In { field: String, value: Value },
  Contains { field: String, value: Value },
  None,
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! query {
  ( ..$lhs:tt && $($rest:tt)+ ) => {{ Query::And { left: Box::new($lhs), right: Box::new(query!($($rest)*)) } }};
  ( ..$lhs:expr; && $($rest:tt)+ ) => {{ Query::And { left: Box::new($lhs), right: Box::new(query!($($rest)*)) } }};
  ( ..$lhs:tt || $($rest:tt)+ ) => {{ Query::Or { left: Box::new($lhs), right: Box::new(query!($($rest)*)) } }};
  ( ..$lhs:expr; || $($rest:tt)+ ) => {{ Query::Or { left: Box::new($lhs), right: Box::new(query!($($rest)*)) } }};
  ( $key:tt in [ $($e:expr),* ] ) => {{
    let mut _temp = ::std::vec::Vec::new();
    $(_temp.push($e);)*
    Query::In { field: $key.to_owned(), value: _temp.into() }
  }};
  ( $key:tt contains $value:tt ) => {{ Query::Contains { field: $key.to_owned(), value: $value.into() } }};
  ( $key:tt contains $value:expr; ) => {{ Query::Contains { field: $key.to_owned(), value: $value.into() } }};
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

  ( $key:tt $op:tt $value:tt && cond($condition:expr) $($rest:tt)+ ) => {{ if $condition { Query::And { left: Box::new(query!($key $op $value)), right: Box::new(query!($($rest)*)) } } else { query!($key $op $value) } }};
  ( $key:tt $op:tt $value:expr; && cond($condition:expr) $($rest:tt)+ ) => {{ if $condition { Query::And { left: Box::new(query!($key $op $value)), right: Box::new(query!($($rest)*)) } } else { query!($key $op $value) } }};
  ( $key:tt $op:tt $value:tt || cond($condition:expr) $($rest:tt)+ ) => {{ if $condition { Query::Or { left: Box::new(query!($key $op $value)), right: Box::new(query!($($rest)*)) } } else { query!($key $op $value) } }};
  ( $key:tt $op:tt $value:expr; || cond($condition:expr) $($rest:tt)+ ) => {{ if $condition { Query::Or { left: Box::new(query!($key $op $value)), right: Box::new(query!($($rest)*)) } } else { query!($key $op $value) } }};

  ( $key:tt $op:tt $value:tt && $($rest:tt)+ ) => {{ Query::And { left: Box::new(query!($key $op $value)), right: Box::new(query!($($rest)*)) } }};
  ( $key:tt $op:tt $value:expr; && $($rest:tt)+ ) => {{ Query::And { left: Box::new(query!($key $op $value)), right: Box::new(query!($($rest)*)) } }};
  ( $key:tt $op:tt $value:tt || $($rest:tt)+ ) => {{ Query::Or { left: Box::new(query!($key $op $value)), right: Box::new(query!($($rest)*)) } }};
  ( $key:tt $op:tt $value:expr; || $($rest:tt)+ ) => {{ Query::Or { left: Box::new(query!($key $op $value)), right: Box::new(query!($($rest)*)) } }};

  ( ($($lhs:tt)+) && cond($condition:expr) $($rhs:tt)+ ) => {{ if $condition { Query::And { left: Box::new(query!($($lhs)*)), right: Box::new(query!($($rhs)*)) } } else { query!($($lhs)*) } }};
  ( ($($lhs:tt)+) || cond($condition:expr) $($rhs:tt)+ ) => {{ if $condition { Query::Or { left: Box::new(query!($($lhs)*)), right: Box::new(query!($($rhs)*)) } } else { query!($($lhs)*) } }};
  ( $($lhs:tt)+ && cond($condition:expr) ($($rhs:tt)+) ) => {{ if $condition { Query::And { left: Box::new(query!($($lhs)*)), right: Box::new(query!($($rhs)*)) } } else { query!($($lhs)*) } }};
  ( $($lhs:tt)+ || cond($condition:expr) ($($rhs:tt)+) ) => {{ if $condition { Query::Or { left: Box::new(query!($($lhs)*)), right: Box::new(query!($($rhs)*)) } } else { query!($($lhs)*) } }};

  ( ($($lhs:tt)+) && $($rhs:tt)+ ) => {{ Query::And { left: Box::new(query!($($lhs)*)), right: Box::new(query!($($rhs)*)) } }};
  ( ($($lhs:tt)+) || $($rhs:tt)+ ) => {{ Query::Or { left: Box::new(query!($($lhs)*)), right: Box::new(query!($($rhs)*)) } }};
  ( $($lhs:tt)+ && ($($rhs:tt)+) ) => {{ Query::And { left: Box::new(query!($($lhs)*)), right: Box::new(query!($($rhs)*)) } }};
  ( $($lhs:tt)+ || ($($rhs:tt)+) ) => {{ Query::Or { left: Box::new(query!($($lhs)*)), right: Box::new(query!($($rhs)*)) } }};

  ( ($($qq:tt)+) ) => {{ query!($($qq)*) }};
}

#[cfg(test)]
mod test {
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

  #[test]
  fn query_in() {
    let q = query!("_id" in ["123", "456","789"] && "deleted" == false);
    let q_r = Query::And {
      left: Box::new(Query::In { field: "_id".to_owned(), value: vec!["123".to_owned(),"456".to_owned(),"789".to_owned()].into() }),
      right: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() })
    };
    let q2 = query!("_id" in [123, 456,789] && "deleted" == false);
    let q2_r = Query::And {
      left: Box::new(Query::In { field: "_id".to_owned(), value: vec![123,456,789].into() }),
      right: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() })
    };
    assert_eq!(q, q_r);
    assert_eq!(q2, q2_r);
  }

  #[test]
  fn query_contains() {
    let q = query!("deleted" == false && "countries" contains "za");
    let q_r = Query::And {
      left: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() }),
      right: Box::new(Query::Contains { field: "countries".to_owned(), value: "za".to_owned().into() }),
    };
    assert_eq!(q, q_r);
  }

  #[test]
  fn query_contains_optional_0() {
    let q = query!("deleted" == false && cond(1==0) "countries" contains "za");
    let q_r = Query::Eq { field: "deleted".to_owned(), value: false.into() };
    assert_eq!(q, q_r);
  }

  #[test]
  fn query_contains_optional_1() {
    let q = query!("deleted" == false && cond(1==1) "countries" contains "za");
    let q_r = Query::And {
      left: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() }),
      right: Box::new(Query::Contains { field: "countries".to_owned(), value: "za".to_owned().into() }),
    };
    assert_eq!(q, q_r);
  }

  #[test]
  fn query_contains_optional_2() {
    let q = query!("deleted" == false && cond(1==1) "countries" contains "za" || cond(1==0) "age" >= 21);
    let q_r = Query::And {
      left: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() }),
      right: Box::new(Query::Contains { field: "countries".to_owned(), value: "za".to_owned().into() }),
    };
    assert_eq!(q, q_r);
  }

  #[test]
  fn query_contains_optional_3() {
    let q = query!("deleted" == false && cond(1==1) "countries" contains "za" || cond(1==1) "age" >= 21);
    let q_r = Query::And {
      left: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() }),
      right: Box::new(Query::Or {
        left: Box::new(Query::Contains { field: "countries".to_owned(), value: "za".to_owned().into() }),
        right: Box::new(Query::GtE { field: "age".to_owned(), value: 21.into() })
      }),
    };
    assert_eq!(q, q_r);
  }

  #[test]
  fn query_contains_optional_4() {
    let q = query!("deleted" == false && cond(1==0) "countries" contains "za" || cond(1==1) "age" >= 21);
    let q_r = Query::Eq { field: "deleted".to_owned(), value: false.into() };
    assert_eq!(q, q_r);
  }

  #[test]
  fn query_contains_optional_5() {
    let q = query!(("deleted" == false && cond(1==0) "countries" contains "za") || cond(1==1) "age" >= 21);
    let q_r = Query::Or {
      left: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() }),
      right: Box::new(Query::GtE { field: "age".to_owned(), value: 21.into() })
    };
    assert_eq!(q, q_r);
  }

  #[test]
  fn complex_optional() {
    let entity = Some(12);
    let state = Some("Pending");
    let currency_iso = Some("USD");
    let q = query!(
      (
        ("deleted" == false && cond(entity.is_some()) ("source.id" == { entity.clone().unwrap() } || "destination.id" == { entity.clone().unwrap() } )) &&
        cond(state.is_some()) "state" == { state.unwrap() }
      ) && cond(currency_iso.is_some()) ("source.currency_iso" == { currency_iso.clone().unwrap() } || ("destination.currency_iso" == { currency_iso.clone().unwrap() }))
    );
    let q_r = Query::And {
      left: Box::new(Query::And {
        left: Box::new(Query::And {
          left: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() }),
          right: Box::new(Query::Or {
            left: Box::new(Query::Eq { field: "source.id".to_owned(), value: 12.into() }),
            right: Box::new(Query::Eq { field: "destination.id".to_owned(), value: 12.into() }),
          })
        }),
        right: Box::new(Query::Eq { field: "state".to_owned(), value: "Pending".into() }),
      }),
      right: Box::new(Query::Or {
        left: Box::new(Query::Eq { field: "source.currency_iso".to_owned(), value: "USD".to_owned().into() }),
        right: Box::new(Query::Eq { field: "destination.currency_iso".to_owned(), value: "USD".to_owned().into() }),
      })
    };
    assert_eq!(q, q_r);
  }

  #[test]
  fn complex_optional_1() {
    let entity: Option<i32> = None;
    let state = Some("Pending");
    let currency_iso = Some("USD");
    let q = query!(
      (
        ("deleted" == false && cond(entity.is_some()) ("source.id" == { entity.clone().unwrap() } || "destination.id" == { entity.clone().unwrap() } )) &&
        cond(state.is_some()) "state" == { state.unwrap() }
      ) && cond(currency_iso.is_some()) ("source.currency_iso" == { currency_iso.clone().unwrap() } || ("destination.currency_iso" == { currency_iso.clone().unwrap() }))
    );
    let q_r = Query::And {
      left: Box::new(Query::And {
        left: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() }),
        right: Box::new(Query::Eq { field: "state".to_owned(), value: "Pending".into() }),
      }),
      right: Box::new(Query::Or {
        left: Box::new(Query::Eq { field: "source.currency_iso".to_owned(), value: "USD".to_owned().into() }),
        right: Box::new(Query::Eq { field: "destination.currency_iso".to_owned(), value: "USD".to_owned().into() }),
      })
    };
    assert_eq!(q, q_r);
  }

  #[test]
  fn complex_optional_2() {
    let entity = Some(12);
    let state: Option<String> = None;
    let currency_iso = Some("USD");
    let q = query!(
      (
        ("deleted" == false && cond(entity.is_some()) ("source.id" == { entity.clone().unwrap() } || "destination.id" == { entity.clone().unwrap() } )) &&
        cond(state.is_some()) "state" == { state.unwrap() }
      ) && cond(currency_iso.is_some()) ("source.currency_iso" == { currency_iso.clone().unwrap() } || ("destination.currency_iso" == { currency_iso.clone().unwrap() }))
    );
    let q_r = Query::And {
      left: Box::new(Query::And {
        left: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() }),
        right: Box::new(Query::Or {
          left: Box::new(Query::Eq { field: "source.id".to_owned(), value: 12.into() }),
          right: Box::new(Query::Eq { field: "destination.id".to_owned(), value: 12.into() }),
        })
      }),
      right: Box::new(Query::Or {
        left: Box::new(Query::Eq { field: "source.currency_iso".to_owned(), value: "USD".to_owned().into() }),
        right: Box::new(Query::Eq { field: "destination.currency_iso".to_owned(), value: "USD".to_owned().into() }),
      })
    };
    assert_eq!(q, q_r);
  }

  #[test]
  fn complex_optional_3() {
    let entity = Some(12);
    let state = Some("Pending");
    let currency_iso: Option<String> = None;
    let q = query!(
      (
        ("deleted" == false && cond(entity.is_some()) ("source.id" == { entity.clone().unwrap() } || "destination.id" == { entity.clone().unwrap() } )) &&
        cond(state.is_some()) "state" == { state.unwrap() }
      ) && cond(currency_iso.is_some()) ("source.currency_iso" == { currency_iso.clone().unwrap() } || ("destination.currency_iso" == { currency_iso.clone().unwrap() }))
    );
    let q_r = Query::And {
      left: Box::new(Query::And {
        left: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() }),
        right: Box::new(Query::Or {
          left: Box::new(Query::Eq { field: "source.id".to_owned(), value: 12.into() }),
          right: Box::new(Query::Eq { field: "destination.id".to_owned(), value: 12.into() }),
        })
      }),
      right: Box::new(Query::Eq { field: "state".to_owned(), value: "Pending".into() }),
    };
    assert_eq!(q, q_r);
  }

  #[test]
  fn complex_optional_4() {
    let entity: Option<i32> = None;
    let state = Some("Pending");
    let currency_iso: Option<String> = None;
    let q = query!(
      (
        ("deleted" == false && cond(entity.is_some()) ("source.id" == { entity.clone().unwrap() } || "destination.id" == { entity.clone().unwrap() } )) &&
        cond(state.is_some()) "state" == { state.unwrap() }
      ) && cond(currency_iso.is_some()) ("source.currency_iso" == { currency_iso.clone().unwrap() } || ("destination.currency_iso" == { currency_iso.clone().unwrap() }))
    );
    let q_r = Query::And {
      left: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() }),
      right: Box::new(Query::Eq { field: "state".to_owned(), value: "Pending".into() }),
    };
    assert_eq!(q, q_r);
  }

  #[test]
  fn complex_optional_5() {
    let entity: Option<i32> = None;
    let state: Option<String> = None;
    let currency_iso: Option<String> = None;
    let q = query!(
      (
        ("deleted" == false && cond(entity.is_some()) ("source.id" == { entity.clone().unwrap() } || "destination.id" == { entity.clone().unwrap() } )) &&
        cond(state.is_some()) "state" == { state.unwrap() }
      ) && cond(currency_iso.is_some()) ("source.currency_iso" == { currency_iso.clone().unwrap() } || ("destination.currency_iso" == { currency_iso.clone().unwrap() }))
    );
    let q_r = Query::Eq { field: "deleted".to_owned(), value: false.into() };
    assert_eq!(q, q_r);
  }

  #[test]
  fn complex_optional_6() {
    let entity: Option<i32> = Some(12);
    let state: Option<String> = None;
    let currency_iso: Option<String> = None;
    let q = query!(
      (
        ("deleted" == false && cond(entity.is_some()) ("source.id" == { entity.clone().unwrap() } || "destination.id" == { entity.clone().unwrap() } )) &&
        cond(state.is_some()) "state" == { state.unwrap() }
      ) && cond(currency_iso.is_some()) ("source.currency_iso" == { currency_iso.clone().unwrap() } || ("destination.currency_iso" == { currency_iso.clone().unwrap() }))
    );
    let q_r = Query::And {
      left: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() }),
      right: Box::new(Query::Or {
        left: Box::new(Query::Eq { field: "source.id".to_owned(), value: 12.into() }),
        right: Box::new(Query::Eq { field: "destination.id".to_owned(), value: 12.into() }),
      })
    };
    assert_eq!(q, q_r);
  }

  #[test]
  fn complex_optional_7() {
    let entity: Option<i32> = None;
    let state: Option<String> = None;
    let currency_iso: Option<&str> = Some("USD");
    let q = query!(
      (
        ("deleted" == false && cond(entity.is_some()) ("source.id" == { entity.clone().unwrap() } || "destination.id" == { entity.clone().unwrap() } )) &&
        cond(state.is_some()) "state" == { state.unwrap() }
      ) && cond(currency_iso.is_some()) ("source.currency_iso" == { currency_iso.clone().unwrap() } || ("destination.currency_iso" == { currency_iso.clone().unwrap() }))
    );
    let q_r = Query::And {
      left: Box::new(Query::Eq { field: "deleted".to_owned(), value: false.into() }),
      right: Box::new(Query::Or {
        left: Box::new(Query::Eq { field: "source.currency_iso".to_owned(), value: "USD".to_owned().into() }),
        right: Box::new(Query::Eq { field: "destination.currency_iso".to_owned(), value: "USD".to_owned().into() }),
      })
    };
    assert_eq!(q, q_r);
  }
}