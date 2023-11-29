import "./App.css";
import Body from "./Body";
import Header from "./components/Header";
import Footer from "./components/Footer";

function App() {
	return (
		<div className="App">
			<Header />
			<div className="flex items-center text-center justify-center mt-8">
				<span className="text-3xl">
					Quick and easy NFA to Regex with steps!
				</span>
			</div>
			<Body />
			<div className="bg-gray-200 p-6 ml-16 mr-16 mt-2 mb-16 rounded-md shadow-md">
				<label className="block text-sm font-semibold text-gray-600">
					Your Input
				</label>
				<input
					type="text"
					className="w-full px-3 py-2 border rounded-md focus:outline-none focus:border-blue-500"
					placeholder="Enter text"
				/>
			</div>

			<Footer />
		</div>
	);
}

export default App;
