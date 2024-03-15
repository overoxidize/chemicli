use std::collections::HashMap;
use crate::commands::element::{Element};
use serde_json::{Result, Value};



pub fn create_periodic_table() -> serde_json::Value {
  let file = std::fs::read_to_string("./data/periodic_table.json").expect("Was not able to open .json file.");
  let table = serde_json::from_str::<Value>(&file).expect("Unable to read value.");
  table
}