<script lang="ts">
	import {
		actionInfo,
		actionSettings,
		eventTarget,
		globalSettings,
	} from "@openaction/svelte-pi";
	import StratagemCombobox from "$lib/StratagemCombobox.svelte";
	import { type Stratagem, stratagemGroups } from "$lib/stratagems";

	// The last UUID segment is the group, e.g. "at.terrorwolf.helldivers.Bridge" -> "Bridge".
	$: group = ($actionInfo?.action ?? "").split(".").pop() ?? "";
	$: stratagems = stratagemGroups[group] ?? [];

	function selectStratagem(item: Stratagem) {
		$actionSettings = { ...$actionSettings, stratagem: item.id };
	}

	const DEFAULT_DELAY_MS = 20;

	function updateDelay(event: Event) {
		const raw = (event.target as HTMLInputElement).value.trim();
		const { delay: _, ...rest } = $actionSettings;
		const ms = Math.round(Number(raw));
		$actionSettings =
			raw === "" || !Number.isFinite(ms) || ms < 0
				? rest
				: { ...rest, delay: ms };
	}

	eventTarget.addEventListener("sendToPropertyInspector", ((
		event: CustomEvent,
	) => {
		const payload = event.detail?.payload;
		if (payload?.event === "globalSettingsSnapshot") {
			globalSettings.set(payload.settings ?? {});
		}
	}) as EventListener);

	type KeyField = {
		setting: "modifier" | "up" | "down" | "left" | "right";
		label: string;
		// What up()/down()/left()/right()/modifier_press() in main.rs fall back to when this
		// setting is unset - shown the same way the Collection dropdown shows its own fallback
		// ("Default (Unorganized)") rather than a disabled placeholder.
		default: string;
	};

	const fields: KeyField[] = [
		{ setting: "modifier", label: "Modifier", default: "ControlLeft" },
		{ setting: "left", label: "LeftKey", default: "LeftArrow" },
		{ setting: "right", label: "RightKey", default: "RightArrow" },
		{ setting: "up", label: "UpKey", default: "UpArrow" },
		{ setting: "down", label: "DownKey", default: "DownArrow" },
	];

	const commonKeys = [
		{ value: "ControlLeft", label: "ControlLeft" },
		{ value: "ControlRight", label: "ControlRight" },
		{ value: "Home", label: "Home/Pos1" },
		{ value: "Alt", label: "LeftAlt" },
	];

	const arrowKeys = [
		{ value: "LeftArrow", label: "LeftArrow" },
		{ value: "RightArrow", label: "RightArrow" },
		{ value: "UpArrow", label: "UpArrow" },
		{ value: "DownArrow", label: "DownArrow" },
	];

	const letterKeys = [
		{ value: "KeyA", label: "A" },
		{ value: "KeyS", label: "S" },
		{ value: "KeyW", label: "W" },
		{ value: "KeyD", label: "D" },
	];

	// These are global settings, not per-button settings - every stratagem action reads the same
	// key mapping. Each pick is written straight to $globalSettings on change, the same
	// immediate-apply pattern ConnectionSettings/+page.svelte used for Tags/Collection - there's
	// nothing to gate behind an edit/save toggle when a select's value is always a closed, valid
	// choice already.
	function updateKey(setting: KeyField["setting"], event: Event) {
		$globalSettings = {
			...$globalSettings,
			[setting]: (event.target as HTMLSelectElement).value,
		};
	}
</script>

<h2 class="mt-4 mb-3 text-sm font-semibold text-neutral-100">
	Action Settings
</h2>

<div class="mb-2">
	<StratagemCombobox
		items={stratagems}
		value={$actionSettings.stratagem}
		onselect={selectStratagem}
	/>
</div>

<div class="mb-2 flex items-center gap-2">
	<label for="delay" class="min-w-22.5 text-xs font-medium text-neutral-200"
		>Delay (ms):</label
	>
	<input
		id="delay"
		type="number"
		min="0"
		step="1"
		placeholder="Default ({DEFAULT_DELAY_MS})"
		value={$actionSettings.delay ?? ""}
		on:change={updateDelay}
		class="flex-1 rounded border border-neutral-700 bg-neutral-800 px-2 py-1 text-xs text-neutral-100 focus:border-neutral-600 focus:ring-1 focus:ring-neutral-600 focus:outline-none"
	/>
</div>

<h2 class="mb-3 text-sm font-semibold text-neutral-100">
	Stratagem Input Keys
</h2>

{#each fields as field (field.setting)}
	<div class="mb-2 flex items-center gap-2">
		<span class="min-w-22.5 text-xs font-medium text-neutral-200"
			>{field.label}:</span
		>
		<div class="select-wrapper flex-1">
			<select
				id={field.setting}
				value={$globalSettings[field.setting] ?? ""}
				on:change={(event) => updateKey(field.setting, event)}
				class="w-full rounded border border-neutral-700 bg-neutral-800 px-2 py-1 text-xs text-neutral-100 focus:border-neutral-600 focus:ring-1 focus:ring-neutral-600 focus:outline-none"
			>
				<option value="">Default ({field.default})</option>
				<optgroup label="Common">
					{#each commonKeys as key (key.value)}
						<option value={key.value}>{key.label}</option>
					{/each}
				</optgroup>
				<optgroup label="Arrow Keys">
					{#each arrowKeys as key (key.value)}
						<option value={key.value}>{key.label}</option>
					{/each}
				</optgroup>
				<optgroup label="Letters">
					{#each letterKeys as key (key.value)}
						<option value={key.value}>{key.label}</option>
					{/each}
				</optgroup>
			</select>
		</div>
	</div>
{/each}
