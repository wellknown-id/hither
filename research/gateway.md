# OpenAI-Compatible Gateway Research (Rust)

This document explores Rust-based solutions for building or deploying an OpenAI-compatible API gateway. This is useful for unified access to multiple LLM providers (OpenAI, Anthropic, Ollama) with a single API interface.

## 1. High-Performance Standalone Gateways

If the goal is a drop-in replacement for Python-based solutions like LiteLLM:

*   **[TensorZero](https://github.com/tensorzero/tensorzero)**:
    *   **Description**: A high-performance, enterprise-grade AI gateway written in Rust.
    *   **Pros**: Ultra-low overhead (<1ms), unified OpenAI-compatible API, built-in observability, and support for "recipes" (optimization paths like fine-tuning).
    *   **Best For**: Replacing LiteLLM in production where performance and scale are critical.
*   **[LangDB AI Gateway](https://github.com/langdb/ai-gateway)**:
    *   **Description**: Focuses on the governance and management side of AI traffic.
    *   **Pros**: Unified interface for OpenAI, Anthropic, Gemini, etc. Includes usage analytics and cost tracking.
*   **[Sentinel](https://github.com/Noveum/ai-gateway)**:
    *   **Description**: A lightweight, local-first gateway.
    *   **Pros**: Automatic failover, PII redaction, and SQLite audit logging.

## 2. Libraries for Custom Implementation

If Hither needs to bundle its own gateway logic within the `hither` binary:

*   **[Axum](https://github.com/tokio-rs/axum)**:
    *   **Description**: The de-facto standard web framework for the Tokio ecosystem.
    *   **Supporting Crates**: 
        *   `axum-reverse-proxy`: Handles path-based routing and forwarding.
        *   `axum_proxy`: A Tower-based service for basic rewriting.
    *   **Pros**: Extremely flexible, integrates with the `tower` middleware ecosystem (auth, rate limiting, logging).
*   **[Actix-web](https://github.com/actix/actix-web)**:
    *   **Supporting Crates**: `actix-proxy`.
    *   **Pros**: Known for extreme performance and a mature ecosystem.

## 3. Specialized Proxy Tools

*   **[openai-realtime-proxy](https://crates.io/crates/openai-realtime-proxy)**:
    *   **Description**: Specifically for proxying OpenAI's WebSocket-based Realtime API.
    *   **Use Case**: If Hither needs to support low-latency voice or multimodal interactions via the Realtime API.

## Recommendations for Hither

| Priority | Choice | Reason |
| :--- | :--- | :--- |
| **Custom Integration** | **Axum** + **`tower`** | Best for embedding within the `hither` binary. It allows us to keep the binary small while having full control over routing and auth. |
| **Streaming Support** | **`reqwest`** (Client) | For forwarding streaming responses (SSE), `reqwest` is the standard and integrates perfectly with Axum. |
| **Standalone Option** | **TensorZero** | If the user wants to run a separate high-performance gateway process. |

### Summary
For the Hither project, embedding a lightweight gateway using **Axum** is the most architectural-aligned choice. It allows Hither to act as a local "hub" that WASM guests can talk to via the `ai.wit` interface, while the host handles the complex logic of routing to OpenAI or local Ollama instances.
