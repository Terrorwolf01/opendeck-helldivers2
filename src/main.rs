mod actions;
mod shared;
mod stratagems;

use crate::actions::*;
use log::*;
use openaction::*;
use rdev::{EventType, SimulateError, simulate};
use serde::{Deserialize, Serialize};
use shared::logger;
use std::sync::OnceLock;
use std::{
	thread,
	time::{self},
};
use tokio::sync::RwLock;

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct GlobalSettings {
	pub modifier: Option<String>,
	pub up: Option<String>,
	pub down: Option<String>,
	pub left: Option<String>,
	pub right: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct ActionSettings {
	pub stratagem: Option<String>,
	// Delay in milliseconds after each simulated key event. If not set, then DEFAULT_DELAY_MS
	pub delay: Option<u64>,
}

const DEFAULT_DELAY_MS: u64 = 20;

impl ActionSettings {
	pub fn delay(&self) -> time::Duration {
		time::Duration::from_millis(self.delay.unwrap_or(DEFAULT_DELAY_MS))
	}
}

struct GlobalSettingsHandler;
#[async_trait]
impl global_events::GlobalEventHandler for GlobalSettingsHandler {
	async fn plugin_ready(&self) -> OpenActionResult<()> {
		if let Err(e) = get_global_settings().await {
			warn!("Failed to request global settings: {e:?}");
		}
		Ok(())
	}
	async fn did_receive_global_settings(
		&self,
		event: global_events::DidReceiveGlobalSettingsEvent,
	) -> OpenActionResult<()> {
		info!("Globale Settings did_receive_global_settings: {:?}", event);
		let settings: GlobalSettings =
			serde_json::from_value(event.payload.settings).unwrap_or_default();
		info!("Global Settings deserialized: {:?}", settings);
		*current_global_settings().write().await = settings;
		Ok(())
	}
}

pub fn current_global_settings() -> &'static RwLock<GlobalSettings> {
	static SETTINGS: OnceLock<RwLock<GlobalSettings>> = OnceLock::new();
	SETTINGS.get_or_init(|| RwLock::new(GlobalSettings::default()))
}

// OpenDeck keeps every action's property inspector iframe alive in the background once opened,
// only toggling which one is visible - it doesn't reload them, and it doesn't forward a
// `setGlobalSettings` made from one PI to any other already-open PI. So a PI that was opened
// earlier (or switched away from) can be left showing a stale snapshot from whenever it first
// connected. `property_inspector_did_appear` fires on the plugin side exactly when a PI becomes
// the focused one again, which is the hook to push it a fresh one. The PI's
// `simple/+page.svelte` listens for this same "globalSettingsSnapshot" event over
// `sendToPropertyInspector` and applies it to its `globalSettings` store.
pub async fn push_global_settings(instance: &Instance) -> OpenActionResult<()> {
	let gs = current_global_settings().read().await;
	instance
		.send_to_property_inspector(serde_json::json!({
			"event": "globalSettingsSnapshot",
			"settings": &*gs,
		}))
		.await
}

fn up(choice: &Option<String>, delay: time::Duration) {
	//info!("up1 {:?}", choice);
	let choice_str = match choice {
		Some(s) => format!("\"{}\"", s),
		None => "\"UpArrow\"".to_string(),
	};
	pressrelease(choice_str, delay);
}

fn down(choice: &Option<String>, delay: time::Duration) {
	let choice_str = match choice {
		Some(s) => format!("\"{}\"", s),
		None => "\"DownArrow\"".to_string(),
	};
	pressrelease(choice_str, delay);
}

fn left(choice: &Option<String>, delay: time::Duration) {
	let choice_str = match choice {
		Some(s) => format!("\"{}\"", s),
		None => "\"LeftArrow\"".to_string(),
	};
	pressrelease(choice_str, delay);
}

fn right(choice: &Option<String>, delay: time::Duration) {
	let choice_str = match choice {
		Some(s) => format!("\"{}\"", s),
		None => "\"RightArrow\"".to_string(),
	};
	pressrelease(choice_str, delay);
}

fn modifier_press(choice: &Option<String>, delay: time::Duration) {
	let choice_str = match choice {
		Some(s) => format!("\"{}\"", s),
		None => "\"ControlLeft\"".to_string(),
	};
	match serde_json::from_str(&choice_str) {
		Ok(key) => {
			send(&EventType::KeyPress(key), delay);
		}
		Err(error) => {
			eprintln!("Failed to deserialize: {}", error);
			return;
		}
	}
}

fn modifier_release(choice: &Option<String>, delay: time::Duration) {
	let choice_str = match choice {
		Some(s) => format!("\"{}\"", s),
		None => "\"ControlLeft\"".to_string(),
	};
	match serde_json::from_str(&choice_str) {
		Ok(key) => {
			send(&EventType::KeyRelease(key), delay);
		}
		Err(error) => {
			eprintln!("Failed to deserialize: {}", error);
			return;
		}
	}
}

fn pressrelease(choice_str: String, delay: time::Duration) {
	match serde_json::from_str(&choice_str) {
		Ok(key) => {
			send(&EventType::KeyPress(key), delay);
			send(&EventType::KeyRelease(key), delay);
		}
		Err(error) => {
			eprintln!("Failed to deserialize: {}", error);
			return;
		}
	}
}

#[tokio::main]
async fn main() -> OpenActionResult<()> {
	logger::init();
	global_events::set_global_event_handler(&GlobalSettingsHandler);

	register_action(Bridge).await;
	register_action(ChemicalAgent).await;
	register_action(EngineeringBay).await;
	register_action(General).await;
	register_action(Hangar).await;
	register_action(OrbitalCannons).await;
	register_action(PatrioticAdministrationCenter).await;
	register_action(RoboticWorkshop).await;
	register_action(UrbanLegends).await;
	register_action(ServantsofFreedom).await;
	register_action(BorderlineJustice).await;
	register_action(MastersOfCeremony).await;
	register_action(ForceOfLaw).await;
	register_action(ControlGroup).await;
	register_action(DustDevils).await;
	register_action(PythonCommandos).await;
	register_action(RedactedRegiment).await;
	register_action(SiegeBreakers).await;
	register_action(EntrenchedDivision).await;
	register_action(ExoExperts).await;
	register_action(CastellansCreed).await;

	run(std::env::args().collect()).await
}

fn send(event_type: &EventType, delay: time::Duration) {
	match simulate(event_type) {
		Ok(()) => (),
		Err(SimulateError) => {
			println!("We could not send {:?}", event_type);
		}
	}
	// Let the OS catch up (at least macOS)
	thread::sleep(delay);
}
