# Production Best Practices

Comprehensive checklist and guidelines for deploying Paladin in production environments.

## Table of Contents

- [Pre-Deployment Checklist](#pre-deployment-checklist)
- [Security](#security)
- [Performance](#performance)
- [Reliability](#reliability)
- [Monitoring](#monitoring)
- [Disaster Recovery](#disaster-recovery)
- [Cost Optimization](#cost-optimization)
- [Maintenance](#maintenance)

## Pre-Deployment Checklist

### Infrastructure

- [ ] **Compute resources** sized appropriately (CPU, memory)
- [ ] **High availability** configured (multiple replicas/zones)
- [ ] **Auto-scaling** enabled with appropriate thresholds
- [ ] **Load balancing** configured with health checks
- [ ] **Network policies** restrict unnecessary traffic
- [ ] **TLS/SSL** certificates configured and valid
- [ ] **DNS** properly configured with failover

### Configuration

- [ ] **Environment variables** properly set (no hardcoded secrets)
- [ ] **Configuration files** validated and tested
- [ ] **API keys** rotated and secured
- [ ] **Log levels** set appropriately (warn/error in prod)
- [ ] **Resource limits** configured (CPU, memory, connections)
- [ ] **Timeouts** set for all external calls
- [ ] **Rate limits** configured to prevent abuse

### Data

- [ ] **Database backups** automated and tested
- [ ] **Volume backups** scheduled and verified
- [ ] **Backup retention** policy defined (7d/30d/365d)
- [ ] **Disaster recovery** plan documented and tested
- [ ] **Data encryption** at rest and in transit
- [ ] **Access controls** properly configured

### Monitoring

- [ ] **Health checks** configured and responding
- [ ] **Metrics collection** enabled (Prometheus/Grafana)
- [ ] **Log aggregation** configured (ELK/Loki)
- [ ] **Alerting** rules defined for critical metrics
- [ ] **On-call rotation** established
- [ ] **Incident response** procedures documented
- [ ] **SLO/SLA** defined and monitored

### Testing

- [ ] **Load testing** performed at expected scale
- [ ] **Integration tests** passing in staging
- [ ] **Rollback procedure** tested
- [ ] **Canary deployment** strategy defined
- [ ] **Blue-green deployment** capability verified
- [ ] **Smoke tests** automated post-deployment

## Security

### Authentication & Authorization

```yaml
# Use strong authentication
auth:
  type: "oauth2"
  provider: "auth0"
  scopes: ["paladin:read", "paladin:write"]

# Implement role-based access control
rbac:
  roles:
    - admin: ["*"]
    - user: ["paladin:execute", "garrison:read"]
    - viewer: ["paladin:read"]
```

### API Key Management

```bash
# Rotate API keys regularly
OPENAI_API_KEY=$(vault kv get -field=api_key secret/openai)
DEEPSEEK_API_KEY=$(vault kv get -field=api_key secret/deepseek)

# Use separate keys for different environments
staging_key="sk-proj-staging-..."
production_key="sk-proj-prod-..."
```

### Network Security

```yaml
# Kubernetes NetworkPolicy
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: paladin-network-policy
spec:
  podSelector:
    matchLabels:
      app: paladin
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: ingress-nginx
    ports:
    - protocol: TCP
      port: 8080
  egress:
  - to:
    - namespaceSelector: {}
    ports:
    - protocol: TCP
      port: 443  # HTTPS only
```

### Container Security

```dockerfile
# Use specific versions (not latest)
FROM rust:1.70-slim-bullseye AS builder

# Run as non-root user
USER paladin:paladin

# Read-only filesystem
docker run --read-only --tmpfs /tmp paladin

# Drop capabilities
docker run --cap-drop=ALL --cap-add=NET_BIND_SERVICE paladin

# Use security scanning
docker scan paladin:latest
snyk container test paladin:latest
```

### Secrets Management

```bash
# Use external secrets managers
# Kubernetes External Secrets
apiVersion: external-secrets.io/v1beta1
kind: ExternalSecret
metadata:
  name: paladin-secrets
spec:
  secretStoreRef:
    name: aws-secrets-manager
  target:
    name: paladin-secrets
  data:
  - secretKey: openai-api-key
    remoteRef:
      key: paladin/prod/openai-api-key

# HashiCorp Vault
vault kv put secret/paladin/prod \
  openai_api_key=sk-... \
  deepseek_api_key=...
```

## Performance

### Resource Allocation

```yaml
# Production resource configuration
resources:
  requests:
    cpu: 1000m      # 1 CPU guaranteed
    memory: 2Gi     # 2GB guaranteed
  limits:
    cpu: 4000m      # 4 CPU max
    memory: 8Gi     # 8GB max (OOM if exceeded)

# Horizontal Pod Autoscaler
autoscaling:
  enabled: true
  minReplicas: 5
  maxReplicas: 20
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

### Connection Pooling

```rust
// Configure connection pools
let redis_config = RedisConfig {
    url: "redis://redis:6379".into(),
    pool_size: 20,
    connection_timeout: Duration::from_secs(5),
    idle_timeout: Some(Duration::from_secs(60)),
};

let minio_config = MinioConfig {
    endpoint: "minio:9000".into(),
    max_connections: 100,
    connection_timeout: Duration::from_secs(10),
};
```

### Caching Strategy

```yaml
# Redis caching configuration
cache:
  enabled: true
  ttl: 3600  # 1 hour
  max_size: 10000
  eviction_policy: "lru"

# Application-level caching
garrison:
  cache_embeddings: true
  cache_ttl: 86400  # 24 hours
```

### LLM Optimization

```yaml
# Optimize LLM calls
llm:
  timeout: 30s
  max_retries: 3
  retry_delay: 1s
  connection_pooling: true
  
  # Use faster models for simple tasks
  model_routing:
    simple_tasks: "gpt-3.5-turbo"
    complex_tasks: "gpt-4"
    
  # Batch similar requests
  batching:
    enabled: true
    max_batch_size: 10
    max_wait_time: 100ms
```

## Reliability

### Health Checks

```yaml
# Liveness probe (restart if fails)
livenessProbe:
  httpGet:
    path: /health/live
    port: 8080
  initialDelaySeconds: 30
  periodSeconds: 10
  timeoutSeconds: 5
  failureThreshold: 3

# Readiness probe (remove from load balancer if fails)
readinessProbe:
  httpGet:
    path: /health/ready
    port: 8080
  initialDelaySeconds: 10
  periodSeconds: 5
  timeoutSeconds: 3
  failureThreshold: 3
  successThreshold: 1
```

### Graceful Shutdown

```rust
// Implement graceful shutdown
use tokio::signal;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    
    tracing::info!("Shutdown signal received, starting graceful shutdown");
}

// In main
let server = axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .with_graceful_shutdown(shutdown_signal());
```

```yaml
# Kubernetes graceful termination
spec:
  terminationGracePeriodSeconds: 30
  containers:
  - lifecycle:
      preStop:
        exec:
          command: ["/bin/sh", "-c", "sleep 15"]
```

### Circuit Breakers

```rust
// Implement circuit breakers for external services
use circuit_breaker::{CircuitBreaker, Config};

let llm_breaker = CircuitBreaker::new(Config {
    failure_threshold: 5,
    success_threshold: 2,
    timeout: Duration::from_secs(60),
});

async fn call_llm_with_breaker(prompt: &str) -> Result<Response> {
    llm_breaker.call(async {
        llm_client.generate(prompt).await
    }).await
}
```

### Retry Logic

```rust
// Implement exponential backoff
use backoff::{ExponentialBackoff, Error as BackoffError};
use backoff::future::retry;

async fn call_with_retry<F, T>(f: F) -> Result<T>
where
    F: Fn() -> Result<T>,
{
    let backoff = ExponentialBackoff {
        max_elapsed_time: Some(Duration::from_secs(60)),
        max_interval: Duration::from_secs(30),
        ..Default::default()
    };
    
    retry(backoff, || async {
        f().map_err(|e| {
            if e.is_retryable() {
                BackoffError::Transient(e)
            } else {
                BackoffError::Permanent(e)
            }
        })
    }).await
}
```

## Monitoring

### Key Metrics

```yaml
# Application metrics
metrics:
  - paladin_requests_total          # Total requests
  - paladin_request_duration_seconds  # Request latency
  - paladin_errors_total            # Error count
  - paladin_active_paladins         # Active Paladins
  - garrison_entries_total          # Memory entries
  - arsenal_tool_calls_total        # Tool invocations
  
# System metrics
  - process_cpu_seconds_total       # CPU usage
  - process_resident_memory_bytes   # Memory usage
  - go_goroutines                   # Goroutines (if applicable)

# External dependencies
  - llm_api_calls_total             # LLM API calls
  - llm_api_duration_seconds        # LLM latency
  - redis_operations_total          # Redis ops
  - minio_operations_total          # MinIO ops
```

### Alerting Rules

```yaml
# Prometheus alerting rules
groups:
- name: paladin
  interval: 30s
  rules:
  - alert: HighErrorRate
    expr: rate(paladin_errors_total[5m]) > 0.05
    for: 5m
    labels:
      severity: critical
    annotations:
      summary: "High error rate detected"
  
  - alert: HighLatency
    expr: histogram_quantile(0.95, paladin_request_duration_seconds) > 2
    for: 10m
    labels:
      severity: warning
    annotations:
      summary: "High P95 latency (>2s)"
  
  - alert: PodCrashLooping
    expr: rate(kube_pod_container_status_restarts_total[15m]) > 0
    for: 15m
    labels:
      severity: critical
    annotations:
      summary: "Pod is crash looping"
```

### Logging Best Practices

```rust
// Structured logging with tracing
use tracing::{info, warn, error, instrument};

#[instrument(skip(paladin), fields(paladin_id = %paladin.id))]
async fn execute_paladin(paladin: &Paladin, input: &str) -> Result<PaladinResult> {
    info!("Starting paladin execution");
    
    match paladin.execute(input).await {
        Ok(result) => {
            info!(
                loops_used = result.loops_used,
                output_length = result.content.len(),
                "Paladin execution completed successfully"
            );
            Ok(result)
        }
        Err(e) => {
            error!(error = %e, "Paladin execution failed");
            Err(e)
        }
    }
}
```

```yaml
# Log aggregation configuration
logging:
  level: warn  # info in staging, warn in production
  format: json
  outputs:
    - type: stdout
    - type: file
      path: /app/logs/paladin.log
      rotation:
        max_size: 100MB
        max_age: 7d
        max_backups: 10
```

## Disaster Recovery

### Backup Strategy

```bash
# Automated backups
# 1. Database backups
0 2 * * * /scripts/backup-garrison-db.sh

# 2. Volume snapshots
kubectl exec -n paladin deployment/backup -- \
  /scripts/snapshot-volumes.sh

# 3. Configuration backups
kubectl get all,cm,secrets -n paladin -o yaml > backup-$(date +%Y%m%d).yaml
```

### Recovery Testing

```bash
# Quarterly disaster recovery drill
1. Simulate complete cluster failure
2. Restore from backups
3. Verify data integrity
4. Measure RTO (Recovery Time Objective)
5. Measure RPO (Recovery Point Objective)
6. Document lessons learned
```

### Multi-Region Deployment

```yaml
# Deploy to multiple regions
regions:
  - name: us-east-1
    primary: true
    replicas: 5
  - name: eu-west-1
    primary: false
    replicas: 3
  - name: ap-southeast-1
    primary: false
    replicas: 3

# Cross-region replication
replication:
  garrison: async  # Eventual consistency
  citadel: sync    # Strong consistency for checkpoints
```

## Cost Optimization

### Resource Right-Sizing

```bash
# Analyze actual usage
kubectl top pods -n paladin
kubectl describe hpa paladin -n paladin

# Adjust based on metrics
resources:
  requests:
    cpu: 800m    # Reduced from 1000m
    memory: 1.5Gi  # Reduced from 2Gi
```

### Auto-Scaling Policies

```yaml
# Aggressive scale-down for cost savings
autoscaling:
  scaleDown:
    stabilizationWindowSeconds: 600  # 10 minutes
    policies:
    - type: Percent
      value: 50
      periodSeconds: 300
```

### Spot Instances

```yaml
# Use spot instances for non-critical workloads
nodeSelector:
  kubernetes.io/lifecycle: spot

tolerations:
- key: spot
  operator: Equal
  value: "true"
  effect: NoSchedule
```

## Maintenance

### Update Strategy

```yaml
# Rolling update configuration
strategy:
  type: RollingUpdate
  rollingUpdate:
    maxSurge: 1        # One extra pod during update
    maxUnavailable: 0  # Zero downtime
```

### Maintenance Windows

```bash
# Schedule maintenance during low-traffic periods
# Example: Sundays 2-4 AM UTC
0 2 * * 0 /scripts/maintenance.sh
```

### Dependency Updates

```bash
# Regular dependency updates
dependabot.yml:
  version: 2
  updates:
    - package-ecosystem: "cargo"
      directory: "/"
      schedule:
        interval: "weekly"
      open-pull-requests-limit: 10
```

## Checklist Summary

Use this checklist before each production deployment:

```markdown
## Pre-Deployment
- [ ] All tests passing (unit, integration, e2e)
- [ ] Code review completed and approved
- [ ] Security scan passed (no high/critical vulnerabilities)
- [ ] Performance benchmarks within acceptable range
- [ ] Documentation updated
- [ ] Changelog updated

## Deployment
- [ ] Backup current state
- [ ] Deploy to staging first
- [ ] Run smoke tests in staging
- [ ] Deploy to production using rolling update
- [ ] Monitor metrics during rollout
- [ ] Verify health checks passing

## Post-Deployment
- [ ] Run smoke tests in production
- [ ] Check error rates and latency
- [ ] Verify auto-scaling working
- [ ] Confirm backups running
- [ ] Update runbook if needed
- [ ] Notify stakeholders of successful deployment
```

## Next Steps

- **[Monitoring](../operations/monitoring.md)** - Detailed monitoring setup
- **[Troubleshooting](../operations/troubleshooting.md)** - Common issues and solutions
- **[Performance Tuning](../operations/performance-tuning.md)** - Optimization guide
