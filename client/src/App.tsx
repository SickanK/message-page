import React, { useState } from "react";

function App() {
  let [message, setMessage] = useState<string>("Message");

  const handleClick = () => {
    fetch(process.env.REACT_DOMAIN_APP + "/send", {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
        "Access-Control-Allow-Origin": process.env.REACT_APP_DOMAIN as string,
      },
      body: JSON.stringify({
        message: message,
      }),
    });
  };

  return (
    <div className="App">
      <p>{message}</p>
      <input
        type="text"
        onChange={(e) => setMessage(e.target.value)}
        value={message}
      />
      <button onClick={handleClick}>Send</button>
    </div>
  );
}

export default App;
