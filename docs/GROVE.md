# Grove Pattern

**Tree-based intelligent agent routing for specialized task distribution**

---

## Table of Contents

1. [Overview](#overview)
2. [Quick Start](#quick-start)
3. [Routing Strategies](#routing-strategies)
4. [Expertise Definition](#expertise-definition)
5. [Fallback Behavior](#fallback-behavior)
6. [Configuration](#configuration)
7. [Examples](#examples)
8. [Best Practices](#best-practices)
9. [API Reference](#api-reference)

---

## Overview

The Grove pattern implements **intelligent agent routing** by organizing specialized Paladin agents into **trees** and dynamically routing tasks to the most suitable agent based on expertise matching. Unlike static routing or round-robin selection, Grove analyzes each task and routes it to the optimal specialist.

### Key Concepts

**Grove**: A collection of expert trees with intelligent routing.

**Tree**: A group of related agents sharing a domain (e.g., Backend Specialists, Frontend Specialists).

**Agent**: A specialized Paladin within a tree with defined expertise.

**Routing Strategy**: Algorithm determining which agent handles a task (KeywordMatch, SemanticSimilarity, LlmRouting).

**Expertise**: Agent's knowledge areas, defined via keywords, embeddings, or descriptions.

**Fallback Tree**: Default tree for tasks that don't match any specialist.

### Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                           Grove                              │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Task: "Optimize database query performance"                 │
│                                                              │
│  ┌─────────────────┐         ┌─────────────────┐            │
│  │ Backend Tree    │         │ Frontend Tree   │            │
│  ├─────────────────┤         ├─────────────────┤            │
│  │ • DB Expert  ✓  │         │ • React Expert  │            │
│  │ • API Expert    │         │ • CSS Expert    │            │
│  │ • Service Expert│         │ • Perf Expert   │            │
│  └─────────────────┘         └─────────────────┘            │
│          ▲                                                   │
│          │                                                   │
│    [Routing Engine]                                          │
│          │                                                   │
│  Matches: database, query, performance                       │
│  Confidence: 87%                                             │
│                                                              │
│  Result: Routed to DB Expert                                 │
└──────────────────────────────────────────────────────────────┘
```

### When to Use Grove

✅ **Ideal Use Cases**:
- **Specialized task routing**: Match tasks to domain experts
- **Load distribution**: Spread work across specialist agents
- **Expertise-based selection**: Choose agent based on required skills
- **Hierarchical specialization**: Organize agents by capability trees
- **Dynamic routing**: Adapt to task requirements automatically

❌ **Not Ideal For**:
- Simple sequential processing → Use **Formation**
- Deliberative discussion → Use **Council**
- All agents needed concurrently → Use **Phalanx**
- Complex conditional logic → Use **Campaign**

### Comparison with Other Patterns

| Pattern | Execution | Selection | Use Case |
|---------|-----------|-----------|----------|
| **Grove** | Single agent | Dynamic routing | Task distribution to specialists |
| **Chain of Command** | Hierarchical | Commander delegation | Task breakdown and routing |
| **Phalanx** | All agents | No selection | Parallel independent analysis |
| **Council** | Sequential turns | Round-robin/moderator | Collaborative discussion |

---

## Quick Start

### Basic Grove Example

```rust
use paladin::core::platform::container::battalion::grove::{
    GroveBuilder, Tree, TreeAgent, RoutingStrategy, GroveConfig
};
use paladin::application::services::battalion::grove_service::GroveExecutionService;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create backend specialists tree
    let backend_tree = Tree::new("Backend Specialists")
        .add_agent(
            TreeAgent::new("DatabaseExpert")
                .with_keywords(vec!["database", "sql", "query", "index", "schema"])
        )
        .add_agent(
            TreeAgent::new("ApiExpert")
                .with_keywords(vec!["api", "rest", "graphql", "endpoint", "route"])
        );
    
    // Create frontend specialists tree
    let frontend_tree = Tree::new("Frontend Specialists")
        .add_agent(
            TreeAgent::new("ReactExpert")
                .with_keywords(vec!["react", "jsx", "hooks", "component", "state"])
        )
        .add_agent(
            TreeAgent::new("CssExpert")
                .with_keywords(vec!["css", "styling", "layout", "responsive", "design"])
        );
    
    // Build grove
    let grove = GroveBuilder::new()
        .name("Tech Specialists Grove")
        .add_tree(backend_tree)
        .add_tree(frontend_tree)
        .config(GroveConfig {
            routing_strategy: RoutingStrategy::KeywordMatch,
            fallback_tree: None,
            similarity_threshold: 0.6,
        })
        .build()?;
    
    // Create execution service
    let service = GroveExecutionService::new(
        Arc::new(paladin_port),
        None, // Optional: embedding service for semantic routing
        None, // Optional: LLM service for LLM routing
    );
    
    // Execute task - routes to DatabaseExpert
    let task = "Optimize database query performance with proper indexing";
    let result = service.execute(&grove, task).await?;
    
    println!("Routed to: {}", result.selected_agent);
    println!("Confidence: {}%", result.confidence * 100.0);
    println!("Result: {}", result.final_output);
    
    Ok(())
}
```

### Output Example

```
Analyzing task: "Optimize database query performance with proper indexing"

Routing Decision:
-----------------
Strategy: KeywordMatch
Keywords found: [database, query, performance, indexing]

Candidates:
- DatabaseExpert: 75% match (3/4 keywords)
- ApiExpert: 0% match
- ReactExpert: 0% match
- CssExpert: 0% match

Selected Agent: DatabaseExpert
Confidence: 75%

Result:
-------
To optimize query performance:

1. Analyze Execution Plan
   - Run EXPLAIN ANALYZE to identify full table scans
   - Look for sequential scans on large tables

2. Add Indexes
   - Create B-tree index on frequently filtered columns
   - Use composite indexes for multi-column WHERE clauses
   - Example: CREATE INDEX idx_users_email ON users(email)

3. Query Optimization
   - Use LIMIT for large result sets
   - Avoid SELECT * - specify needed columns
   - Leverage query result caching

Expected Impact: 80-90% latency reduction for indexed queries
```

---

## Routing Strategies

Grove supports three routing strategies with increasing intelligence and cost:

| Strategy | Speed | Cost | Accuracy | Requirements |
|----------|-------|------|----------|--------------|
| **KeywordMatch** | <10ms | Free | Good | Keywords only |
| **SemanticSimilarity** | ~100ms | Low ($0.0001) | Better | Embedding service |
| **LlmRouting** | ~300ms | Medium ($0.001) | Best | LLM service |

### 1. KeywordMatch (Fast & Simple)

**How it Works**:
1. Extract keywords from task description
2. Compare with each agent's keyword list
3. Calculate overlap percentage
4. Route to agent with highest overlap above threshold

**Advantages**:
- ⚡ Instant: <10ms routing time
- 💰 Free: No external API calls
- 🔍 Transparent: Clear why agent was selected
- 🎯 Deterministic: Same keywords → same route
- 📡 Offline: Works without internet

**Limitations**:
- Requires exact keyword matches
- Doesn't understand synonyms
- Limited by predefined keyword lists

**Example**:
```rust
let tree = Tree::new("Backend Specialists")
    .add_agent(
        TreeAgent::new("DatabaseExpert")
            .with_keywords(vec![
                "database", "sql", "query", "index", 
                "schema", "migration", "postgres"
            ])
    )
    .add_agent(
        TreeAgent::new("ApiExpert")
            .with_keywords(vec![
                "api", "rest", "graphql", "endpoint",
                "route", "controller", "authentication"
            ])
    );

let grove = GroveBuilder::new()
    .add_tree(tree)
    .config(GroveConfig {
        routing_strategy: RoutingStrategy::KeywordMatch,
        similarity_threshold: 0.6, // 60% overlap required
        ..Default::default()
    })
    .build()?;
```

**Routing Example**:
```
Task: "Design REST API endpoints for user management"
Keywords: [design, rest, api, endpoints, user, management]

DatabaseExpert: 1/6 = 16% (user matches)
ApiExpert: 3/6 = 50% (rest, api, endpoints match)

Result: No match (50% < 60% threshold)
Action: Route to fallback tree
```

**Best For**:
- Well-defined domains with clear keywords
- Low-latency requirements
- Cost-sensitive applications
- Offline operation needed

---

### 2. SemanticSimilarity (Contextual & Flexible)

**How it Works**:
1. Generate embedding for task description
2. Compare with pre-computed agent embeddings (cosine similarity)
3. Route to agent with highest similarity above threshold

**Advantages**:
- 🧠 Contextual: Understands meaning, not just words
- 🔄 Flexible: Handles paraphrasing and synonyms
- 💪 Robust: Works with varied phrasings
- 📊 Quality: Better accuracy than keyword matching

**Requirements**:
- Embedding service (OpenAI, local model, etc.)
- Pre-computed agent embeddings
- ~50-100ms additional latency
- ~$0.0001 per routing (OpenAI)

**Example**:
```rust
let tree = Tree::new("Security Specialists")
    .add_agent(
        TreeAgent::new("AppSecExpert")
            .with_expertise_description(
                "Application security: OWASP Top 10, SQL injection, \
                 XSS, CSRF, authentication, authorization, secure coding"
            )
    )
    .add_agent(
        TreeAgent::new("InfraSecExpert")
            .with_expertise_description(
                "Infrastructure security: network security, firewall, \
                 VPC, IAM, encryption, compliance, cloud security"
            )
    );

let grove = GroveBuilder::new()
    .add_tree(tree)
    .config(GroveConfig {
        routing_strategy: RoutingStrategy::SemanticSimilarity,
        similarity_threshold: 0.72, // 72% similarity required
        ..Default::default()
    })
    .build()?;

let service = GroveExecutionService::new(
    Arc::new(paladin_port),
    Some(Arc::new(embedding_port)), // Required for semantic routing
    None,
);
```

**Routing Example**:
```
Task: "Our login form is vulnerable to automated attacks"

Task embedding: [0.234, -0.567, 0.891, ...] (1536 dimensions)

Similarity scores:
- AppSecExpert: 0.84 (understands: login, vulnerable, attacks → auth security)
- InfraSecExpert: 0.56 (relates to: security, but more infrastructure-focused)

Result: Route to AppSecExpert (84% > 72% threshold)
```

**Synonym Understanding**:
```
"slow page loads" ≈ "performance issues" ≈ "sluggish rendering" ≈ "high latency"
→ All route to PerformanceExpert
```

**Best For**:
- Natural language queries
- User-facing applications
- When task phrasing varies
- Balance of speed and accuracy needed

---

### 3. LlmRouting (Intelligent & Explainable)

**How it Works**:
1. LLM receives task description and all agent descriptions
2. LLM analyzes task requirements and complexity
3. LLM reasons about which agent is best suited
4. LLM provides routing decision with confidence and explanation

**Advantages**:
- 🎯 Intelligent: Deep understanding of task context
- 💡 Explainable: Provides reasoning for decisions
- 🔀 Multi-factor: Considers complexity, domain, requirements
- 🧩 Adaptive: Handles novel or ambiguous scenarios
- 📝 Contextual: Understands nuanced distinctions

**Requirements**:
- LLM service (OpenAI, Anthropic, DeepSeek, etc.)
- Rich agent descriptions
- ~200-500ms additional latency
- ~$0.001-0.005 per routing (GPT-4)

**Example**:
```rust
let tree = Tree::new("Backend Specialists")
    .add_agent(
        TreeAgent::new("DatabaseExpert")
            .with_agent_description(
                "Expert database architect specializing in schema design, \
                 query optimization, indexing strategies, database scaling \
                 (sharding, replication), and migration planning. Best for \
                 tasks involving database design, query performance, or data modeling."
            )
    )
    .add_agent(
        TreeAgent::new("ApiExpert")
            .with_agent_description(
                "Expert API architect specializing in REST and GraphQL design, \
                 API versioning, authentication (OAuth, JWT), rate limiting, \
                 and API documentation (OpenAPI). Best for tasks involving \
                 API endpoint design, protocol selection, or API security."
            )
    );

let grove = GroveBuilder::new()
    .add_tree(tree)
    .config(GroveConfig {
        routing_strategy: RoutingStrategy::LlmRouting,
        similarity_threshold: 0.65, // 65% confidence required
        ..Default::default()
    })
    .build()?;

let service = GroveExecutionService::new(
    Arc::new(paladin_port),
    None,
    Some(Arc::new(llm_port)), // Required for LLM routing
);
```

**Routing Example with Reasoning**:
```
Task: "Users complain about seeing stale data after making updates"

LLM Analysis:
-------------
This could be multiple issues:
1. Frontend state management (React state not updating)
2. Backend caching (stale cache entries)
3. Database replication lag

Key phrase: "users complain about seeing" suggests a UI/presentation issue
rather than data persistence. The problem is likely in how the frontend
reflects updates, not in data storage or API layer.

Decision: ReactExpert
Confidence: 78%

Reasoning: The user-facing symptom ("seeing stale data") indicates a frontend
state management problem. While backend caching could cause this, the phrasing
suggests the issue manifests in the UI. React Expert should investigate state
updates, cache invalidation, and optimistic UI updates.

Alternative considered: DatabaseExpert (for replication lag) - 22% confidence
```

**Complex Multi-Domain Example**:
```
Task: "Reduce API latency - dashboard loads slowly, bottleneck unclear"

LLM Analysis:
-------------
Multi-faceted performance problem involving:
- API layer (endpoint response times)
- Database layer (query performance)
- Frontend layer (rendering, data fetching)

Primary bottleneck likely in data fetching based on "API latency" mention.
Database queries are often the root cause of slow API responses.

Decision: DatabaseExpert
Confidence: 72%

Reasoning: "API latency" with "dashboard" suggests data-heavy queries.
Dashboards typically aggregate data from multiple sources, which often
results in N+1 query problems or missing indexes. DatabaseExpert should
analyze query patterns and recommend optimization (indexes, caching,
query restructuring).

Recommendation: After DB optimization, consider ApiExpert for API-level
caching and FrontendExpert for client-side optimization.
```

**Best For**:
- Complex, ambiguous tasks
- Critical routing decisions
- Need for explainability
- Multi-factor analysis required
- Novel or unusual scenarios

---

## Expertise Definition

Agents can define expertise in three complementary ways:

### 1. Keywords (for KeywordMatch)

**Purpose**: Fast exact/partial matching

```rust
TreeAgent::new("DatabaseExpert")
    .with_keywords(vec![
        "database",
        "sql",
        "nosql",
        "query",
        "schema",
        "index",
        "migration",
        "postgres",
        "mysql",
        "mongodb",
    ])
```

**Best Practices**:
- 5-15 keywords per agent
- Include variations: "db", "database", "databases"
- Use domain-specific terms: "schema", not "structure"
- Include tools: "postgres", "redis"
- Be specific: "api" too broad, "rest-api" better

---

### 2. Expertise Description (for SemanticSimilarity)

**Purpose**: Contextual understanding via embeddings

```rust
TreeAgent::new("SecurityExpert")
    .with_expertise_description(
        "Application security specialist focusing on secure coding practices, \
         vulnerability assessment, penetration testing, OWASP Top 10, \
         SQL injection, XSS attacks, CSRF protection, authentication, \
         authorization, session management, input validation, output encoding, \
         security headers, secure API design, threat modeling."
    )
```

**Best Practices**:
- 50-200 words optimal
- Use natural language, not keyword stuffing
- Describe both skills and typical tasks
- Include specific technologies and methodologies
- Mention common problems solved

---

### 3. Agent Description (for LlmRouting)

**Purpose**: Rich context for LLM reasoning

```rust
TreeAgent::new("PerformanceExpert")
    .with_agent_description(
        "Expert web performance engineer specializing in:
        - Core Web Vitals optimization (LCP, INP, CLS)
        - Bundle size reduction and code splitting
        - Image optimization (WebP, AVIF, lazy loading)
        - Caching strategies (service workers, HTTP caching, CDN)
        - Build optimization (Webpack, Vite, Rollup)
        - Runtime performance (JavaScript execution, rendering)
        
        Best suited for tasks involving:
        • Page load performance optimization
        • Core Web Vitals improvement
        • Bundle size reduction
        • Asset optimization strategies
        • Performance monitoring and profiling
        • Build tool configuration"
    )
```

**Best Practices**:
- 100-300 words optimal
- Structure: Skills + Best suited for
- Use bullet points for clarity
- Specify measurable outcomes
- Include relevant tools and frameworks
- Mention typical deliverables

---

### Combined Example

```rust
TreeAgent::new("ApiArchitect")
    // For KeywordMatch
    .with_keywords(vec![
        "api", "rest", "graphql", "endpoint", "authentication"
    ])
    // For SemanticSimilarity
    .with_expertise_description(
        "API design expert: RESTful principles, GraphQL schema design, \
         authentication (OAuth, JWT), API versioning, documentation"
    )
    // For LlmRouting
    .with_agent_description(
        "Expert API architect specializing in:
        - RESTful API design following OpenAPI standards
        - GraphQL schema design and optimization
        - API authentication (OAuth 2.0, JWT, API keys)
        - API versioning and backwards compatibility
        
        Best suited for:
        • API endpoint design and structure
        • Protocol selection (REST vs GraphQL vs gRPC)
        • API security and authentication
        • API documentation (OpenAPI/Swagger)"
    )
```

---

## Fallback Behavior

When no agent meets the similarity threshold, Grove can route to a **fallback tree** containing generalist agents.

### Configuration

```rust
let generalist_tree = Tree::new("GeneralistTree")
    .add_agent(
        TreeAgent::new("GeneralEngineer")
            .with_expertise_description(
                "Full-stack software engineer with broad expertise across \
                 web development, architecture, and best practices"
            )
    );

let grove = GroveBuilder::new()
    .add_tree(backend_tree)
    .add_tree(frontend_tree)
    .add_tree(generalist_tree)
    .config(GroveConfig {
        routing_strategy: RoutingStrategy::KeywordMatch,
        fallback_tree: Some("GeneralistTree".to_string()),
        similarity_threshold: 0.6,
    })
    .build()?;
```

### Fallback Scenarios

**Scenario 1: No Match Above Threshold**
```
Task: "Help me with my project"
Keywords: [help, project]

All specialists: <60% match
→ Route to GeneralistTree
```

**Scenario 2: Ambiguous Task**
```
Task: "Improve the application"
(Too vague for specific routing)
→ Route to GeneralistTree
```

**Scenario 3: Cross-Domain Task**
```
Task: "Build a full-stack feature with frontend, backend, and database"
(Requires multiple specialties)
→ Route to GeneralistTree (can delegate or provide overview)
```

### Fallback Strategy Options

```rust
pub enum FallbackStrategy {
    /// Route to specified fallback tree
    FallbackTree(String),
    
    /// Return error if no match
    Error,
    
    /// Route to first agent in first tree (default)
    FirstAvailable,
    
    /// Route to random agent
    Random,
}
```

**Recommendation**: Use `FallbackTree` with generalist agents for best UX.

---

## Configuration

### GroveConfig

```rust
pub struct GroveConfig {
    /// Routing strategy
    pub routing_strategy: RoutingStrategy,
    
    /// Fallback tree name (optional)
    pub fallback_tree: Option<String>,
    
    /// Similarity threshold (0.0-1.0)
    /// - KeywordMatch: keyword overlap percentage
    /// - SemanticSimilarity: cosine similarity
    /// - LlmRouting: confidence score
    pub similarity_threshold: f32,
}

impl Default for GroveConfig {
    fn default() -> Self {
        Self {
            routing_strategy: RoutingStrategy::KeywordMatch,
            fallback_tree: None,
            similarity_threshold: 0.6, // 60%
        }
    }
}
```

### Threshold Recommendations

| Strategy | Strict | Moderate | Permissive |
|----------|--------|----------|------------|
| **KeywordMatch** | 0.7-0.8 | 0.6-0.7 | 0.5-0.6 |
| **SemanticSimilarity** | 0.75-0.85 | 0.7-0.75 | 0.65-0.7 |
| **LlmRouting** | 0.7-0.8 | 0.65-0.7 | 0.6-0.65 |

**Tuning**:
- Too high → Many fallback routes
- Too low → Incorrect specialist selection
- Monitor routing decisions and adjust

---

## Examples

### Example 1: Tech Support Grove

```rust
let backend_tree = Tree::new("Backend Support")
    .add_agent(TreeAgent::new("DatabaseExpert")
        .with_keywords(vec!["database", "sql", "query", "schema"]))
    .add_agent(TreeAgent::new("ApiExpert")
        .with_keywords(vec!["api", "endpoint", "rest", "graphql"]));

let frontend_tree = Tree::new("Frontend Support")
    .add_agent(TreeAgent::new("ReactExpert")
        .with_keywords(vec!["react", "component", "hooks", "state"]))
    .add_agent(TreeAgent::new("CssExpert")
        .with_keywords(vec!["css", "styling", "layout", "responsive"]));

let grove = GroveBuilder::new()
    .name("Tech Support Grove")
    .add_tree(backend_tree)
    .add_tree(frontend_tree)
    .config(GroveConfig {
        routing_strategy: RoutingStrategy::KeywordMatch,
        fallback_tree: None,
        similarity_threshold: 0.6,
    })
    .build()?;

// Route customer support tickets to appropriate expert
let tickets = vec![
    "Database connection pool exhausted",
    "React component not re-rendering",
    "CSS grid layout not working on mobile",
];

for ticket in tickets {
    let result = service.execute(&grove, ticket).await?;
    println!("Ticket: {}\nRouted to: {}", ticket, result.selected_agent);
}
```

### Example 2: Semantic Routing for Natural Language

```rust
let security_tree = Tree::new("Security Team")
    .add_agent(TreeAgent::new("AppSecExpert")
        .with_expertise_description(
            "Application security: OWASP vulnerabilities, secure coding, \
             auth, SQL injection, XSS, CSRF protection"
        ))
    .add_agent(TreeAgent::new("CloudSecExpert")
        .with_expertise_description(
            "Cloud and infrastructure security: AWS/Azure/GCP security, \
             IAM, VPC, network security, compliance"
        ));

let grove = GroveBuilder::new()
    .add_tree(security_tree)
    .config(GroveConfig {
        routing_strategy: RoutingStrategy::SemanticSimilarity,
        similarity_threshold: 0.72,
        ..Default::default()
    })
    .build()?;

let service = GroveExecutionService::new(
    Arc::new(paladin_port),
    Some(Arc::new(embedding_port)),
    None,
);

// Natural language queries - semantic matching handles variations
let queries = vec![
    "Our login form is vulnerable to automated attacks",
    "How do we secure our AWS infrastructure?",
    "Prevent SQL injection in user inputs",
];

for query in queries {
    let result = service.execute(&grove, query).await?;
    println!("Query: {}\nExpert: {}\nConfidence: {:.0}%", 
        query, result.selected_agent, result.confidence * 100.0);
}
```

### Example 3: LLM Routing for Complex Tasks

```rust
let grove = GroveBuilder::new()
    .add_tree(backend_tree)
    .add_tree(frontend_tree)
    .add_tree(devops_tree)
    .config(GroveConfig {
        routing_strategy: RoutingStrategy::LlmRouting,
        fallback_tree: Some("GeneralistTree".to_string()),
        similarity_threshold: 0.65,
    })
    .build()?;

let service = GroveExecutionService::new(
    Arc::new(paladin_port),
    None,
    Some(Arc::new(llm_port)),
);

// Complex, ambiguous task - LLM provides reasoning
let task = "Users report intermittent 500 errors on the dashboard during peak hours";
let result = service.execute(&grove, task).await?;

println!("Task: {}", task);
println!("Routed to: {}", result.selected_agent);
println!("Confidence: {:.0}%", result.confidence * 100.0);
println!("Reasoning: {}", result.routing_reasoning.unwrap());
```

---

## Best Practices

### 1. Tree Organization

✅ **Do**:
- Group related agents: "Backend Specialists", "Frontend Specialists"
- 2-5 agents per tree (manageable)
- Clear tree names reflecting domain
- Logical hierarchy: Tree → Agents

❌ **Don't**:
- Mix unrelated specialties in one tree
- Create single-agent trees (unless intentional)
- Use vague names: "Experts", "Team"

### 2. Agent Specialization

✅ **Do**:
- Define clear expertise boundaries
- Avoid overlapping specialties
- Use descriptive agent names
- Provide comprehensive expertise definitions

❌ **Don't**:
- Create overly broad agents (handle everything)
- Duplicate specialties across trees
- Use generic names: "Agent1", "Expert"

### 3. Routing Strategy Selection

| Scenario | Recommended Strategy |
|----------|---------------------|
| Clear keyword domains | KeywordMatch |
| Natural language queries | SemanticSimilarity |
| Complex ambiguous tasks | LlmRouting |
| Cost-sensitive | KeywordMatch |
| Latency-sensitive | KeywordMatch |
| Accuracy-critical | LlmRouting |

### 4. Expertise Definition

**For KeywordMatch**:
- 8-12 keywords per agent
- Mix broad and specific terms
- Include tool names
- Test with real queries

**For SemanticSimilarity**:
- 75-150 word descriptions
- Natural language, not keyword lists
- Describe tasks and outcomes
- Include methodology and tools

**For LlmRouting**:
- 150-300 word descriptions
- Structure: Skills + Best for
- Be specific about capabilities
- Provide context for decision-making

### 5. Threshold Tuning

**Start with defaults**:
- KeywordMatch: 0.6
- SemanticSimilarity: 0.72
- LlmRouting: 0.65

**Monitor and adjust**:
```rust
// Log routing decisions for analysis
println!("Agent: {} | Confidence: {:.2} | Task: {}", 
    result.selected_agent, result.confidence, task);

// Collect data over time
// Adjust threshold based on:
// - Fallback rate (too high? lower threshold)
// - Incorrect routes (too many? raise threshold)
// - User feedback
```

### 6. Fallback Strategy

✅ **Recommended**:
```rust
let generalist = Tree::new("GeneralistTree")
    .add_agent(TreeAgent::new("GeneralExpert")
        .with_expertise_description("Full-stack generalist"));

config.fallback_tree = Some("GeneralistTree".to_string());
```

This provides graceful degradation for edge cases.

### 7. Performance Optimization

**KeywordMatch** (already optimal):
- <10ms routing
- No external calls

**SemanticSimilarity**:
- Pre-compute agent embeddings at initialization
- Cache task embeddings (if repeated queries)
- Use batch embedding API calls
- Consider local embedding models

**LlmRouting**:
- Use faster models for routing (gpt-4o-mini vs gpt-4)
- Reduce max_tokens (200-300 sufficient)
- Cache routing decisions for identical tasks
- Consider dedicated routing model

### 8. Cost Optimization

```
KeywordMatch: $0 per routing
SemanticSimilarity: ~$0.0001 per routing (OpenAI)
LlmRouting: ~$0.001-0.005 per routing (GPT-4)

For 10,000 tasks/day:
- KeywordMatch: $0/day
- SemanticSimilarity: $1/day
- LlmRouting: $10-50/day
```

**Cost Reduction Strategies**:
1. Use KeywordMatch for well-defined domains
2. Upgrade to SemanticSimilarity only when needed
3. Reserve LlmRouting for critical/ambiguous tasks
4. Use cheaper LLM models for routing
5. Cache routing decisions

---

## API Reference

### Core Types

```rust
// Grove configuration
pub struct Grove {
    pub id: String,
    pub name: String,
    pub trees: Vec<Tree>,
    pub config: GroveConfig,
}

// Expert tree
pub struct Tree {
    pub name: String,
    pub agents: Vec<TreeAgent>,
}

// Tree agent
pub struct TreeAgent {
    pub paladin_id: String,
    pub expertise_keywords: Vec<String>,
    pub expertise_description: Option<String>,
    pub agent_description: Option<String>,
    pub expertise_embedding: Option<Vec<f32>>,
}

// Routing strategies
pub enum RoutingStrategy {
    KeywordMatch,
    SemanticSimilarity,
    LlmRouting,
}

// Grove result
pub struct GroveResult {
    pub final_output: String,
    pub selected_agent: String,
    pub selected_tree: String,
    pub confidence: f32,
    pub routing_reasoning: Option<String>,
}
```

### Services

```rust
// Grove execution service
pub struct GroveExecutionService {
    paladin_port: Arc<dyn PaladinPort>,
    embedding_port: Option<Arc<dyn EmbeddingPort>>,
    llm_port: Option<Arc<dyn LlmPort>>,
}

impl GroveExecutionService {
    pub fn new(
        paladin_port: Arc<dyn PaladinPort>,
        embedding_port: Option<Arc<dyn EmbeddingPort>>,
        llm_port: Option<Arc<dyn LlmPort>>,
    ) -> Self;
    
    pub async fn execute(
        &self,
        grove: &Grove,
        task: &str,
    ) -> Result<GroveResult, GroveError>;
}
```

### Builder

```rust
pub struct GroveBuilder {
    // ...
}

impl GroveBuilder {
    pub fn new() -> Self;
    pub fn name(self, name: impl Into<String>) -> Self;
    pub fn add_tree(self, tree: Tree) -> Self;
    pub fn config(self, config: GroveConfig) -> Self;
    pub fn build(self) -> Result<Grove, GroveError>;
}

pub struct TreeBuilder {
    // ...
}

impl Tree {
    pub fn new(name: impl Into<String>) -> Self;
    pub fn add_agent(self, agent: TreeAgent) -> Self;
}

impl TreeAgent {
    pub fn new(paladin_id: impl Into<String>) -> Self;
    pub fn with_keywords(self, keywords: Vec<String>) -> Self;
    pub fn with_expertise_description(self, desc: impl Into<String>) -> Self;
    pub fn with_agent_description(self, desc: impl Into<String>) -> Self;
}
```

---

## See Also

- [Battalion Overview](BATTALION.md) - All orchestration patterns
- [Council Pattern](COUNCIL.md) - Collaborative deliberation
- [Commander](BATTALION.md#commander-strategy-router) - Strategy selection
- [Configuration Examples](../examples/cli_configs/) - YAML configs
- [Code Examples](../examples/) - Rust examples

---

**Next Steps**:
- Try the [Quick Start](#quick-start) example
- Explore [YAML configurations](../examples/cli_configs/grove_keyword.yaml)
- See [practical examples](examples/grove_examples.md)
- Review [API documentation](https://docs.rs/paladin)
