import React, { useState, useEffect } from "react";
import {
  fetchSessions,
  fetchStatus,
  startSessionAPI,
  stopSessionAPI,
  deleteSessionAPI,
} from "../api";
import { formatDateTime } from "../utils/date";
import SessionItem from "../components/SessionItem";

export default function TimeTracker() {
    // setStatus is a function you call when you want to change the status
  const [status, setStatus] = useState({
    running: false,
    start_time: null,
    duration_seconds: 0,
  });

  // create a session is an array and setSessions is a function to update it
  const [sessions, setSessions] = useState([]);

  // Load initial data, when the component is mounted, run once at the beginning
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
        // If the session is not running, do nothing
        if (!prev.running || !prev.start_time) return prev;
        const start = new Date(prev.start_time);
        const now = new Date();
        // (now - start_time) converts to milliseconds ÷ 1000 gives seconds
        const diffSeconds = Math.floor((now - start) / 1000);
        return { ...prev, duration_seconds: diffSeconds };
      });
    }, 1000);

    // Cleanup the interval on unmount
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
    setSessions(prev => prev.filter(s => s.id !== id)); // delete first on the frontend
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
          <SessionItem 
            key={s.id} 
            session={s} 
            idx={idx} 
            onDelete={deleteSessionHandler} 
          />
        ))}
      </ul>
    </div>
  );
}