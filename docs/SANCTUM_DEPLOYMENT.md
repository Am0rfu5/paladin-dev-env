# Sanctum Deployment Guide

This guide covers deployment scenarios for Sanctum's production-ready Qdrant adapter across various environments.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Local Development](#local-development)
- [Docker Compose](#docker-compose)
- [Kubernetes](#kubernetes)
- [Cloud Deployments](#cloud-deployments)
- [Production Best Practices](#production-best-practices)
- [Monitoring](#monitoring)
- [Backup and Recovery](#backup-and-recovery)

## Prerequisites

### For Qdrant Deployment

- Docker 20.10+ (for Docker deployments)
- Kubernetes 1.21+ (for K8s deployments)
- Minimum 2GB RAM for Qdrant
- Sufficient disk space (estimate ~1KB per vector with 1536 dimensions)

### Resource Estimation

| Entries | Dimension | Estimated Storage | Recommended RAM |
|---------|-----------|-------------------|-----------------|
| 10,000 | 1536 | ~15 MB | 512 MB |
| 100,000 | 1536 | ~150 MB | 1 GB |
| 1,000,000 | 1536 | ~1.5 GB | 4 GB |
| 10,000,000 | 1536 | ~15 GB | 16 GB |

## Local Development

### Using InMemory Adapter

The simplest option for development - no infrastructure needed:

```yaml
# config.yml
sanctum:
  enabled: true
  adapter_type: "in_memory"
```

```rust
use paladin::infrastructure::adapters::sanctum::InMemorySanctum;

#[tokio::main]
async fn main() {
    let sanctum = InMemorySanctum::new();
    // Ready to use immediately
}
```

### Local Qdrant Instance

For testing Qdrant locally:

```bash
# Pull and run Qdrant
docker run -p 6333:6333 -p 6334:6334 \
    -v $(pwd)/qdrant_storage:/qdrant/storage \
    qdrant/qdrant:latest
```

```yaml
# config.yml
sanctum:
  enabled: true
  adapter_type: "qdrant"
  qdrant:
    url: "http://localhost:6334"
    collection_name: "dev_memories"
    vector_dimension: 1536
```

Access Qdrant dashboard at: http://localhost:6333/dashboard

## Docker Compose

### Basic Setup

```yaml
# docker-compose.yml
version: '3.8'

services:
  qdrant:
    image: qdrant/qdrant:v1.7.4
    container_name: paladin-qdrant
    ports:
      - "6333:6333"  # HTTP API
      - "6334:6334"  # gRPC API
    volumes:
      - qdrant_data:/qdrant/storage
    environment:
      QDRANT__SERVICE__HTTP_PORT: 6333
      QDRANT__SERVICE__GRPC_PORT: 6334
    restart: unless-stopped

  paladin:
    build: .
    container_name: paladin-app
    depends_on:
      - qdrant
    environment:
      APP_SANCTUM_ENABLED: "true"
      APP_SANCTUM_ADAPTER_TYPE: "qdrant"
      APP_SANCTUM_QDRANT_URL: "http://qdrant:6334"
      APP_SANCTUM_QDRANT_COLLECTION_NAME: "paladin_memories"
      APP_SANCTUM_QDRANT_VECTOR_DIMENSION: "1536"
    volumes:
      - ./config.yml:/app/config.yml
    restart: unless-stopped

volumes:
  qdrant_data:
    driver: local
```

Start services:

```bash
docker-compose up -d
```

Verify Qdrant health:

```bash
curl http://localhost:6333/health
```

### Production Docker Compose

Enhanced with resource limits and monitoring:

```yaml
# docker-compose.prod.yml
version: '3.8'

services:
  qdrant:
    image: qdrant/qdrant:v1.7.4
    container_name: paladin-qdrant-prod
    ports:
      - "6333:6333"
      - "6334:6334"
    volumes:
      - qdrant_data:/qdrant/storage
      - ./qdrant-config.yaml:/qdrant/config/production.yaml
    environment:
      QDRANT__SERVICE__HTTP_PORT: 6333
      QDRANT__SERVICE__GRPC_PORT: 6334
      QDRANT__LOG_LEVEL: INFO
    deploy:
      resources:
        limits:
          cpus: '4'
          memory: 8G
        reservations:
          cpus: '2'
          memory: 4G
    restart: always
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:6333/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s

  paladin:
    build:
      context: .
      dockerfile: Dockerfile.prod
    container_name: paladin-app-prod
    depends_on:
      qdrant:
        condition: service_healthy
    environment:
      APP_SANCTUM_ENABLED: "true"
      APP_SANCTUM_ADAPTER_TYPE: "qdrant"
      APP_SANCTUM_QDRANT_URL: "http://qdrant:6334"
      APP_SANCTUM_QDRANT_COLLECTION_NAME: "production_memories"
      APP_SANCTUM_QDRANT_VECTOR_DIMENSION: "1536"
      RUST_LOG: "info,paladin=debug"
    volumes:
      - ./config.prod.yml:/app/config.yml:ro
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 4G
        reservations:
          cpus: '1'
          memory: 2G
    restart: always
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

volumes:
  qdrant_data:
    driver: local
```

## Kubernetes

### Qdrant StatefulSet

```yaml
# k8s/qdrant-statefulset.yaml
apiVersion: v1
kind: Service
metadata:
  name: qdrant
  namespace: paladin
spec:
  selector:
    app: qdrant
  ports:
    - name: http
      port: 6333
      targetPort: 6333
    - name: grpc
      port: 6334
      targetPort: 6334
  clusterIP: None
---
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: qdrant
  namespace: paladin
spec:
  serviceName: qdrant
  replicas: 1
  selector:
    matchLabels:
      app: qdrant
  template:
    metadata:
      labels:
        app: qdrant
    spec:
      containers:
      - name: qdrant
        image: qdrant/qdrant:v1.7.4
        ports:
        - containerPort: 6333
          name: http
        - containerPort: 6334
          name: grpc
        env:
        - name: QDRANT__SERVICE__HTTP_PORT
          value: "6333"
        - name: QDRANT__SERVICE__GRPC_PORT
          value: "6334"
        - name: QDRANT__LOG_LEVEL
          value: "INFO"
        volumeMounts:
        - name: qdrant-storage
          mountPath: /qdrant/storage
        resources:
          requests:
            memory: "2Gi"
            cpu: "500m"
          limits:
            memory: "8Gi"
            cpu: "4000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 6333
          initialDelaySeconds: 30
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /readyz
            port: 6333
          initialDelaySeconds: 10
          periodSeconds: 5
  volumeClaimTemplates:
  - metadata:
      name: qdrant-storage
    spec:
      accessModes: ["ReadWriteOnce"]
      storageClassName: "standard"
      resources:
        requests:
          storage: 50Gi
```

### Paladin Deployment

```yaml
# k8s/paladin-deployment.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: paladin-config
  namespace: paladin
data:
  config.yml: |
    sanctum:
      enabled: true
      adapter_type: "qdrant"
      qdrant:
        url: "http://qdrant:6334"
        collection_name: "k8s_memories"
        vector_dimension: 1536
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: paladin
  namespace: paladin
spec:
  replicas: 3
  selector:
    matchLabels:
      app: paladin
  template:
    metadata:
      labels:
        app: paladin
    spec:
      containers:
      - name: paladin
        image: paladin:latest
        ports:
        - containerPort: 8080
        env:
        - name: APP_SANCTUM_ENABLED
          value: "true"
        - name: APP_SANCTUM_ADAPTER_TYPE
          value: "qdrant"
        - name: APP_SANCTUM_QDRANT_URL
          value: "http://qdrant:6334"
        - name: APP_SANCTUM_QDRANT_COLLECTION_NAME
          value: "k8s_memories"
        - name: APP_SANCTUM_QDRANT_VECTOR_DIMENSION
          value: "1536"
        volumeMounts:
        - name: config
          mountPath: /app/config.yml
          subPath: config.yml
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "4Gi"
            cpu: "2000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 5
      volumes:
      - name: config
        configMap:
          name: paladin-config
```

Deploy to Kubernetes:

```bash
# Create namespace
kubectl create namespace paladin

# Apply configurations
kubectl apply -f k8s/qdrant-statefulset.yaml
kubectl apply -f k8s/paladin-deployment.yaml

# Verify deployment
kubectl get pods -n paladin
kubectl logs -n paladin -l app=paladin
```

## Cloud Deployments

### AWS (EKS + Qdrant)

#### Option 1: Self-Hosted on EKS

Use the Kubernetes manifests above with EKS-specific storage class:

```yaml
# Use AWS EBS for storage
volumeClaimTemplates:
  - metadata:
      name: qdrant-storage
    spec:
      accessModes: ["ReadWriteOnce"]
      storageClassName: "gp3"  # AWS EBS GP3
      resources:
        requests:
          storage: 100Gi
```

#### Option 2: Qdrant Cloud

```yaml
# config.yml
sanctum:
  enabled: true
  adapter_type: "qdrant"
  qdrant:
    url: "https://your-cluster.qdrant.io:6334"
    collection_name: "aws_memories"
    vector_dimension: 1536
```

Set API key via environment:

```bash
export QDRANT_API_KEY=your_api_key_here
```

### GCP (GKE + Qdrant)

Use GCP persistent disk:

```yaml
volumeClaimTemplates:
  - metadata:
      name: qdrant-storage
    spec:
      accessModes: ["ReadWriteOnce"]
      storageClassName: "standard-rwo"  # GCP persistent disk
      resources:
        requests:
          storage: 100Gi
```

### Azure (AKS + Qdrant)

Use Azure managed disk:

```yaml
volumeClaimTemplates:
  - metadata:
      name: qdrant-storage
    spec:
      accessModes: ["ReadWriteOnce"]
      storageClassName: "managed-premium"  # Azure premium SSD
      resources:
        requests:
          storage: 100Gi
```

## Production Best Practices

### 1. High Availability

**Qdrant Cluster Mode** (v1.2.0+):

```yaml
# qdrant-config.yaml
cluster:
  enabled: true
  consensus:
    tick_period_ms: 100
  p2p:
    port: 6335
```

Deploy multiple Qdrant replicas:

```yaml
replicas: 3  # Minimum for HA
```

### 2. Resource Allocation

**CPU Guidelines**:
- Development: 0.5-1 CPU
- Production: 2-4 CPUs
- High load: 4-8 CPUs

**Memory Guidelines**:
- Base: 2 GB + (vectors * dimension * 4 bytes)
- Example: 1M vectors × 1536 dim = ~6 GB + 2 GB buffer = 8 GB

**Storage**:
- Use SSD for production (NVMe preferred)
- Plan for 2x growth capacity
- Enable compression (built into Qdrant)

### 3. Network Configuration

**Firewall Rules**:
- Port 6333: HTTP API (internal only)
- Port 6334: gRPC API (application access)
- Port 6335: P2P cluster communication (Qdrant cluster only)

**TLS Configuration**:

```yaml
service:
  http_port: 6333
  grpc_port: 6334
  enable_tls: true
  tls_cert: /path/to/cert.pem
  tls_key: /path/to/key.pem
```

### 4. Collection Configuration

**Optimal Settings**:

```rust
use qdrant_client::prelude::*;

// Configure collection for production
let collection_config = CreateCollection {
    collection_name: "production_memories".to_string(),
    vectors_config: Some(VectorsConfig {
        params: Some(VectorParams {
            size: 1536,
            distance: Distance::Cosine,
            hnsw_config: Some(HnswConfig {
                m: 16,  // Number of edges per node (higher = better recall, more memory)
                ef_construct: 200,  // Build-time accuracy (higher = better quality, slower build)
                full_scan_threshold: 10000,
            }),
            quantization_config: Some(QuantizationConfig {
                scalar: Some(ScalarQuantization {
                    type_: ScalarType::Int8,  // Reduce memory by 4x
                    quantile: 0.99,
                    always_ram: true,
                }),
            }),
            on_disk: false,  // Keep vectors in RAM for speed
        }),
    }),
    // ... other settings
};
```

### 5. Security

**Authentication**:

```yaml
# qdrant-config.yaml
service:
  api_key: ${QDRANT_API_KEY}  # Use environment variable
```

**Network Policies (Kubernetes)**:

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: qdrant-network-policy
  namespace: paladin
spec:
  podSelector:
    matchLabels:
      app: qdrant
  policyTypes:
  - Ingress
  ingress:
  - from:
    - podSelector:
        matchLabels:
          app: paladin
    ports:
    - protocol: TCP
      port: 6334
```

### 6. Backup Strategy

**Automated Snapshots**:

```bash
# Create snapshot
curl -X POST 'http://localhost:6333/collections/paladin_memories/snapshots'

# List snapshots
curl 'http://localhost:6333/collections/paladin_memories/snapshots'

# Download snapshot
curl -O 'http://localhost:6333/collections/paladin_memories/snapshots/snapshot-2024-01-30.snapshot'
```

**Kubernetes CronJob**:

```yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: qdrant-backup
  namespace: paladin
spec:
  schedule: "0 2 * * *"  # Daily at 2 AM
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: backup
            image: curlimages/curl:latest
            command:
            - sh
            - -c
            - |
              curl -X POST http://qdrant:6333/collections/paladin_memories/snapshots
              # Upload to S3/GCS/Azure Storage
          restartPolicy: OnFailure
```

## Monitoring

### Metrics to Track

**Qdrant Metrics**:
- Collection size (number of vectors)
- Search latency (p50, p95, p99)
- Memory usage
- CPU utilization
- Disk I/O

**Application Metrics**:
- Store operation latency
- Search operation latency
- Error rates
- Cache hit rates

### Prometheus Integration

```yaml
# prometheus-config.yaml
scrape_configs:
  - job_name: 'qdrant'
    static_configs:
      - targets: ['qdrant:6333']
    metrics_path: '/metrics'
```

### Grafana Dashboard

Key panels:
1. **Search Performance**: p95 latency over time
2. **Storage Growth**: Collection size trend
3. **Resource Usage**: CPU/Memory utilization
4. **Error Rates**: Failed operations per minute

## Backup and Recovery

### Full Backup

```bash
#!/bin/bash
# backup-qdrant.sh

COLLECTION="paladin_memories"
BACKUP_DIR="/backups/$(date +%Y%m%d)"
QDRANT_URL="http://localhost:6333"

# Create snapshot
SNAPSHOT=$(curl -s -X POST "${QDRANT_URL}/collections/${COLLECTION}/snapshots" | jq -r '.result.name')

# Download snapshot
curl -o "${BACKUP_DIR}/${SNAPSHOT}" \
  "${QDRANT_URL}/collections/${COLLECTION}/snapshots/${SNAPSHOT}"

# Upload to S3
aws s3 cp "${BACKUP_DIR}/${SNAPSHOT}" \
  "s3://paladin-backups/qdrant/${COLLECTION}/${SNAPSHOT}"
```

### Restore from Backup

```bash
#!/bin/bash
# restore-qdrant.sh

COLLECTION="paladin_memories"
SNAPSHOT_FILE="$1"
QDRANT_URL="http://localhost:6333"

# Upload snapshot to Qdrant
curl -X POST "${QDRANT_URL}/collections/${COLLECTION}/snapshots/upload" \
  -F "snapshot=@${SNAPSHOT_FILE}"

# Restore from snapshot
curl -X PUT "${QDRANT_URL}/collections/${COLLECTION}/snapshots/recover" \
  -H "Content-Type: application/json" \
  -d "{\"location\": \"${SNAPSHOT_FILE}\"}"
```

### Disaster Recovery Plan

1. **Regular Backups**: Daily automated snapshots
2. **Off-site Storage**: Copy to cloud storage (S3/GCS/Azure)
3. **Test Restores**: Monthly restore validation
4. **RPO/RTO**: Define acceptable data loss and recovery time
5. **Runbook**: Document recovery procedures

## Troubleshooting

### High Memory Usage

**Symptoms**: OOM kills, swapping

**Solutions**:
1. Enable quantization to reduce memory 4x:
   ```rust
   quantization_config: Some(QuantizationConfig {
       scalar: Some(ScalarQuantization {
           type_: ScalarType::Int8,
       }),
   })
   ```

2. Move vectors to disk:
   ```rust
   on_disk: true  // Slower but uses less RAM
   ```

3. Increase node resources

### Slow Search Performance

**Symptoms**: Search > 500ms consistently

**Solutions**:
1. Increase HNSW ef parameter:
   ```rust
   ef_construct: 200  // Higher = better accuracy
   ```

2. Tune search parameters:
   ```rust
   search_params: Some(SearchParams {
       hnsw_ef: Some(128),  // Higher = more accurate but slower
       exact: false,
   })
   ```

3. Add filters to reduce search space

### Connection Timeouts

**Symptoms**: "Failed to connect to Qdrant"

**Solutions**:
1. Verify Qdrant is running:
   ```bash
   curl http://localhost:6333/health
   ```

2. Check network connectivity:
   ```bash
   telnet qdrant 6334
   ```

3. Increase timeouts:
   ```rust
   QdrantClient::builder()
       .with_timeout(Duration::from_secs(30))
       .build()
   ```

## Cost Optimization

### Resource Right-Sizing

**Start Small**:
- 2 GB RAM for <100K vectors
- 4 GB RAM for <1M vectors
- Scale based on metrics

### Storage Optimization

**Techniques**:
1. **Quantization**: Reduce memory/storage by 75%
2. **Compression**: Built into Qdrant (ZSTD)
3. **Pruning**: Delete old/unused memories

### Cloud Cost Management

**Tips**:
- Use spot/preemptible instances for non-critical workloads
- Scale down non-prod environments off-hours
- Use Qdrant Cloud for predictable costs
- Monitor and set budget alerts

---

**Next Steps**:
- [Migration Guide](./SANCTUM_MIGRATION.md)
- [Main Documentation](./SANCTUM.md)
- [Performance Tuning](./guides/performance-tuning.md)
