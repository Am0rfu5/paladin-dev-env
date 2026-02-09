# paladin muster - AI-Powered Battalion Generation

Generate production-ready Battalion configurations from natural language descriptions using LLM intelligence.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Command Syntax](#command-syntax)
- [Generation Workflow](#generation-workflow)
- [Configuration Options](#configuration-options)
- [Output Formats](#output-formats)
- [Best Practices](#best-practices)
- [Examples](#examples)
- [Troubleshooting](#troubleshooting)

## Overview

The `muster` command leverages LLM intelligence to:
- **Translate** natural language descriptions into Battalion configurations
- **Suggest** optimal orchestration patterns (Formation, Phalanx, Campaign, Chain of Command)
- **Generate** complete YAML/JSON configurations with validation
- **Preview** the generated configuration before saving
- **Validate** configuration against Paladin schema

### When to Use Muster

✅ **Use muster when:**
- Creating complex multi-agent workflows from scratch
- Prototyping new orchestration patterns
- Need AI suggestions for optimal agent coordination
- Want validated, production-ready configurations quickly

❌ **Don't use muster when:**
- You have existing configurations (use `paladin run` instead)
- Need precise manual control over every parameter
- Working with sensitive/proprietary orchestration logic

## Quick Start

### Basic Usage

```bash
# Generate a simple sequential workflow
paladin muster "Create a data analysis pipeline: fetch data, clean it, analyze patterns, generate report"

# Generate a parallel processing workflow
paladin muster "Process customer reviews in parallel: sentiment analysis, topic extraction, summary generation"

# Generate with specific pattern
paladin muster --pattern formation "Three-step research workflow"

# Generate and save directly
paladin muster "Code review workflow" --output code_review.yaml --yes
```

## Command Syntax

```bash
paladin muster [OPTIONS] <DESCRIPTION>

Arguments:
  <DESCRIPTION>
      Natural language description of the desired Battalion workflow
      Can be a sentence, paragraph, or detailed specification

Options:
  -p, --pattern <PATTERN>
      Preferred orchestration pattern (formation, phalanx, campaign, chain_of_command)
      If not specified, LLM will suggest the best pattern
      
  -o, --output <FILE>
      Output file path (YAML or JSON based on extension)
      If not specified, displays configuration without saving
      
  -f, --format <FORMAT>
      Output format: yaml (default) or json
      
  -y, --yes
      Auto-confirm and save without preview
      
  --provider <PROVIDER>
      LLM provider to use for generation (openai, deepseek, anthropic)
      Default: Uses default provider from configuration
      
  --model <MODEL>
      Specific LLM model to use
      Example: gpt-4, deepseek-chat, claude-3-opus
      
  --temperature <TEMP>
      Generation temperature (0.0-2.0)
      Lower = more focused, Higher = more creative
      Default: 0.7
      
  --validate
      Validate the generated configuration against schema
      Enabled by default, use --no-validate to skip
      
  --interactive
      Interactive mode - refine the generated config through conversation
      
  -v, --verbose
      Show detailed generation process
```

## Generation Workflow

### 1. Analysis Phase

```bash
paladin muster "Build a content moderation system"
```

```
🧠 Analyzing workflow requirements...

Requirements Analysis:
- Task Type: Sequential processing with decision points
- Agents Required: 3-4 specialized Paladins
- Suggested Pattern: Campaign (graph-based workflow)
- Estimated Complexity: Medium
```

### 2. Configuration Generation

```
⚙️  Generating Battalion configuration...

Generating:
  ✓ Paladin definitions (4 agents)
  ✓ Orchestration pattern (Campaign)
  ✓ Dependencies and data flow
  ✓ Configuration parameters
```

### 3. Validation Phase

```
✅ Validating configuration...

Validation Results:
  ✓ Schema validation passed
  ✓ All Paladin references valid
  ✓ No circular dependencies
  ✓ Resource requirements satisfied
```

### 4. Preview & Confirmation

```yaml
# Generated Battalion Configuration
# Pattern: Campaign
# Paladins: 4
# Estimated Duration: 30-60 seconds

name: content_moderation_system
description: Automated content moderation with classification and review

battalion:
  type: campaign
  graph:
    nodes:
      - id: content_classifier
        paladin: classifier
      - id: toxicity_detector
        paladin: toxicity
      - id: human_review
        paladin: reviewer
        condition: "{{toxicity_detector.score}} > 0.7"
      - id: final_decision
        paladin: decision_maker
    
    edges:
      - from: content_classifier
        to: toxicity_detector
      - from: toxicity_detector
        to: human_review
      - from: toxicity_detector
        to: final_decision
      - from: human_review
        to: final_decision

paladins:
  classifier:
    system_prompt: "Classify content into categories..."
    model: gpt-4
    temperature: 0.3
  # ... additional paladins

Save configuration? [Y/n]:
```

## Configuration Options

### Orchestration Patterns

#### Formation (Sequential)
```bash
paladin muster --pattern formation "Data processing pipeline"
```
- Best for: Linear workflows, step-by-step processing
- Use when: Output of one step feeds into the next
- Example: Extract → Transform → Load

#### Phalanx (Parallel)
```bash
paladin muster --pattern phalanx "Analyze documents from multiple perspectives"
```
- Best for: Independent parallel tasks
- Use when: Tasks don't depend on each other
- Example: Multiple AI models processing same input

#### Campaign (Graph/DAG)
```bash
paladin muster --pattern campaign "Complex workflow with conditional branches"
```
- Best for: Complex workflows with branching logic
- Use when: Need conditional execution or task dependencies
- Example: Approval workflows, decision trees

#### Chain of Command (Hierarchical)
```bash
paladin muster --pattern chain_of_command "Hierarchical task delegation"
```
- Best for: Manager-worker patterns
- Use when: Need dynamic task distribution
- Example: Project management, ticket routing

### Provider Selection

```bash
# Use specific provider
paladin muster --provider openai "Customer support workflow"

# Use specific model
paladin muster --provider anthropic --model claude-3-opus "Research synthesis"

# High creativity
paladin muster --temperature 1.5 "Creative brainstorming workflow"

# High precision
paladin muster --temperature 0.2 "Code analysis workflow"
```

## Output Formats

### YAML (Default)

```bash
paladin muster "Simple workflow" -o workflow.yaml
```

```yaml
name: simple_workflow
description: Generated by paladin muster

battalion:
  type: formation
  sequence:
    - analyzer
    - processor
    - reporter

paladins:
  analyzer:
    system_prompt: "Analyze input data..."
    model: gpt-4
```

### JSON

```bash
paladin muster "Simple workflow" -o workflow.json -f json
```

```json
{
  "name": "simple_workflow",
  "description": "Generated by paladin muster",
  "battalion": {
    "type": "formation",
    "sequence": ["analyzer", "processor", "reporter"]
  },
  "paladins": {
    "analyzer": {
      "system_prompt": "Analyze input data...",
      "model": "gpt-4"
    }
  }
}
```

## Best Practices

### 1. Write Clear Descriptions

✅ **Good:**
```bash
paladin muster "Create a 3-stage content pipeline: 
1. Extract key information from articles
2. Summarize findings into bullet points  
3. Generate social media posts from summaries"
```

❌ **Avoid:**
```bash
paladin muster "do content stuff"
```

### 2. Specify Requirements

```bash
paladin muster "
Research workflow that:
- Searches multiple sources in parallel
- Synthesizes findings sequentially
- Requires 4-5 specialized agents
- Should complete within 2 minutes
"
```

### 3. Iterate with Interactive Mode

```bash
paladin muster --interactive "Customer onboarding workflow"
```

Then refine through conversation:
```
You: Add a validation step after data collection
Assistant: Adding validation paladin between collector and processor...
You: Make the welcome message more friendly
Assistant: Updating welcome_agent system prompt...
```

### 4. Validate Before Production

```bash
# Always validate generated configs
paladin muster "Workflow" -o config.yaml

# Test before deploying
paladin run -c config.yaml --dry-run

# Test with sample input
paladin run -c config.yaml -i "test input"
```

### 5. Use Version Control

```bash
# Save with descriptive names
paladin muster "v2 with retry logic" -o workflow_v2.yaml

# Track changes
git add workflow_v2.yaml
git commit -m "feat: add retry logic to workflow"
```

## Examples

### Example 1: Data Analysis Pipeline

```bash
paladin muster "
Sequential data analysis:
1. Fetch data from API
2. Clean and validate data
3. Perform statistical analysis
4. Generate visualization recommendations
5. Create final report
" -o data_pipeline.yaml
```

### Example 2: Parallel Content Processing

```bash
paladin muster --pattern phalanx "
Process a blog post in parallel:
- Generate SEO keywords
- Create social media summaries
- Extract key quotes
- Suggest related topics
- Analyze sentiment
" -o content_processor.yaml
```

### Example 3: Approval Workflow

```bash
paladin muster --pattern campaign "
Document approval workflow:
1. Initial review checks format and completeness
2. If incomplete, request revisions
3. If complete, route to appropriate reviewer based on category
4. Technical docs go to tech reviewer
5. Business docs go to business reviewer
6. Final approval from manager
" -o approval_workflow.yaml
```

### Example 4: Customer Support Routing

```bash
paladin muster --pattern chain_of_command "
Customer support ticket routing:
- Manager paladin receives all tickets
- Routes technical questions to tech support team
- Routes billing questions to billing team
- Routes general inquiries to customer service
- Escalates complex issues to senior support
" -o support_routing.yaml
```

### Example 5: Research & Synthesis

```bash
paladin muster --interactive "
Research workflow:
1. Parallel search across academic papers, news, and blogs
2. Collect and filter relevant information
3. Synthesize findings into coherent summary
4. Generate citation list
" -o research_workflow.yaml
```

## Troubleshooting

### Common Issues

#### Issue: Generated config is too simple

**Solution:**
```bash
# Provide more detailed description
paladin muster "Detailed workflow with specific steps: ..." --verbose

# Use higher temperature for more creativity
paladin muster "..." --temperature 1.2

# Try interactive mode to refine
paladin muster --interactive "..."
```

#### Issue: Wrong orchestration pattern suggested

**Solution:**
```bash
# Explicitly specify the pattern
paladin muster --pattern campaign "..."

# Provide clearer requirements about dependencies
paladin muster "Workflow where step B depends on step A, and step C depends on step B"
```

#### Issue: Validation fails

**Solution:**
```bash
# Check validation errors
paladin muster "..." --verbose

# Fix common issues:
# - Invalid Paladin names (use lowercase with underscores)
# - Circular dependencies in Campaign graphs
# - Missing required fields

# Generate again with corrections
paladin muster "corrected description" -o fixed.yaml
```

#### Issue: Configuration doesn't match expectations

**Solution:**
```bash
# Use interactive mode to refine
paladin muster --interactive "..."

# Or iterate manually
paladin muster "..." -o v1.yaml
# Edit v1.yaml as needed
paladin run -c v1.yaml  # Test
paladin muster "improved description" -o v2.yaml
```

#### Issue: LLM provider errors

**Solution:**
```bash
# Check API keys
paladin setup-check

# Try different provider
paladin muster --provider deepseek "..."

# Reduce complexity
paladin muster "simplified version of workflow"
```

### Getting Help

```bash
# View all muster options
paladin muster --help

# Check provider status
paladin setup-check

# Enable verbose output for debugging
paladin muster --verbose "..."

# Test generated config
paladin run -c generated.yaml --dry-run
```

## Advanced Usage

### Custom System Prompts

While `muster` generates system prompts, you can provide hints:

```bash
paladin muster "
Code review workflow:
- Use technical, professional tone
- Focus on security and performance
- Provide actionable feedback
"
```

### Resource Requirements

Specify computational constraints:

```bash
paladin muster "
Fast processing workflow:
- Each step should complete in under 5 seconds
- Use lighter models (gpt-3.5-turbo)
- Minimize agent loops
"
```

### Integration with Existing Configs

```bash
# Generate a new component
paladin muster "Add retry logic component" -o retry_component.yaml

# Manually integrate into existing config
# Or use as reference for manual updates
```

## See Also

- [CLI Usage Guide](../CLI_USAGE.md) - Overview of all CLI commands
- [Battalion Documentation](../BATTALION.md) - Understanding orchestration patterns
- [Paladin Configuration](../QUICKSTART.md) - Manual configuration guide
- [Council Command](COUNCIL.md) - Quick group discussions
- [Examples Directory](../../examples/cli_configs/) - Sample configurations

## Support

- **Issues**: Report bugs at https://github.com/yourusername/paladin/issues
- **Discussions**: Ask questions in GitHub Discussions
- **Documentation**: Full docs at https://paladin-ai.dev

---

*Generated configurations should be reviewed before production use. Always test with sample inputs first.*
