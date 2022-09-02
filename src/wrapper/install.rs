use std::process::{Command, Stdio};

use anyhow::Result;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct InstallablePackage {
	repo: String,
	name: String,
	as_dependency: bool,
}

pub fn install_pkg(pkg: &InstallablePackage) -> Result<()> {
	let mut cmd = Vec::new();
	let mut pkg_name = String::new();
	cmd.push(String::from("-S"));
	cmd.push(String::from("--noconfirm"));
	pkg_name.push_str(&pkg.repo);
	pkg_name.push('/');
	pkg_name.push_str(&pkg.name);
	cmd.push(pkg_name);
	if pkg.as_dependency {
		cmd.push(String::from("--asdeps"));
	}
	info!("Installing {}/{}",pkg.repo,pkg.name);
	let mut child = Command::new("pacman").args(&cmd).stdout(Stdio::null()).stdin(Stdio::null()).spawn()?;
	child.wait()?;
	Ok(())
}