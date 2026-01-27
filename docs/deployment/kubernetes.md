# Kubernetes Deployment Guide

Complete guide for deploying Paladin on Kubernetes with high availability, scalability, and production best practices.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Architecture](#architecture)
- [Kubernetes Manifests](#kubernetes-manifests)
- [ConfigMaps and Secrets](#configmaps-and-secrets)
- [Helm Chart](#helm-chart)
- [Resource Management](#resource-management)
- [High Availability](#high-availability)
- [Horizontal Scaling](#horizontal-scaling)
- [Storage](#storage)
- [Networking](#networking)
- [Monitoring](#monitoring)
- [Security](#security)
- [Troubleshooting](#troubleshooting)

## Overview

Paladin on Kubernetes provides:
- **High Availability**: Multi-replica deployments with health checks
- **Auto-scaling**: HPA based on CPU/memory/custom metrics
- **Rolling Updates**: Zero-downtime deployments
- **Resource Management**: CPU/memory limits and requests
- **Service Discovery**: Internal DNS for service communication

## Prerequisites

```bash
# Kubernetes 1.25+
kubectl version

# Helm 3.0+ (optional but recommended)
helm version

# kubectl-ctx and kubectl-ns (optional, for context switching)
kubectl ctx
kubectl ns
```

## Quick Start

### Using Kubectl

```bash
# Create namespace
kubectl create namespace paladin

# Apply manifests
kubectl apply -f k8s/ -n paladin

# Check status
kubectl get pods -n paladin
kubectl get svc -n paladin

# View logs
kubectl logs -f deployment/paladin -n paladin
```

### Using Helm

```bash
# Add Paladin Helm repository
helm repo add paladin https://charts.paladin.dev
helm repo update

# Install with default values
helm install paladin paladin/paladin -n paladin --create-namespace

# Install with custom values
helm install paladin paladin/paladin \
  -n paladin \
  --create-namespace \
  --values values.yaml

# Upgrade
helm upgrade paladin paladin/paladin -n paladin

# Uninstall
helm uninstall paladin -n paladin
```

## Architecture

```
┌──────────────────────────────────────────────────────┐
│              Kubernetes Cluster                       │
│                                                       │
│  ┌────────────────────────────────────────────────┐ │
│  │           Namespace: paladin                    │ │
│  │                                                  │ │
│  │  ┌──────────────┐      ┌──────────────┐       │ │
│  │  │   Ingress    │      │   Service    │       │ │
│  │  │  (External)  │─────▶│ (ClusterIP)  │       │ │
│  │  └──────────────┘      └───────┬──────┘       │ │
│  │                                 │               │ │
│  │                        ┌────────▼────────┐     │ │
│  │                        │   Deployment    │     │ │
│  │                        │  (Paladin x3)   │     │ │
│  │                        └────┬───┬───┬────┘     │ │
│  │                             │   │   │          │ │
│  │                 ┌───────────┼───┼───┼───────┐ │ │
│  │                 │           │   │   │       │ │ │
│  │            ┌────▼───┐  ┌───▼───▼───▼────┐  │ │ │
│  │            │ Redis  │  │ MinIO/S3        │  │ │ │
│  │            │StatefulSet│ │ StatefulSet    │  │ │ │
│  │            └────────┘  └────────────────┘  │ │ │
│  │                                              │ │ │
│  │  ┌──────────────┐      ┌──────────────┐   │ │ │
│  │  │  ConfigMap   │      │   Secret     │   │ │ │
│  │  │  (config.yml)│      │  (API keys)  │   │ │ │
│  │  └──────────────┘      └──────────────┘   │ │ │
│  └─────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────┘
```

## Kubernetes Manifests

### Namespace

```yaml
# k8s/00-namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: paladin
  labels:
    app: paladin
    environment: production
```

### Deployment

```yaml
# k8s/10-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: paladin
  namespace: paladin
  labels:
    app: paladin
    component: server
spec:
  replicas: 3
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  selector:
    matchLabels:
      app: paladin
      component: server
  template:
    metadata:
      labels:
        app: paladin
        component: server
      annotations:
        prometheus.io/scrape: "true"
        prometheus.io/port: "8081"
        prometheus.io/path: "/metrics"
    spec:
      serviceAccountName: paladin
      securityContext:
        runAsNonRoot: true
        runAsUser: 1000
        fsGroup: 1000
      
      initContainers:
      - name: wait-for-redis
        image: busybox:1.35
        command: ['sh', '-c', 'until nc -zv redis 6379; do echo waiting for redis; sleep 2; done;']
      
      containers:
      - name: paladin
        image: ghcr.io/your-org/paladin:v0.1.0
        imagePullPolicy: IfNotPresent
        
        ports:
        - name: http
          containerPort: 8080
          protocol: TCP
        - name: metrics
          containerPort: 8081
          protocol: TCP
        
        env:
        - name: SERVER_HOST
          value: "0.0.0.0"
        - name: SERVER_PORT
          value: "8080"
        - name: LOG_LEVEL
          value: "info"
        - name: RUST_LOG
          value: "info,paladin=debug"
        
        # Secrets from Secret resource
        - name: OPENAI_API_KEY
          valueFrom:
            secretKeyRef:
              name: paladin-secrets
              key: openai-api-key
        - name: DEEPSEEK_API_KEY
          valueFrom:
            secretKeyRef:
              name: paladin-secrets
              key: deepseek-api-key
              optional: true
        - name: ANTHROPIC_API_KEY
          valueFrom:
            secretKeyRef:
              name: paladin-secrets
              key: anthropic-api-key
              optional: true
        
        # Mount configuration
        volumeMounts:
        - name: config
          mountPath: /app/config.yml
          subPath: config.yml
          readOnly: true
        - name: data
          mountPath: /app/data
        - name: tmp
          mountPath: /tmp
        
        # Resource limits
        resources:
          requests:
            cpu: 500m
            memory: 1Gi
          limits:
            cpu: 2000m
            memory: 4Gi
        
        # Health checks
        livenessProbe:
          httpGet:
            path: /health
            port: http
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3
        
        readinessProbe:
          httpGet:
            path: /health/ready
            port: http
          initialDelaySeconds: 10
          periodSeconds: 5
          timeoutSeconds: 3
          failureThreshold: 3
        
        # Graceful shutdown
        lifecycle:
          preStop:
            exec:
              command: ["/bin/sh", "-c", "sleep 10"]
      
      volumes:
      - name: config
        configMap:
          name: paladin-config
      - name: data
        persistentVolumeClaim:
          claimName: paladin-data
      - name: tmp
        emptyDir: {}
      
      # Affinity for spreading pods across nodes
      affinity:
        podAntiAffinity:
          preferredDuringSchedulingIgnoredDuringExecution:
          - weight: 100
            podAffinityTerm:
              labelSelector:
                matchExpressions:
                - key: app
                  operator: In
                  values:
                  - paladin
              topologyKey: kubernetes.io/hostname
```

### Service

```yaml
# k8s/20-service.yaml
apiVersion: v1
kind: Service
metadata:
  name: paladin
  namespace: paladin
  labels:
    app: paladin
spec:
  type: ClusterIP
  selector:
    app: paladin
    component: server
  ports:
  - name: http
    port: 80
    targetPort: http
    protocol: TCP
  - name: metrics
    port: 8081
    targetPort: metrics
    protocol: TCP
  sessionAffinity: ClientIP
  sessionAffinityConfig:
    clientIP:
      timeoutSeconds: 10800
```

### Ingress

```yaml
# k8s/21-ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: paladin
  namespace: paladin
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
    nginx.ingress.kubernetes.io/proxy-body-size: "50m"
    nginx.ingress.kubernetes.io/proxy-read-timeout: "600"
    nginx.ingress.kubernetes.io/rate-limit: "100"
spec:
  ingressClassName: nginx
  tls:
  - hosts:
    - paladin.example.com
    secretName: paladin-tls
  rules:
  - host: paladin.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: paladin
            port:
              number: 80
```

## ConfigMaps and Secrets

### ConfigMap

```yaml
# k8s/30-configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: paladin-config
  namespace: paladin
data:
  config.yml: |
    server:
      host: "0.0.0.0"
      port: 8080
      log_level: "info"
    
    paladin:
      default_model: "gpt-4"
      default_temperature: 0.7
      default_max_loops: 3
      timeout_seconds: 300
    
    garrison:
      type: "sqlite"
      path: "/app/data/garrison.db"
      max_entries: 1000
      max_tokens: 8000
    
    arsenal:
      mcp_servers:
        - name: "web_search"
          type: "stdio"
          command: "uvx"
          args: ["mcp-web-search"]
    
    llm:
      openai:
        base_url: "https://api.openai.com/v1"
      deepseek:
        base_url: "https://api.deepseek.com/v1"
      anthropic:
        base_url: "https://api.anthropic.com/v1"
    
    storage:
      type: "minio"
      endpoint: "minio.paladin.svc.cluster.local:9000"
      bucket: "paladin"
      use_ssl: false
    
    queue:
      type: "redis"
      url: "redis://redis.paladin.svc.cluster.local:6379"
```

### Secret

```bash
# Create secret from literals
kubectl create secret generic paladin-secrets \
  --from-literal=openai-api-key="sk-..." \
  --from-literal=deepseek-api-key="..." \
  --from-literal=anthropic-api-key="..." \
  -n paladin

# Or from env file
kubectl create secret generic paladin-secrets \
  --from-env-file=secrets.env \
  -n paladin

# Or from YAML (base64 encoded)
# k8s/31-secret.yaml
apiVersion: v1
kind: Secret
metadata:
  name: paladin-secrets
  namespace: paladin
type: Opaque
data:
  openai-api-key: <base64-encoded-key>
  deepseek-api-key: <base64-encoded-key>
  anthropic-api-key: <base64-encoded-key>
```

## Helm Chart

### Chart Structure

```
paladin-chart/
├── Chart.yaml
├── values.yaml
├── templates/
│   ├── _helpers.tpl
│   ├── deployment.yaml
│   ├── service.yaml
│   ├── ingress.yaml
│   ├── configmap.yaml
│   ├── secret.yaml
│   ├── serviceaccount.yaml
│   ├── hpa.yaml
│   ├── pdb.yaml
│   └── NOTES.txt
└── crds/
```

### values.yaml

```yaml
# Default values for paladin
replicaCount: 3

image:
  repository: ghcr.io/your-org/paladin
  tag: "v0.1.0"
  pullPolicy: IfNotPresent

serviceAccount:
  create: true
  name: paladin

service:
  type: ClusterIP
  port: 80
  targetPort: 8080

ingress:
  enabled: true
  className: nginx
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
  hosts:
    - host: paladin.example.com
      paths:
        - path: /
          pathType: Prefix
  tls:
    - secretName: paladin-tls
      hosts:
        - paladin.example.com

resources:
  requests:
    cpu: 500m
    memory: 1Gi
  limits:
    cpu: 2000m
    memory: 4Gi

autoscaling:
  enabled: true
  minReplicas: 3
  maxReplicas: 10
  targetCPUUtilizationPercentage: 70
  targetMemoryUtilizationPercentage: 80

persistence:
  enabled: true
  storageClass: "fast-ssd"
  accessMode: ReadWriteOnce
  size: 10Gi

# Paladin configuration
config:
  paladin:
    defaultModel: "gpt-4"
    defaultTemperature: 0.7
    defaultMaxLoops: 3
  
  garrison:
    type: "sqlite"
    maxEntries: 1000
    maxTokens: 8000
  
  redis:
    url: "redis://redis:6379"
  
  minio:
    endpoint: "minio:9000"
    bucket: "paladin"

# Secrets (should be overridden)
secrets:
  openaiApiKey: ""
  deepseekApiKey: ""
  anthropicApiKey: ""
```

### Install with Helm

```bash
# Create values-prod.yaml
cat > values-prod.yaml <<EOF
replicaCount: 5

ingress:
  hosts:
    - host: paladin.prod.example.com
      paths:
        - path: /
          pathType: Prefix

resources:
  requests:
    cpu: 1000m
    memory: 2Gi
  limits:
    cpu: 4000m
    memory: 8Gi

autoscaling:
  enabled: true
  minReplicas: 5
  maxReplicas: 20

secrets:
  openaiApiKey: ${OPENAI_API_KEY}
EOF

# Install
helm install paladin ./paladin-chart \
  -n paladin \
  --create-namespace \
  -f values-prod.yaml
```

## Resource Management

### Resource Requests and Limits

```yaml
resources:
  requests:
    cpu: 500m       # Guaranteed CPU
    memory: 1Gi     # Guaranteed memory
  limits:
    cpu: 2000m      # Max CPU (burst)
    memory: 4Gi     # Max memory (OOM if exceeded)
```

### QoS Classes

| Class | Configuration | Behavior |
|-------|---------------|----------|
| **Guaranteed** | requests = limits | Highest priority, last to evict |
| **Burstable** | requests < limits | Medium priority |
| **BestEffort** | No requests/limits | Lowest priority, first to evict |

**Recommendation**: Use **Burstable** for production (requests < limits).

### Resource Quotas

```yaml
# k8s/40-resourcequota.yaml
apiVersion: v1
kind: ResourceQuota
metadata:
  name: paladin-quota
  namespace: paladin
spec:
  hard:
    requests.cpu: "10"
    requests.memory: "20Gi"
    limits.cpu: "20"
    limits.memory: "40Gi"
    pods: "50"
    services: "10"
    persistentvolumeclaims: "10"
```

## High Availability

### Pod Disruption Budget

```yaml
# k8s/41-pdb.yaml
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: paladin
  namespace: paladin
spec:
  minAvailable: 2
  selector:
    matchLabels:
      app: paladin
```

### Multi-Zone Deployment

```yaml
affinity:
  podAntiAffinity:
    preferredDuringSchedulingIgnoredDuringExecution:
    - weight: 100
      podAffinityTerm:
        labelSelector:
          matchExpressions:
          - key: app
            operator: In
            values:
            - paladin
        topologyKey: topology.kubernetes.io/zone
```

## Horizontal Scaling

### Horizontal Pod Autoscaler

```yaml
# k8s/42-hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: paladin
  namespace: paladin
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: paladin
  minReplicas: 3
  maxReplicas: 10
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
  behavior:
    scaleDown:
      stabilizationWindowSeconds: 300
      policies:
      - type: Percent
        value: 50
        periodSeconds: 60
    scaleUp:
      stabilizationWindowSeconds: 0
      policies:
      - type: Percent
        value: 100
        periodSeconds: 30
      - type: Pods
        value: 2
        periodSeconds: 30
      selectPolicy: Max
```

## Storage

### PersistentVolumeClaim

```yaml
# k8s/50-pvc.yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: paladin-data
  namespace: paladin
spec:
  accessModes:
    - ReadWriteOnce
  storageClassName: fast-ssd
  resources:
    requests:
      storage: 10Gi
```

### StatefulSet for Redis

```yaml
# k8s/51-redis-statefulset.yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: redis
  namespace: paladin
spec:
  serviceName: redis
  replicas: 1
  selector:
    matchLabels:
      app: redis
  template:
    metadata:
      labels:
        app: redis
    spec:
      containers:
      - name: redis
        image: redis:7-alpine
        ports:
        - containerPort: 6379
          name: redis
        volumeMounts:
        - name: data
          mountPath: /data
  volumeClaimTemplates:
  - metadata:
      name: data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      storageClassName: fast-ssd
      resources:
        requests:
          storage: 5Gi
```

## Networking

### Network Policies

```yaml
# k8s/60-networkpolicy.yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: paladin
  namespace: paladin
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
    - podSelector:
        matchLabels:
          app: redis
    ports:
    - protocol: TCP
      port: 6379
  - to:
    - podSelector:
        matchLabels:
          app: minio
    ports:
    - protocol: TCP
      port: 9000
  - to: []  # Allow all external (LLM APIs)
```

## Monitoring

### ServiceMonitor (Prometheus Operator)

```yaml
# k8s/70-servicemonitor.yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: paladin
  namespace: paladin
  labels:
    app: paladin
spec:
  selector:
    matchLabels:
      app: paladin
  endpoints:
  - port: metrics
    interval: 30s
    path: /metrics
```

## Security

### ServiceAccount and RBAC

```yaml
# k8s/80-rbac.yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: paladin
  namespace: paladin

---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: paladin
  namespace: paladin
rules:
- apiGroups: [""]
  resources: ["configmaps", "secrets"]
  verbs: ["get", "list"]

---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: paladin
  namespace: paladin
subjects:
- kind: ServiceAccount
  name: paladin
  namespace: paladin
roleRef:
  kind: Role
  name: paladin
  apiGroup: rbac.authorization.k8s.io
```

## Troubleshooting

### Common Issues

```bash
# Pods not starting
kubectl describe pod <pod-name> -n paladin
kubectl logs <pod-name> -n paladin

# Service not accessible
kubectl get svc -n paladin
kubectl get endpoints -n paladin

# Config issues
kubectl get configmap paladin-config -o yaml -n paladin
kubectl get secret paladin-secrets -o yaml -n paladin

# Resource constraints
kubectl top pods -n paladin
kubectl describe node <node-name>

# Network issues
kubectl exec -it <pod-name> -n paladin -- curl http://redis:6379
kubectl get networkpolicy -n paladin
```

## Next Steps

- **[CI/CD](cicd.md)** - Automated deployments
- **[Monitoring](../operations/monitoring.md)** - Observability
- **[Production Best Practices](production-best-practices.md)** - Production checklist
