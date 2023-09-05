/**
 * Bring data on patient samples from the diagnosis machine to the laboratory
 * with enough molecules to produce medicine!
 **/

/**
 * 1. Collect sample data at the SAMPLES module
 * 2. Analyze them at the DIAGNOSIS module
 * 3. Gather required molecules for the medicines at the MOLECULES module
 * 4. Produce the medicines at the LABORATORY modue
 * 
 * can carry up to 3 sample data files and 10 molecules
 * molecule types = A,B,C,D,E
 */

function readline() { return "";}

const { ILLEGAL_VALUE, MODULES, GOTO, CARRIED_BY, SAMPLE_RANKS, SAMPLE_INVENTORY_SPACE, CONNECT, MOLECULE_INVENTORY_SPACE } = generateConstants();

function generateConstants() {
	const GOTO = "GOTO";
	const CONNECT = "CONNECT";
	const MODULES = {
		diagnosis: "DIAGNOSIS",
		molecules: "MOLECULES",
		laboratory: "LABORATORY",
		sample: "SAMPLES",
		spawn: "START_POS"
	};

	const SAMPLE_INVENTORY_SPACE = 3;
	const MOLECULE_INVENTORY_SPACE = 10;
	const ILLEGAL_VALUE = -1;

	const CARRIED_BY = {
		me: 0,
		other: 1,
		cloud: -1
	};

	const SAMPLE_RANKS = {
		lots_of_health: 3,
		some_health: 2,
		little_health: 1
	};
	return { ILLEGAL_VALUE, MODULES, GOTO, CARRIED_BY, SAMPLE_RANKS, SAMPLE_INVENTORY_SPACE, CONNECT, MOLECULE_INVENTORY_SPACE };
}

class Molecules {
	public A: number;
	public B: number;
	public C: number;
	public D: number;
	public E: number;

	public listMissing() {
		let missing: string = "";
		while (this.A < 0) { missing += "A"; this.A++ }
		while (this.B < 0) { missing += "B"; this.B++ }
		while (this.C < 0) { missing += "C"; this.C++ }
		while (this.D < 0) { missing += "D"; this.D++ }
		while (this.E < 0) { missing += "E"; this.E++ }
		return missing;
	}

	public get count() {
		return this.A + this.B + this.C + this.D + this.E;
	};

	public get isNotPositive(): boolean {
		return this.A <= 0 &&
			this.B <= 0 &&
			this.C <= 0 &&
			this.D <= 0 &&
			this.E <= 0;
	}

	public canBeFinishedWith(inventory: Molecules): boolean {
		const diff = new Molecules().substract(inventory, this).listMissing.length;
		const result = diff <= MOLECULE_INVENTORY_SPACE - inventory.count;
		return result;
	}

	constructor() {
		this.A = 0;
		this.B = 0;
		this.C = 0;
		this.D = 0;
		this.E = 0;
	}

	set(values: { A: number, B: number, C: number, D: number, E: number }) {
		this.A = values.A;
		this.B = values.B;
		this.C = values.C;
		this.D = values.D;
		this.E = values.E;
		return this;
	}

	add(one: Molecules, two: Molecules) {
		this.A = one.A + two.A;
		this.B = one.B + two.B;
		this.C = one.C + two.C;
		this.D = one.D + two.D;
		this.E = one.E + two.E;
		return this;
	}

	substract(one: Molecules, two: Molecules) {
		this.A = one.A - two.A;
		this.B = one.B - two.B;
		this.C = one.C - two.C;
		this.D = one.D - two.D;
		this.E = one.E - two.E;
		return this;
	}
}

class Robot {

	private _latestInput: {
		/**
		 * current module
		 */
		target: string; eta: number;
		/**
		 * current health
		 */
		score: number;
		/**
		 * molecules held by me right now
		 */
		storage: Molecules;
		expertise: Molecules;
		/**
		 * molecules available
		 */
		available: Molecules;
		/**
		 * all of the samples in game
		 */
		samples: {
			/**
			 * used for CONNECT command when in DIAGNOSIS module
			 */
			id: number;
			/**
			 * me = 0
			 * other = 1
			 * cloud = -1 (can be taken when in DIAGNOSIS)
			 */
			carriedBy: number;
			/**
			 * scale of health bonus
			 */
			rank: number; expertiseGain: string;
			/**
			 * amount of health/score points I'll get after completing the sample 
			 */
			health: number;
			/**
			 * molecules required for generating the cure
			 */
			cost: Molecules;
		}[];
	};

