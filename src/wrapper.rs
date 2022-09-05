use std::process::{Child, Command, Stdio};

use anyhow::Result;
use serde_json::{json, Value};

use crate::{list_all, list_installed, right_join};
use crate::util::{Array, group, obj_match};

pub mod repo;
pub mod info;
pub mod install;

pub trait PacmanArg<'a> {
	fn to_args(&'a self) -> Vec<&'a str>;
}

impl<'a> PacmanArg<'a> for &'a str {
	fn to_args(&'a self) -> Vec<&'a str> {
		vec![self]
	}
}

impl<'a> PacmanArg<'a> for &'static [&'a str] {
	fn to_args(&'a self) -> Vec<&'a str> {
		self.to_vec()
	}
}

impl<'a> PacmanArg<'a> for Vec<String> {
	fn to_args(&'a self) -> Vec<&'a str> {
		self.iter().map(|it| it.as_str()).collect()
	}
}

pub fn pacman<'a>(args: &'a impl PacmanArg<'a>) -> Result<Child> {
	Ok(Command::new("pacman")
		.args(args.to_args())
		.stdout(Stdio::piped())
		.stdin(Stdio::null())
		.stderr(Stdio::null())
		.spawn()?)
}

pub fn list_installed_with_detail() -> Vec<Array<Value>> {
	let mut right = list_all().unwrap().into_iter().map(|it| it.cvt_object()).collect::<Vec<_>>();
	let left = list_installed().unwrap();
	right_join(&left, &mut right, "name");
	let packages = right.into_iter().filter(|it| obj_match(&json!({"installed":"*"}), it)).collect();
	group(packages, "name")
}