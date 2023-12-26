import React, { useState } from "react";
import Alerts from "./components/Alerts";
import { validation } from "./validation";

function Body() {
	// Make the GET request using fetch
	const [tSystem, setTSystem] = useState({
		alphabet: "",
		states: "",
		initial: "",
		accepting: "",
		transitions: "",
	});
	const [regex, setRegex] = useState("");
	const [isValid, setIsValid] = useState(true);
	const [errorMessage, setMessage] = useState("");

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
	const handleButtonClick = async () => {
		// Create an object with values from all input fields
		const [isVal, message] = validation(tSystem);
		setIsValid(isVal);
		setMessage(message);
		console.log(message);
		if (isVal) {
			const apiUrl = "http://127.0.0.1:8080/regex";
			console.log("Input Object:", tSystem);
			const request_options = {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify(tSystem),
			};
			const response = await fetch(apiUrl, request_options);
			// const response = await fetch("http://127.0.0.1:8080/generate_regex");
			const val = await response.json();
			setRegex(val.regex);
		}
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
					{!isValid && (
						<div className="mr-32 ml-32 mt-3">
							<Alerts message={errorMessage} />
						</div>
					)}
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
