use super::data::*;
use std::fs::File;
use std::error::Error;
use csv::Reader;

pub fn parse(filename: &str) -> Result<Vec<Data>, Box<dyn Error>> {
  let file = File::open(filename)?;
  let mut csv_input = Reader::from_reader(file);
  let mut input = Vec::new();
  for record in csv_input.records() {
    let mut data = Data::new();
    let record = record?;
    data.len = record.len() - 1;
    for i in 0..data.len {
      let value = record.get(i).unwrap();
      if value == "1" {
        data.values.push(true);
      } else {
        data.values.push(false);
      }
    }
    let value = record.get(data.len).unwrap();
    if value == "1" {
      data.result = true;
    } else {
      data.result = false;
    }
    input.push(data);
  }
  Ok(input)
}