# Troubleshooting Guide

Common issues, diagnostic procedures, and solutions for Paladin deployments.

## Table of Contents

- [Diagnostic Tools](#diagnostic-tools)
- [Common Issues](#common-issues)
- [Performance Issues](#performance-issues)
- [Configuration Issues](#configuration-issues)
- [Deployment Issues](#deployment-issues)
- [Integration Issues](#integration-issues)
- [Getting Help](#getting-help)

## Diagnostic Tools

### Check Application Status

```bash
# Check health endpoint
curl http://localhost:8080/health

# Check metrics
curl http://localhost:8081/metrics

# View logs
kubectl logs -f deployment/paladin -n paladin

# Check pod status
kubectl describe pod <pod-name> -n paladin
```

### Enable Debug Logging

```bash
# Set environment variable
export RUST_LOG=debug,paladin=trace

# Or in config.yml
logging:
  level: "debug"
  modules:
    paladin: "trace"
```

### Collect Diagnostic Information

```bash
# System information
uname -a
rustc --version
cargo --version

# Application logs
kubectl logs deployment/paladin -n paladin --tail=1000 > paladin.log

# Metrics snapshot
curl http://localhost:8081/metrics > metrics.txt

# Configuration
kubectl get cm paladin-config -o yaml > config.yaml
```

## Common Issues

### 1. Paladin Execution Fails

**Symptoms:**
- `PaladinError::ExecutionError`
- Empty or truncated responses
- Timeout errors

**Diagnosis:**
```bash
# Check logs for error details
kubectl logs deployment/paladin | grep ERROR

# Verify LLM configuration
curl http://localhost:8080/health | jq .components.llm
```

**Solutions:**

**A. Invalid API Key**
```yaml
# Fix: Update secret with valid key
kubectl create secret generic paladin-secrets \
  --from-literal=openai-api-key="sk-..." \
  --dry-run=client -o yaml | kubectl apply -f -
```

**B. Model Not Found**
```rust,ignore
// Fix: Use valid model name
let paladin = PaladinBuilder::new(llm_port)
    .model("gpt-4")  // Not "gpt-4-invalid"
    .build()?;
```

**C. Rate Limiting**
```yaml
# Fix: Add retry logic and backoff
llm:
  max_retries: 3
  retry_delay: 2s
  timeout: 60s
```

### 2. High Memory Usage

**Symptoms:**
- OOMKilled pods
- Memory usage > 80%
- Slow performance

**Diagnosis:**
```bash
# Check memory usage
kubectl top pods -n paladin

# Check Garrison size
curl http://localhost:8081/metrics | grep garrison_entries
```

**Solutions:**

**A. Garrison Too Large**
```yaml
# Fix: Reduce garrison limits
garrison:
  max_entries: 500  # Reduce from 1000
  max_tokens: 4000  # Reduce from 8000
```

**B. Memory Leak**
```bash
# Fix: Update to latest version
docker pull ghcr.io/your-org/paladin:latest
kubectl rollout restart deployment/paladin
```

**C. Insufficient Resources**
```yaml
# Fix: Increase resource limits
resources:
  limits:
    memory: 8Gi  # Increase from 4Gi
```

### 3. Connection Refused

**Symptoms:**
- Cannot connect to external services
- `ConnectionRefused` errors
- Network timeout

**Diagnosis:**
```bash
# Test connectivity from pod
kubectl exec -it <pod-name> -- curl http://redis:6379
kubectl exec -it <pod-name> -- nslookup redis

# Check network policies
kubectl get networkpolicy -n paladin
```

**Solutions:**

**A. Service Not Running**
```bash
# Fix: Start the service
kubectl get svc redis -n paladin
kubectl scale statefulset redis --replicas=1
```

**B. Wrong Hostname**
```yaml
# Fix: Use correct service DNS
queue:
  url: "redis://redis.paladin.svc.cluster.local:6379"
```

**C. Network Policy Blocking**
```yaml
# Fix: Allow egress to Redis
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: allow-redis
spec:
  podSelector:
    matchLabels:
      app: paladin
  egress:
  - to:
    - podSelector:
        matchLabels:
          app: redis
    ports:
    - protocol: TCP
      port: 6379
```

### 4. Battalion Execution Hangs

**Symptoms:**
- Battalion never completes
- High CPU usage
- No error messages

**Diagnosis:**
```bash
# Check active Paladins
curl http://localhost:8081/metrics | grep paladin_active

# Look for deadlocks
kubectl logs deployment/paladin | grep -i "deadlock\|timeout"
```

**Solutions:**

**A. Circular Dependencies (Campaign)**
```rust,ignore
// Fix: Ensure DAG has no cycles
campaign.validate()?;  // Will error if cyclic
```

**B. Infinite Loop**
```rust,ignore
// Fix: Set reasonable max_loops
let paladin = PaladinBuilder::new(llm_port)
    .max_loops(10)  // Prevent infinite loops
    .build()?;
```

**C. Timeout Not Set**
```yaml
# Fix: Add execution timeout
paladin:
  timeout_seconds: 300  # 5 minutes
```

## Performance Issues

### Slow Response Times

**Symptoms:**
- P95 latency > 2s
- High request duration

**Diagnosis:**
```bash
# Check latency metrics
curl http://localhost:8081/metrics | grep duration

# Profile with flamegraph
cargo flamegraph --bin paladin-server
```

**Solutions:**

**A. Slow LLM Responses**
```yaml
# Fix: Use faster model or increase timeout
llm:
  default_model: "gpt-3.5-turbo"  # Faster than gpt-4
  timeout: 30s
```

**B. Garrison Query Slow**
```sql
-- Fix: Add index to Garrison database
CREATE INDEX idx_garrison_timestamp ON garrison_entries(timestamp);
CREATE INDEX idx_garrison_session ON garrison_entries(session_id);
```

**C. Too Many Tool Calls**
```yaml
# Fix: Limit concurrent tool executions
arsenal:
  max_concurrent_tools: 5
```

### High CPU Usage

**Symptoms:**
- CPU throttling
- Slow processing
- Increased costs

**Diagnosis:**
```bash
# Check CPU usage
kubectl top pods -n paladin

# Profile CPU
cargo build --release
perf record -F 99 -g ./target/release/paladin-server
perf script | stackcollapse-perf.pl | flamegraph.pl > cpu.svg
```

**Solutions:**

**A. Too Many Replicas**
```yaml
# Fix: Reduce replica count
spec:
  replicas: 3  # Reduce from 10
```

**B. Inefficient Code**
```bash
# Fix: Update to optimized version
git pull origin main
cargo build --release
```

## Configuration Issues

### Invalid Configuration

**Symptoms:**
- Application won't start
- Configuration validation errors

**Diagnosis:**
```bash
# Validate configuration
paladin config validate config.yml

# Check for syntax errors
yamllint config.yml
```

**Solutions:**
```yaml
# Fix: Correct YAML syntax
paladin:
  default_temperature: 0.7  # Must be number
  max_loops: 3              # Must be integer
```

### Missing Environment Variables

**Symptoms:**
- `environment variable not set` errors
- API calls fail

**Diagnosis:**
```bash
# Check environment
kubectl exec deployment/paladin -- env | grep -i key
```

**Solutions:**
```bash
# Fix: Set missing variables
kubectl create secret generic paladin-secrets \
  --from-literal=openai-api-key="$OPENAI_API_KEY"
```

## Deployment Issues

### Pod CrashLoopBackOff

**Symptoms:**
- Pods constantly restarting
- `CrashLoopBackOff` status

**Diagnosis:**
```bash
# Check pod events
kubectl describe pod <pod-name> -n paladin

# View crash logs
kubectl logs <pod-name> -n paladin --previous
```

**Solutions:**

**A. Missing Dependencies**
```dockerfile
# Fix: Add runtime dependencies
RUN apt-get install -y libssl1.1 ca-certificates
```

**B. Health Check Failing**
```yaml
# Fix: Adjust health check timing
livenessProbe:
  initialDelaySeconds: 60  # Increase from 30
  periodSeconds: 30        # Increase from 10
```

### Image Pull Errors

**Symptoms:**
- `ImagePullBackOff` or `ErrImagePull`
- Pods stuck in pending

**Diagnosis:**
```bash
# Check image pull status
kubectl describe pod <pod-name> -n paladin | grep -A5 Events
```

**Solutions:**
```bash
# Fix: Authenticate with registry
kubectl create secret docker-registry ghcr-secret \
  --docker-server=ghcr.io \
  --docker-username=$GITHUB_USER \
  --docker-password=$GITHUB_TOKEN

# Update deployment to use secret
spec:
  imagePullSecrets:
  - name: ghcr-secret
```

## Integration Issues

### Redis Connection Failed

**Symptoms:**
- Queue operations fail
- `ConnectionRefused` errors

**Diagnosis:**
```bash
# Test Redis connectivity
kubectl exec deployment/paladin -- redis-cli -h redis ping
```

**Solutions:**
```bash
# Fix: Restart Redis
kubectl rollout restart statefulset redis

# Or check authentication
kubectl get secret redis-auth -o jsonpath='{.data.password}' | base64 -d
```

### MinIO/S3 Errors

**Symptoms:**
- File storage operations fail
- `AccessDenied` errors

**Diagnosis:**
```bash
# Test MinIO connectivity
kubectl exec deployment/paladin -- \
  curl -v http://minio:9000/minio/health/live
```

**Solutions:**
```bash
# Fix: Update credentials
kubectl create secret generic minio-credentials \
  --from-literal=access-key="minioadmin" \
  --from-literal=secret-key="minioadmin"
```

### LLM Provider Issues

**Symptoms:**
- API rate limiting
- Invalid credentials
- Model unavailable

**Solutions:**

**A. Rate Limit Exceeded**
```yaml
# Fix: Add rate limiting
llm:
  rate_limit:
    requests_per_minute: 60
    tokens_per_minute: 90000
```

**B. Switch Provider**
```yaml
# Fix: Use fallback provider
llm:
  providers:
    - openai
    - deepseek  # Fallback
    - anthropic # Fallback
```

## Getting Help

### Collect Debug Bundle

```bash
#!/bin/bash
# debug-bundle.sh

NAMESPACE="paladin"
OUTPUT="debug-bundle-$(date +%Y%m%d-%H%M%S).tar.gz"

mkdir -p debug-bundle
cd debug-bundle

# Logs
kubectl logs deployment/paladin -n $NAMESPACE > paladin.log

# Configuration
kubectl get all,cm,secrets -n $NAMESPACE -o yaml > resources.yaml

# Metrics
curl http://localhost:8081/metrics > metrics.txt

# Events
kubectl get events -n $NAMESPACE > events.txt

cd ..
tar czf $OUTPUT debug-bundle/
echo "Debug bundle created: $OUTPUT"
```

### Open an Issue

Include:
1. Paladin version
2. Deployment environment (Docker/K8s)
3. Error messages and logs
4. Steps to reproduce
5. Expected vs actual behavior

### Community Support

- **GitHub Issues**: Bug reports and feature requests
- **Discussions**: Questions and community help
- **Discord**: Real-time chat support

## Next Steps

- **[Monitoring](monitoring.md)** - Set up monitoring
- **[Performance Tuning](performance-tuning.md)** - Optimize performance
- **[Logging](logging.md)** - Configure logging
