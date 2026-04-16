import { useState, useRef, useEffect } from "react";

const competitors = [
  {
    name: "GraphBit",
    tagline: "Rust core + Python wrapper",
    status: "Active",
    stars: "~200",
    org: "InfinitiBit GmbH (Munich)",
    license: "Proprietary / Patent Pending",
    approach: "Rust core, Python-first API",
    strengths: [
      "Enterprise customer (Grant Thornton Germany)",
      "ISO 27001 / TISAX / EU AI Act alignment",
      "Claims 68× lower CPU, 140× lower memory vs Python frameworks",
      "Built-in observability, tracing, circuit breakers",
      "Patent-pending orchestration architecture",
    ],
    weaknesses: [
      "Rust core is not directly consumable—Python wrapper only",
      "Newer ecosystem, sparse independent reviews",
      "Vendor benchmarks not independently reproduced",
      "Proprietary core limits community contribution depth",
      "No direct Rust-native developer experience",
    ],
    memory: "Workflow state serialization, persistence across steps",
    tools: "Document loaders (PDF, DOCX, CSV, etc.), text splitters",
    orchestration: "Sequential, parallel, multi-agent chains",
    llmProviders: "OpenAI, Anthropic, Ollama",
    mcp: false,
    hexagonal: false,
    ddd: false,
    color: "#E63946",
  },
  {
    name: "Rig",
    tagline: "Modular LLM library for Rust",
    status: "Active",
    stars: "~6,200",
    org: "0xPlaygrounds",
    license: "MIT",
    approach: "Trait-based LLM abstraction library",
    strengths: [
      "Largest Rust AI community (~6.2k GitHub stars)",
      "Clean trait abstractions (CompletionModel, EmbeddingModel)",
      "Strong production adoption (Dria, Nethermind, Neon, Listen, Cairnify)",
      "Extensive vector store integrations (Qdrant, MongoDB, LanceDB, SQLite)",
      "WASM-compatible (WasmCompatSend/Sync)",
      "Pipeline API for composable workflows",
    ],
    weaknesses: [
      "Not a multi-agent orchestration framework—single agent focus",
      "No built-in orchestration patterns (no swarm/formation/campaign)",
      "Function calling, vision, tool-use still incomplete",
      "No memory management beyond vector store RAG",
      "No CLI or scaffolding tools",
    ],
    memory: "Vector store via companion crates (Qdrant, MongoDB, etc.)",
    tools: "Tool trait with ToolDefinition, limited built-in tools",
    orchestration: "Pipeline API only—no multi-agent coordination",
    llmProviders: "OpenAI, Anthropic, Cohere, Groq, Ollama + community",
    mcp: false,
    hexagonal: false,
    ddd: false,
    color: "#457B9D",
  },
  {
    name: "Swarms-rs",
    tagline: "Rust port of Swarms Python framework",
    status: "Active",
    stars: "~350",
    org: "The Swarm Corporation",
    license: "MIT",
    approach: "Multi-agent orchestration with workflow patterns",
    strengths: [
      "Direct Rust port of popular Swarms Python ecosystem",
      "Sequential and concurrent workflow primitives",
      "MCP support (STDIO + SSE)",
      "Agent autosave and state persistence",
      "Published on crates.io with documentation",
    ],
    weaknesses: [
      "Limited orchestration patterns (sequential + concurrent only)",
      "No DAG/graph-based workflows",
      "No hierarchical delegation or dynamic routing",
      "Relatively thin Rust codebase—port-quality concerns",
      "No hexagonal architecture or clean layer separation",
      "Limited memory system (no vector DB integration)",
    ],
    memory: "Basic autosave to filesystem",
    tools: "MCP STDIO + SSE tool integration",
    orchestration: "Sequential workflows, concurrent workflows",
    llmProviders: "OpenAI, DeepSeek (OpenAI-compatible API)",
    mcp: true,
    hexagonal: false,
    ddd: false,
    color: "#2A9D8F",
  },
  {
    name: "ADK-Rust",
    tagline: "Google ADK patterns ported to Rust",
    status: "Very Active",
    stars: "~150",
    org: "Zavora AI",
    license: "Apache 2.0",
    approach: "Modular workspace of 25 publishable crates",
    strengths: [
      "Most feature-rich: 25 crates covering agents, tools, memory, voice, graphs",
      "Realtime voice agents (OpenAI, Gemini Live, Vertex AI Live)",
      "Graph-based workflows with checkpoints and HITL",
      "A2A (Agent-to-Agent) protocol support",
      "Guardrails system (PII, content filtering)",
      "UI generation via tool calls (adk-ui)",
      "Browser automation (adk-browser)",
      "Eval framework (adk-eval)",
      "VS Code extension and cargo-adk CLI",
      "120+ examples in adk-playground",
    ],
    weaknesses: [
      "Young project (first release Nov 2025)—API stability uncertain",
      "Only ~2,000 crates.io downloads total",
      "docs.rs builds failing for some crates",
      "Not affiliated with Google—community project risk",
      "No DDD or hexagonal architecture",
      "Breadth may sacrifice depth—too many crates for maturity",
    ],
    memory: "adk-memory for semantic search, adk-session for persistence",
    tools: "MCP via rmcp, Google Search, FunctionTool with #[tool] macro",
    orchestration: "Sequential, Parallel, Loop, Conditional, Graph agents",
    llmProviders: "Gemini, OpenAI, Anthropic, DeepSeek, Groq, Ollama, Fireworks, Together, Mistral, Bedrock, Azure + more",
    mcp: true,
    hexagonal: false,
    ddd: false,
    color: "#E9C46A",
  },
  {
    name: "AutoAgents",
    tagline: "LiquidOS Rust agent SDK + Odyssey runtime",
    status: "Active",
    stars: "~470",
    org: "LiquidOS AI",
    license: "MIT / Apache 2.0",
    approach: "Actor-based agents with WASM sandboxing",
    strengths: [
      "WASM sandboxed tool execution—unique security model",
      "Actor-based concurrency via Ractor framework",
      "Odyssey runtime for portable agent bundles",
      "LLM pipeline optimization (cache, retry passes)",
      "Guardrails (Block, Sanitize, Audit policies)",
      "Edge deployment (Raspberry Pi, Jetson, ARM)",
      "Python bindings via PyO3",
      "Speech support (TTS/STT)",
      "Zero-trust execution model in LiquidOS runtime",
    ],
    weaknesses: [
      "LiquidOS runtime not yet launched (coming soon)",
      "Limited orchestration patterns—no DAG/graph workflows",
      "Smaller community vs Rig",
      "Documentation still maturing",
      "No CLI scaffolding for rapid project setup",
    ],
    memory: "Sliding window memory, Qdrant vector store",
    tools: "Derive macros, WASM runtime, MCP integration, filesystem, search",
    orchestration: "Direct agents, actor-based concurrency, chaining, routing, parallel",
    llmProviders: "OpenAI, Anthropic, Groq, Google, Azure, DeepSeek, Ollama, xAI + more",
    mcp: true,
    hexagonal: false,
    ddd: false,
    color: "#8338EC",
  },
  {
    name: "Zeph",
    tagline: "Single-binary AI agent with research-backed memory",
    status: "Active",
    stars: "~New",
    org: "bug-ops (individual)",
    license: "MIT",
    approach: "Monolithic agent binary with deep memory architecture",
    strengths: [
      "Most advanced memory: SYNAPSE graph with 5 edge types + MAGMA retrieval",
      "Self-learning skills with Wilson score Bayesian ranking",
      "RL-based SleepGate admission control",
      "OWASP AI security (sanitization, injection detection, PII filtering)",
      "Cascade quality routing across providers",
      "DAG task orchestration with planner/scheduler/aggregator",
      "Sub-agent spawning with transcript persistence",
      "AST code indexing with semantic retrieval",
      "Research-grounded (cites ICML, ICLR, NeurIPS papers)",
    ],
    weaknesses: [
      "Single-developer project—bus factor of 1",
      "Monolithic binary, not a reusable library/framework",
      "Not designed for embedding into other systems",
      "Very new, minimal community adoption",
      "No published benchmarks or production deployments",
    ],
    memory: "SYNAPSE graph memory (5 edge types), spreading activation, community detection",
    tools: "Shell/web/file/composite executors, MCP multi-server",
    orchestration: "DAG task graphs with planner, scheduler, aggregator + sub-agents",
    llmProviders: "Ollama, Claude, Gemini, OpenAI, GGUF local",
    mcp: true,
    hexagonal: false,
    ddd: false,
    color: "#F77F00",
  },
];

