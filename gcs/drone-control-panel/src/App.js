import logo from './logo.svg';
import './App.css';
import React, { useEffect } from 'react';

function App() {

  useEffect(() => {
    // Initialize WebSocket connection to backend for signaling
    const ws = new WebSocket('ws://localhost:8080/signal');

    ws.onopen = () => {
        console.log('Connected to the backend');
        // You can send/receive messages here
    };
  }, []);

  return (
    <div className="App">
      <header className="App-header">
      <div>
            <h1>EZ Drone Control Panel</h1>
            {/* Video component and control elements */}
        </div>
        {/* <p>
          Edit <code>src/App.js</code> and save to reload.
        </p> */}
        {/* <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          ...
        </a> */}
      </header>
    </div>
  );
}

export default App;
