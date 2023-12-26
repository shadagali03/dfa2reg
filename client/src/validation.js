function hasOneCommaBetweenCharacters(inputString) {
	const stringWithoutSpaces = inputString.replace(/\s/g, "");
	return /^(?:[^,]+,)*[^,]+$/.test(stringWithoutSpaces);
}

function areAllCharactersAlphanumeric(inputString) {
	// Split the string by commas
	const parts = inputString.split(",");

	// Check if all parts contain only alphanumeric characters
	return parts.every((part) => /^[a-zA-Z0-9]+$/.test(part.trim()));
}

function splitAndAddToSet(inputString) {
	// Split the string by commas
	const parts = inputString.split(",");

	// Create a Set and add each part to it
	const uniqueSet = new Set(parts.map((part) => part.trim()));

	return uniqueSet;
}

function areAllElementsInSet(inputString, mySet) {
	// Split the string by commas
	const parts = inputString.split(",");

	// Check if every element in the string exists in the set
	return parts.every((part) => mySet.has(part.trim()));
}

function validTransitions(inputString, stateSet, alphabetSet) {
	const transitions = inputString.split("\n");

	for (let i = 0; i < transitions.length; i++) {
		const t = transitions[i];
		const parts = t.split(",");
		if (
			!stateSet.has(parts[0]) ||
			!stateSet.has(parts[2]) ||
			!alphabetSet.has(parts[1])
		) {
			return false;
		}
	}
	return true;
}

export const validation = (user_system) => {
	// Check if alphabet has comma between each
	if (!hasOneCommaBetweenCharacters(user_system.alphabet)) {
		return [false, "Make sure alphabet has a comma between each character"];
	}

	// Check if the alphabet is single characters alphanumeric
	if (!areAllCharactersAlphanumeric(user_system.alphabet)) {
		return [false, "Make sure alphabet is alphanumeric"];
	}

	// Convert states to a set
	const states = splitAndAddToSet(user_system.states);
	const alphabets = splitAndAddToSet(user_system.alphabet);

	// Check if initial state is valid
	if (!states.has(user_system.initial)) {
		return [false, "Initial state does not exist within given states"];
	}

	// Check if accepting states are valid
	if (!areAllElementsInSet(user_system.accepting, states)) {
		return [false, "Accepting state does not exist within given states"];
	}

	// Check if each part of transition is valid
	if (!validTransitions(user_system.transitions, states, alphabets)) {
		return [false, "Transitions format is incorrect"];
	}

	return [true, ""];
};
