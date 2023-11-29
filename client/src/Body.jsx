import React from "react";

function Body() {
	return (
		<div className="flex">
			{/* First Column */}
			<div className="w-full sm:w-1/2 p-4">
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
							/>
						</div>

						{/* Text Area */}
						<div className="mb-4">
							<label className="block text-sm font-semibold text-gray-600">
								Enter Transitions
							</label>
							<textarea
								className="w-full h-32 px-3 py-2 border rounded-md resize-none focus:outline-none focus:border-blue-500"
								placeholder="Ex. q0,a,q1"
							/>
						</div>
					</div>
					<div className="flex items-center justify-center mt-4">
						<button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full">
							Generate Regex
						</button>
					</div>
				</div>
			</div>

			{/* Second Column */}
			<div className="w-full sm:w-1/2 flex-grow p-4">
				<div className="bg-gray-200 shadow-md h-full rounded-md">
					{/* Content for the second box */}
					<p className="text-black text-center text-xl">How To Use</p>
				</div>
			</div>
		</div>
	);
}

export default Body;
