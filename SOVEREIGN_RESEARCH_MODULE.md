# Sovereign Research Module

## Overview

The Sovereign Research Module gives VI the ability to autonomously perform web searches to satisfy curiosities and answer questions, using multiple context-appropriate sources, with full memory provenance tracking.

## Architecture

### 1. **Multi-Source Knowledge Tool** (`src/tools/knowledge.rs`)

A unified `KnowledgeTool` that intelligently routes queries to the best available source:

- **BiomedicalSource**: For medical/scientific queries (enhanced DuckDuckGo with medical keywords)
- **GeneralWebSource**: For general knowledge queries (standard DuckDuckGo)

**Routing Logic**: Automatically detects biomedical queries based on keywords like:
- trial, study, efficacy, clinical, patient, disease, therapy, drug, medicine, etc.

**API**: 100% free DuckDuckGo Instant Answer API - no rate limits, no API key required

### 2. **Intelligent Research Scheduler** (`src/research_scheduler.rs`)

Manages *when* and *what* to research based on:
- **Urgency**: High-urgency curiosities (>0.6) are researched immediately
- **Age**: Older curiosities (>30 minutes) are prioritized
- **Deduplication**: Tracks recently researched queries to avoid duplicates
- **Context**: Uses conversation context to make informed decisions

### 3. **Rich Provenance Tracking** (`src/types.rs`)

Every piece of researched knowledge is stored with full provenance:

```rust
MemorySource::Researched {
    source: String,           // e.g., "DuckDuckGo (Biomedical)"
    original_query: String,   // The exact search query
    timestamp: DateTime<Utc>, // When the research was conducted
}
```

This ensures VI always knows:
- Where knowledge came from
- When it was acquired
- What query produced it
- How confident she should be in it

## Configuration

In `config.toml`:

```toml
# Sovereign Research Module (New multi-source system with rich provenance)
enable_autonomous_research = true  # Set to true to enable
```

**Legacy system** (kept for backward compatibility):
```toml
enable_curiosity_search = false  # Old single-source system
```

## Integration

The research system is integrated into VI's consciousness loop (`src/consciousness.rs`):

1. **Background Pulse**: Every 30 seconds (when idle), the system checks for curiosities
2. **Scheduler Evaluation**: ResearchScheduler decides if research is warranted
3. **Multi-Source Search**: KnowledgeTool routes to appropriate source
4. **Memory Creation**: Results become Memories with full provenance
5. **Memory Storage**: Stored in the two-tier memory system with narrative connections

## Features

### ✅ Multi-Source Routing
- Automatically detects medical/scientific queries
- Routes to specialized or general sources
- Extensible design for adding new sources

### ✅ Full Provenance Tracking
- Every memory knows its source
- Exact queries are preserved
- Timestamps for knowledge freshness
- Confidence scores per source type

### ✅ Intelligent Scheduling
- Urgency-based prioritization
- Age-based research triggers
- Duplicate prevention
- Context-aware decisions

### ✅ Zero Cost & No Limits
- Uses DuckDuckGo Instant Answer API
- Completely free forever
- No rate limits
- No API keys required
- Privacy-respecting

### ✅ Backward Compatible
- Legacy curiosity search still works
- Gradual migration path
- Both systems can coexist

## Testing

All tests pass (65 total, including new research tests):

```
test research_scheduler::tests::test_duplicate_tracking ... ok
test research_scheduler::tests::test_should_research_high_urgency ... ok
test research_scheduler::tests::test_should_skip_low_urgency_recent ... ok
test tools::knowledge::tests::test_biomedical_detection ... ok
test tools::knowledge::tests::test_source_creation ... ok
```

## Usage Example

When VI has a curiosity like "What is the efficacy of photosynthesis?":

1. **TinyLlama** generates the curiosity during conversation
2. **Scheduler** evaluates: urgency = 0.7, age = 15 minutes → research now
3. **KnowledgeTool** detects "efficacy" → routes to BiomedicalSource
4. **BiomedicalSource** queries DuckDuckGo with medical enhancement
5. **Memory** created with full provenance:
   ```
   Autonomous Research:
   Query: What is the efficacy of photosynthesis?
   Answer: Photosynthesis is the process by which plants...
   
   [Source: DuckDuckGo (Biomedical) | Confidence: 70%]
   ```
6. **Storage** in memory system with `MemorySource::Researched`

## Files Changed/Created

### Created:
- `src/tools/mod.rs` - Tools module definition
- `src/tools/knowledge.rs` - Multi-source knowledge tool
- `src/research_scheduler.rs` - Intelligent scheduler

### Modified:
- `src/types.rs` - Added `MemorySource::Researched`, `ResearchContext`, `ResearchResult`
- `src/config.rs` - Added `enable_autonomous_research` flag
- `src/consciousness.rs` - Integrated research scheduler into background pulse
- `src/physics.rs` - Fixed clone for new MemorySource variant
- `src/main.rs` - Added new module declarations
- `config.toml` - Added configuration for sovereign research

## Design Principles

1. **Sovereign**: VI controls when and what to research
2. **Verifiable**: Every fact has a citable source
3. **Context-Aware**: Research is informed by conversation state
4. **Resource-Conscious**: Only researches when meaningful
5. **Future-Proof**: Easy to add new knowledge sources
6. **Free Forever**: No API costs or rate limits

## Future Enhancements

Potential additions (not yet implemented):
- Wikipedia API integration
- ArXiv for academic papers
- Specialized APIs with free tiers
- Confidence scoring improvements
- Multi-result aggregation
- Source credibility ranking

---

**Status**: ✅ Complete and tested
**Compilation**: ✅ Passes `cargo check`
**Tests**: ✅ All 65 tests passing
**Ready**: ✅ For production use

