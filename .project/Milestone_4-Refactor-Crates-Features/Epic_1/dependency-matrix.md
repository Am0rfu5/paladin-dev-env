# Dependency Classification Matrix

**Epic:** Milestone 4, Epic 1 — Expand Feature Flags
**Date:** April 14, 2026

---

## Classification

| Dependency | Current State | Classification | Proposed Feature Flag | Rationale |
|---|---|---|---|---|
| `reqwest` | unconditional | **core** | — | Shared by all LLM providers; gating adapter modules is sufficient |
| `serde` | unconditional | **core** | — | Used everywhere for serialization |
| `serde_json` | unconditional | **core** | — | Ubiquitous JSON handling |
| `tokio` | unconditional | **core** | — | Async runtime |
| `tokio-util` | unconditional | **core** | — | Async utilities |
| `async-trait` | unconditional | **core** | — | Required by all port traits |
| `futures` | unconditional | **core** | — | Used throughout |
| `uuid` | unconditional | **core** | — | Entity identity throughout domain |
| `chrono` | unconditional | **core** | — | Timestamps throughout domain |
| `thiserror` | unconditional | **core** | — | Error types in every layer |
| `sqlx` | unconditional | **core** | — | Used by garrison/sanctum/user repos |
| `dotenv` | unconditional | **core** | — | Config loading at startup |
| `env_logger` | unconditional | **core** | — | Logging initialization |
| `log` | unconditional | **core** | — | Logging macros throughout |
| `config` | unconditional | **core** | — | Settings loading |
| `tracing-subscriber` | unconditional | **core** | — | Tracing infrastructure |
| `once_cell` | unconditional | **core** | — | Lazy statics throughout |
| `lazy_static` | unconditional | **core** | — | Lazy statics |
| `regex` | unconditional | **core** | — | Used in content parsing and validation |
| `url` | unconditional | **core** | — | URL handling in adapters |
| `urlencoding` | unconditional | **core** | — | URL encoding utilities |
| `base64` | unconditional | **core** | — | Encoding in auth/API adapters |
| `bytes` | unconditional | **core** | — | Streaming byte buffers |
| `mime_guess` | unconditional | **core** | — | File type detection |
| `sha2` | unconditional | **core** | — | Hashing in auth/content |
| `blake3` | unconditional | **core** | — | Fast hashing |
| `md5` | unconditional | **core** | — | Legacy hash support |
| `murmur3` | unconditional | **core** | — | Fast hash for caching |
| `fasthash` | unconditional | **core** | — | Fast hash utilities |
| `argon2` | unconditional | **core** | — | Password hashing in user auth |
| `lock_api` | unconditional | **core** | — | Sync primitives |
| `toml` | unconditional | **core** | — | Config deserialization |
| `rand` | unconditional | **core** | — | Random number generation |
| `petgraph` | unconditional | **core** | — | Graph data structures for Campaign battalion |
| `tempfile` | unconditional | **core** | — | Used in tests and file processing |
| `tokio-cron-scheduler` | unconditional | **core** | — | Scheduling service |
| `chacha20poly1305` | unconditional | **core** | — | General encryption in `security/encryption.rs`; **not** vision-specific |
| `zeroize` | unconditional | **core** | — | Secure memory in `security/encryption.rs`; **not** vision-specific |
| `serde_yaml` | unconditional | **core** | — | Used in config loading and CLI |
| `redis` | optional (existing) | **optional** | `redis-queue` (existing) | Redis queue adapter |
| `rust-s3` | optional (existing) | **optional** | `s3-storage` (existing) | MinIO/S3 file storage adapter |
| `qdrant-client` | optional (existing) | **optional** | `qdrant` (existing) | Qdrant vector DB adapter |
| `actix-web` | unconditional | **optional** | `web-server` | HTTP server framework; not needed for library/CLI usage |
| `axum` | unconditional | **optional** | `web-server` | HTTP server framework used in `web/user_controller.rs` |
| `lettre` | unconditional | **optional** | `notifications` | Email transport; only used in `adapters/notifications/` |
| `handlebars` | unconditional | **optional** | `notifications` | Template engine; **only** used in `email_notification_adapter.rs` |
| `pdf-extract` | unconditional | **optional** | `content-processing` | PDF parsing in `adapters/document/pdf_extractor.rs` |
| `scraper` | unconditional | **optional** | `content-processing` | HTML parsing (in Cargo.toml but not yet imported in src/) |
| `tiktoken-rs` | unconditional | **optional** | `content-processing` | Token counting in `adapters/garrison/token_counter.rs` |
| `rss` | unconditional | **optional** | `content-processing` | RSS feed parsing (in Cargo.toml but not yet imported in src/) |
| `clap` | unconditional | **optional** | `cli` (Epic 3) | CLI argument parsing; used in `src/bin/paladin-cli.rs` |
| `structopt` | unconditional | **optional** | `cli` (Epic 3) | Legacy CLI arg parsing; used in `src/main.rs` |
| `colored` | unconditional | **optional** | `cli` (Epic 3) | Terminal color; CLI output only |
| `comfy-table` | unconditional | **optional** | `cli` (Epic 3) | ASCII tables; CLI output only |
| `indicatif` | unconditional | **optional** | `cli` (Epic 3) | Progress bars; CLI output only |
| `console` | unconditional | **optional** | `cli` (Epic 3) | Terminal control; CLI output only |
| `dialoguer` | unconditional | **optional** | `cli` (Epic 3) | Interactive prompts; CLI output only |
| `testcontainers-modules` | unconditional | **note** | — | Should move to `[dev-dependencies]` |

