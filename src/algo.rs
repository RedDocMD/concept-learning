use super::data::*;

pub fn find_s(entries: Vec<Data>) -> Hypothesis{
  let len = entries[0].len;
  let mut hypothesis = Hypothesis::most_specific(len);
  for entry in entries {
    if entry.result {
      for i in 0..entry.len {
        if entry.values[i] {
          let next = match hypothesis.values[i] {
            Value::NONE | Value::TRUE => Value::TRUE,
            _ => Value::ANY
          };
          hypothesis.values[i] = next;
        } else {
          let next = match hypothesis.values[i] {
            Value::NONE | Value::FALSE => Value::FALSE,
            _ => Value::ANY
          };
          hypothesis.values[i] = next;
        }
      }
    }
  }
  return hypothesis;
}