const paladin = {
  name: "Paladin",
  tagline: "Hexagonal DDD multi-agent framework",
  status: "Active Development",
  stars: "Private",
  org: "DF3NDR",
  license: "MIT",
  approach: "Hexagonal Architecture + DDD with medieval ubiquitous language",
  features: [
    "Formation (sequential), Phalanx (parallel), Campaign (DAG), ChainOfCommand (hierarchical)",
    "Conclave (expert panel), Council (deliberation), Grove (routing), Maneuver (flow DSL)",
    "Commander (auto strategy selection)",
    "Garrison (short-term: in-memory + SQLite) + Sanctum (long-term: Qdrant vectors)",
    "RAG integration with configurable retrieval",
    "Arsenal tool system via MCP (STDIO + SSE)",
    "Citadel (state persistence + checkpoint/recovery)",
    "Sentinel (vision / multi-modal)",
    "Herald (output formatting)",
    "Multi-provider LLM (OpenAI, Anthropic, DeepSeek)",
    "CLI with scaffolding (agent new, battalion new, arsenal tools)",
    "Circuit breaker, retry, comprehensive error handling",
    "YAML-driven configuration",
  ],
};

const dimensions = [
  { key: "arch", label: "Architecture", icon: "◆" },
  { key: "orch", label: "Orchestration Depth", icon: "⬡" },
  { key: "mem", label: "Memory Systems", icon: "◈" },
  { key: "tools", label: "Tool Ecosystem", icon: "⚙" },
  { key: "llm", label: "LLM Breadth", icon: "◎" },
  { key: "enterprise", label: "Enterprise Readiness", icon: "▣" },
  { key: "community", label: "Community / Maturity", icon: "◉" },
  { key: "dx", label: "Developer Experience", icon: "⌘" },
];

