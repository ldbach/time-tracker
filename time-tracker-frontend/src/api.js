const BASE_URL = process.env.REACT_APP_BACKEND_URL;

// Get all completed sessions
export const fetchSessions = async () => {
  const res = await fetch(`${BASE_URL}/sessions`);
  const data = await res.json();
  return data.map(s => ({
    id: s.id,
    start: s.start_time,
    end: s.end_time,
    duration: s.duration_seconds,
  }));
};

// Get the current running session status
export const fetchStatus = async () => {
  const res = await fetch(`${BASE_URL}/status`);
  const data = await res.json();
  return data;
};

// Start a session
export const startSessionAPI = async () => {
  const res = await fetch(`${BASE_URL}/start`, { method: "POST" });
  const data = await res.json();
  return data;
};

// Stop a session
export const stopSessionAPI = async () => {
  const res = await fetch(`${BASE_URL}/stop`, { method: "POST" });
  await res.json();
  return {
    running: false,
    start_time: null,
    duration_seconds: 0,
  };
};

// Delete a session
export const deleteSessionAPI = async (id) => {
  await fetch(`${BASE_URL}/sessions/${id}`, { method: "DELETE" });
};