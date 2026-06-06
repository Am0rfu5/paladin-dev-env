# Queue / Worker (Distributed)

Decouple *requesting* an agent run from *executing* it: **producers enqueue jobs** onto a
Redis-backed queue, and a pool of **workers** dequeue and run them. This gives you
horizontal scale, backpressure under load, retries, and fault isolation — a slow or failing
worker doesn't block producers.

> The example below is compiled code pulled from the `paladin-doc-examples` crate via mdBook
> `{{#include}}`. The Redis calls compile but are not executed by the check gate, so it stays
> in sync with the `RedisQueueAdapter` API without needing a live Redis.

> **Prerequisites:** Run `make dev` (starts Redis) first, and enable the `redis-queue`
> feature on `paladin-storage`.

## When to choose it

- **Choose it when** you need scale-out across workers/hosts, backpressure for bursty load,
  automatic retries, or isolation between job execution and your request path.
- **Look elsewhere when** load is low and in-process execution suffices
  ([embedded library](embedded-library.md)), or you only need synchronous request/response
  ([HTTP service host](http-service-host.md)).

## Producer and worker

The producer enqueues a typed `AgentJob`; the worker dequeues it (as generic JSON), runs the
agent through a `PaladinExecutionService`, and marks the item complete:

```rust
{{#include ../../../crates/doc-examples/src/queue_worker.rs:queue}}
```

Run many workers — in this process via several `tokio` tasks, or as separate processes across
hosts — all pulling from the same queue. `start_processing` / `complete_processing` /
`fail_processing` track each item's lifecycle, and failures can retry up to the configured
limit.

## Configuring the queue

`RedisQueueConfig` is typically populated from `config.yml`:

```yaml
queue:
  redis_host: "localhost"
  redis_port: 6379
  redis_db: 0
  connection_timeout: 30
  key_prefix: "paladin:queue"
  max_retries: 3
```

## See also

- Standing up Redis and the adapter in detail —
  [Redis Queue Adapter Setup](../appendix/redis-queue-adapter-setup.md).
- Each worker is itself an [embedded](embedded-library.md) agent host; a worker can also run a
  [Battalion](battalion-orchestration.md) as its unit of work.

---

← Back to [Choosing a topology](overview.md)