const scores = {
  "Paladin":    { arch: 10, orch: 10, mem: 9, tools: 7, llm: 6, enterprise: 8, community: 2, dx: 8 },
  "GraphBit":   { arch: 6,  orch: 6,  mem: 5, tools: 6, llm: 5, enterprise: 9, community: 4, dx: 5 },
  "Rig":        { arch: 7,  orch: 3,  mem: 7, tools: 5, llm: 8, enterprise: 5, community: 10, dx: 8 },
  "Swarms-rs":  { arch: 4,  orch: 5,  mem: 3, tools: 6, llm: 4, enterprise: 4, community: 5, dx: 5 },
  "ADK-Rust":   { arch: 6,  orch: 8,  mem: 7, tools: 9, llm: 10, enterprise: 5, community: 3, dx: 9 },
  "AutoAgents": { arch: 7,  orch: 6,  mem: 5, tools: 8, llm: 8, enterprise: 7, community: 5, dx: 6 },
  "Zeph":       { arch: 5,  orch: 7,  mem: 10, tools: 7, llm: 7, enterprise: 4, community: 1, dx: 4 },
};

function RadarChart({ selected, hovered }) {
  const size = 320;
  const cx = size / 2;
  const cy = size / 2;
  const maxR = 130;
  const n = dimensions.length;

  const getPoint = (i, val) => {
    const angle = (Math.PI * 2 * i) / n - Math.PI / 2;
    const r = (val / 10) * maxR;
    return { x: cx + r * Math.cos(angle), y: cy + r * Math.sin(angle) };
  };

  const makePath = (scoreObj) => {
    return dimensions
      .map((d, i) => {
        const pt = getPoint(i, scoreObj[d.key]);
        return `${i === 0 ? "M" : "L"} ${pt.x} ${pt.y}`;
      })
      .join(" ") + " Z";
  };

  const rings = [2, 4, 6, 8, 10];

  return (
    <svg viewBox={`0 0 ${size} ${size}`} style={{ width: "100%", maxWidth: 360 }}>
      {/* Grid */}
      {rings.map((r) => (
        <polygon
          key={r}
          points={dimensions
            .map((_, i) => {
              const pt = getPoint(i, r);
              return `${pt.x},${pt.y}`;
            })
            .join(" ")}
          fill="none"
          stroke="var(--border-subtle)"
          strokeWidth={r === 10 ? 1.2 : 0.5}
          opacity={0.4}
        />
      ))}
      {/* Axes */}
      {dimensions.map((d, i) => {
        const pt = getPoint(i, 10);
        const labelPt = getPoint(i, 12.2);
        return (
          <g key={d.key}>
            <line x1={cx} y1={cy} x2={pt.x} y2={pt.y} stroke="var(--border-subtle)" strokeWidth={0.5} opacity={0.3} />
            <text
              x={labelPt.x}
              y={labelPt.y}
              textAnchor="middle"
              dominantBaseline="middle"
              fontSize={9}
              fill="var(--text-muted)"
              fontFamily="'IBM Plex Mono', monospace"
            >
              {d.label}
            </text>
          </g>
        );
      })}
      {/* Paladin always shown */}
      <polygon
        points={dimensions.map((d, i) => { const pt = getPoint(i, scores["Paladin"][d.key]); return `${pt.x},${pt.y}`; }).join(" ")}
        fill="rgba(45,212,191,0.15)"
        stroke="#2DD4BF"
        strokeWidth={2.2}
      />
      {/* Hovered competitor */}
      {hovered && hovered !== "Paladin" && scores[hovered] && (
        <polygon
          points={dimensions.map((d, i) => { const pt = getPoint(i, scores[hovered][d.key]); return `${pt.x},${pt.y}`; }).join(" ")}
          fill={`${competitors.find(c => c.name === hovered)?.color || "#888"}22`}
          stroke={competitors.find(c => c.name === hovered)?.color || "#888"}
          strokeWidth={1.8}
          strokeDasharray="6 3"
        />
      )}
      {/* Selected competitor */}
      {selected && selected !== "Paladin" && scores[selected] && selected !== hovered && (
        <polygon
          points={dimensions.map((d, i) => { const pt = getPoint(i, scores[selected][d.key]); return `${pt.x},${pt.y}`; }).join(" ")}
          fill={`${competitors.find(c => c.name === selected)?.color || "#888"}18`}
          stroke={competitors.find(c => c.name === selected)?.color || "#888"}
          strokeWidth={1.5}
          opacity={0.7}
        />
      )}
      {/* Paladin dots */}
      {dimensions.map((d, i) => {
        const pt = getPoint(i, scores["Paladin"][d.key]);
        return <circle key={d.key} cx={pt.x} cy={pt.y} r={3.5} fill="#2DD4BF" />;
      })}
    </svg>
  );
}

