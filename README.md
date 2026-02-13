<h1 align="center">2SCloud DNS</h1>

<p align="center">
  <a href="https://2scloud.github.io/scloud-dns/coverage/">
    <img src="https://img.shields.io/endpoint?url=https://2scloud.github.io/scloud-dns/coverage.json" alt="Coverage">
  </a>
  <a href="https://github.com/2SCloud/scloud-dns/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/2SCloud/scloud-dns/coverage.yml?branch=main" alt="Build Status">
  </a>
  <a href="https://github.com/2SCloud/scloud-dns/issues">
    <img src="https://img.shields.io/github/issues/2SCloud/scloud-dns" alt="Issues">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/LICENSE-MIT_(Non_Commercial)-blue" alt="License">
  </a>
</p>


`scloud-dns` is a Rust application for managing and querying DNS servers. It allows you to build and send DNS queries, analyze responses, and view DNS records conveniently.

---

## What is 2SCloud DNS?

2SCloud DNS is a modern DNS server written entirely in Rust.

It is designed to be:

- Fast
- Concurrent
- Memory-safe
- Cloud-native
- Architected around a modular worker system

Unlike traditional DNS utilities, 2SCloud DNS is a real DNS server engine built from the ground up with a focus on performance, scalability and clean architecture.

It runs:

- On Linux
- Inside Docker
- In Kubernetes clusters
- In cloud-native environments
  
---

## For Non-Technical Readers

When you open a website like:

    github.com

Your device asks a DNS server:

    “What is the IP address of github.com?”

2SCloud DNS is a program that answers that question.

Its goal is to:

- Handle many requests at the same time
- Respond quickly
- Remain stable under heavy load
- Be ready for modern cloud infrastructure

It is not just a simple command-line DNS tool.
It is a fully designed DNS server architecture.

---

## Features

- Fully asynchronous architecture (Tokio)
- Multi-worker pipeline
- Semaphore-based backpressure
- Zero shared mutable state between workers
- Linux runtime introspection support
- UDP support (TCP supported via architecture)
- Designed for Kubernetes
- Distroless container ready
- Structured logging
- Built for high throughput DNS workloads

Planned:

- DNS zone management
- Recursive resolution
- Intelligent caching with TTL
- DNSSEC support
- Metrics endpoint

---

## Installation

Clone the project and build with Cargo:

```bash
git clone https://github.com/2SCloud/scloud-dns.git
cd scloud-dns
cargo run --release --package scloud-dns --bin scloud-dns
sudo mkdir -p /var/log/scloud-dns
sudo chown -R $USER:$USER /var/log/scloud-dns
cd target/release
./scloud-dns
```

The compiled binary will be located in `target/release/scloud-dns`.

---

## Architecture Overview (Technical)

2SCloud DNS uses an in-process actor-style runtime built with:

- Rust
- Tokio
- Message passing (tokio::mpsc)
- Semaphore-based backpressure
- Atomic state tracking

There is no external message broker in the hot path.

Each component of the DNS pipeline runs in a dedicated worker:

| Worker              | Responsibility                         |
|---------------------|----------------------------------------|
| [LISTENER](https://github.com/2SCloud/scloud-dns/blob/ae8efac70824dd1229c21b911afd4b450d77e2d8/src/threads/mod.rs#L563)                      | Receives incoming UDP DNS packets                  |
| [DECODER](https://github.com/2SCloud/scloud-dns/blob/ae8efac70824dd1229c21b911afd4b450d77e2d8/src/threads/mod.rs#L564)                       | Parses raw DNS packets into structured objects     |
| [QUERY_DISPATCHER](https://github.com/2SCloud/scloud-dns/blob/ae8efac70824dd1229c21b911afd4b450d77e2d8/src/threads/mod.rs#L565)              | Determines how queries should be processed         |
| [CACHE_LOOKUP](https://github.com/2SCloud/scloud-dns/blob/ae8efac70824dd1229c21b911afd4b450d77e2d8/src/threads/mod.rs#L566)                  | Checks in-memory cache before resolution           |
| [ZONE_MANAGER](https://github.com/2SCloud/scloud-dns/blob/ae8efac70824dd1229c21b911afd4b450d77e2d8/src/threads/mod.rs#L567)                  | Manages authoritative DNS zones and records        |
| [RESOLVER](https://github.com/2SCloud/scloud-dns/blob/ae8efac70824dd1229c21b911afd4b450d77e2d8/src/threads/mod.rs#L568)                      | Performs recursive or authoritative resolution     |
| [CACHE_WRITER](https://github.com/2SCloud/scloud-dns/blob/ae8efac70824dd1229c21b911afd4b450d77e2d8/src/threads/mod.rs#L569)                  | Updates cache entries after resolution             |
| [ENCODER](https://github.com/2SCloud/scloud-dns/blob/ae8efac70824dd1229c21b911afd4b450d77e2d8/src/threads/mod.rs#L570)                       | Builds DNS response packets                        |
| [SENDER](https://github.com/2SCloud/scloud-dns/blob/ae8efac70824dd1229c21b911afd4b450d77e2d8/src/threads/mod.rs#L571)                        | Sends DNS responses back to clients                |
| [CACHE_JANITOR](https://github.com/2SCloud/scloud-dns/blob/ae8efac70824dd1229c21b911afd4b450d77e2d8/src/threads/mod.rs#L573)                 | Cleans expired cache entries and manages TTL logic |
| [METRICS](https://github.com/2SCloud/scloud-dns/blob/ae8efac70824dd1229c21b911afd4b450d77e2d8/src/threads/mod.rs#L575)                       | Collects and aggregates runtime metrics            |
| [TCP_ACCEPTOR](https://github.com/2SCloud/scloud-dns/blob/ae8efac70824dd1229c21b911afd4b450d77e2d8/src/threads/mod.rs#L576)                  | Accepts and manages incoming TCP DNS connections   |

Each worker:

- Communicates through in-memory channels
- Has concurrency limits enforced by a Semaphore

---

## Concurrency Model

Instead of using distributed brokers (RabbitMQ / Pulsar), 2SCloud DNS relies on:

- In-process message passing
- Lock-free atomic counters
- Controlled in-flight limits
- Async task orchestration

Example:

- `in_flight` tracks active request processing
- `Semaphore` enforces max concurrency (e.g., 512 simultaneous packets)
- No disk writes on hot path
- No external network hop

This allows microsecond-level packet handling and high request-per-second throughput.

---

## Testing

```bash
cargo test
```

Coverage:
[https://2scloud.github.io/scloud-dns/](https://2scloud.github.io/scloud-dns/)

---

## Observability & Debugging `scloud-dns`

This section lists useful commands to inspect, debug, and analyze the runtime behavior and performance of the `scloud-dns` process.

All commands assume a Linux or WSL2 environment.

**Find the running `scloud-dns` process**

`ps -f -u $USER | grep scloud-dns`

Displays:
- PID / PPID
- approximate CPU usage
- start time
- controlling terminal
- command used to launch the binary

Example output:
```
UID    PID     PPID   C  STIME  TTY     TIME     CMD
onhlt  784566  784565 3  14:26  pts/2   00:00:09 target/debug/scloud-dns
```

---

**Inspect CPU and memory usage**

`ps -p <PID> -o pid,%cpu,%mem,rss,vsz,cmd`

Where:
- %cpu = CPU usage
- %mem = memory usage
- rss  = resident memory (KB)
- vsz  = virtual memory size

---

**Inspect threads (SCloudWorker & ThreadsOS)**

`ps -T -p <PID>`

Live view per thread:
`top -H -p <PID>`

Useful to:
- verify how many Tokio worker threads are running
- detect blocked or imbalanced threads

---

## Docker

```bash
docker build -t scloud-dns .
docker run --rm -p 53:53/udp -p 53:53/tcp scloud-dns
```

Test:

```bash
dig @127.0.0.1 -p 53 github.com
```

---

## Kubernetes

```bash
kubectl apply -f k8s/scloud-dns.yaml
kubectl get pods -n scloud-dns
```

Stress test inside cluster:

```bash
dnsperf -s rust-dns -p 53 -Q 1000 -l 30
```

---

## Performance Testing

Basic query:

```bash
dig @127.0.0.1 -p 53 example.com
```

High throughput:

```bash
dnsperf -s <SERVICE> -p 53 -Q 1000 -l 30
```

---

## Runtime Inspection (Linux)

Check threads:

```bash
ps -T -p <PID>
top -H -p <PID>
```

Check UDP sockets:

```bash
ss -u -n -p | grep <PID>
```

Kernel UDP stats:

```bash
cat /proc/net/snmp | tail -n 2
```

Trace syscalls:

```bash
sudo strace -tt -p <PID> -f -e trace=network
```

These tools help diagnose:

- Packet drops
- Kernel buffer saturation
- CPU bottlenecks
- Thread imbalance

---

## Why No Message Broker?

`2scloud-dns` intentionally avoids external brokers (RabbitMQ, Pulsar) in the DNS request path.

Reasons:

- DNS requires extremely low latency
- External brokers add disk and network overhead
- In-process concurrency control is sufficient
- Designed for high PPS workloads

External systems may later handle:

- Logging
- Metrics streaming
- Distributed coordination

But never the hot request path.

---

## Contribution

Contributions are welcome!

1. Fork the project
2. Create your branch: `git checkout -b feature/issue-XXXX` or `fix/issue-XXXX`
3. Commit your changes: `git commit -m "feat(scope-here): your message here"` or `git commit -m "fix(scope-here): your message here"`...
4. Push your branch: `git push origin feature/issue-XXXX` or `fix/issue-XXXX`
5. Open a Pull Request

---

## Licence

This project is licensed under the MIT (Non-Commercial) License. See the [LICENSE](LICENSE) file for details.

---

> This project is part of the **2SCloud** organization.
> 2scloud-dns is part of the broader 2SCloud ecosystem focused on building modern, cloud-native infrastructure components.
