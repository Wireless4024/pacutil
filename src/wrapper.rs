use serde_json::{json, Value};

use crate::{list_all, list_installed, right_join};
use crate::util::{Array, group, obj_match};

pub mod repo;
pub mod info;
pub mod install;

pub fn list_installed_with_detail() -> Vec<Array<Value>> {
	let mut right = list_all().unwrap().into_iter().map(|it| it.to_object()).collect::<Vec<_>>();
	let left = list_installed().unwrap();
	right_join(&left, &mut right, "name");
	let packages = right.into_iter().filter(|it| obj_match(&json!({"installed":"*"}), it)).collect();
	group(packages, "name")
}