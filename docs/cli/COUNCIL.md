# paladin council - Quick Group Discussions

Execute quick multi-agent discussions without writing configuration files. Get diverse perspectives from multiple AI Paladins on any topic.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Command Syntax](#command-syntax)
- [Agent Roles](#agent-roles)
- [Discussion Modes](#discussion-modes)
- [Output Options](#output-options)
- [Best Practices](#best-practices)
- [Examples](#examples)
- [Troubleshooting](#troubleshooting)

## Overview

The `council` command enables:
- **Ad-hoc** multi-agent discussions without configuration files
- **Diverse perspectives** from multiple AI personas
- **Parallel or sequential** execution modes
- **Structured output** with synthesis and analysis
- **Quick iterations** for brainstorming and decision-making

### When to Use Council

✅ **Use council when:**
- Need quick input from multiple AI perspectives
- Brainstorming solutions to problems
- Evaluating options from different viewpoints
- Quick analysis without formal configuration
- Prototyping multi-agent workflows

❌ **Don't use council when:**
- Need precise control over agent configuration
- Building production workflows (use `paladin run` instead)
- Require state persistence across sessions
- Need custom tools or memory systems

## Quick Start

### Basic Usage

```bash
# Simple discussion with default agents
paladin council "What are the best practices for API design?"

# Specify number of agents
paladin council -n 5 "Should we migrate to microservices?"

# Use specific discussion mode
paladin council --mode sequential "Analyze this business proposal..."

# Save results to file
paladin council -o results.md "Security implications of cloud migration"
```

## Command Syntax

```bash
paladin council [OPTIONS] <QUESTION>

Arguments:
  <QUESTION>
      The question, topic, or problem to discuss
      Can be a question, statement, or detailed scenario

Options:
  -n, --num-agents <N>
      Number of agents to participate (2-10)
      Default: 3
      
  -m, --mode <MODE>
      Discussion mode: parallel, sequential, or debate
      Default: parallel
      
  -r, --roles <ROLES>
      Comma-separated agent roles
      Example: "technical,business,security,ux"
      If not specified, uses default diverse roles
      
  -o, --output <FILE>
      Save discussion results to file
      Supports: .md, .txt, .json
      
  -f, --format <FORMAT>
      Output format: markdown (default), json, or plain
      
  --synthesize
      Generate a synthesis/summary of all perspectives
      Enabled by default, use --no-synthesize to disable
      
  --provider <PROVIDER>
      LLM provider to use (openai, deepseek, anthropic)
      
  --model <MODEL>
      Specific LLM model for all agents
      Example: gpt-4, deepseek-chat, claude-3-sonnet
      
  --temperature <TEMP>
      Temperature for agent responses (0.0-2.0)
      Default: 0.7
      
  --max-tokens <N>
      Maximum tokens per agent response
      Default: 500
      
  --timeout <SECONDS>
      Timeout for the entire council session
      Default: 120 seconds
      
  -v, --verbose
      Show detailed execution information
```

## Agent Roles

### Default Roles

When roles aren't specified, council uses diverse default perspectives:

1. **Analyst** - Data-driven, analytical approach
2. **Critic** - Identifies risks, challenges, and weaknesses
3. **Optimist** - Focuses on opportunities and benefits

### Custom Roles

```bash
# Technical perspectives
paladin council --roles "architect,security,devops,qa" "System design question"

# Business perspectives
paladin council --roles "ceo,cfo,cmo,product" "Product launch strategy"

# Creative perspectives
paladin council --roles "creative,pragmatic,critic,synthesizer" "Marketing campaign"

# Domain-specific
paladin council --roles "legal,compliance,privacy,security" "Data governance policy"
```

### Role Examples

| Role | Perspective | Best For |
|------|-------------|----------|
| **technical** | Engineering, architecture, implementation | Technical decisions |
| **business** | ROI, market fit, business value | Business strategy |
| **security** | Threats, vulnerabilities, compliance | Security reviews |
| **ux** | User experience, usability, accessibility | Design decisions |
| **legal** | Compliance, liability, regulations | Legal considerations |
| **creative** | Innovation, alternative approaches | Brainstorming |
| **critic** | Risks, challenges, weaknesses | Risk analysis |
| **pragmatic** | Practical, realistic, achievable | Implementation planning |
| **optimist** | Opportunities, benefits, positives | Opportunity discovery |
| **analyst** | Data, metrics, evidence-based | Data-driven decisions |

## Discussion Modes

### Parallel Mode (Default)

All agents respond simultaneously without seeing each other's responses.

```bash
paladin council --mode parallel "What are the pros and cons of NoSQL?"
```

**Characteristics:**
- ✅ Fastest execution
- ✅ Independent perspectives
- ✅ No groupthink
- ❌ No interaction between agents
- ❌ May have redundant points

**Best for:**
- Quick diverse input
- Independent perspectives needed
- Time-sensitive discussions

### Sequential Mode

Agents respond one after another, each seeing previous responses.

```bash
paladin council --mode sequential "How should we approach this technical debt?"
```

**Characteristics:**
- ✅ Builds on previous ideas
- ✅ More coherent discussion
- ✅ Can challenge/refine points
- ❌ Slower execution
- ❌ May create groupthink

**Best for:**
- Building consensus
- Iterative refinement
- Complex problem-solving

### Debate Mode

Agents present opposing viewpoints and counter-arguments.

```bash
paladin council --mode debate "Should we use serverless architecture?"
```

**Characteristics:**
- ✅ Explores trade-offs deeply
- ✅ Identifies weaknesses
- ✅ Structured pro/con analysis
- ❌ Slower than parallel
- ❌ May be adversarial

**Best for:**
- Decision between alternatives
- Risk/benefit analysis
- Evaluating trade-offs

## Output Options

### Markdown (Default)

```bash
paladin council -o discussion.md "Cloud strategy"
```

```markdown
# Council Discussion: Cloud Strategy

## Question
What cloud strategy should we adopt?

## Participants
- Technical Architect
- Business Analyst  
- Security Specialist

## Responses

### Technical Architect
**Perspective:** Technical Implementation

[Response content...]

**Key Points:**
- Multi-cloud for redundancy
- Containerization strategy
- Migration roadmap

### Business Analyst
**Perspective:** Business Value

[Response content...]

**Key Points:**
- Cost optimization
- Scalability benefits
- Time to market

### Security Specialist
**Perspective:** Security & Compliance

[Response content...]

**Key Points:**
- Data sovereignty
- Encryption standards
- Compliance requirements

## Synthesis

[Synthesized recommendations...]

## Action Items

1. Evaluate cloud providers
2. Conduct security audit
3. Create migration plan
```

### JSON Format

```bash
paladin council -f json -o discussion.json "API design"
```

```json
{
  "question": "What are best practices for API design?",
  "mode": "parallel",
  "participants": [
    {
      "role": "technical",
      "model": "gpt-4"
    },
    {
      "role": "business",
      "model": "gpt-4"
    },
    {
      "role": "ux",
      "model": "gpt-4"
    }
  ],
  "responses": [
    {
      "role": "technical",
      "perspective": "Technical Implementation",
      "response": "...",
      "key_points": ["...", "..."],
      "duration_ms": 1250
    }
  ],
  "synthesis": {
    "summary": "...",
    "recommendations": ["...", "..."],
    "action_items": ["...", "..."]
  },
  "metadata": {
    "timestamp": "2024-01-15T10:30:00Z",
    "total_duration_ms": 3500
  }
}
```

### Plain Text

```bash
paladin council -f plain "Design patterns discussion"
```

Simple text output without formatting, useful for piping to other tools.

## Best Practices

### 1. Frame Questions Clearly

✅ **Good:**
```bash
paladin council "
Should we adopt GraphQL for our public API?

Context:
- RESTful API with 50+ endpoints
- 100k requests/day
- Mobile and web clients
- Team of 5 backend developers
"
```

❌ **Avoid:**
```bash
paladin council "graphql?"
```

### 2. Choose Appropriate Roles

```bash
# For technical decisions
paladin council --roles "architect,security,devops" "Kubernetes vs. ECS"

# For product decisions
paladin council --roles "product,ux,engineering,business" "Feature prioritization"

# For strategic decisions
paladin council --roles "ceo,cto,cfo,cmo" "Market expansion strategy"
```

### 3. Select the Right Mode

```bash
# Quick diverse input → parallel
paladin council --mode parallel "Initial thoughts on blockchain integration"

# Building on ideas → sequential  
paladin council --mode sequential "Refine our architecture approach"

# Evaluating options → debate
paladin council --mode debate "Build vs. buy for authentication"
```

### 4. Synthesize Results

```bash
# Always get synthesis (default)
paladin council "Complex decision" --synthesize

# Review synthesis for action items
paladin council "Decision" -o results.md
# Then extract action items from results.md
```

### 5. Iterate and Refine

```bash
# First pass - broad input
paladin council "App architecture options" -o round1.md

# Review results, then deep dive
paladin council "Microservices concerns from round 1" -o round2.md

# Final decision
paladin council "Final architecture decision" --mode debate -o final.md
```

## Examples

### Example 1: Quick Technical Decision

```bash
paladin council -n 4 "
Should we use TypeScript or JavaScript for our new service?

Context:
- Team has JavaScript experience
- Large codebase (100k+ LOC)
- Need to maintain velocity
- Some junior developers
"
```

### Example 2: Security Review

```bash
paladin council --roles "security,privacy,compliance,devops" --mode sequential "
Review our authentication approach:

Current:
- JWT tokens
- 1-hour expiration  
- Stored in localStorage
- No refresh tokens

Concerns:
- XSS vulnerability?
- CSRF protection?
- Mobile app considerations?
"
```

### Example 3: Architecture Debate

```bash
paladin council --mode debate --roles "monolith-advocate,microservices-advocate" "
Should we migrate from monolith to microservices?

Current state:
- Monolithic Rails app
- 5-year-old codebase
- 10 developers
- Deployment issues
- Scaling challenges
"
```

### Example 4: Product Strategy

```bash
paladin council --roles "product,marketing,sales,engineering,support" -o strategy.md "
Should we build a mobile app or focus on responsive web?

Data:
- 60% mobile traffic
- Limited mobile team
- 6-month timeline
- Competitor has native apps
"
```

### Example 5: Incident Post-Mortem

```bash
paladin council --mode sequential --roles "sre,security,engineering,management" "
Post-mortem for database outage:

Incident:
- 2-hour downtime
- Caused by failed migration
- No rollback plan
- Manual recovery

Questions:
- What went wrong?
- How to prevent?
- Process improvements?
"
```

### Example 6: Code Review Perspectives

```bash
paladin council --roles "security,performance,maintainability,testing" "
Review this architecture decision:

Plan to use Redis for:
- Session storage
- Cache layer  
- Message queue
- Rate limiting

Is this appropriate?
"
```

## Troubleshooting

### Common Issues

#### Issue: Responses are too generic

**Solution:**
```bash
# Provide more context
paladin council "Question with detailed context: ..."

# Use more specific roles
paladin council --roles "senior-architect,principal-engineer" "..."

# Try sequential mode for depth
paladin council --mode sequential "..."
```

#### Issue: Conflicting perspectives without resolution

**Solution:**
```bash
# Ensure synthesis is enabled (default)
paladin council --synthesize "..."

# Use debate mode for structured comparison
paladin council --mode debate "..."

# Do a follow-up round
paladin council "Based on previous discussion, recommend best approach"
```

#### Issue: Timeout before completion

**Solution:**
```bash
# Increase timeout
paladin council --timeout 300 "complex question"

# Reduce number of agents
paladin council -n 3 "..."

# Use parallel mode (faster)
paladin council --mode parallel "..."

# Reduce max tokens per response
paladin council --max-tokens 300 "..."
```

#### Issue: Not enough detail in responses

**Solution:**
```bash
# Increase max tokens
paladin council --max-tokens 1000 "detailed analysis needed"

# Ask more specific questions
paladin council "Specific aspect of broader topic"

# Use higher temperature for creativity
paladin council --temperature 1.0 "creative problem-solving"
```

#### Issue: Agent perspectives are too similar

**Solution:**
```bash
# Use more diverse roles
paladin council --roles "conservative,progressive,radical,pragmatic" "..."

# Try debate mode
paladin council --mode debate "..."

# Increase temperature
paladin council --temperature 1.2 "diverse viewpoints needed"
```

### Debugging

```bash
# Enable verbose mode to see execution details
paladin council --verbose "..."

# Test with simpler question first
paladin council "Hello, how are you?" -n 2

# Check provider configuration
paladin setup-check

# Try different provider
paladin council --provider deepseek "..."
```

## Advanced Usage

### Combining with Other Commands

```bash
# Generate config, then discuss it
paladin muster "workflow" -o workflow.yaml
paladin council "Review this workflow config: $(cat workflow.yaml)"

# Council for planning, then execute
paladin council "Best approach for task X" -o plan.md
# Review plan.md
paladin run -c final_approach.yaml
```

### Batch Processing

```bash
# Multiple questions from file
while IFS= read -r question; do
    paladin council "$question" -o "output_$(echo "$question" | md5sum | cut -c1-8).md"
done < questions.txt

# Different role combinations
for roles in "tech,security" "business,legal" "ux,product"; do
    paladin council --roles "$roles" "Same question" -o "perspective_${roles}.md"
done
```

### Custom Synthesis

```bash
# Get detailed JSON output
paladin council -f json -o raw.json "Complex decision"

# Process with jq or custom script
jq '.responses[].key_points[]' raw.json > all_points.txt

# Feed back for meta-analysis
paladin council "Synthesize these points: $(cat all_points.txt)"
```

### Integration with Scripts

```python
#!/usr/bin/env python3
import subprocess
import json

def council_discussion(question, roles, mode="parallel"):
    result = subprocess.run([
        "paladin", "council",
        "--format", "json",
        "--mode", mode,
        "--roles", roles,
        question
    ], capture_output=True, text=True)
    
    return json.loads(result.stdout)

# Use in automation
discussion = council_discussion(
    "Should we proceed with migration?",
    "technical,business,security",
    mode="sequential"
)

# Extract recommendations
recommendations = discussion["synthesis"]["recommendations"]
print(f"Recommendations: {recommendations}")
```

## Performance Tips

| Scenario | Recommended Settings |
|----------|---------------------|
| **Quick input** | `-n 3 --mode parallel --max-tokens 300` |
| **Detailed analysis** | `-n 5 --mode sequential --max-tokens 1000` |
| **Fast iteration** | `-n 2 --mode parallel --no-synthesize` |
| **Deep dive** | `-n 4 --mode sequential --synthesize` |
| **Cost-effective** | `--provider deepseek --max-tokens 400` |
| **High quality** | `--provider anthropic --model claude-3-opus` |

## See Also

- [CLI Usage Guide](../CLI_USAGE.md) - Overview of all CLI commands
- [Muster Command](MUSTER.md) - Generate full Battalion configurations
- [Conclave Pattern](../COUNCIL.md) - Detailed council/conclave documentation
- [Battalion Patterns](../BATTALION.md) - Understanding orchestration patterns
- [Examples Directory](../../examples/) - Sample implementations

## Support

- **Issues**: Report bugs at https://github.com/yourusername/paladin/issues
- **Discussions**: Ask questions in GitHub Discussions
- **Documentation**: Full docs at https://paladin-ai.dev

---

*Council discussions are ephemeral and don't persist state. For production workflows with state management, use `paladin run` with configuration files.*
