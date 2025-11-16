# Overview

A chat service using mistralrs paged attention libraries (vLLM) written in Rust 

## Mistral.rs implementation of paged-attention (used in vLLM)

- Blazingly fast cross platform LLM inference
- Performance: ISQ, PagedAttention, FlashAttention, per-layer topology optimization

Performance

- CPU acceleration (MKL, AVX, NEON, Accelerate)
- GPU acceleration (CUDA with FlashAttention & cuDNN, Metal)
- Automatic tensor parallelism for splitting models across multiple devices
- CUDA-specialized NCCL
- Heterogeneous, flexible Ring backend

Quantization & Optimization

- Per-layer topology: Fine-tune quantization per layer for optimal quality/speed balance
- In-place quantization (ISQ) of Hugging Face models
- GGML & GGUF support: 2–8 bit
- GPTQ, AWQ, AFQ, HQQ, FP8, BNB (int8/fp4/nf4)
- Auto-select the fastest quant method
- KV cache quantization

## ISQ

In situ quantization works by quantizing models inplace, with the chief benefit being reduced memory footprint when running the model. This enables larger model to be run on devices which would not fit the full weights, and may increase model inference performance.

Refer to this [link](https://github.com/EricLBuehler/mistral.rs/blob/master/docs/ISQ.md) for more details on ISQ 

Each layer can have an ISQ, this can be set by using a model topology.

For the TextModel builder here is an example 

```bash

.with_topology(
            Topology::empty()
                .with_range(
                    0..8,
                    LayerTopology {
                        isq: Some(IsqType::Q3K),
                        device: None,
                    },
                )
                .with_range(
                    8..16,
                    LayerTopology {
                        isq: Some(IsqType::Q4K),
                        device: None,
                    },
                )
                .with_range(
                    16..24,
                    LayerTopology {
                        isq: Some(IsqType::Q6K),
                        device: None,
                    },
                )
                .with_range(
                    24..32,
                    LayerTopology {
                        isq: Some(IsqType::Q8_0),
                        device: None,
                    },
                ),
        )

```

## Usage

clone the repo

```
cd rust-mitsralvllm-chat

# build (using cuda)

cargo build --release --features cuda,cudnn,flash-attn

#execute

./target/release/rust-mistralvllm-chat --config/config.json
```

## Acknowledment

[mistral.rs](https://github.com/EricLBuehler/mistral.rs)
