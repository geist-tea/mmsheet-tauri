<script lang="ts">
	import '../data.ts';
	import { invoke } from '@tauri-apps/api/tauri';
  import { save, open } from '@tauri-apps/api/dialog';


	function NewSheet() {
		return {
			charinfo: {
				name: '',
				player: '',
				identity: '',
				secret: false,
				group: '',
				base: '',
				powerlevel: 10,
				exp: 0
			},
			abilities: {
				strength: 0,
				agility: 0,
				fighting: 0,
				awareness: 0,
				stamina: 0,
				dexterity: 0,
				intellect: 0,
				presence: 0
			},
			defense: {
				dodge: 0,
				parry: 0,
				fortitude: 0,
				toughness: 0,
				will: 0
			},
			skills: [],
			offense: [],
			advantages: [],
			powers: []
		};
	}
	let sheet: CharSheet = NewSheet();
	async function getNewSheet() {
		sheet = await invoke('new_sheet');
	}
	getNewSheet()
		.catch((x) => {
			console.log('no!' + x);
		});

	let advantageList: AdvantageInfo[] = [];
	async function getAdvantageList() {
		advantageList = await invoke('get_advantage_list');
	}
	getAdvantageList();

	function getSkillTotals(s: Skill) {
		switch (s.ability) {
			case 'Strength':
				return s.ranks + sheet.abilities.strength;
			case 'Agility':
				return s.ranks + sheet.abilities.agility;
			case 'Fighting':
				return s.ranks + sheet.abilities.fighting;
			case 'Awareness':
				return s.ranks + sheet.abilities.awareness;
			case 'Stamina':
				return s.ranks + sheet.abilities.stamina;
			case 'Dexterity':
				return s.ranks + sheet.abilities.dexterity;
			case 'Intelligence':
				return s.ranks + sheet.abilities.intellect;
			case 'Presence':
				return s.ranks + sheet.abilities.presence;
		}
	}

	let selected = 0;

	function newOffense() {
		sheet.offense.push({
			name: '',
			otype: 'Close',
			ranks: 1,
			damage: 0,
			other: ''
		});
		sheet.offenses = sheet.offenses;
	}

	function newSkill() {
		sheet.skills.push({
			name: '',
			ability: 'Intelligence',
			ranks: 0,
			expertise: true
		});
		sheet.skills = sheet.skills;
	}
	function addPowerEffect(p: Power) {
		p.effects.push({
			name: '',
			cost: 0,
			description: '',
			descriptors: '',
			extras: '',
			extrasrankcost: 0,
			extrasflatcost: 0,
			flaws: '',
			flawscost: 0,
			ranks: 1
		});
		sheet.powers = sheet.powers;
	}
	function newPower() {
		sheet.powers.push({
			name: '',
			effects: Array<PowerEffect>()
		});
		sheet.powers = sheet.powers;
	}

	function addAdvantage() {
		sheet.advantages.push({
			info: advantageList[selected],
			ranks: 1
		});
		sheet.advantages = sheet.advantages;
	}

  function removeAdvantage(idx: number) {
    sheet.advantages.splice(idx, 1);
    sheet.advantages = sheet.advantages;
  }


  function removeOffense(idx: number) {
    sheet.offense.splice(idx, 1);
    sheet.offense = sheet.offense;
  }


  function removePower(idx: number) {
    sheet.powers.splice(idx, 1);
    sheet.powers = sheet.powers;
  }


  function removePowerEffect(p: Power, idx: number) {
    p.effects.splice(idx, 1);
    sheet.powers = sheet.powers;
  }

  let total = 0;
  function calculateTotalPoints() {
    total= 2 * (sheet.abilities.stamina + sheet.abilities.agility + sheet.abilities.awareness + sheet.abilities.dexterity + sheet.abilities.fighting + sheet.abilities.intellect + sheet.abilities.presence + sheet.abilities.strength);

    total += sheet.defense.dodge + sheet.defense.parry + sheet.defense.fortitude + sheet.defense.will;

    sheet.advantages.forEach((x) => {total += x.ranks});
    sheet.skills.forEach((x) => {total += x.ranks * 0.5});
    sheet.offense.forEach((x) => {total += x.ranks});
    sheet.powers.forEach((p: Power) => {
      p.effects.forEach((e: PowerEffect) => {
        total += (e.ranks * (e.cost + e.extrasrankcost - e.flawscost)) + e.extrasflatcost;
      });
    });

    return total;
  }

  async function savesheet() {
    const path = await save({
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    });
    await invoke('save_sheet', {c: sheet, path: path});
  }

  async function loadsheet() {
    const path = await open({
      multiple: false,
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    });
    if(path === null) {
      return;
    }
    sheet = await invoke('load_sheet', {path: path});
    calculateTotalPoints();
  }
</script>

