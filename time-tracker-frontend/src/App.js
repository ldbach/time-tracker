import React from "react";
import { HashRouter as Router, Routes, Route, NavLink, Navigate } from "react-router-dom";
import TimeTracker from "./pages/TimeTracker";
import WorkflowDesign from "./pages/WorkflowDesign";

function App() {
  const linkStyle = {
    marginRight: "10px",
    textDecoration: "none",
    color: "blue",
  };

  return (
    <Router>
      <div style={{ textAlign: "center", marginTop: "50px" }}>
        {/* Navigation */}
        <nav style={{ marginBottom: "20px" }}>
          <NavLink
            to="/"
            end
            style={({ isActive }) => ({
              ...linkStyle,
              fontWeight: isActive ? "bold" : "normal",
              color: isActive ? "darkblue" : "blue",
            })}
          >
            Time Tracker
          </NavLink>
          <NavLink
            to="/workflow-design"
            style={({ isActive }) => ({
              ...linkStyle,
              fontWeight: isActive ? "bold" : "normal",
              color: isActive ? "darkblue" : "blue",
            })}
          >
            Workflow Design
          </NavLink>
        </nav>

        <Routes>
          {/* Default route */}
          <Route path="/" element={<TimeTracker />} />
          {/* Workflow route */}
          <Route path="/workflow-design" element={<WorkflowDesign />} />
          {/* Redirect unknown routes to default */}
          <Route path="*" element={<Navigate to="/" />} />
        </Routes>
      </div>
    </Router>
  );
}

export default App;