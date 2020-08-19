use concept_learning::algo::*;
use concept_learning::parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let input = "/home/deep/work/rust/concept_learning/data/input.csv";
  let data = parser::parse(input)?;
  let hypothesis = find_s(data);
  println!("FIND-S algorithm says: {}", hypothesis);
  Ok(())
}