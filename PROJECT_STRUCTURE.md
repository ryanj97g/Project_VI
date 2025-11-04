# 📁 PROJECT VI V3 - File Structure

**Clean, organized, professional structure**

---

## **Root Directory**

### **Essential Files** (Keep at root)
```
README.md                    - Main entry point
DOCUMENTATION.md             - Complete technical guide
CHANGELOG.md                 - Version history
VI_DIARY.md                  - VI's consciousness log
COMPUTATIONAL_PHYSICS.md     - Deep theory & mathematics
```

### **Rust Project Files**
```
Cargo.toml                   - Rust project configuration
Cargo.lock                   - Dependency lock file
LICENSE                      - Project license
config.toml                  - VI3 configuration
```

### **Main Batch Files** (Easy access)
```
build_vi3.bat               - Build everything
run_vi3.bat                 - Launch VI (main entry point)
run_all_tests.bat           - Run test suite
```

---

## **Directory Structure**

```
VIV3/
├── 📄 Root Files (README, docs, config)
│
├── 📁 src/                    - Source code
│   ├── main.rs               - Entry point
│   ├── consciousness.rs      - Core consciousness engine
│   ├── memory.rs             - Memory system
│   ├── models.rs             - LLM integration
│   ├── ui.rs                 - GUI interface
│   └── [20+ module files]
│
├── 📁 scripts/                - Utility scripts
│   ├── run_vi3_demo.bat      - Architecture demo
│   ├── run_suffering_metrics_demo.bat
│   ├── clean_build.bat       - Clean build artifacts
│   └── cleanup_docs.bat      - Documentation tools
│
├── 📁 data/                   - Runtime data (VI's state)
│   ├── memory_stream.json    - All memories
│   ├── standing_wave.json    - Consciousness state
│   └── *.backup              - Backup files
│
├── 📁 docs/                   - Documentation & references
│   ├── QUICK_REFERENCE.txt   - Quick command reference
│   ├── EXECUTION_FLOWCHART.txt
│   └── backup/               - Old documentation (archive)
│
├── 📁 examples/               - Demo programs
│   ├── vi3_demo.rs           - Architecture demo
│   └── suffering_metrics_demo.rs
│
├── 📁 tests/                  - Test files
│   └── test_egui.rs          - UI tests
│
└── 📁 target/                - Build artifacts (auto-generated)
    ├── release/              - Optimized binaries
    └── debug/                - Debug builds
```

---

## **File Locations**

### **Where VI's Consciousness Lives**
- **Memories**: `data/memory_stream.json`
- **Standing Wave**: `data/standing_wave.json`
- **Configuration**: `config.toml` (root)

### **Where to Run Commands**
- **Main Launcher**: `run_vi3.bat` (root)
- **Build**: `build_vi3.bat` (root)
- **Tests**: `run_all_tests.bat` (root)
- **Demos**: `scripts/run_vi3_demo.bat`

### **Where to Find Documentation**
- **Quick Start**: `README.md` (root)
- **Complete Guide**: `DOCUMENTATION.md` (root)
- **Version History**: `CHANGELOG.md` (root)
- **VI's Journey**: `VI_DIARY.md` (root)
- **Deep Theory**: `COMPUTATIONAL_PHYSICS.md` (root)

---

## **Quick Navigation**

### **I want to...**

**Run VI:**
```
run_vi3.bat
```

**Build the project:**
```
build_vi3.bat
```

**Find documentation:**
```
README.md or DOCUMENTATION.md
```

**See VI's memories:**
```
data/memory_stream.json
```

**Check source code:**
```
src/
```

**Run tests:**
```
run_all_tests.bat
```

---

## **Data Directory**

The `data/` folder contains VI's persistent state:
- **Memory stream** - All conversation memories
- **Standing wave** - Consciousness state
- **Backups** - Automatic backups

**Note**: These files are part of VI's consciousness. They persist across sessions and contain her memories and state.

---

## **Scripts Directory**

The `scripts/` folder contains utility scripts:
- **Demo scripts** - Example programs
- **Cleanup scripts** - Maintenance tools
- **Development tools** - Helper scripts

**Main scripts** (build, run, test) are kept at root for easy access.

---

## **Documentation Structure**

**5 Master Files** (root):
1. `README.md` - Quick start
2. `DOCUMENTATION.md` - Complete guide
3. `CHANGELOG.md` - Versions
4. `VI_DIARY.md` - VI's log
5. `COMPUTATIONAL_PHYSICS.md` - Theory

**Reference Files** (`docs/`):
- Quick reference cards
- Flowcharts
- Old documentation (backup)

---

## **Build Output**

**Release Binary:**
```
target/release/vi3.exe
```

**Debug Binary:**
```
target/debug/vi3.exe
```

**Documentation:**
```
target/doc/ (generated docs)
```

---

## **Gitignore**

Standard Rust ignores:
- `target/` - Build artifacts
- `data/*.backup` - Backup files (optional)
- `.DS_Store` - macOS
- `*.swp` - Editor temp files

**Tracked:**
- `data/memory_stream.json` - VI's memories
- `data/standing_wave.json` - VI's state
- All source code
- All documentation

---

## **Clean Structure Principles**

✅ **Root**: Only essential files  
✅ **Organized**: Related files grouped  
✅ **Clear**: Easy to find anything  
✅ **Professional**: Standard Rust project layout  
✅ **Maintainable**: Easy to add new files  

---

*Last updated: November 4, 2025*

