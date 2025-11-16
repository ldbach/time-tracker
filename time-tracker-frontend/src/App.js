import React, { useState, useEffect } from "react";

function App() {
  const BASE_URL = process.env.REACT_APP_BACKEND_URL;
  console.log("Using backend URL:", BASE_URL);

  const [status, setStatus] = useState({
    running: false,
    start_time: null,
    duration_seconds: 0,
  });

  const [sessions, setSessions] = useState([]);

  const fetchSessions = async () => {
    const res = await fetch(`${BASE_URL}/sessions`);
    const data = await res.json();

    // Map backend fields to frontend expected fields
    const mapped = data.map(s => ({
      id: s.id,
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
    const fetchSessions = async () => {
      const res = await fetch(`${BASE_URL}/sessions`);
      const data = await res.json();

      const mapped = data.map(s => ({
        id: s.id,
        start: s.start_time,
        end: s.end_time,
        duration: s.duration_seconds,
      }));

      setSessions(mapped);
    };

    fetch(`${BASE_URL}/status`)
      .then(res => res.json())
      .then(data => setStatus(data));

    fetchSessions();
  }, [BASE_URL]);

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
    const res = await fetch(`${BASE_URL}/start`, { method: "POST" });
    const data = await res.json();
    setStatus(data);
  };

  // Stop session
  const stopSession = async () => {
    const res = await fetch(`${BASE_URL}/stop`, { method: "POST" });
    await res.json();

    setStatus({
      running: false,
      start_time: null,
      duration_seconds: 0,
    });

    fetchSessions(); // refresh completed sessions from backend
  };

  // Delete session from backend and frontend
  const deleteSession = async (id) => {
    setSessions(prev => prev.filter(s => s.id !== id)); // remove immediately
    console.log(id);
    try {
      // Call backend to delete session
      await fetch(`${BASE_URL}/sessions/${id}`, {
        method: "DELETE",
      });

      // Refresh sessions from backend
      fetchSessions();
    } catch (err) {
      console.error("Failed to delete session:", err);
    }
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
            <button onClick={() => deleteSession(s.id)}>Delete</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;