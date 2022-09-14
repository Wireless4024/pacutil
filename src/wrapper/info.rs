use std::collections::HashMap;
use std::io::Read;
use std::process::{Command, Stdio};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::db::Repository;

#[derive(Debug)]
pub struct PackageInfo<'a> {
	pub name: &'a str,
	pub installed: &'a str,
	pub architecture: &'a str,
	pub url: &'a str,
	pub as_dependency: bool,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct InstalledPackage {
	pub name: String,
	pub installed: String,
	pub architecture: String,
	pub url: String,
	pub packager: String,
	pub as_dependency: i8, // bool but unsupported by serde_json
}

// impl<'a> PackageInfo<'a> {
// 	pub fn to_object(self) -> Value {
// 		//	"packager": self.packager.to_string(),
// 		json!({
// 			"name": self.name.to_string(),
// 			"installed": self.installed.to_string(),
// 			"architecture": self.architecture.to_string(),
// 			"url": self.url.to_string(),
// 			"as_dependency": self.as_dependency
// 		})
// 	}
// }

pub fn list_installed(repo: &Repository<InstalledPackage>) -> Result<u64> {
	info!("Executing `pacman -Qi`");
	let mut cmd = Command::new("pacman")
		.arg("-Qi")
		.stdout(Stdio::piped())
		.stdin(Stdio::null())
		.stderr(Stdio::null())
		.spawn()?;
	let mut data = String::new();
	cmd.stdout.take().unwrap().read_to_string(&mut data)?;
	Ok(parse(&data, repo))
}

fn parse(str: &str, repo: &Repository<InstalledPackage>) -> u64 {
	let mut packages_count = 0;
	let packages = str.split("\n\n");
	for package in packages {
		let lines = package.split('\n');
		let mut map = HashMap::new();
		for line in lines {
			let mut l = line.splitn(2, ':');
			if let (Some(k), Some(v)) = (l.next(), l.next()) {
				map.insert(k.trim(), v.trim());
			}
		}
		if let Some(name) = map.get("Name") {
			repo.add(InstalledPackage {
				name: name.to_string(),
				installed: map.get("Version").unwrap().to_string(),
				architecture: map.get("Architecture").unwrap().to_string(),
				url: map.get("URL").unwrap().to_string(),
				packager: map.get("Packager").unwrap().to_string(),
				as_dependency: map.get("Install Reason").map(|it| it.contains("as a dependency")).unwrap_or_default() as i8,
			});
			packages_count += 1;
		}
	}
	info!("Found {} installed package", packages_count);
	packages_count
}