function CompetitorCard({ comp, isSelected, onClick, onHover, onLeave }) {
  return (
    <div
      onClick={onClick}
      onMouseEnter={onHover}
      onMouseLeave={onLeave}
      style={{
        padding: "14px 16px",
        borderRadius: 10,
        border: isSelected ? `2px solid ${comp.color}` : "1px solid var(--border-subtle)",
        background: isSelected ? `${comp.color}0A` : "var(--bg-surface)",
        cursor: "pointer",
        transition: "all 0.2s ease",
        transform: isSelected ? "scale(1.02)" : "scale(1)",
      }}
    >
      <div style={{ display: "flex", alignItems: "center", gap: 8, marginBottom: 6 }}>
        <div style={{ width: 10, height: 10, borderRadius: "50%", background: comp.color, flexShrink: 0 }} />
        <span style={{ fontFamily: "'IBM Plex Mono', monospace", fontWeight: 700, fontSize: 14, color: "var(--text-primary)" }}>
          {comp.name}
        </span>
        <span style={{
          marginLeft: "auto",
          fontSize: 10,
          fontFamily: "'IBM Plex Mono', monospace",
          padding: "2px 6px",
          borderRadius: 4,
          background: comp.status === "Very Active" ? "#2DD4BF22" : comp.status === "Active" ? "#E9C46A22" : "#88888822",
          color: comp.status === "Very Active" ? "#2DD4BF" : comp.status === "Active" ? "#E9C46A" : "#888",
        }}>
          {comp.status}
        </span>
      </div>
      <div style={{ fontSize: 11, color: "var(--text-muted)", fontFamily: "'IBM Plex Sans', sans-serif", lineHeight: 1.4 }}>
        {comp.tagline}
      </div>
      <div style={{ fontSize: 10, color: "var(--text-muted)", marginTop: 4, fontFamily: "'IBM Plex Mono', monospace", opacity: 0.7 }}>
        ★ {comp.stars} · {comp.license}
      </div>
    </div>
  );
}

