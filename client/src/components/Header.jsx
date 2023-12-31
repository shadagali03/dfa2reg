import React from "react";
import logo from "../logo.png"; // Tell webpack this JS file uses this image

function Header() {
	return (
		<nav className="bg-gray-200 p-2">
			<div className="container mx-auto flex items-center">
				{/* Image and Text in the Upper Left */}
				<div className="flex items-center">
					<img src={logo} alt="Logo" className="w-32 h-16 mr-2" />
					<p className="text-zinc-800 font-extrabold text-4xl">
						NFA2REG
					</p>
				</div>

				{/* Add more items for the rest of your navbar */}
				<div className="ml-auto">
					{/* Add more navbar items here */}
					<a
						href="https://celadon-dango-d36e62.netlify.app"
						className="text-black text-xl hover:underline inline-block transition duration-300 ease-in-out"
						target="_blank"
						rel="noopener noreferrer"
					>
						About Me
					</a>
				</div>
			</div>
		</nav>
	);
}

export default Header;
