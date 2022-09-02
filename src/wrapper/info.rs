use std::collections::HashMap;
use std::io::Read;
use std::process::{Command, Stdio};

use anyhow::Result;
use serde_json::{json, Value};
use tracing::info;

#[derive(Debug)]
pub struct PackageInfo<'a> {
	pub name: &'a str,
	pub installed: &'a str,
	pub architecture: &'a str,
	pub url: &'a str,
	//pub packager: &'a str,
	pub as_dependency: bool,
}

impl<'a> PackageInfo<'a> {
	pub fn to_object(self) -> Value {
		//	"packager": self.packager.to_string(),
		json!({
			"name": self.name.to_string(),
			"installed": self.installed.to_string(),
			"architecture": self.architecture.to_string(),
			"url": self.url.to_string(),
			"as_dependency": self.as_dependency
		})
	}
}

pub fn list_installed() -> Result<Vec<Value>> {
	info!("Executing `pacman -Qi`");
	let mut cmd = Command::new("pacman")
		.arg("-Qi")
		.stdout(Stdio::piped())
		.stdin(Stdio::null())
		.stderr(Stdio::null())
		.spawn()?;
	let mut data = String::new();
	cmd.stdout.take().unwrap().read_to_string(&mut data)?;
	Ok(parse(&data).into_iter().map(|it| it.to_object()).collect())
}

fn parse(str: &str) -> Vec<PackageInfo> {
	let mut res = Vec::new();
	let mut packages = str.split("\n\n");
	while let Some(package) = packages.next() {
		let mut lines = package.split("\n");
		let mut map = HashMap::new();
		while let Some(line) = lines.next() {
			let mut l = line.splitn(2, ":");
			if let (Some(k), Some(v)) = (l.next(), l.next()) {
				map.insert(k.trim(), v.trim());
			}
		}
		if let Some(name) = map.get("Name") {
			res.push(PackageInfo {
				name,
				installed: map.get("Version").unwrap(),
				architecture: map.get("Architecture").unwrap(),
				url: map.get("URL").unwrap(),
				//packager: map.get("Packager").unwrap(),
				as_dependency: map.get("Install Reason").map(|it| it.contains("as a dependency")).unwrap_or_default(),
			});
		}
	}
	info!("Found {} installed package", res.len());
	res
}