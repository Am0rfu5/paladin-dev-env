# Paladin Documentation

Welcome to the Paladin documentation! Paladin is a Rust-based enterprise multi-agent orchestration framework built with Hexagonal Architecture and Domain-Driven Design principles.

## 🚀 Getting Started

New to Paladin? Start here:

1. **[Quickstart Guide](QUICKSTART.md)** - Get your first Paladin agent running in 15 minutes
2. **[Installation](INSTALLATION.md)** - Detailed setup instructions for all platforms
3. **[Examples Gallery](../examples/README.md)** - Working code examples for common use cases

## 📚 User Guides

Learn how to build with Paladin:

- **[Paladin Configuration](guides/paladin-configuration.md)** - Configure agents, models, and behaviors
- **[Battalion Patterns](guides/battalion-patterns.md)** - Multi-agent orchestration patterns
- **[Tool Integration](guides/tool-integration.md)** - Integrate external tools via MCP protocol
- **[Memory Management](guides/memory-management.md)** - Conversation context and persistence
- **[Output Formatting](guides/output-formatting.md)** - Format and stream agent responses

## 🏗️ Architecture

Understand Paladin's design:

- **[Architecture Overview](architecture/overview.md)** - Three-layer hexagonal architecture
- **[Hexagonal Design](architecture/hexagonal-design.md)** - Port/adapter pattern implementation
- **[Domain Model](architecture/domain-model.md)** - DDD entities and relationships
- **[Design Patterns](architecture/design-patterns.md)** - Patterns used throughout Paladin

## 🚢 Deployment

Deploy Paladin to production:

- **[Docker](deployment/docker.md)** - Containerized deployment
- **[Kubernetes](deployment/kubernetes.md)** - Cloud-native orchestration
- **[CI/CD](deployment/cicd.md)** - Automated pipelines with GitHub Actions
- **[Production Best Practices](deployment/production-best-practices.md)** - Security, scaling, and reliability

## 🔧 Operations

Monitor and maintain Paladin:

- **[Logging](operations/logging.md)** - Structured logging configuration
- **[Monitoring](operations/monitoring.md)** - Metrics and dashboards
- **[Troubleshooting](operations/troubleshooting.md)** - Common issues and solutions
- **[Performance Tuning](operations/performance-tuning.md)** - Optimize for throughput and latency

## 🤝 Contributing

Extend and improve Paladin:

- **[Contribution Guide](contributing/CONTRIBUTING.md)** - How to contribute
- **[Adapter Development](contributing/adapter-development.md)** - Create custom adapters
- **[Testing Guide](contributing/testing-guide.md)** - Testing requirements and patterns

## 📖 API Reference

Comprehensive API documentation is available via rustdoc:

```bash
cargo doc --open
```

Or browse online at: [https://docs.rs/paladin](https://docs.rs/paladin) (when published)

## 🎯 Key Concepts

### Medieval Military Theme

Paladin uses a consistent Medieval Military naming convention:

| Term | Definition |
|------|------------|
| **Paladin** | An autonomous AI agent |
| **Battalion** | A coordinated group of Paladins |
| **Formation** | Sequential Paladin execution |
| **Phalanx** | Concurrent Paladin execution |
| **Campaign** | Graph-based orchestration |
| **Chain of Command** | Hierarchical delegation |
| **Garrison** | Agent memory storage |
| **Arsenal** | Tool and capability registry |
| **Armament** | A single tool |
| **Citadel** | State persistence system |
| **Herald** | Output formatting |

### Architecture Layers

Paladin follows hexagonal (ports and adapters) architecture:

- **Core Layer** - Pure domain logic, no external dependencies
- **Application Layer** - Use cases and port definitions (interfaces)
- **Infrastructure Layer** - Adapter implementations for external systems

Dependencies flow inward only: Infrastructure → Application → Core

## 💡 Support

- **Issues**: [GitHub Issues](https://github.com/DF3NDR/paladin-dev-env/issues)
- **Discussions**: [GitHub Discussions](https://github.com/DF3NDR/paladin-dev-env/discussions)
- **Documentation**: You're reading it!

## 📄 License

See [LICENSE](../LICENSE) for details.
