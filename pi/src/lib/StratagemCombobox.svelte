<script lang="ts">
	import { tick } from "svelte";
	import { iconUrl, PLACEHOLDER_ICON, type Stratagem } from "$lib/stratagems";

	let {
		items,
		value,
		onselect,
	}: {
		items: Stratagem[];
		value: string | undefined;
		onselect: (item: Stratagem) => void;
	} = $props();

	let open = $state(false);
	let query = $state("");
	let activeIndex = $state(0);
	let input: HTMLInputElement | undefined = $state();
	let listbox: HTMLDivElement | undefined = $state();

	const selected = $derived(items.find((item) => item.id === value));
	const filtered = $derived.by(() => {
		const q = query.trim().toLowerCase();
		if (!q) return items;
		return items.filter(
			(item) =>
				item.title.toLowerCase().includes(q) ||
				item.id.toLowerCase().includes(q),
		);
	});

	// Splits a title around the first match so it can be highlighted without {@html}.
	function parts(title: string) {
		const q = query.trim();
		const i = q ? title.toLowerCase().indexOf(q.toLowerCase()) : -1;
		if (i === -1) return { before: title, match: "", after: "" };
		return {
			before: title.slice(0, i),
			match: title.slice(i, i + q.length),
			after: title.slice(i + q.length),
		};
	}

	async function scrollToActive() {
		await tick();
		listbox?.children[activeIndex]?.scrollIntoView({ block: "nearest" });
	}

	async function openList() {
		query = "";
		activeIndex = Math.max(
			0,
			items.findIndex((item) => item.id === value),
		);
		open = true;
		await tick();
		input?.focus();
		scrollToActive();
	}

	function commit(item: Stratagem) {
		open = false;
		onselect(item);
	}

	function onkeydown(event: KeyboardEvent) {
		if (event.key === "ArrowDown" || event.key === "ArrowUp") {
			event.preventDefault();
			const step = event.key === "ArrowDown" ? 1 : -1;
			activeIndex = Math.min(
				Math.max(activeIndex + step, 0),
				filtered.length - 1,
			);
			scrollToActive();
		} else if (event.key === "Enter") {
			event.preventDefault();
			if (filtered[activeIndex]) commit(filtered[activeIndex]);
		} else if (event.key === "Escape") {
			event.preventDefault();
			open = false;
		}
	}
</script>

{#if !open}
	<button
		type="button"
		role="combobox"
		aria-haspopup="listbox"
		aria-expanded="false"
		aria-controls="stratagem-listbox"
		onclick={openList}
		class="flex w-full cursor-pointer items-center gap-2 rounded border border-neutral-700 bg-neutral-800 px-2 py-1 text-left hover:bg-neutral-700/60 focus:border-neutral-600 focus:ring-1 focus:ring-neutral-600 focus:outline-none"
	>
		<img
			src={iconUrl(selected?.icon ?? PLACEHOLDER_ICON)}
			alt=""
			class="size-7 flex-none rounded border border-neutral-700"
		/>
		{#if selected}
			<span class="min-w-0 flex-1 truncate text-xs text-neutral-100"
				>{selected.title}</span
			>
		{:else}
			<span class="flex-1 text-xs text-neutral-400">Select a stratagem…</span>
		{/if}
		<span class="flex-none text-xs text-neutral-400">▾</span>
	</button>
{:else}
	<input
		bind:this={input}
		bind:value={query}
		oninput={() => (activeIndex = 0)}
		{onkeydown}
		onblur={() => (open = false)}
		type="text"
		placeholder="Filter stratagems…"
		role="combobox"
		aria-expanded="true"
		aria-controls="stratagem-listbox"
		aria-autocomplete="list"
		class="w-full rounded border border-neutral-600 bg-neutral-800 px-2 py-1.5 text-xs text-neutral-100 ring-1 ring-neutral-600 outline-none placeholder:text-neutral-500"
	/>
	<!-- In the normal flow rather than absolute, so the PI iframe grows/scrolls instead of clipping it. -->
	<div
		bind:this={listbox}
		id="stratagem-listbox"
		role="listbox"
		class="mt-1 max-h-64 overflow-y-auto rounded border border-neutral-700 bg-neutral-800 p-1"
	>
		{#each filtered as item, i (item.id)}
			{@const p = parts(item.title)}
			<!-- mousedown + preventDefault keeps focus in the input, so onblur doesn't close first. -->
			<div
				role="option"
				tabindex="-1"
				aria-selected={item.id === value}
				onmousedown={(event) => {
					event.preventDefault();
					commit(item);
				}}
				onmousemove={() => (activeIndex = i)}
				class={[
					"flex cursor-pointer items-center gap-2 rounded px-1.5 py-1",
					i === activeIndex && "bg-neutral-700",
					item.id === value && i !== activeIndex && "bg-neutral-700/40",
				]}
			>
				<img
					src={iconUrl(item.icon)}
					alt=""
					class="size-7 flex-none rounded border border-neutral-700"
				/>
				<span class="min-w-0 flex-1 truncate text-xs text-neutral-200"
					>{p.before}
					<mark class="bg-transparent font-semibold text-neutral-50"
						>{p.match}</mark
					>{p.after}</span
				>
				<span
					class={[
						"w-3 flex-none text-xs text-neutral-100",
						item.id !== value && "invisible",
					]}>✓</span
				>
			</div>
		{:else}
			<div class="px-2 py-3 text-center text-xs text-neutral-500">
				No stratagem matches "{query}"
			</div>
		{/each}
	</div>
{/if}
