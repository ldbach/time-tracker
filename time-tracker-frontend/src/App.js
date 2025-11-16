import React from "react";
import { BrowserRouter as Router, Routes, Route, Link, Navigate } from "react-router-dom";
import TimeTracker from "./pages/TimeTracker";
import WorkflowDesign from "./pages/WorkflowDesign";

function App() {
  return (
    <Router>
      <div style={{ textAlign: "center", marginTop: "50px" }}>
        {/* Navigation */}
        <nav style={{ marginBottom: "20px" }}>
          <Link to="/" style={{ marginRight: "10px" }}>Time Tracker</Link>
          <Link to="/workflow-design">Workflow Design</Link>
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