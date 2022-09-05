use anyhow::Result;
use serde::Deserialize;
use tracing::info;

use crate::wrapper::pacman;

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
	let mut child = pacman(&cmd)?;
	child.wait()?;
	Ok(())
}

pub fn install_pkgs(pkgs: &[impl AsRef<InstallablePackage>]) -> Result<()> {
	let mut deps = pkgs.iter()
		.map(|it| it.as_ref())
		.filter(|it| it.as_dependency)
		.map(|p| format!("{}/{}", p.repo, p.name))
		.collect::<Vec<_>>();
	let mut explicit = pkgs.iter()
		.map(|it| it.as_ref())
		.filter(|it| !it.as_dependency)
		.map(|p| format!("{}/{}", p.repo, p.name))
		.collect::<Vec<_>>();

	let (deps_len, exp_len) = (deps.len(), explicit.len());

	// TODO: redirect stdout to file
	let mut exps_cmd = Vec::new();
	exps_cmd.push(String::from("-S"));
	exps_cmd.push(String::from("--noconfirm"));
	exps_cmd.extend(explicit);

	info!("Installing {} packages",exp_len);
	let mut child = pacman(&exps_cmd)?;
	child.wait()?;

	let mut deps_cmd = Vec::new();
	deps_cmd.push(String::from("-S"));
	deps_cmd.push(String::from("--noconfirm"));
	deps_cmd.extend(deps);
	deps_cmd.push(String::from("--asdeps"));

	info!("Installing {} packages as dependency",deps_len);
	let mut child = pacman(&deps_cmd)?;
	child.wait()?;
	Ok(())
}