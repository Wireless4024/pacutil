use std::collections::HashMap;
use std::process::exit;

use serde_json::Value;
use tracing::error;

pub use json_manipulator::*;

mod json_manipulator;

pub fn parse_json(input: &str) -> Value {
	if let Ok(value) = serde_json::from_str::<Value>(input) {
		return value;
	}
	error!("Invalid json `{input}`");
	exit(1);
}

pub fn split<T>(arr: Vec<T>, mut filter: impl FnMut(&T) -> bool) -> (Vec<T>, Vec<T>) {
	let mut left = Vec::new();
	let mut right = Vec::new();
	for x in arr.into_iter() {
		if filter(&x) {
			left.push(x);
		} else {
			right.push(x);
		}
	}
	(left, right)
}


pub fn group<T>(arr: Vec<T>, mut by: impl FnMut(&T) -> String) -> HashMap<String, Vec<T>> {
	let mut map: HashMap<String, Vec<T>> = HashMap::new();
	for x in arr {
		let key = by(&x);
		map.entry(key).or_default().push(x);
	}
	map
}