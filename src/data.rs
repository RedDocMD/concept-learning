use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

pub enum Value {
  NONE,
  ANY,
  TRUE,
  FALSE,
}

pub struct Hypothesis {
  pub len: usize,
  pub values: Vec<Value>,
}

pub struct Data {
  pub len: usize,
  pub values: Vec<bool>,
  pub result: bool,
}

impl Data {
  pub fn new() -> Self {
    Data { len: 0, values: Vec::new(), result: false }
  }
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match self.cmp(other) {
      Ordering::Equal => true,
      _ => false,
    }
  }
}

impl Eq for Value {}

impl PartialOrd for Value {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

// More general => Greater
impl Ord for Value {
  fn cmp(&self, other: &Self) -> Ordering {
    match self {
      Value::ANY => match other {
        Value::ANY => Ordering::Equal,
        _ => Ordering::Greater,
      },
      Value::FALSE | Value::TRUE => match other {
        Value::ANY => Ordering::Less,
        Value::FALSE | Value::TRUE => Ordering::Equal,
        Value::NONE => Ordering::Greater,
      },
      Value::NONE => match other {
        Value::NONE => Ordering::Equal,
        _ => Ordering::Less,
      },
    }
  }
}

impl Hypothesis {
  pub fn most_general(len: usize) -> Self {
    let mut values = Vec::new();
    for _ in 0..len {
      values.push(Value::ANY);
    }
    Hypothesis { len, values }
  }

  pub fn most_specific(len: usize) -> Self {
    let mut values = Vec::new();
    for _ in 0..len {
      values.push(Value::NONE);
    }
    Hypothesis { len, values }
  }

  pub fn is_more_general_or_equal(&self, other: &Self) -> Option<bool> {
    if self.len != other.len {
      return None;
    }
    for i in 0..self.len {
      if self.values[i] < other.values[i] {
        return Some(false);
      }
    }
    return Some(true);
  }
}

impl Display for Hypothesis {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let mut output = String::from("[");
    for i in 0..self.len {
      let character = match self.values[i] {
        Value::NONE => '\u{3D5}',
        Value::TRUE => 'T',
        Value::FALSE => 'F',
        Value::ANY => '?'
      };
      output.push(character);
      if i < self.len - 1 {
        output.push_str(", ");
      } else {
        output.push(']');
      }
    }
    write!(f, "{}", output)
  }
}
