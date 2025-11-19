use crate::stratagems::{self, Direction, Stratagem};
use crate::{
	ActionSettings, current_global_settings, down, left, modifier_press, modifier_release,
	push_global_settings, right, up,
};
use base64::prelude::*;
use log::{info, warn};
use openaction::{Action, ActionUuid, Instance, OpenActionResult, async_trait};

const PLACEHOLDER_ICON: &str = "icon/helldivers2";

fn selected(group: &str, settings: &ActionSettings) -> Option<&'static Stratagem> {
	stratagems::find(group, settings.stratagem.as_deref()?)
}

// OpenDeck wants key images as data URLs. The icons ship in the plugin folder, which is where
// build.sh also puts the binary.
fn icon_data_url(icon: &str) -> Option<String> {
	let path = std::env::current_exe()
		.ok()?
		.parent()?
		.join(format!("{icon}.png"));
	match std::fs::read(&path) {
		Ok(bytes) => Some(format!(
			"data:image/png;base64,{}",
			BASE64_STANDARD.encode(bytes)
		)),
		Err(error) => {
			warn!("Could not read {}: {}", path.display(), error);
			None
		}
	}
}

// Show the chosen stratagem on the key; with nothing chosen, show the Helldivers 2 logo.
async fn show_selected(
	instance: &Instance,
	group: &str,
	settings: &ActionSettings,
) -> OpenActionResult<()> {
	match selected(group, settings) {
		Some(s) => {
			instance.set_title(Some(s.title.as_str()), None).await?;
			instance.set_image(icon_data_url(&s.icon), None).await
		}
		None => {
			instance.set_title(None::<String>, None).await?;
			instance
				.set_image(icon_data_url(PLACEHOLDER_ICON), None)
				.await
		}
	}
}

async fn run_selected(group: &str, settings: &ActionSettings) {
	// Nothing selected yet: the key shows the placeholder and a press does nothing.
	let Some(s) = selected(group, settings) else {
		info!("{group}: no stratagem selected, ignoring key press");
		return;
	};
	let gs = current_global_settings().read().await;
	let delay = settings.delay();
	modifier_press(&gs.modifier, delay);
	for direction in &s.seq {
		match direction {
			Direction::Up => up(&gs.up, delay),
			Direction::Down => down(&gs.down, delay),
			Direction::Left => left(&gs.left, delay),
			Direction::Right => right(&gs.right, delay),
		}
	}
	modifier_release(&gs.modifier, delay);
	info!("{} pressed", s.id);
}

// One action per stratagem group. `$group` is both the UUID suffix and the key in
// stratagems.json.
macro_rules! group_action {
	($name:ident, $group:literal) => {
		pub struct $name;

		#[async_trait]
		impl Action for $name {
			const UUID: ActionUuid = concat!("at.terrorwolf.helldivers.", $group);
			type Settings = ActionSettings;

			async fn will_appear(
				&self,
				instance: &Instance,
				settings: &Self::Settings,
			) -> OpenActionResult<()> {
				show_selected(instance, $group, settings).await
			}

			async fn did_receive_settings(
				&self,
				instance: &Instance,
				settings: &Self::Settings,
			) -> OpenActionResult<()> {
				show_selected(instance, $group, settings).await
			}

			async fn property_inspector_did_appear(
				&self,
				instance: &Instance,
				_settings: &Self::Settings,
			) -> OpenActionResult<()> {
				push_global_settings(instance).await
			}

			async fn key_down(
				&self,
				_instance: &Instance,
				settings: &Self::Settings,
			) -> OpenActionResult<()> {
				run_selected($group, settings).await;
				Ok(())
			}
		}
	};
}

group_action!(Bridge, "Bridge");
group_action!(ChemicalAgent, "ChemicalAgent");
group_action!(EngineeringBay, "EngineeringBay");
group_action!(General, "General");
group_action!(Hangar, "Hangar");
group_action!(OrbitalCannons, "OrbitalCannons");
group_action!(
	PatrioticAdministrationCenter,
	"PatrioticAdministrationCenter"
);
group_action!(RoboticWorkshop, "RoboticWorkshop");
group_action!(UrbanLegends, "UrbanLegends");
group_action!(ServantsofFreedom, "ServantsofFreedom");
group_action!(BorderlineJustice, "BorderlineJustice");
group_action!(MastersOfCeremony, "MastersOfCeremony");
group_action!(ForceOfLaw, "ForceOfLaw");
group_action!(ControlGroup, "ControlGroup");
group_action!(DustDevils, "DustDevils");
group_action!(PythonCommandos, "PythonCommandos");
group_action!(RedactedRegiment, "RedactedRegiment");
group_action!(SiegeBreakers, "SiegeBreakers");
group_action!(EntrenchedDivision, "EntrenchedDivision");
group_action!(ExoExperts, "ExoExperts");
group_action!(CastellansCreed, "CastellansCreed");
