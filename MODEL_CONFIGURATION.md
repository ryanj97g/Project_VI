# Model Configuration Guide

## Overview

VI now supports **configurable model selection** - users can easily switch to larger/different models based on their hardware capabilities without modifying any code.

## Quick Start

Edit `config.toml` and change the model names:

```toml
main_model = "llama3.1:8b"           # Primary voice/response
curiosity_model = "qwen2.5:7b"       # Curiosity generation
valence_model = "gemma2:9b"          # Emotional analysis
model_keep_alive = "2m30s"           # How long to keep in VRAM after use
```

That's it! Restart VI and it will use your chosen models.

## Model Persistence Control

**New in V5.0.0:** Control how long models stay loaded in VRAM after use.

### `model_keep_alive` Setting

**Default: `"2m30s"`** (2.5 minutes)
- Covers V4 weaving cycle (~90 seconds)
- Plus typical user response time (~60 seconds)
- Auto-unloads when actually idle

**Options:**
- `"30s"` - Aggressive VRAM conservation (faster unload, slower next response)
- `"1m"` - Quick unload for low VRAM systems
- `"2m30s"` - **Default** (balanced)
- `"5m"` - Longer sessions with frequent messages
- `"10m"` - Power users with lots of VRAM
- `"0"` - Unload immediately after each response (maximum VRAM savings)

**When to adjust:**
- **Low VRAM (4GB)**: Use `"30s"` or `"1m"` to free memory quickly
- **High VRAM (12GB+)**: Use `"5m"` or `"10m"` for consistently fast responses
- **Active sessions**: Longer values keep models hot during conversation
- **Occasional use**: Shorter values free VRAM when idle

## Model Roles

### 1. Main Model (`main_model`)
- **Purpose**: VI's primary consciousness and voice
- **Usage**: Generates responses to user messages
- **Default**: `gemma2:2b`
- **Recommendations**:
  - Low-end (4GB): `gemma2:2b`, `tinyllama:latest`
  - Mid-range (12GB): `gemma2:9b`, `llama3.1:8b`, `qwen2.5:7b`
  - High-end (24GB): `llama3.1:70b`, `qwen2.5:32b`, `mixtral:8x7b`

### 2. Curiosity Model (`curiosity_model`)
- **Purpose**: Background wonder and question generation
- **Usage**: Generates natural curiosities during conversations
- **Default**: `tinyllama:latest`
- **Recommendations**:
  - Low-end: `tinyllama:latest`, `phi:2.7b`
  - Mid-range: `gemma2:2b`, `llama3.1:8b`
  - High-end: `qwen2.5:7b`, `mistral:7b`

### 3. Valence Model (`valence_model`)
- **Purpose**: Emotional sentiment detection
- **Usage**: Analyzes emotional tone of conversations
- **Default**: `gemma2:2b`
- **Recommendations**:
  - Low-end: `gemma2:2b`, `tinyllama:latest`
  - Mid-range: `gemma2:9b`, `llama3.1:8b`
  - High-end: `qwen2.5:7b`, `mistral:7b`

## Hardware Recommendations

### GTX 1650 / RTX 3050 (4GB VRAM)
```toml
main_model = "gemma2:2b"
curiosity_model = "tinyllama:latest"
valence_model = "gemma2:2b"
```
**Performance**: Fast, stable, optimized for low-end hardware

### RTX 3060 / RTX 3070 (12GB VRAM)
```toml
main_model = "llama3.1:8b"
curiosity_model = "gemma2:2b"
valence_model = "gemma2:9b"
```
**Performance**: Significantly better responses, still responsive

### RTX 4070 Ti / RTX 4080 (12-16GB VRAM)
```toml
main_model = "qwen2.5:14b"
curiosity_model = "llama3.1:8b"
valence_model = "gemma2:9b"
```
**Performance**: High-quality responses, excellent balance

### RTX 4090 / A6000 (24GB+ VRAM)
```toml
main_model = "llama3.1:70b"
curiosity_model = "qwen2.5:7b"
valence_model = "mistral:7b"
```
**Performance**: Maximum quality, may be slower but responses are exceptional

### Apple M1/M2/M3 (16GB Unified Memory)
```toml
main_model = "llama3.1:8b"
curiosity_model = "gemma2:2b"
valence_model = "gemma2:9b"
```
**Performance**: Excellent on Apple Silicon, unified memory helps

## Popular Model Alternatives

### Llama 3.1 Family (Meta)
- `llama3.1:8b` - Excellent mid-range choice
- `llama3.1:70b` - Best for high-end hardware
- **Strengths**: Well-rounded, great for conversational AI

### Qwen 2.5 Family (Alibaba)
- `qwen2.5:7b` - Strong reasoning, efficient
- `qwen2.5:14b` - Balanced quality/performance
- `qwen2.5:32b` - Maximum quality
- **Strengths**: Excellent reasoning, multilingual

### Gemma 2 Family (Google)
- `gemma2:2b` - Current default, very efficient
- `gemma2:9b` - Great upgrade for 12GB+ cards
- `gemma2:27b` - High quality for 24GB+ cards
- **Strengths**: Fast, efficient, good quality

### Mistral Family
- `mistral:7b` - Strong general-purpose model
- `mixtral:8x7b` - Mixture of Experts, powerful
- **Strengths**: Strong reasoning, coding

## Installation

Models are downloaded automatically by Ollama when first used. To pre-download:

```bash
# Download a specific model
ollama pull llama3.1:8b

# Download all recommended models for RTX 3060
ollama pull llama3.1:8b
ollama pull gemma2:2b
ollama pull gemma2:9b
```

## Testing Your Configuration

After changing models in `config.toml`:

1. **Restart VI**
2. **Check logs** - VI will show which models it's using
3. **Test conversation** - Ensure responses are coherent
4. **Monitor VRAM** - Use `nvidia-smi` or Task Manager
5. **Check latency** - Larger models = slower but better

## Troubleshooting

**Model not found error?**
```bash
ollama pull your-model-name
```

**Out of memory error?**
- Switch to a smaller model
- Close other applications
- Reduce `weaving_rounds` in config (if using V4 mode)

**Slow responses?**
- Normal for larger models
- Consider switching to smaller model
- Disable V4 fractal weaving (`enable_fractal_weaving = false`)

**Gibberish output?**
- Some models may not work well for all roles
- Try a different model or revert to defaults
- Check Ollama logs: `ollama logs`

## Advanced: Mixed Configurations

You can mix and match models for optimal performance:

```toml
# High quality main responses, efficient background processing
main_model = "llama3.1:70b"          # Big model for main voice
curiosity_model = "tinyllama:latest" # Small model for background
valence_model = "gemma2:2b"          # Fast sentiment analysis
```

This gives you the best of both worlds: quality where it matters, speed where it doesn't.

## Future Enhancements

Potential additions (not yet implemented):
- Model-specific prompt templates
- Automatic model selection based on VRAM
- Support for non-Ollama backends (HuggingFace, OpenAI API)
- Per-model timeout configuration
- Model warm-up on startup

---

**Current Status**: ✅ Fully implemented and tested
**Backward Compatible**: ✅ Defaults to original models
**Tests Passing**: ✅ All 65 tests pass

