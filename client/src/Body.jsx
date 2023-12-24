import React, { useState } from "react";

function Body() {
	const [tSystem, setTSystem] = useState({
		alphabet: "",
		states: "",
		initial: "",
		accepting: "",
		transitions: "",
	});
	const [regex, setRegex] = useState("");

	const handleAlphabetChange = (e) => {
		const value = e.target.value;
		setTSystem((prevValues) => ({
			...prevValues,
			alphabet: value,
		}));
	};
	const handleStatesChange = (e) => {
		const value = e.target.value;
		setTSystem((prevValues) => ({
			...prevValues,
			states: value,
		}));
	};

	const handleInitialChange = (e) => {
		const value = e.target.value;
		setTSystem((prevValues) => ({
			...prevValues,
			initial: value,
		}));
	};

	const handleAcceptingChange = (e) => {
		const value = e.target.value;
		setTSystem((prevValues) => ({
			...prevValues,
			accepting: value,
		}));
	};

	const handleTransitionsChange = (e) => {
		const value = e.target.value;
		setTSystem((prevValues) => ({
			...prevValues,
			transitions: value,
		}));
	};
	const handleButtonClick = () => {
		// Create an object with values from all input fields
		const inputObject = { ...tSystem };
		console.log("Input Object:", inputObject);
		setRegex("here");
		// You can use the 'inputObject' as needed, such as sending it to an API, etc.
	};
	return (
		<div className="flex justify-center">
			{/* First Column */}
			<div className="w-3/4 p-4">
				<div className="bg-gray-200 shadow-md rounded-md flex-grow p-4">
					<div className="bg-white p-4 h-full rounded-md shadow-md">
						{/* Input 1 */}
						<div className="mb-4">
							<label className="block text-sm font-semibold text-gray-600">
								Enter Alphabet
							</label>
							<input
								type="text"
								className="w-full px-3 py-2 border rounded-md focus:outline-none focus:border-blue-500"
								placeholder="Ex. a,b,c"
								value={tSystem.alphabet}
								onChange={handleAlphabetChange}
							/>
						</div>

						{/* Input 2 */}
						<div className="mb-4">
							<label className="block text-sm font-semibold text-gray-600">
								Enter States
							</label>
							<input
								type="text"
								className="w-full px-3 py-2 border rounded-md focus:outline-none focus:border-blue-500"
								placeholder="Ex. q0, q1, q2"
								value={tSystem.states}
								onChange={handleStatesChange}
							/>
						</div>

						{/* Input 3 */}
						<div className="mb-4">
							<label className="block text-sm font-semibold text-gray-600">
								Enter Initial State
							</label>
							<input
								type="text"
								className="w-full px-3 py-2 border rounded-md focus:outline-none focus:border-blue-500"
								placeholder="Ex. q0"
								value={tSystem.initial}
								onChange={handleInitialChange}
							/>
						</div>

						{/* Input 4 */}
						<div className="mb-4">
							<label className="block text-sm font-semibold text-gray-600">
								Enter Accepting States
							</label>
							<input
								type="text"
								className="w-full px-3 py-2 border rounded-md focus:outline-none focus:border-blue-500"
								placeholder="Ex. q1, q2"
								value={tSystem.accepting}
								onChange={handleAcceptingChange}
							/>
						</div>

						{/* Text Area */}
						<div className="mb-4">
							<label className="block text-sm font-semibold text-gray-600">
								Enter Transitions
							</label>
							<textarea
								className="w-full h-32 px-3 py-2 border rounded-md resize-y focus:outline-none focus:border-blue-500"
								placeholder="Ex. q0,a,q1"
								value={tSystem.transitions}
								onChange={handleTransitionsChange}
							/>
						</div>
					</div>
					<div className="flex items-center justify-center mt-4">
						<button
							onClick={handleButtonClick}
							className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full"
						>
							Generate Regex
						</button>
					</div>
				</div>
				<div className="bg-gray-200 p-6 mt-8 mb-16 rounded-md shadow-md">
					<label className="block text-sm font-semibold text-gray-600">
						Regular Expression
					</label>
					<input
						type="text"
						className="w-full px-3 py-2 border rounded-md focus:outline-none focus:border-blue-500"
						placeholder=""
						value={regex}
						readOnly
					/>
				</div>
			</div>

			{/* Second Column
			<div className="w-full sm:w-1/2 flex-grow p-4">
				<div className="bg-gray-200 shadow-md h-full rounded-md">
					<p className="text-black text-center text-xl">How To Use</p>
				</div>
			</div> */}
		</div>
	);
}

export default Body;
