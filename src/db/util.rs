use rusqlite::{Row, Rows};
use rusqlite::types::{Value as SqlValue, ValueRef};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Map, Number, to_value, Value};

#[derive(Deserialize, Debug)]
pub struct QueryFilter {
	#[serde(default = "default_value", rename = "$ne")]
	ne: Option<Value>,

	#[serde(default = "default_value", rename = "$lt")]
	lt: Option<Value>,

	#[serde(default = "default_value", rename = "$lte")]
	lte: Option<Value>,

	#[serde(default = "default_value", rename = "$eq")]
	eq: Option<Value>,

	#[serde(default = "default_value", rename = "$gte")]
	gte: Option<Value>,

	#[serde(default = "default_value", rename = "$gt")]
	gt: Option<Value>,

	#[serde(default, rename = "$in")]
	include: Option<Vec<Value>>,

	#[serde(default, rename = "$nin")]
	not_include: Option<Vec<Value>>,
}

fn default_value() -> Option<Value> {
	Some(Value::Null)
}

fn value_to_sql(value: Value) -> SqlValue {
	match value {
		Value::Null => {
			SqlValue::Null
		}
		Value::Bool(b) => {
			SqlValue::from(b)
		}
		Value::Number(n) => {
			if n.is_f64() {
				SqlValue::from(n.as_f64())
			} else {
				SqlValue::from(n.as_i64())
			}
		}
		Value::String(s) => {
			SqlValue::from(s)
		}
		_ => unreachable!()
	}
}

#[macro_export]
macro_rules! try_and {
    ($sql:ident) => {
	    if !$sql.is_empty() { $sql.push_str(" AND "); }
    };
}

macro_rules! try_concat {
    ($sql:ident, $param:ident, $field:ident, $value:expr, $e:literal,$param_name:literal) => {
	    if let Some(inner) = $value {
		    if !inner.is_null() {
			    try_and!($sql);
				$sql.push_str($field);
				$sql.push(' ');
				$sql.push_str($e);
				$sql.push(' ');
				$sql.push(':');
			    let mut param_name = String::new();
			    param_name.push_str($field);
				param_name.push_str($param_name);
			    $sql.push_str(&param_name);
				$param.push((param_name, value_to_sql(inner.clone())));
			}
	    } else {
			$sql.push_str($field);
			$sql.push_str(" IS NOT NULL");
		}
    };
}

impl QueryFilter {
	pub fn from_json(json: Value) -> Self {
		from_value(json).unwrap()
	}

	pub fn to_sql(&self, field: &str) -> (String, Vec<(String, SqlValue)>) {
		let mut sql = String::new();
		let mut param = Vec::new();
		if let Some(inner) = &self.ne {
			if !inner.is_null() {
				let mut param_name = String::new();
				param_name.push_str(field);
				param_name.push_str("_ne");
				sql.push_str(&param_name);
				sql.push_str("!=");
				param_name.insert(0, ':');
				param.push((param_name, value_to_sql(inner.clone())));
			}
		} else {
			sql.push_str(field);
			sql.push_str(" IS NOT NULL");
		}

		try_concat!(sql,param,field,{&self.lt},"<","_lt");
		try_concat!(sql,param,field,{&self.lte},"<=","_lte");
		try_concat!(sql,param,field,{&self.eq},"=","_eq");
		try_concat!(sql,param,field,{&self.gte},">=","_gte");
		try_concat!(sql,param,field,{&self.gt},">","_gt");

		(sql, param)
	}
}

pub fn from_rows(mut rows: Rows) -> Value {
	let mut arr = Vec::new();
	while let Ok(Some(row)) = rows.next() {
		arr.push(from_row(row));
	}
	Value::Array(arr)
}

pub fn from_row(row: &Row<'_>) -> Value {
	let mut map = Map::new();
	for x in row.as_ref().column_names() {
		map.insert(x.to_string(), match row.get_ref(x).unwrap() {
			ValueRef::Null => {
				Value::Null
			}
			ValueRef::Integer(i) => {
				Value::Number(Number::from(i))
			}
			ValueRef::Real(i) => {
				Value::Number(Number::from_f64(i).unwrap())
			}
			ValueRef::Text(t) => {
				Value::String(String::from_utf8_lossy(t).to_string())
			}
			ValueRef::Blob(blob) => {
				Value::Array(blob.iter().map(|&it| Value::Number(Number::from(it))).collect())
			}
		});
	}
	Value::Object(map)
}

pub fn to_named_param<T: Serialize>(val: &T) -> Vec<(String, SqlValue)> {
	json_to_named_param(to_value(val).expect("Convert value to json"))
}

pub fn json_to_named_param(json: Value) -> Vec<(String, SqlValue)> {
	let mut pairs = Vec::new();
	if let Value::Object(obj) = json {
		for (mut key, value) in obj {
			key.insert(0, ':');
			pairs.push((key, value_to_sql(value)));
		}
	}
	pairs
}