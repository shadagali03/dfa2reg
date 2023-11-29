import React from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import GitHubIcon from "@mui/icons-material/GitHub";

function Header() {
	return (
		<nav className="bg-gray-200 h-32 flex">
			<div className="container mx-auto flex items-center justify-center">
				{/* GitHub Icon */}
				<a
					href="https://github.com/shadagali03/dfa2reg"
					target="_blank"
					rel="noopener noreferrer"
					className="mr-2"
				>
					<GitHubIcon />
				</a>

				{/* GitHub Link */}
				<a
					href="https://github.com/shadagali03/dfa2reg"
					target="_blank"
					rel="noopener noreferrer"
					className="text-lg"
				>
					Visit GitHub
				</a>
			</div>
		</nav>
	);
}

export default Header;
