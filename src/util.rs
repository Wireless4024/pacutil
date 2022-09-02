use std::collections::{HashMap, LinkedList};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

use serde_json::Value;

pub enum Array<T> {
	Single(T),
	Multiple(Vec<T>),
}

impl<T: Debug> Debug for Array<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Array::Single(val) => {
				Debug::fmt(val, f)
			}
			Array::Multiple(val) => {
				Debug::fmt(val, f)
			}
		}
	}
}

pub fn group(arr: Vec<Value>, by: &str) -> Vec<Array<Value>> {
	let mut map: HashMap<OwnedValueWrapper, Vec<Value>> = HashMap::new();

	for x in arr {
		map.entry(OwnedValueWrapper { inner: x.get_json_key(by).clone() })
			.or_default()
			.push(x);
	}
	map.into_values()
		.map(|it| if it.len() == 1 { Array::Single(it.into_iter().next().unwrap()) } else { Array::Multiple(it) })
		.collect()
}

pub trait GetJson where {
	fn get_json_key(&self, key: &str) -> &Value;
}

impl GetJson for Value {
	fn get_json_key(&self, key: &str) -> &Value {
		let mut keys = key.split('.').collect::<LinkedList<&str>>();
		let mut ptr = self;
		while let Some(key) = keys.pop_front() {
			match ptr {
				Value::Null => {
					return &Value::Null;
				}
				Value::Object(map) => {
					if let Some(val) = map.get(key) {
						ptr = val
					} else {
						return &Value::Null;
					}
				}
				_ => {
					return if keys.is_empty() { ptr } else { &Value::Null };
				}
			}
		}
		if keys.is_empty() {
			ptr
		} else {
			&Value::Null
		}
	}
}

#[derive(Eq, PartialEq)]
struct ValueWrapper<'a> {
	inner: &'a Value,
}

#[derive(Eq, PartialEq)]
struct OwnedValueWrapper {
	inner: Value,
}

fn hash_value<H: Hasher>(value: &Value, state: &mut H) {
	match value {
		Value::Null => {}
		Value::Bool(b) => {
			b.hash(state)
		}
		Value::Number(n) => {
			n.hash(state)
		}
		Value::String(s) => {
			s.hash(state)
		}
		Value::Array(arr) => {
			for v in arr {
				hash_value(v, state)
			}
		}
		Value::Object(map) => {
			for (k, v) in map {
				k.hash(state);
				hash_value(v, state);
			}
		}
	}
}

impl<'a> Hash for ValueWrapper<'a> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		hash_value(self.inner, state)
	}
}

impl Hash for OwnedValueWrapper {
	fn hash<H: Hasher>(&self, state: &mut H) {
		hash_value(&self.inner, state)
	}
}

pub fn right_join(left: &[Value], right: &mut Vec<Value>, on: &str) {
	let mut left_map = HashMap::new();
	for inner in left {
		left_map.insert(ValueWrapper { inner: inner.get_json_key(on) }, inner);
	}
	for target in right {
		if !target.is_object() {
			continue;
		}
		let left = {
			let right_key = target.get_json_key(on);
			let value_wrapper = ValueWrapper { inner: right_key };
			if let Some(left) = left_map.get(&value_wrapper) { // fail if use remove
				if !left.is_object() { continue; }
				left.as_object().map(|it| it.clone())
			} else {
				None
			}
		};
		if let Some(left) = left {
			let right = target.as_object_mut().unwrap();
			for (k, v) in left {
				right.insert(k, v);
			}
		}
	}
}
pub fn obj_match(left: &Value, right: &Value) -> bool {
	match left {
		Value::Null => {
			right.is_null()
		}
		Value::Bool(b1) => {
			if let Value::Bool(b2) = right {
				b1 == b2
			} else {
				false
			}
		}
		Value::Number(n1) => {
			if let Value::Number(n2) = right {
				n1 == n2
			} else {
				false
			}
		}
		Value::String(s1) => {
			if let Value::String(s2) = right {
				wildmatch::WildMatch::new(s1).matches(s2)
			} else {
				false
			}
		}
		Value::Array(arr) => {
			arr.iter().fold(false, |a, b| a || obj_match(b, right))
		}
		Value::Object(map) => {
			for (k, v1) in map {
				if let Some(v2) = right.get(k) {
					if !obj_match(v1, v2) {
						return false;
					}
				} else {
					// right don't have this key
					return false;
				}
			}
			return true;
		}
	}
}