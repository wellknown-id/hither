# LLM Research: OpenAI-Compatible Rust Libraries & SDKs

This document summarizes the research into Rust libraries that can interact with OpenAI-compatible APIs, including local AI inference providers (like Ollama, LocalAI, vLLM, etc.).

## 1. High-Level Frameworks (Agentic & RAG)
These libraries provide a unified interface for multiple providers, making it easy to swap between OpenAI and local models.

*   **[Rig](https://github.com/0xPlaygrounds/rig)**: A modular library for building LLM applications (agents, RAG). It offers a unified API for 20+ providers, including OpenAI and local backends like Ollama. Highly recommended for production-ready agentic workflows.
*   **[llm-chain](https://github.com/sobelio/llm-chain)**: A framework for chaining LLM steps together. It supports multiple drivers, including OpenAI and local models.

## 2. OpenAI-Compatible Clients
If you are running a local server that mimics the OpenAI API, these are the best tools to interact with it.

*   **[async-openai](https://github.com/64bit/async-openai)**: The most popular unofficial Rust client for OpenAI. It is highly configurable; you can easily point it to a local endpoint by changing the `base_url` in the configuration.
*   **[async-llm](https://github.com/64bit/async-llm)**: A newer, specialized library designed specifically for OpenAI-compatible providers (OpenAI, Gemini, Ollama, etc.). It focuses on handling the slight differences between "compatible" APIs.
*   **[mini-openai](https://github.com/jofas/mini-openai)**: A lightweight, minimal-dependency crate for interacting with OpenAI-compatible servers.

## 3. Local Inference Engines (Native Rust)
These libraries allow you to run models directly within your Rust binary without needing an external Python runtime or C++ server.

*   **[Candle](https://github.com/huggingface/candle)**: Hugging Face’s minimalist ML framework for Rust. It is the "PyTorch of Rust" and serves as the foundation for many local inference projects.
*   **[Crane](https://github.com/lucasjinreal/Crane)**: A pure Rust inference engine built on Candle. It includes a built-in OpenAI-compatible API server and supports models like Qwen and Llama with hardware acceleration.
*   **[candle-vllm](https://github.com/EricLBuehler/candle-vllm)**: A Rust implementation of vLLM concepts using Candle, providing an OpenAI-compatible server optimized for high-throughput.
*   **[llm](https://github.com/rustformers/llm)**: A library for running GGUF/GGML models (like Llama) natively in Rust.

## 4. Specialized Local Clients
*   **[ollama-rs](https://github.com/pepperoni21/ollama-rs)**: A specialized library for using Ollama's native API rather than its OpenAI-compatible wrapper.

## Recommendations for Hither

| Priority | Library | Reason |
| :--- | :--- | :--- |
| **Primary (Client)** | `async-openai` | Industry standard for OpenAI-compatible APIs. Reliable and well-maintained. |
| **Agentic Framework** | `rig` | If the goal is to build complex agents or RAG pipelines, `rig` provides the best high-level abstraction. |
| **Native Inference** | `Candle` | If we want to bundle the model directly in the WASM guest or host without a separate server process. |

### Summary
For maximum compatibility with both OpenAI and local servers (Ollama, LocalAI), **`async-openai`** is the safest choice for a client. If we want to build a more complex system, **`rig`** is the better architectural choice.
