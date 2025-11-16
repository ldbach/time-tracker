import React from "react";
import { Handle, Position } from "react-flow-renderer";

export default function CustomNode({ data }) {
  const { label } = data;

  return (
    <div
      style={{
        padding: "4px 6px",
        border: "1px solid #777",
        borderRadius: "3px",
        background: "#fff",
        minWidth: "80px",
        cursor: "grab", // shows that it can be dragged
        fontSize: "12px",
      }}
    >
      <div>{label}</div>

      {/* Handles for connecting edges */}
      <Handle type="target" position={Position.Top} />
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
}