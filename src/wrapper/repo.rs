use std::io::{BufRead, BufReader};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::info;

use crate::wrapper::pacman;

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
	repo: String,
	name: String,
	version: String,
	installed: Option<String>,
}

impl Default for Package {
	fn default() -> Self {
		Self {
			repo: Default::default(),
			name: Default::default(),
			version: Default::default(),
			installed: Some(Default::default()),
		}
	}
}

impl Package {
	pub fn cvt_object(self) -> Value {
		let installed = self.installed.map(Value::String).unwrap_or_default();
		json!({
			"repo": self.repo,
			"name": self.name,
			"version": self.version,
			"installed": installed,
		})
	}
}

pub fn list_all() -> Result<Vec<Package>> {
	info!("Running `pacman -Sl`");
	let mut cmd = pacman(&"-Sl")?;
	let mut stdout = BufReader::new(cmd.stdout.take().unwrap());
	let mut data = Vec::new();
	let mut buf = String::new();
	while let Ok(n) = stdout.read_line(&mut buf) {
		if n == 0 { break; }
		let mut info = buf.splitn(4, ' ');
		if let (Some(repo), Some(name), Some(version), installed) = (info.next(), info.next(), info.next(), info.next()) {
			let version = version.trim().to_string();
			let installed = installed.and_then(|it| {
				if it.starts_with("[installed") {
					if it.as_bytes()[10] == b']' {
						Some(version.clone())
					} else {
						let mut ver = it.splitn(2, ' ');
						ver.next();
						let ver = ver.next().unwrap();
						Some(ver[..ver.len() - 2].to_string())
					}
				} else {
					None
				}
			});
			data.push(Package {
				repo: repo.to_string(),
				name: name.to_string(),
				version,
				installed,
			});
		}

		buf = String::new();
	};
	info!("Found {} available packages", data.len());
	Ok(data)
}