function DetailPanel({ comp }) {
  if (!comp) return (
    <div style={{ padding: 32, textAlign: "center", color: "var(--text-muted)", fontFamily: "'IBM Plex Sans', sans-serif" }}>
      <div style={{ fontSize: 32, marginBottom: 12, opacity: 0.3 }}>⚔</div>
      <div style={{ fontSize: 13 }}>Select a competitor to compare against Paladin</div>
    </div>
  );

  const s = scores[comp.name];
  const p = scores["Paladin"];

  return (
    <div style={{ padding: "0 4px" }}>
      <div style={{ display: "flex", alignItems: "center", gap: 10, marginBottom: 16 }}>
        <div style={{ width: 14, height: 14, borderRadius: "50%", background: comp.color }} />
        <div>
          <div style={{ fontFamily: "'IBM Plex Mono', monospace", fontWeight: 700, fontSize: 18, color: "var(--text-primary)" }}>
            {comp.name}
          </div>
          <div style={{ fontSize: 11, color: "var(--text-muted)", fontFamily: "'IBM Plex Sans', sans-serif" }}>
            {comp.org} · {comp.license}
          </div>
        </div>
      </div>

      <div style={{ fontSize: 12, color: "var(--text-secondary)", fontFamily: "'IBM Plex Sans', sans-serif", marginBottom: 16, lineHeight: 1.6, padding: "10px 12px", background: "var(--bg-base)", borderRadius: 8, borderLeft: `3px solid ${comp.color}` }}>
        <strong>Approach:</strong> {comp.approach}
      </div>

      {/* Score comparison */}
      <div style={{ marginBottom: 20 }}>
        <div style={{ fontSize: 10, fontFamily: "'IBM Plex Mono', monospace", color: "var(--text-muted)", marginBottom: 8, textTransform: "uppercase", letterSpacing: 1 }}>
          Dimension Scores — Paladin vs {comp.name}
        </div>
        {dimensions.map((d) => {
          const diff = p[d.key] - s[d.key];
          return (
            <div key={d.key} style={{ display: "flex", alignItems: "center", gap: 8, marginBottom: 5 }}>
              <span style={{ fontFamily: "'IBM Plex Mono', monospace", fontSize: 10, width: 110, color: "var(--text-muted)", flexShrink: 0 }}>
                {d.icon} {d.label}
              </span>
              <div style={{ flex: 1, display: "flex", alignItems: "center", gap: 4 }}>
                <div style={{ width: `${s[d.key] * 10}%`, height: 6, borderRadius: 3, background: comp.color, opacity: 0.7, transition: "width 0.4s ease" }} />
                <span style={{ fontFamily: "'IBM Plex Mono', monospace", fontSize: 10, color: comp.color, width: 16, textAlign: "right" }}>{s[d.key]}</span>
              </div>
              <div style={{ flex: 1, display: "flex", alignItems: "center", gap: 4 }}>
                <div style={{ width: `${p[d.key] * 10}%`, height: 6, borderRadius: 3, background: "#2DD4BF", transition: "width 0.4s ease" }} />
                <span style={{ fontFamily: "'IBM Plex Mono', monospace", fontSize: 10, color: "#2DD4BF", width: 16, textAlign: "right" }}>{p[d.key]}</span>
              </div>
              <span style={{
                fontFamily: "'IBM Plex Mono', monospace",
                fontSize: 10,
                width: 28,
                textAlign: "right",
                color: diff > 0 ? "#2DD4BF" : diff < 0 ? comp.color : "var(--text-muted)",
                fontWeight: diff !== 0 ? 700 : 400,
              }}>
                {diff > 0 ? `+${diff}` : diff < 0 ? `${diff}` : "="}
              </span>
            </div>
          );
        })}
        <div style={{ display: "flex", justifyContent: "flex-end", gap: 16, marginTop: 6, fontSize: 9, fontFamily: "'IBM Plex Mono', monospace", color: "var(--text-muted)" }}>
          <span style={{ color: comp.color }}>■ {comp.name}</span>
          <span style={{ color: "#2DD4BF" }}>■ Paladin</span>
        </div>
      </div>

      {/* Strengths / Weaknesses */}
      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 12, marginBottom: 16 }}>
        <div>
          <div style={{ fontSize: 10, fontFamily: "'IBM Plex Mono', monospace", color: "#2DD4BF", marginBottom: 6, textTransform: "uppercase", letterSpacing: 1 }}>
            Strengths
          </div>
          {comp.strengths.map((s, i) => (
            <div key={i} style={{ fontSize: 11, color: "var(--text-secondary)", fontFamily: "'IBM Plex Sans', sans-serif", lineHeight: 1.5, marginBottom: 4, paddingLeft: 10, borderLeft: "2px solid #2DD4BF33" }}>
              {s}
            </div>
          ))}
        </div>
        <div>
          <div style={{ fontSize: 10, fontFamily: "'IBM Plex Mono', monospace", color: "#F77F00", marginBottom: 6, textTransform: "uppercase", letterSpacing: 1 }}>
            Weaknesses
          </div>
          {comp.weaknesses.map((w, i) => (
            <div key={i} style={{ fontSize: 11, color: "var(--text-secondary)", fontFamily: "'IBM Plex Sans', sans-serif", lineHeight: 1.5, marginBottom: 4, paddingLeft: 10, borderLeft: "2px solid #F77F0033" }}>
              {w}
            </div>
          ))}
        </div>
      </div>

      {/* Capabilities matrix */}
      <div style={{ fontSize: 10, fontFamily: "'IBM Plex Mono', monospace", color: "var(--text-muted)", marginBottom: 8, textTransform: "uppercase", letterSpacing: 1 }}>
        Capability Snapshot
      </div>
      <div style={{ background: "var(--bg-base)", borderRadius: 8, padding: 12, fontSize: 11, fontFamily: "'IBM Plex Sans', sans-serif", color: "var(--text-secondary)" }}>
        <div style={{ marginBottom: 6 }}><strong style={{ color: "var(--text-muted)", fontFamily: "'IBM Plex Mono', monospace", fontSize: 10 }}>Memory:</strong> {comp.memory}</div>
        <div style={{ marginBottom: 6 }}><strong style={{ color: "var(--text-muted)", fontFamily: "'IBM Plex Mono', monospace", fontSize: 10 }}>Tools:</strong> {comp.tools}</div>
        <div style={{ marginBottom: 6 }}><strong style={{ color: "var(--text-muted)", fontFamily: "'IBM Plex Mono', monospace", fontSize: 10 }}>Orchestration:</strong> {comp.orchestration}</div>
        <div style={{ marginBottom: 6 }}><strong style={{ color: "var(--text-muted)", fontFamily: "'IBM Plex Mono', monospace", fontSize: 10 }}>LLM Providers:</strong> {comp.llmProviders}</div>
        <div style={{ display: "flex", gap: 12, marginTop: 8 }}>
          <span style={{ padding: "2px 8px", borderRadius: 4, background: comp.mcp ? "#2DD4BF22" : "#88888822", color: comp.mcp ? "#2DD4BF" : "#666", fontSize: 10, fontFamily: "'IBM Plex Mono', monospace" }}>
            MCP {comp.mcp ? "✓" : "✗"}
          </span>
          <span style={{ padding: "2px 8px", borderRadius: 4, background: comp.hexagonal ? "#2DD4BF22" : "#88888822", color: comp.hexagonal ? "#2DD4BF" : "#666", fontSize: 10, fontFamily: "'IBM Plex Mono', monospace" }}>
            Hexagonal {comp.hexagonal ? "✓" : "✗"}
          </span>
          <span style={{ padding: "2px 8px", borderRadius: 4, background: comp.ddd ? "#2DD4BF22" : "#88888822", color: comp.ddd ? "#2DD4BF" : "#666", fontSize: 10, fontFamily: "'IBM Plex Mono', monospace" }}>
            DDD {comp.ddd ? "✓" : "✗"}
          </span>
        </div>
      </div>
    </div>
  );
}