---

## Feature Flag Summary

| Feature Flag | Dependencies Gated | Source Modules Gated |
|---|---|---|
| `llm-openai` | — (reqwest is core) | `adapters/llm/openai_adapter.rs`, `adapters/output/openai_llm_adapter.rs` |
| `llm-anthropic` | — | `adapters/llm/anthropic_adapter.rs` |
| `llm-deepseek` | — | `adapters/llm/deepseek_adapter.rs` |
| `llm-all` | (implies all 3 above) | — |
| `content-processing` | `pdf-extract`, `scraper`, `tiktoken-rs`, `rss` | `adapters/document/*`, `adapters/garrison/token_counter.rs` |
| `web-server` | `actix-web`, `axum` | `infrastructure/web/*`, `adapters/output/api_content_deliverer.rs` |
| `notifications` | `lettre`, `handlebars` | `adapters/notifications/*` |
| `vision` | — (reqwest is core) | `adapters/llm/openai_vision.rs`, `adapters/llm/anthropic_vision.rs`, `ports/output/vision_port.rs`, `ports/output/vision_llm_port.rs` |
| `mcp-arsenal` | — (pure Rust) | `adapters/arsenal/*` |
| `redis-queue` | `redis` | `adapters/queue/redis_adapter.rs` (existing) |
| `s3-storage` | `rust-s3` | `adapters/file_storage/minio_adapter.rs` (existing) |
| `qdrant` | `qdrant-client` | `adapters/sanctum/*` (existing) |
| `full` | all of the above | all optional modules |

---

## Notes

- **`reqwest`** stays core: all three LLM provider adapters use it for HTTP. Only adapter *modules* are gated.
- **`chacha20poly1305` / `zeroize`**: live in `src/infrastructure/security/encryption.rs` for *general* encryption (user auth, citadel). They are **not** vision-only and remain unconditional.
- **CLI deps** (`clap`, `structopt`, `colored`, etc.): deferred to Epic 3. Not gated in this Epic.
- **`testcontainers-modules`**: Currently in `[dependencies]` but should be in `[dev-dependencies]`. Flagged as a cleanup item but out of scope for this Epic.
- **`scraper` / `rss`**: Present in `Cargo.toml` but no `use` statements found in `src/`. May be unused or intended for future content adapters. Still gated behind `content-processing` for correctness.