	processInput(input: {
		target: string; eta: number; score:
		number; storage: Molecules;
		expertise: Molecules;
		available: Molecules;
		samples: {
			id: number; carriedBy: number; rank: number; expertiseGain: string; health: number;
			cost: Molecules;
		}[];
	}) {
		this._latestInput = input;
		switch (this._latestInput.target) {
			case MODULES.molecules:
			    return this.moleculeModule();
			case MODULES.diagnosis:
			    return this.diagnosisModule();
			case MODULES.sample:
			    return this.sampleModule();
			case MODULES.laboratory:
			    return this.laboratoryModule();
			case MODULES.spawn:
			    return GOTO + " " + MODULES.sample;
			default:
			    console.error("Uknown module: " + this._latestInput.target);
		}
	}

	sampleModule() {
		let allSamples = this._latestInput.samples;
		let currentlyHeldSamples = allSamples.filter(sample => sample.carriedBy == CARRIED_BY.me);
		let valuableCloudSamples = allSamples.filter((sample) =>
			sample.carriedBy == CARRIED_BY.cloud &&
			sample.rank == SAMPLE_RANKS.lots_of_health &&
			sample.cost.count <= MOLECULE_INVENTORY_SPACE
		);
		const interestingCloud = currentlyHeldSamples.length + valuableCloudSamples.length >= SAMPLE_INVENTORY_SPACE
		if (currentlyHeldSamples.length == SAMPLE_INVENTORY_SPACE || interestingCloud) {
			return GOTO + " " + MODULES.diagnosis;
		}
		return CONNECT + " " + SAMPLE_RANKS.lots_of_health
	}

	diagnosisModule() {
		let allSamples = this._latestInput.samples;
		let currentlyHeldSamples = allSamples.filter(sample => sample.carriedBy == CARRIED_BY.me);

		let cloudSamples = allSamples.filter(sample =>
			sample.carriedBy == CARRIED_BY.cloud &&
			sample.rank == SAMPLE_RANKS.lots_of_health &&
			sample.cost.count <= MOLECULE_INVENTORY_SPACE
		);
		if (currentlyHeldSamples.length < SAMPLE_INVENTORY_SPACE && cloudSamples.length > 0) {
			return CONNECT + " " + cloudSamples.sort(healthComparison)[0].id;
		}

		let diagnosedSamples = currentlyHeldSamples.filter(sample => sample.health != ILLEGAL_VALUE);
		let impossibleSamples = diagnosedSamples.filter(sample => sample.cost.count > MOLECULE_INVENTORY_SPACE);
		if (impossibleSamples.length > 0) {
			return CONNECT + " " + impossibleSamples[0].id;
		}

		let undiagnosedSamples = currentlyHeldSamples.filter(sample => sample.health == ILLEGAL_VALUE);
		if (undiagnosedSamples.length > 0) {
			return CONNECT + " " + undiagnosedSamples[0].id;
		}

		if (currentlyHeldSamples.length <= SAMPLE_INVENTORY_SPACE / 3) {
			return GOTO + " " + MODULES.sample;
		}
		return GOTO + " " + MODULES.molecules;
	}

	moleculeModule() {
		let currentMolecules = this._latestInput.storage;
		const currentMoleculesCount = currentMolecules.count;

		const currentSamples = this._latestInput.samples
			.filter((sample) => sample.carriedBy == CARRIED_BY.me);
		const applicableSamples = currentSamples
			.filter(sample => sample.cost.canBeFinishedWith(currentMolecules))
			.sort(healthComparison);
		const availableMolecules = this._latestInput.available; // will be used later
		let neededMolecules: string = "";
		applicableSamples.map((sample) => {
			currentMolecules.substract(currentMolecules, sample.cost);
			neededMolecules += currentMolecules.listMissing();
		});
		if (currentMoleculesCount == MOLECULE_INVENTORY_SPACE || neededMolecules.length == 0) {
			return GOTO + " " + MODULES.laboratory;
		}
		return CONNECT + " " + neededMolecules[0];
	}

