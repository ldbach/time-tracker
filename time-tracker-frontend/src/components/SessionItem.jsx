import React from "react";
import { formatDate, formatTime, formatWeekday } from "../utils/date";

export default function SessionItem({ session, idx, onDelete }) {
  return (
    <li style={{ marginBottom: "10px" }}>
      <b>Session {idx + 1}</b><br />
      Date: {formatDate(session.start)} ({formatWeekday(session.start)})<br />
      Start: {formatTime(session.start)}<br />
      End: {formatTime(session.end)}<br />
      Duration: <b>{session.duration}</b> seconds<br />
      <button onClick={() => onDelete(session.id)}>Delete</button>
    </li>
  );
}