import React, { useState, useEffect } from "react";
import {
  fetchSessions,
  fetchStatus,
  startSessionAPI,
  stopSessionAPI,
  deleteSessionAPI,
} from "./api";
import {
  formatDateTime,
  formatTime,
  formatDate,
  formatWeekday,
} from "./utils/date";

function App() {
  const BASE_URL = process.env.REACT_APP_BACKEND_URL;
  console.log("Using backend URL:", BASE_URL);

  const [status, setStatus] = useState({
    running: false,
    start_time: null,
    duration_seconds: 0,
  });

  const [sessions, setSessions] = useState([]);

  // Load initial data
  useEffect(() => {
    const loadData = async () => {
      const statusData = await fetchStatus();
      setStatus(statusData);

      const sessionData = await fetchSessions();
      setSessions(sessionData);
    };
    loadData();
  }, []);

  // Live timer that updates every second
  useEffect(() => {
    const interval = setInterval(() => {
      setStatus(prev => {
        if (!prev.running || !prev.start_time) return prev;
        const start = new Date(prev.start_time);
        const now = new Date();
        const diffSeconds = Math.floor((now - start) / 1000);
        return { ...prev, duration_seconds: diffSeconds };
      });
    }, 1000);
    
    return () => clearInterval(interval);
  }, []);

  // Handlers
  const startSessionHandler = async () => {
    const data = await startSessionAPI();
    setStatus(data);
  };

  const stopSessionHandler = async () => {
    const data = await stopSessionAPI();
    setStatus(data);

    const updatedSessions = await fetchSessions();
    setSessions(updatedSessions);
  };

  const deleteSessionHandler = async (id) => {
    setSessions(prev => prev.filter(s => s.id !== id)); // optimistic update
    await deleteSessionAPI(id);

    const updatedSessions = await fetchSessions();
    setSessions(updatedSessions);
  };

  return (
    <div style={{ textAlign: "center", marginTop: "50px" }}>
      <h1>Time Tracker</h1>

      <p>Running: {status.running ? "Yes" : "No"}</p>
      <p>Start Time: {formatDateTime(status.start_time)}</p>
      <p>Duration: <b>{status.duration_seconds}</b> seconds</p>

      <button onClick={startSessionHandler} style={{ marginRight: "10px" }}>
        Start
      </button>
      <button onClick={stopSessionHandler}>Stop</button>

      <hr />

      <h2>Completed Sessions</h2>
      <ul>
        {sessions.map((s, idx) => (
          <li key={idx} style={{ marginBottom: "10px" }}>
            <b>Session {idx + 1}</b><br />
            Date: {formatDate(s.start)} ({formatWeekday(s.start)})<br />
            Start: {formatTime(s.start)}<br />
            End: {formatTime(s.end)}<br />
            Duration: <b>{s.duration}</b> seconds<br />
            <button onClick={() => deleteSessionHandler(s.id)}>Delete</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;