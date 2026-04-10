# Vector Search & Embedding Research

This document explores small, fast, and bundleable solutions for vector search and text embeddings for Hither.

## 1. Vector Search Engines (SQLite-Based)

Since Hither already targets multiple platforms and aims for simplicity, using a SQLite extension is a strong candidate.

*   **[sqlite-vec](https://github.com/asg017/sqlite-vec)**: 
    *   **Description**: The successor to `sqlite-vss`, written in pure C with zero dependencies.
    *   **Pros**: Extremely portable (Windows, Mac, Linux, WASM, Mobile). Supports SIMD acceleration. Easy to bundle as a static or dynamic library.
    *   **Cons**: Primarily focused on brute-force search (with SIMD), so it may be slower than ANN (Approximate Nearest Neighbor) for massive datasets (1M+ vectors), but perfect for typical local CLI use cases.
*   **[sqlite-vector](https://github.com/asg017/sqlite-vector)**: 
    *   **Description**: A high-performance alternative that stores vectors in regular BLOB columns.
    *   **Pros**: Claims to be up to 17x faster than `sqlite-vec`. 
    *   **Cons**: Slightly more complex to integrate if virtual table abstractions are preferred.
*   **[native-rust-alternatives]**:
    *   **[LanceDB](https://github.com/lancedb/lancedb)**: A serverless vector database written in Rust. Very fast, but might be "heavy" for a simple CLI tool compared to a SQLite extension.
    *   **[Faiss-rs](https://github.com/cognotiv/faiss-rs)**: Rust bindings for FAISS. Powerful but brings in heavy C++ dependencies, making cross-compilation difficult.

## 2. Embedding Models (Small & Fast)

To generate vectors locally, we need models that can run efficiently on a CPU.

*   **[BGE-Micro-v2](https://huggingface.co/BAAI/bge-micro-v2)**: 
    *   **Size**: ~20-30MB.
    *   **Performance**: One of the best performing models for its size.
*   **[All-MiniLM-L6-v2](https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2)**:
    *   **Size**: ~80MB.
    *   **Performance**: The "industry standard" for small, fast embeddings.
*   **[snowflake-arctic-embed-tiny](https://huggingface.co/Snowflake/snowflake-arctic-embed-tiny)**:
    *   **Size**: ~30MB.
    *   **Performance**: Highly optimized for retrieval tasks.

## 3. Inference Engines (Native Rust)

To run the models above without external dependencies:

*   **[Candle](https://github.com/huggingface/candle)**: Hugging Face's Rust-native ML framework. Best for bundling models directly into the Hither binary.
*   **[Ort (ONNX Runtime)](https://github.com/pykeio/ort)**: Rust bindings for ONNX Runtime. Very fast and supports a wide range of models, but requires bundling the ONNX shared library.

## Recommendations for Hither

| Component | Choice | Reason |
| :--- | :--- | :--- |
| **Vector Search** | **`sqlite-vec`** | Zero dependencies, pure C, cross-platform, and perfectly matches the "portable" nature of Hither. |
| **Inference** | **`Candle`** | Pure Rust, no C++ toolchain required for consumers, and small binary overhead. |
| **Model** | **`BGE-Micro-v2`** | Exceptional balance of size (~25MB) and retrieval quality. |

### Integration Strategy
1.  **Host**: Embeds `Candle` and `sqlite-vec`.
2.  **Interface**: The `ai.wit` file already defines `get-embeddings`. The host will implement this using `Candle` + `BGE-Micro-v2`.
3.  **Storage**: The host provides a way for guests to query the vector store, or guests can use the SQLite extension directly if we expose it.
