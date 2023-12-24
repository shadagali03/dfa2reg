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

			<Footer />
		</div>
	);
}

export default App;
