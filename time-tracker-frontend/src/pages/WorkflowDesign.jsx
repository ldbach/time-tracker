import React from "react";
import ReactFlow, {
  ReactFlowProvider,
  addEdge,
  useNodesState,
  useEdgesState,
} from "react-flow-renderer";
import CustomNode from "../components/CustomNode";

export default function WorkflowDesign() {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  const nodeTypes = { customNode: CustomNode };

  // Add a new node
  const addNode = () => {
    const newNode = {
      id: `${nodes.length + 1}`,
      type: "customNode",
      position: { x: 250, y: 100 + nodes.length * 100 },
      data: { label: `Node ${nodes.length + 1}` },
    };
    setNodes((nds) => [...nds, newNode]);
  };

  return (
    <div style={{ width: "100%", height: "600px" }}>
      <div style={{ textAlign: "center", marginBottom: "10px" }}>
        <button onClick={addNode} style={{ padding: "8px 16px" }}>
          Add Node
        </button>
      </div>

      <ReactFlowProvider>
        <ReactFlow
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={(params) => setEdges((eds) => addEdge(params, eds))}
          nodeTypes={nodeTypes}
          fitView
        />
      </ReactFlowProvider>
    </div>
  );
}