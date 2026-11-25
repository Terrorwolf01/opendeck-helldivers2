use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Deserialize, Debug)]
pub struct Stratagem {
	pub id: String,
	pub title: String,
	// Path inside the plugin folder, without ".png" - same format as the manifest's "Icon".
	pub icon: String,
	pub seq: Vec<Direction>,
}

// Same file the PI imports, so the combobox and the plugin can never disagree about ids.
fn all() -> &'static HashMap<String, Vec<Stratagem>> {
	static DATA: OnceLock<HashMap<String, Vec<Stratagem>>> = OnceLock::new();
	DATA.get_or_init(|| {
		serde_json::from_str(include_str!("../pi/src/lib/stratagems.json"))
			.expect("pi/src/lib/stratagems.json is invalid")
	})
}

pub fn find(group: &str, id: &str) -> Option<&'static Stratagem> {
	all().get(group)?.iter().find(|s| s.id == id)
}

#[cfg(test)]
mod tests {
	#[test]
	fn loads() {
		let s = super::find("General", "GENERAL_Reinforce").unwrap();
		assert_eq!(s.seq.len(), 5);
		for s in super::all().values().flatten() {
			assert!(!s.seq.is_empty(), "{} has no input sequence", s.id);
			assert!(!s.icon.is_empty(), "{} has no icon", s.id);
		}
	}

	// Every stratagem's icon must exist in assets/, or its key shows no image.
	#[test]
	fn icons_exist() {
		let assets = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
		for s in super::all().values().flatten() {
			let path = assets.join(format!("{}.png", s.icon));
			assert!(path.is_file(), "{}: missing icon {}", s.id, path.display());
		}
	}

	#[test]
	fn unknown_ids_are_not_found() {
		assert!(super::find("General", "DOES_NOT_EXIST").is_none());
		assert!(super::find("NoSuchGroup", "GENERAL_Reinforce").is_none());
	}
}
