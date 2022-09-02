use serde_json::{from_value, json};

use crate::util::{obj_match, right_join};
use crate::util::Array::Multiple;
use crate::wrapper::info::list_installed;
use crate::wrapper::install::{install_pkg, InstallablePackage};
use crate::wrapper::list_installed_with_detail;
use crate::wrapper::repo::list_all;

mod wrapper;
mod util;

fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt::init();
	//exec().expect("1");
	/*for x in list().unwrap().into_iter().map(|it| it.to_object()) {
		println!("{:?}", x);
	}*/

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
	}
	Ok(())
}
