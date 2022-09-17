use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use clap::Parser;

use crate::cli::CommandLine;
use crate::db::db_init;
use crate::util::{group, parse_json, split};
use crate::wrapper::info::{InstalledPackage, list_installed};
use crate::wrapper::repo::{list_to_db, Package};

mod wrapper;
mod util;
mod db;
mod ser;
mod cli;

fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt::init();
	let arg = CommandLine::parse();

	//exec().expect("1");
	/*for x in list().unwrap().into_iter().map(|it| it.to_object()) {
		println!("{:?}", x);
	}*/
//	list_to_db(&repo)?;
	//println!("{:?}", list_installed(&repo)?);
	//println!("{:?}", repo.all());

	//println!("{:?}", repo.all());
	/*
		for x in list_installed_with_detail() {
			if let Multiple(arr) = x {
				if arr.iter().any(|it| obj_match(&json!({"repo":"cachyos*"}), it)) {
					let pkg = arr.into_iter().find(|it| !obj_match(&json!({"repo":"cachyos*"}), it))
						.and_then(|it| from_value::<InstallablePackage>(it).ok());
					if let Some(pkg) = pkg {
						install_pkg(&pkg)?;
					}
				}
			}
		}*/
	match arg {
		CommandLine::RemoveAll { filter } => {
			remove_all(filter)
		}
	}
}

fn remove_all(filter: String) -> anyhow::Result<()> {
	let db = db_init()?;

	let repo = db.get_repository::<Package>();
	list_to_db(&repo)?;
	let json = parse_json(&filter);
	let packages = repo.find(json);
	println!("{:?}", packages);
	let (matched, available) = split(packages, |it| it.installed.as_ref().map(|i| i == &it.version).unwrap_or_default());
	let mut alternatives = group(available, |it| it.name.clone());

	let installed = db.get_repository::<InstalledPackage>();
	list_installed(&installed)?;
	let installed = group(installed.take_all(), |it| it.name.clone()).into_iter().map(|it| (it.0, it.1.into_iter().next().unwrap()));
	let installed: HashMap<_, _, RandomState> = HashMap::from_iter(installed);
	println!("{:?}", installed);
todo!("Replace package from matched with another");
	Ok(())
}