	laboratoryModule() {
		let currentSamples = this._latestInput.samples
			.filter((sample) => sample.carriedBy == CARRIED_BY.me)
		let currentMolecules = this._latestInput.storage;
		let applicableSamples = currentSamples
			.sort(healthComparison)
			.filter((sample) => {
			    return new Molecules().substract(sample.cost, currentMolecules).isNotPositive
			});
		if (applicableSamples.length > 0) {
			return CONNECT + " " + applicableSamples[0].id;
		}
		if (currentSamples.length > 0) {
			return GOTO + " " + MODULES.molecules;
		} else {
			return GOTO + " " + MODULES.sample;
		}
	}
}


function healthComparison(sampleOne: { health: number; }, sampleTwo: { health: number; }) {
	return sampleTwo.health - sampleOne.health;
}

function readInput() {
	var inputs: string[] = readline().split(' ');
	const target: string = inputs[0];
	const eta: number = parseInt(inputs[1]);
	const score: number = parseInt(inputs[2]);
	const storageA: number = parseInt(inputs[3]);
	const storageB: number = parseInt(inputs[4]);
	const storageC: number = parseInt(inputs[5]);
	const storageD: number = parseInt(inputs[6]);
	const storageE: number = parseInt(inputs[7]);
	const expertiseA: number = parseInt(inputs[8]);
	const expertiseB: number = parseInt(inputs[9]);
	const expertiseC: number = parseInt(inputs[10]);
	const expertiseD: number = parseInt(inputs[11]);
	const expertiseE: number = parseInt(inputs[12]);

	let secondPlayerInfo = readline();

	var inputs: string[] = readline().split(' ');
	const availableA: number = parseInt(inputs[0]);
	const availableB: number = parseInt(inputs[1]);
	const availableC: number = parseInt(inputs[2]);
	const availableD: number = parseInt(inputs[3]);
	const availableE: number = parseInt(inputs[4]);
	const sampleCount: number = parseInt(readline());
	let samples: {
		id: number;
		carriedBy: number;
		rank: number;
		expertiseGain: string;
		health: number;
		cost: Molecules
	}[]
		= [];
	for (let i = 0; i < sampleCount; i++) {
		var inputs: string[] = readline().split(' ');
		const sampleId: number = parseInt(inputs[0]);
		const carriedBy: number = parseInt(inputs[1]);
		const rank: number = parseInt(inputs[2]);
		const expertiseGain: string = inputs[3];
		const health: number = parseInt(inputs[4]);
		const costA: number = parseInt(inputs[5]);
		const costB: number = parseInt(inputs[6]);
		const costC: number = parseInt(inputs[7]);
		const costD: number = parseInt(inputs[8]);
		const costE: number = parseInt(inputs[9]);
		samples.push({
			id: sampleId,
			carriedBy: carriedBy,
			rank: rank,
			expertiseGain: expertiseGain,
			health: health,
			cost: new Molecules().set({
			    A: costA,
			    B: costB,
			    C: costC,
			    D: costD,
			    E: costE,
			})
		});
	}
	return {
		target: target,
		eta: eta,
		score: score,
		storage: new Molecules().set({
			A: storageA,
			B: storageB,
			C: storageC,
			D: storageD,
			E: storageE
		}),
		expertise: new Molecules().set({
			A: expertiseA,
			B: expertiseB,
			C: expertiseC,
			D: expertiseD,
			E: expertiseE
		}),
		available: new Molecules().set({
			A: availableA,
			B: availableB,
			C: availableC,
			D: availableD,
			E: availableE
		}),
		samples
	};
}

const projectCount: number = parseInt(readline());
const bot = new Robot();

for (let i = 0; i < projectCount; i++) {
	var inputs: string[] = readline().split(' ');
	const a: number = parseInt(inputs[0]);
	const b: number = parseInt(inputs[1]);
	const c: number = parseInt(inputs[2]);
	const d: number = parseInt(inputs[3]);
	const e: number = parseInt(inputs[4]);
}

// game loop
while (true) {

	// Write an action using console.log()
	// To debug: console.error('Debug messages...');

	console.log(bot.processInput(readInput()));
}

