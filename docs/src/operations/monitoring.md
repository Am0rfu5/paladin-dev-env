# Monitoring Guide

Complete guide for monitoring Paladin with Prometheus, Grafana, and observability best practices.

## Table of Contents

- [Overview](#overview)
- [Metrics Collection](#metrics-collection)
- [Prometheus Setup](#prometheus-setup)
- [Grafana Dashboards](#grafana-dashboards)
- [Alerting](#alerting)
- [Key Metrics](#key-metrics)
- [Distributed Tracing](#distributed-tracing)
- [Health Checks](#health-checks)

## Overview

**Corrected 2026-08-24 (D-09/D-12 currency sweep).** No `/metrics` endpoint exists in this
codebase today: `prometheus` and `opentelemetry` are not dependencies anywhere in the workspace
(`grep -rn 'prometheus\|opentelemetry' Cargo.toml crates/*/Cargo.toml` → 0 hits), and no route
named `/metrics` is registered anywhere (`grep -rn '"/metrics"' src/ crates/` → 0 hits). The
Dockerfile does `EXPOSE 8080 9090` (`Dockerfile:68`) and the shipped `k8s/service.yaml` test
fixture does declare a `paladin-metrics` service on port 9090, but nothing listens on 9090 for
metrics — the port and service name are reserved, not wired up (consistent with 16-04's
`docker.md`/`production.md` findings). The two endpoints that actually exist are liveness and
readiness, both unauthenticated (`crates/paladin-web/src/health.rs`):
- `GET /health` → always `200 {"status": "ok"}`
- `GET /ready` → `200 {"status": "ready", "agents": N}` once the registry is built (a shallow
  check, no network I/O)

Everything below this point (Prometheus, Grafana, Alertmanager, Jaeger) describes a monitoring
stack that is not implemented in this codebase — treat it as an illustrative target
architecture, not a description of shipped code. Per D-12 the sections are left in place with
this standing correction rather than rewritten line by line.

**Monitoring Stack (target architecture, not yet implemented):**
- **Prometheus**: Metrics collection and storage
- **Grafana**: Visualization and dashboards
- **Alertmanager**: Alert routing and notification
- **Jaeger** (optional): Distributed tracing

## Metrics Collection

### Exposing Metrics

```rust,ignore
// Example metrics module
use prometheus::{Encoder, TextEncoder, Registry};
use axum::{Router, routing::get};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();

    // Application metrics
    pub static ref PALADIN_REQUESTS: IntCounter = IntCounter::new(
        "paladin_requests_total",
        "Total number of Paladin execution requests"
    ).unwrap();

    pub static ref PALADIN_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new(
            "paladin_request_duration_seconds",
            "Paladin execution duration in seconds"
        ).buckets(vec![0.1, 0.5, 1.0, 2.0, 5.0, 10.0])
    ).unwrap();

    pub static ref PALADIN_ERRORS: IntCounter = IntCounter::new(
        "paladin_errors_total",
        "Total number of Paladin execution errors"
    ).unwrap();
}

pub fn init_metrics() {
    REGISTRY.register(Box::new(PALADIN_REQUESTS.clone())).unwrap();
    REGISTRY.register(Box::new(PALADIN_DURATION.clone())).unwrap();
    REGISTRY.register(Box::new(PALADIN_ERRORS.clone())).unwrap();
}

pub async fn metrics_handler() -> String {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

// Add to router
let app = Router::new()
    .route("/metrics", get(metrics_handler));
```

### Recording Metrics

```rust,ignore
// Metrics are configured via RUST_LOG and tracing subscriber

#[instrument(skip(paladin))]
pub async fn execute_paladin(paladin: &Paladin, input: &str) -> Result<PaladinResult> {
    PALADIN_REQUESTS.inc();
    let timer = PALADIN_DURATION.start_timer();

    match paladin.execute(input).await {
        Ok(result) => {
            timer.observe_duration();
            Ok(result)
        }
        Err(e) => {
            PALADIN_ERRORS.inc();
            Err(e)
        }
    }
}
```

## Prometheus Setup

### Prometheus Configuration

```yaml
# prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s
  external_labels:
    cluster: 'production'
    environment: 'prod'

scrape_configs:
  - job_name: 'paladin'
    kubernetes_sd_configs:
      - role: pod
        namespaces:
          names:
            - paladin
    relabel_configs:
      - source_labels: [__meta_kubernetes_pod_label_app]
        action: keep
        regex: paladin
      - source_labels: [__meta_kubernetes_pod_annotation_prometheus_io_scrape]
        action: keep
        regex: true
      - source_labels: [__meta_kubernetes_pod_annotation_prometheus_io_port]
        action: replace
        target_label: __address__
        regex: ([^:]+)(?::\d+)?
        replacement: $1:8081
      - source_labels: [__meta_kubernetes_namespace]
        target_label: namespace
      - source_labels: [__meta_kubernetes_pod_name]
        target_label: pod

alerting:
  alertmanagers:
    - static_configs:
        - targets:
            - alertmanager:9093
```

### Docker Compose Setup

```yaml
version: '3.8'

services:
  paladin:
    image: paladin:latest
    ports:
      - "8080:8080"
      - "8081:8081"  # Metrics port
    labels:
      - "prometheus.scrape=true"
      - "prometheus.port=8081"

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus-data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - grafana-data:/var/lib/grafana
      - ./grafana/dashboards:/etc/grafana/provisioning/dashboards
      - ./grafana/datasources:/etc/grafana/provisioning/datasources

  alertmanager:
    image: prom/alertmanager:latest
    ports:
      - "9093:9093"
    volumes:
      - ./alertmanager.yml:/etc/alertmanager/alertmanager.yml

volumes:
  prometheus-data:
  grafana-data:
```

## Grafana Dashboards

### Datasource Configuration

```yaml
# grafana/datasources/prometheus.yml
apiVersion: 1

datasources:
  - name: Prometheus
    type: prometheus
    access: proxy
    url: http://prometheus:9090
    isDefault: true
    editable: true
```

### Dashboard JSON

```json
{
  "dashboard": {
    "title": "Paladin Monitoring",
    "panels": [
      {
        "title": "Request Rate",
        "targets": [
          {
            "expr": "rate(paladin_requests_total[5m])",
            "legendFormat": "{{pod}}"
          }
        ],
        "type": "graph"
      },
      {
        "title": "P95 Latency",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(paladin_request_duration_seconds_bucket[5m]))",
            "legendFormat": "P95"
          },
          {
            "expr": "histogram_quantile(0.99, rate(paladin_request_duration_seconds_bucket[5m]))",
            "legendFormat": "P99"
          }
        ],
        "type": "graph"
      },
      {
        "title": "Error Rate",
        "targets": [
          {
            "expr": "rate(paladin_errors_total[5m])",
            "legendFormat": "Errors/sec"
          }
        ],
        "type": "graph"
      }
    ]
  }
}
```

## Alerting

### Alert Rules

```yaml
# alerts/paladin.yml
groups:
  - name: paladin_alerts
    interval: 30s
    rules:
      - alert: HighErrorRate
        expr: rate(paladin_errors_total[5m]) > 0.05
        for: 5m
        labels:
          severity: critical
          component: paladin
        annotations:
          summary: "High error rate detected"
          description: "Error rate is {{ $value | humanize }} errors/sec"

      - alert: HighLatency
        expr: histogram_quantile(0.95, rate(paladin_request_duration_seconds_bucket[5m])) > 2
        for: 10m
        labels:
          severity: warning
          component: paladin
        annotations:
          summary: "High P95 latency"
          description: "P95 latency is {{ $value | humanize }}s (threshold: 2s)"

      - alert: PaladinDown
        expr: up{job="paladin"} == 0
        for: 1m
        labels:
          severity: critical
          component: paladin
        annotations:
          summary: "Paladin instance is down"
          description: "Instance {{ $labels.instance }} has been down for 1 minute"
```

### Alertmanager Configuration

```yaml
# alertmanager.yml
global:
  resolve_timeout: 5m
  slack_api_url: 'https://hooks.slack.com/services/YOUR/WEBHOOK/URL'

route:
  group_by: ['alertname', 'cluster', 'service']
  group_wait: 10s
  group_interval: 10s
  repeat_interval: 12h
  receiver: 'slack-notifications'

  routes:
    - match:
        severity: critical
      receiver: 'pagerduty-critical'

    - match:
        severity: warning
      receiver: 'slack-notifications'

receivers:
  - name: 'slack-notifications'
    slack_configs:
      - channel: '#paladin-alerts'
        title: '{{ .GroupLabels.alertname }}'
        text: '{{ range .Alerts }}{{ .Annotations.description }}{{ end }}'

  - name: 'pagerduty-critical'
    pagerduty_configs:
      - service_key: 'YOUR_PAGERDUTY_KEY'
```

## Key Metrics

### Application Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `paladin_requests_total` | Counter | Total execution requests |
| `paladin_request_duration_seconds` | Histogram | Request latency |
| `paladin_errors_total` | Counter | Total errors |
| `paladin_active_paladins` | Gauge | Currently executing Paladins |
| `garrison_entries_total` | Gauge | Memory entries stored |
| `garrison_tokens_total` | Gauge | Total tokens in memory |
| `arsenal_tool_calls_total` | Counter | Tool invocations |
| `arsenal_tool_duration_seconds` | Histogram | Tool execution time |
| `battalion_executions_total` | Counter | Battalion executions |
| `battalion_duration_seconds` | Histogram | Battalion execution time |

### System Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `process_cpu_seconds_total` | Counter | CPU time used |
| `process_resident_memory_bytes` | Gauge | Memory usage |
| `process_open_fds` | Gauge | Open file descriptors |
| `process_max_fds` | Gauge | Max file descriptors |

### External Dependencies

| Metric | Type | Description |
|--------|------|-------------|
| `llm_api_calls_total` | Counter | LLM API calls |
| `llm_api_duration_seconds` | Histogram | LLM API latency |
| `llm_api_errors_total` | Counter | LLM API errors |
| `redis_operations_total` | Counter | Redis operations |
| `minio_operations_total` | Counter | MinIO operations |

## Distributed Tracing

### Jaeger Integration

```rust,ignore
use opentelemetry::global;
use tracing_subscriber::layer::SubscriberExt;
use tracing_opentelemetry::OpenTelemetryLayer;

pub fn init_tracing(service_name: &str) -> Result<()> {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(service_name)
        .with_endpoint("jaeger:6831")
        .install_simple()?;

    let opentelemetry = OpenTelemetryLayer::new(tracer);

    tracing_subscriber::registry()
        .with(opentelemetry)
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())
}
```

## Health Checks

### Health Endpoint

**Corrected 2026-08-24.** The struct below was fabricated — no `HealthStatus`/`ComponentHealth`
type exists anywhere in this tree, and neither endpoint reports per-component (LLM/Garrison/
Arsenal/queue) health or `uptime`. The real handlers are in
`crates/paladin-web/src/health.rs`:

```rust,ignore
/// `GET /health` — liveness probe. Always `200 { "status": "ok" }`; depends on nothing.
pub async fn health() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}

/// `GET /ready` — readiness probe. `200 { "status": "ready", "agents": N }` once the
/// registry is built. Performs no network I/O (shallow check).
pub async fn ready(State(state): State<AgentApiState>) -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({ "status": "ready", "agents": state.registry.len() })),
    )
}
```

## Next Steps

- **[Troubleshooting](troubleshooting.md)** - Common issues and solutions
- **[Performance Tuning](performance-tuning.md)** - Optimization guide
- **[Logging](logging.md)** - Log configuration