<div id="container">
	<div id="charInfo" class="sheetSection">
		<label for="heroName">Hero: </label>
		<input id="heroName" bind:value={sheet.charinfo.name} />
		<label for="player">Player: </label>
		<input id="player" bind:value={sheet.charinfo.player} />
		<label for="identity">Identity: </label>
		<input id="identity" bind:value={sheet.charinfo.identity} />
		<label for="secret">Secret? </label>
		<input id="secret" type="checkbox" bind:checked={sheet.charinfo.secret} />
		<label for="group">Group Affiliation: </label>
		<input id="group" bind:value={sheet.charinfo.group} />
		<label for="base">Base of Operations: </label>
		<input id="base" bind:value={sheet.charinfo.base} />
		<label for="powerLevel">Power Level: </label>
		<input id="powerLevel" type="number" bind:value={sheet.charinfo.powerlevel} />
		<label for="addPoints">Additional Points: </label>
		<input id="addPoints" type="number" bind:value={sheet.charinfo.exp} />
		Spent Points: {total}/{(sheet.charinfo.powerlevel * 15) + sheet.charinfo.exp}
    <button on:click={calculateTotalPoints}>Calculate</button>
    <button on:click={savesheet}>Save</button>
    <button on:click={loadsheet}>Load</button>
	</div>

	<div id="abilities" class="sheetSection">
		<h2>Ability Scores</h2>
		<table>
			<tr>
				<td><label for="strength">Strength:</label></td>
				<td><input id="strength" type="number" size="1" bind:value={sheet.abilities.strength} /></td
				>
			</tr>
			<tr>
				<td><label for="agility">Agility:</label></td>
				<td><input id="agility" type="number" size="1" bind:value={sheet.abilities.agility} /></td>
			</tr>
			<tr>
				<td><label for="fighting">Fighting:</label></td>
				<td><input id="fighting" type="number" size="1" bind:value={sheet.abilities.fighting} /></td
				>
			</tr>
			<tr>
				<td><label for="awareness">Awareness:</label></td>
				<td
					><input
						id="awareness"
						type="number"
						size="1"
						bind:value={sheet.abilities.awareness}
					/></td
				>
			</tr>
			<tr>
				<td><label for="stamina">Stamina:</label></td>
				<td><input id="stamina" type="number" size="1" bind:value={sheet.abilities.stamina} /></td>
			</tr>
			<tr>
				<td><label for="dexterity">Dexterity:</label></td>
				<td
					><input
						id="dexterity"
						type="number"
						size="1"
						bind:value={sheet.abilities.dexterity}
					/></td
				>
			</tr>
			<tr>
				<td><label for="intellect">Intellect:</label></td>
				<td
					><input
						id="intellect"
						type="number"
						size="1"
						bind:value={sheet.abilities.intellect}
					/></td
				>
			</tr>
			<tr>
				<td><label for="presence">Presence:</label></td>
				<td><input id="presence" type="number" size="1" bind:value={sheet.abilities.presence} /></td
				>
			</tr>
		</table>
	</div>

	<div id="defense" class="sheetSection">
		<div class="sectionContent">
			<h2>Defense</h2>
			<table>
				<tr>
					<td>Dodge</td>
					<td><input id="dodge" type="number" bind:value={sheet.defense.dodge} /></td>
					<td>{sheet.defense.dodge + sheet.abilities.agility}</td>
				</tr>
				<tr>
					<td>Parry</td>
					<td><input id="parry" type="number" bind:value={sheet.defense.parry} /></td>
					<td>{sheet.defense.parry + sheet.abilities.fighting}</td>
				</tr>
				<tr>
					<td>Fortitude</td>
					<td><input id="fortitude" type="number" bind:value={sheet.defense.fortitude} /></td>
					<td>{sheet.defense.fortitude + sheet.abilities.stamina}</td>
				</tr>
				<tr>
					<td>Toughness</td>
					<td />
					<td>{sheet.abilities.stamina}</td>
				</tr>
				<tr>
					<td>Will</td>
					<td><input id="will" type="number" bind:value={sheet.defense.will} /></td>
					<td>{sheet.defense.will + sheet.abilities.awareness}</td>
				</tr>
			</table>
		</div>
	</div>
	<div id="skills" class="sheetSection">
		<h2>Skills</h2>
		<table>
			<tr>
				<th>Name</th>
				<th>Ranks</th>
				<th>Total</th>
			</tr>
			{#each sheet.skills as skill}
				<tr>
					{#if skill.expertise}
						<td>Expertise: <input bind:value={skill.name} /></td>
					{:else}
						<td>{skill.name}</td>
					{/if}
					<td><input type="number" bind:value={skill.ranks} /></td>
					<td>{getSkillTotals(skill)}</td>
				</tr>
			{/each}
		</table>
		<button on:click={newSkill}>+</button>
	</div>
	<div id="offense" class="sheetSection">
		<h2>Offense</h2>
		<table>
			<tr>
				<th>Name</th>
				<th>Ranks</th>
				<th>Bonus</th>
				<th>Type</th>
				<th>Damage</th>
				<th>Details</th>
			</tr>
			{#each sheet.offense as offense, idx}
				<tr>
					<td><input bind:value={offense.name} /></td>
					<td><input type="number" bind:value={offense.ranks} /></td>
					<td>+{offense.otype === 'Close'
							? offense.ranks + sheet.abilities.fighting
							: offense.ranks + sheet.abilities.dexterity}</td>
					<td>
						<select bind:value={offense.otype}>
							<option value="Close">Close</option>
							<option value="Ranged">Ranged</option>
						</select>
					</td>
					<td><input type="number" bind:value={offense.damage} /></td>
					<td><input bind:value={offense.other} /></td>
          <td><button on:click={() => removeOffense(idx)}>-</button></td>
				</tr>
			{/each}
		</table>
		<button on:click={newOffense}>+</button>
	</div>
	<div id="advantages" class="sheetSection">
		<h2>Advantages</h2>
		<table>
			<tr>
				<th>Name</th>
				<th>Ranks</th>
				<th>Description</th>
			</tr>
			{#each sheet.advantages as advantage, idx}
				<tr>
					<td>{advantage.info.name}</td>
					{#if advantage.info.ranked}
						<td><input type="number" bind:value={advantage.ranks} /></td>
					{:else}
						<td />
					{/if}
					<td>{advantage.info.summary}</td>
          <td><button on:click={() => {removeAdvantage(idx)}}>-</button></td>
				</tr>
			{/each}
		</table>
		<select bind:value={selected}>
			{#each advantageList as opt, i}
				<option value={i}>{opt.name}</option>
			{/each}
		</select>
		<button on:click={addAdvantage}>+</button>
	</div>

	<div id="powers" class="sheetSection">
		<h2>Powers</h2>
		<list>
			{#each sheet.powers as power, idx}
				<li>
					<p><input type="text" bind:value={power.name} /><button on:click={() => removePower(idx)}>-</button></p>
					<table>
						<tr>
							<th>Name</th>
							<th>Base Cost</th>
							<th>Descriptors</th>
							<th>Extras</th>
							<th>Extras Cost</th>
							<th>Flat Cost Mod</th>
							<th>Flaws</th>
							<th>Flaws Cost</th>
							<th>Ranks</th>
						</tr>
						{#each power.effects as effect, eidx}
							<tr>
								<td><input type="text" bind:value={effect.name} /></td>
								<td><input type="number" bind:value={effect.cost} /></td>
								<td><input type="text" bind:value={effect.descriptors} /></td>
								<td><input type="text" bind:value={effect.extras} /></td>
								<td><input type="number" bind:value={effect.extrasrankcost} /></td>
								<td><input type="number" bind:value={effect.extrasflatcost} /></td>
								<td><input type="text" bind:value={effect.flaws} /></td>
								<td><input type="number" bind:value={effect.flawscost} /></td>
								<td><input type="number" bind:value={effect.ranks} /></td>
                <td><button on:click={() => removePowerEffect(power, eidx)}>-</button></td>
							</tr>
						{/each}
					</table>
					<button on:click={() => addPowerEffect(power)}>Add Effect</button>
				</li>
			{/each}
		</list>
		<button on:click={newPower}>+</button>
	</div>
</div>

<style>
	:root {
		color: #f5f5f5;
		font-family: sans-serif;
		background-color: #293333;
	}
	h2 {
		position: centered;
	}
	td {
		text-align: center;
		border-bottom: 0.1em solid #809999;
	}
	input[type='number'] {
		width: 3em;
	}
	input {
		resize: both;
		min-width: 3em;
		max-width: 10em;
	}
	.sheetSection {
		padding: 0.5em 0.5em;
		border: 0.25em solid #809999;
		border-radius: 1em;
		text-align: center;
	}
	table {
		width: 100%;
		padding: 0 0.5em;
	}
	.sectionContent {
		margin: auto;
	}
	@media (min-width: 36em) and (max-width: 50em) {
		#container {
			display: grid;
			grid-gap: 0.25em;
			grid-template-areas:
				'info info info'
				'abilities defense skills'
				'advantages advantages skills'
				'offense offense offense'
				'powers powers powers';
		}
	}
	@media (min-width: 50em) {
		#container {
			display: grid;
			grid-gap: 0.25em;
			grid-template-areas:
				'info info info'
				'abilities defense skills'
				'advantages advantages skills'
				'offense offense skills'
				'powers powers powers';
		}
	}
	#charInfo {
		grid-area: info;
	}
	#skills {
		grid-area: skills;
	}
	#abilities {
		grid-area: abilities;
	}
	#defense {
		grid-area: defense;
	}
	#offense {
		grid-area: offense;
    overflow-y: scroll;
	}
	#advantages {
		grid-area: advantages;
	}
	#powers {
		grid-area: powers;
		list-style: none;
    overflow-y: scroll;
	}
</style>
