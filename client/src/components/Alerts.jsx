import Alert from "@mui/material/Alert";

import React from "react";

function Alerts({ message }) {
	return (
		<div>
			<Alert variant="filled" severity="error">
				{message}
			</Alert>
		</div>
	);
}

export default Alerts;