const CSS_VARS = `
  :root {
    --bg-base: #0D1117;
    --bg-surface: #161B22;
    --bg-elevated: #1C2129;
    --border-subtle: #30363D;
    --text-primary: #E6EDF3;
    --text-secondary: #B1BAC4;
    --text-muted: #7D8590;
    --accent: #2DD4BF;
  }
  @media (prefers-color-scheme: light) {
    :root {
      --bg-base: #F6F8FA;
      --bg-surface: #FFFFFF;
      --bg-elevated: #FFFFFF;
      --border-subtle: #D0D7DE;
      --text-primary: #1F2328;
      --text-secondary: #424A53;
      --text-muted: #656D76;
    }
  }
  @import url('https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500;700&family=IBM+Plex+Sans:wght@400;500;600;700&display=swap');
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body { background: var(--bg-base); }
  ::-webkit-scrollbar { width: 6px; }
  ::-webkit-scrollbar-track { background: transparent; }
  ::-webkit-scrollbar-thumb { background: var(--border-subtle); border-radius: 3px; }
`;

export default function CompetitiveAnalysis() {
  const [selected, setSelected] = useState(null);
  const [hovered, setHovered] = useState(null);
  const [tab, setTab] = useState("compare");

  const selectedComp = competitors.find((c) => c.name === selected);

  return (
    <>
      <style>{CSS_VARS}</style>
      <div style={{ minHeight: "100vh", background: "var(--bg-base)", color: "var(--text-primary)", fontFamily: "'IBM Plex Sans', sans-serif" }}>
        {/* Header */}
        <div style={{ padding: "28px 28px 0", borderBottom: "1px solid var(--border-subtle)" }}>
          <div style={{ display: "flex", alignItems: "baseline", gap: 12, marginBottom: 6 }}>
            <span style={{ fontFamily: "'IBM Plex Mono', monospace", fontWeight: 700, fontSize: 22, color: "#2DD4BF", letterSpacing: -0.5 }}>
              ⚔ PALADIN
            </span>
            <span style={{ fontFamily: "'IBM Plex Mono', monospace", fontSize: 12, color: "var(--text-muted)" }}>
              Competitive Intelligence Report
            </span>
          </div>
          <div style={{ fontSize: 12, color: "var(--text-muted)", fontFamily: "'IBM Plex Mono', monospace", marginBottom: 16 }}>
            Rust Multi-Agent Orchestration Landscape · April 2026
          </div>

          {/* Tabs */}
          <div style={{ display: "flex", gap: 0 }}>
            {[
              { id: "compare", label: "Head-to-Head" },
              { id: "matrix", label: "Feature Matrix" },
              { id: "paladin", label: "Paladin Capabilities" },
            ].map((t) => (
              <button
                key={t.id}
                onClick={() => setTab(t.id)}
                style={{
                  padding: "8px 20px",
                  border: "none",
                  borderBottom: tab === t.id ? "2px solid #2DD4BF" : "2px solid transparent",
                  background: "transparent",
                  color: tab === t.id ? "#2DD4BF" : "var(--text-muted)",
                  fontFamily: "'IBM Plex Mono', monospace",
                  fontSize: 11,
                  cursor: "pointer",
                  transition: "all 0.2s",
                  letterSpacing: 0.5,
                }}
              >
                {t.label}
              </button>
            ))}
          </div>
        </div>

        {/* Content */}
        <div style={{ padding: 28 }}>
          {tab === "compare" && (
            <div style={{ display: "grid", gridTemplateColumns: "280px 1fr", gap: 28, alignItems: "start" }}>
              {/* Left: competitor list + radar */}
              <div>
                <div style={{ display: "flex", flexDirection: "column", gap: 8, marginBottom: 24 }}>
                  {competitors.map((c) => (
                    <CompetitorCard
                      key={c.name}
                      comp={c}
                      isSelected={selected === c.name}
                      onClick={() => setSelected(selected === c.name ? null : c.name)}
                      onHover={() => setHovered(c.name)}
                      onLeave={() => setHovered(null)}
                    />
                  ))}
                </div>
                <div style={{
                  background: "var(--bg-surface)",
                  borderRadius: 12,
                  padding: 16,
                  border: "1px solid var(--border-subtle)",
                }}>
                  <div style={{ fontSize: 10, fontFamily: "'IBM Plex Mono', monospace", color: "var(--text-muted)", marginBottom: 8, textTransform: "uppercase", letterSpacing: 1, textAlign: "center" }}>
                    Radar — Paladin (solid) vs Selected (dashed)
                  </div>
                  <RadarChart selected={selected} hovered={hovered} />
                </div>
              </div>

              {/* Right: detail panel */}
              <div style={{
                background: "var(--bg-surface)",
                borderRadius: 12,
                padding: 24,
                border: "1px solid var(--border-subtle)",
                minHeight: 500,
              }}>
                <DetailPanel comp={selectedComp} />
              </div>
            </div>
          )}

          {tab === "matrix" && (
            <div style={{ overflowX: "auto" }}>
              <table style={{ width: "100%", borderCollapse: "collapse", fontFamily: "'IBM Plex Mono', monospace", fontSize: 11 }}>
                <thead>
                  <tr style={{ borderBottom: "2px solid var(--border-subtle)" }}>
                    <th style={{ padding: "10px 12px", textAlign: "left", color: "var(--text-muted)", fontSize: 10, textTransform: "uppercase", letterSpacing: 1 }}>Capability</th>
                    <th style={{ padding: "10px 12px", textAlign: "center", color: "#2DD4BF", fontWeight: 700 }}>Paladin</th>
                    {competitors.map((c) => (
                      <th key={c.name} style={{ padding: "10px 12px", textAlign: "center", color: c.color, fontWeight: 600 }}>{c.name}</th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {[
                    { label: "Hexagonal Architecture", vals: ["✓", "✗", "✗", "✗", "✗", "~", "✗"] },
                    { label: "Domain-Driven Design", vals: ["✓", "✗", "✗", "✗", "✗", "✗", "✗"] },
                    { label: "Sequential Workflows", vals: ["✓", "✓", "✓", "✓", "✓", "✓", "✓"] },
                    { label: "Parallel Execution", vals: ["✓", "✓", "~", "✓", "✓", "✓", "✓"] },
                    { label: "DAG/Graph Workflows", vals: ["✓", "✗", "✗", "✗", "✓", "~", "✓"] },
                    { label: "Hierarchical Delegation", vals: ["✓", "✗", "✗", "✗", "✗", "✗", "✗"] },
                    { label: "Expert Panel / Consensus", vals: ["✓", "✗", "✗", "✗", "✗", "✗", "✗"] },
                    { label: "Dynamic Flow DSL", vals: ["✓", "✗", "✗", "✗", "✗", "✗", "✗"] },
                    { label: "Auto Strategy Selection", vals: ["✓", "✗", "✗", "✗", "✗", "✗", "✗"] },
                    { label: "MCP Tool Protocol", vals: ["✓", "✗", "✗", "✓", "✓", "✓", "✓"] },
                    { label: "Short-term Memory", vals: ["✓", "✓", "~", "✓", "✓", "✓", "✓"] },
                    { label: "Vector DB (long-term)", vals: ["✓", "✗", "✓", "✗", "✓", "✓", "✓"] },
                    { label: "Graph Memory", vals: ["✗", "✗", "✗", "✗", "✗", "✗", "✓"] },
                    { label: "RAG Integration", vals: ["✓", "✗", "✓", "✗", "✓", "~", "✓"] },
                    { label: "Vision / Multi-modal", vals: ["✓", "✗", "~", "✗", "✓", "✗", "✗"] },
                    { label: "State Persistence", vals: ["✓", "✓", "✗", "✓", "~", "✗", "~"] },
                    { label: "Circuit Breaker", vals: ["✓", "✓", "✗", "✗", "✗", "~", "✗"] },
                    { label: "Guardrails", vals: ["~", "✓", "✗", "✗", "✓", "✓", "✓"] },
                    { label: "CLI Scaffolding", vals: ["✓", "✗", "✗", "✗", "✓", "~", "✓"] },
                    { label: "YAML Config", vals: ["✓", "✓", "✗", "✗", "✗", "✓", "✓"] },
                    { label: "Voice / Realtime", vals: ["✗", "✗", "✗", "✗", "✓", "✓", "✗"] },
                    { label: "WASM Support", vals: ["✗", "✗", "~", "✗", "✗", "✓", "✗"] },
                    { label: "A2A Protocol", vals: ["✗", "✗", "✗", "✗", "✓", "✗", "✗"] },
                    { label: "EU AI Act Alignment", vals: ["✗", "✓", "✗", "✗", "✗", "✗", "✗"] },
                  ].map((row, ri) => (
                    <tr key={row.label} style={{ borderBottom: "1px solid var(--border-subtle)", background: ri % 2 === 0 ? "transparent" : "var(--bg-surface)" }}>
                      <td style={{ padding: "8px 12px", color: "var(--text-secondary)", fontSize: 11 }}>{row.label}</td>
                      <td style={{ padding: "8px 12px", textAlign: "center", fontWeight: 700, color: row.vals[0] === "✓" ? "#2DD4BF" : row.vals[0] === "~" ? "#E9C46A" : "#F7707066", fontSize: 14 }}>{row.vals[0]}</td>
                      {row.vals.slice(1).map((v, ci) => (
                        <td key={ci} style={{ padding: "8px 12px", textAlign: "center", color: v === "✓" ? "#2DD4BF" : v === "~" ? "#E9C46A" : "#F7707066", fontSize: 14 }}>
                          {v}
                        </td>
                      ))}
                    </tr>
                  ))}
                </tbody>
              </table>
              <div style={{ marginTop: 12, fontSize: 10, fontFamily: "'IBM Plex Mono', monospace", color: "var(--text-muted)" }}>
                ✓ = Implemented · ~ = Partial/Planned · ✗ = Not present
              </div>
            </div>
          )}

          {tab === "paladin" && (
            <div style={{ maxWidth: 800 }}>
              <div style={{ fontSize: 10, fontFamily: "'IBM Plex Mono', monospace", color: "var(--text-muted)", marginBottom: 16, textTransform: "uppercase", letterSpacing: 1 }}>
                Paladin Unique Differentiators
              </div>
              <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 16, marginBottom: 28 }}>
                {[
                  { title: "Architecture", desc: "Only framework with true Hexagonal Architecture + DDD. Clean ports/adapters boundaries with zero core→infra dependencies. Medieval ubiquitous language." },
                  { title: "Orchestration Depth", desc: "8 distinct patterns: Formation, Phalanx, Campaign, ChainOfCommand, Conclave, Council, Grove, Maneuver + Commander auto-selection. No competitor matches this breadth." },
                  { title: "Memory Architecture", desc: "Dual-layer: Garrison (session-scoped, SQLite) + Sanctum (persistent, Qdrant vectors). RAG integration with configurable retrieval triggers. Gap: no graph memory yet." },
                  { title: "Enterprise Patterns", desc: "Circuit breaker, checkpoint/recovery (Citadel), comprehensive error types, audit-ready logging. Strongest TDD discipline in the space." },
                ].map((d, i) => (
                  <div key={i} style={{ background: "var(--bg-surface)", borderRadius: 10, padding: 18, border: "1px solid var(--border-subtle)" }}>
                    <div style={{ fontFamily: "'IBM Plex Mono', monospace", fontWeight: 700, fontSize: 13, color: "#2DD4BF", marginBottom: 8 }}>{d.title}</div>
                    <div style={{ fontSize: 12, color: "var(--text-secondary)", lineHeight: 1.6 }}>{d.desc}</div>
                  </div>
                ))}
              </div>

              <div style={{ fontSize: 10, fontFamily: "'IBM Plex Mono', monospace", color: "#F77F00", marginBottom: 12, textTransform: "uppercase", letterSpacing: 1 }}>
                Competitive Gaps to Address
              </div>
              <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr 1fr", gap: 12 }}>
                {[
                  { gap: "LLM Provider Breadth", who: "ADK-Rust (15+ providers)", priority: "High" },
                  { gap: "Realtime Voice", who: "ADK-Rust, AutoAgents", priority: "Medium" },
                  { gap: "Graph Memory", who: "Zeph (SYNAPSE)", priority: "Medium" },
                  { gap: "WASM Sandboxing", who: "AutoAgents (Odyssey)", priority: "Low" },
                  { gap: "A2A Protocol", who: "ADK-Rust", priority: "Medium" },
                  { gap: "Guardrails System", who: "ADK-Rust, AutoAgents, Zeph", priority: "High" },
                  { gap: "Community Growth", who: "Rig (6.2k stars)", priority: "High" },
                  { gap: "EU AI Act / Compliance", who: "GraphBit (ISO/TISAX)", priority: "Medium" },
                  { gap: "crates.io Publishing", who: "All competitors", priority: "High" },
                ].map((g, i) => (
                  <div key={i} style={{
                    background: "var(--bg-surface)",
                    borderRadius: 8,
                    padding: 12,
                    border: "1px solid var(--border-subtle)",
                    borderLeft: `3px solid ${g.priority === "High" ? "#E63946" : g.priority === "Medium" ? "#E9C46A" : "#457B9D"}`,
                  }}>
                    <div style={{ fontFamily: "'IBM Plex Mono', monospace", fontSize: 11, fontWeight: 600, color: "var(--text-primary)", marginBottom: 4 }}>
                      {g.gap}
                    </div>
                    <div style={{ fontSize: 10, color: "var(--text-muted)", marginBottom: 4 }}>Leader: {g.who}</div>
                    <span style={{
                      fontSize: 9,
                      fontFamily: "'IBM Plex Mono', monospace",
                      padding: "1px 6px",
                      borderRadius: 3,
                      background: g.priority === "High" ? "#E6394622" : g.priority === "Medium" ? "#E9C46A22" : "#457B9D22",
                      color: g.priority === "High" ? "#E63946" : g.priority === "Medium" ? "#E9C46A" : "#457B9D",
                    }}>
                      {g.priority} Priority
                    </span>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>
    </>
  );
}
