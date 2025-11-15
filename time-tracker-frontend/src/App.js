import React, { useState, useEffect } from "react";

function App() {
  const [status, setStatus] = useState({
    running: false,
    start_time: null,
    duration_seconds: 0,
  });

  const [sessions, setSessions] = useState([]);

  const fetchSessions = async () => {
    const res = await fetch("http://127.0.0.1:3001/sessions");
    const data = await res.json();

    // Map backend fields to frontend expected fields
    const mapped = data.map(s => ({
      start: s.start_time,
      end: s.end_time,
      duration: s.duration_seconds,
    }));

    setSessions(mapped); // data should be an array of sessions from backend
  };

  // Helper: format ISO datetime string nicely
  const formatDateTime = (isoString) => {
    if (!isoString) return "-";
    const date = new Date(isoString);
    return date.toLocaleString(); // local timezone
    // For fixed UTC: return date.toISOString().replace("T", " ").replace("Z", "");
  };

  const formatTime = (isoString) => {
    if (!isoString) return "-";
    return new Date(isoString).toLocaleTimeString();
  };

  const formatDate = (isoString) => {
    if (!isoString) return "-";
    const date = new Date(isoString);
    return date.toLocaleDateString();
  };

  const formatWeekday = (isoString) => {
    if (!isoString) return "-";
    const date = new Date(isoString);
    return date.toLocaleDateString(undefined, { weekday: "long" });
  };

  // Load initial status from backend
  useEffect(() => {
    fetch("http://127.0.0.1:3001/status")
      .then(res => res.json())
      .then(data => setStatus(data));

    fetchSessions(); // load completed sessions from backend
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

  // Start session
  const startSession = async () => {
    const res = await fetch("http://127.0.0.1:3001/start", { method: "POST" });
    const data = await res.json();
    setStatus(data);
  };

  // Stop session
  const stopSession = async () => {
    const res = await fetch("http://127.0.0.1:3001/stop", { method: "POST" });
    await res.json();

    setStatus({
      running: false,
      start_time: null,
      duration_seconds: 0,
    });

    fetchSessions(); // refresh completed sessions from backend
  };

  // Delete session from frontend only
  const deleteSession = (index) => {
    setSessions(prev => prev.filter((_, i) => i !== index));
  };

  return (
    <div style={{ textAlign: "center", marginTop: "50px" }}>
      <h1>Time Tracker</h1>

      <p>Running: {status.running ? "Yes" : "No"}</p>
      <p>Start Time: {formatDateTime(status.start_time)}</p>
      <p>Duration: <b>{status.duration_seconds}</b> seconds</p>

      <button onClick={startSession} style={{ marginRight: "10px" }}>
        Start
      </button>
      <button onClick={stopSession}>Stop</button>

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
            <button onClick={() => deleteSession(idx)}>Delete</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;