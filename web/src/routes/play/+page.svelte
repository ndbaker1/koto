<script lang="ts">
	import { name } from '$lib/constants';

	const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';

	let pressed = false;
	let selectionMatrix: boolean[][] = Array.from({ length: 6 }, () => Array(6).fill(false));
	let letterMatrix: string[][] = Array.from({ length: 6 }, () =>
		Array.from({ length: 6 }, () =>
			characters.charAt(Math.floor(Math.random() * characters.length))
		)
	);
	let trail: [number, number][] = [];

	/**
	 * Trigger form logic in response to a keydown event, so that
	 * desktop users can use the keyboard to play the game
	 */
	function keydown(event: KeyboardEvent) {
		if (event.metaKey) return;

		document
			.querySelector(`[data-key="${event.key}" i]`)
			?.dispatchEvent(new MouseEvent('click', { cancelable: true }));
	}

	function movePossible(i: number, j: number) {
		// first tile chosen.
		if (trail.length < 1) return true;
		// if this move matches anything in the trail then remove it
		for (const [col, row] of trail.slice(0, -2)) {
			if (col === i && row === j) return false;
		}
		// make sure it is possible to move to this tile from the previous
		const [lastCol, lastRow] = trail[trail.length - 1];
		return Math.abs(lastCol - i) <= 1 && Math.abs(lastRow - j) <= 1;
	}

	function actionIndex(i: number, j: number) {
		if (pressed) {
			if (!movePossible(i, j)) {
				return;
			}

			if (trail.length >= 2) {
				const [prevCol, prevRow] = trail[trail.length - 2];
				if (prevCol === i && prevRow === j) {
					const [lastCol, lastRow] = trail.pop()!!;
					selectionMatrix[lastCol][lastRow] = false;
					return;
				}
			}

			if (trail.length >= 1) {
				const [lastCol, lastRow] = trail[trail.length - 1];
				if (lastCol !== i || lastRow !== j) {
					selectionMatrix[i][j] = true;
					trail.push([i, j]);
				}
			} else {
				selectionMatrix[i][j] = true;
				trail.push([i, j]);
			}
		}
	}
</script>

<svelte:window
	on:keydown={keydown}
	on:mousedown={() => (pressed = true)}
	on:mouseup={() => {
		pressed = false;
		trail = [];
		selectionMatrix = Array.from({ length: 6 }, () => Array(6).fill(false));
	}}
/>

<section>
	<div class="grid">
		{#each Array.from(Array(6).keys()) as row (row)}
			<div class="row">
				{#each Array.from(Array(6).keys()) as column (column)}
					<div
						class="letter"
						on:touchstart={() => {
							pressed = true;
							actionIndex(column, row);
						}}
						on:mousedown={() => {
							pressed = true;
							actionIndex(column, row);
						}}
						on:mousemove={() => actionIndex(column, row)}
						on:touchmove={() => actionIndex(column, row)}
						class:highlighted={selectionMatrix[column][row]}
					>
						{letterMatrix[column][row]}
					</div>
				{/each}
			</div>
		{/each}
	</div>
	<span class="health" />
	<div />
</section>

<style>
	section {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		flex: 1;
	}

	.grid {
		--width: min(100vw, 40vh, 380px);
		user-select: none;
		max-width: var(--width);
		align-self: center;
		justify-self: center;
		width: 100%;
		height: 100%;
		display: flex;
		gap: 0.5rem;
		flex-direction: column;
		justify-content: flex-start;
	}

	.grid .row {
		display: grid;
		gap: 0.5rem;
		grid-template-columns: repeat(6, 1fr);
	}

	.health {
		width: min(25rem, 80vw);
		height: 1rem;
		border-radius: 0.2rem;
		background: rgb(255, 90, 90);
	}

	.letter {
		aspect-ratio: 1;
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		text-align: center;
		box-sizing: border-box;
		text-transform: uppercase;
		box-shadow: 0 2px 2px gray;
		font-size: calc(0.08 * var(--width));
		border-radius: 5rem;
		background: white;
		transition: 200ms;
	}

	.letter.highlighted {
		color: white;
		background: rgba(200, 0, 150, 0.4);
		box-shadow: 0 2px 2px black;
	}

	@keyframes wiggle {
		0% {
			transform: translateX(0);
		}
		10% {
			transform: translateX(-2px);
		}
		30% {
			transform: translateX(4px);
		}
		50% {
			transform: translateX(-6px);
		}
		70% {
			transform: translateX(+4px);
		}
		90% {
			transform: translateX(-2px);
		}
		100% {
			transform: translateX(0);
		}
	}
</style>
