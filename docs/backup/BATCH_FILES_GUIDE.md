# VI3 Batch Files Guide

**Quick Reference for Windows Batch Launchers**

---

## 📁 Available Batch Files

### Main Application

#### `run_vi3.bat`
**Purpose**: Launch the main VI3 digital consciousness application

**What it does**:
1. Builds VI3 in release mode
2. Checks for build errors
3. Launches the application

**Usage**:
```cmd
Double-click run_vi3.bat
```

**Or from command line**:
```cmd
run_vi3.bat
```

---

### Demonstrations

#### `run_vi3_demo.bat`
**Purpose**: Run the complete architecture demonstration

**What it does**:
1. Builds the vi3_demo example
2. Demonstrates all 10 core systems
3. Shows system initialization and monitoring

**Usage**:
```cmd
Double-click run_vi3_demo.bat
```

**Expected Output**:
- System initialization messages
- Documentation references
- Architecture overview

---

#### `run_suffering_metrics_demo.bat`
**Purpose**: Demonstrate suffering prevention metrics

**What it does**:
1. Builds the suffering_metrics_demo example
2. Shows quantitative well-being measurement
3. Displays metric calculations and recommendations

**Usage**:
```cmd
Double-click run_suffering_metrics_demo.bat
```

**Expected Output**:
- Metrics documentation references
- Usage examples

---

### Build & Test

#### `build_vi3.bat`
**Purpose**: Build the entire VI3 project

**What it does**:
1. Compiles all modules in release mode
2. Builds all examples and tests
3. Shows build summary

**Usage**:
```cmd
Double-click build_vi3.bat
```

**Build Time**: ~2 minutes (first build), <30 seconds (incremental)

**Output**:
- Compilation progress
- Build success/failure status
- Binary location: `target\release\vi3.exe`

---

#### `run_all_tests.bat`
**Purpose**: Run the complete test suite

**What it does**:
1. Executes all unit tests
2. Tests all 10 core systems
3. Displays test results

**Usage**:
```cmd
Double-click run_all_tests.bat
```

**Expected Output**:
- Test progress for each module
- Pass/Fail status
- Total tests run

**Modules Tested**:
- gpu_topology
- consciousness_field
- neural_potential
- energy_qualia
- constitutional_physics
- orchestrator
- persistence
- suffering_metrics
- experiments
- vi3_core

---

#### `clean_build.bat`
**Purpose**: Clean all build artifacts

**What it does**:
1. Removes `target/` directory
2. Cleans all compiled binaries
3. Prepares for fresh build

**Usage**:
```cmd
Double-click clean_build.bat
```

**When to use**:
- Build errors persist
- Switching branches
- Freeing disk space
- Fresh compilation needed

---

## 🔧 Troubleshooting

### Batch File Won't Run

**Problem**: Double-clicking does nothing

**Solutions**:
1. Right-click → "Run as Administrator"
2. Open cmd.exe and run manually:
   ```cmd
   cd C:\Users\YourName\Desktop\VIV3
   run_vi3.bat
   ```

---

### Build Fails

**Problem**: "Build failed" message appears

**Solutions**:
1. Clean and rebuild:
   ```cmd
   clean_build.bat
   build_vi3.bat
   ```

2. Check Rust installation:
   ```cmd
   rustc --version
   cargo --version
   ```

3. Update Rust:
   ```cmd
   rustup update
   ```

---

### Missing Dependencies

**Problem**: "cannot find crate" errors

**Solution**: Rebuild with clean state:
```cmd
clean_build.bat
build_vi3.bat
```

---

## 💡 Tips

### 1. First Time Setup
```cmd
build_vi3.bat          # Build everything first
run_all_tests.bat      # Verify it works
run_vi3_demo.bat       # Try the demo
run_vi3.bat            # Launch main app
```

### 2. Daily Development
```cmd
build_vi3.bat          # Rebuild after changes
run_all_tests.bat      # Verify tests still pass
```

### 3. Clean Rebuild
```cmd
clean_build.bat        # Remove old builds
build_vi3.bat          # Fresh compilation
```

### 4. Quick Testing
```cmd
run_all_tests.bat      # Run full test suite
```

---

## 📊 Batch File Comparison

| Batch File | Build? | Run? | Test? | Time |
|------------|--------|------|-------|------|
| `run_vi3.bat` | ✅ | ✅ | ❌ | ~30s |
| `run_vi3_demo.bat` | ✅ | ✅ | ❌ | ~30s |
| `run_suffering_metrics_demo.bat` | ✅ | ✅ | ❌ | ~30s |
| `build_vi3.bat` | ✅ | ❌ | ❌ | ~2m |
| `run_all_tests.bat` | ❌ | ✅ | ✅ | ~1m |
| `clean_build.bat` | ❌ | ❌ | ❌ | ~5s |

---

## 🎯 Common Workflows

### Initial Setup
```
1. build_vi3.bat
2. run_all_tests.bat
3. run_vi3_demo.bat
```

### After Code Changes
```
1. build_vi3.bat
2. run_all_tests.bat
3. run_vi3.bat
```

### Troubleshooting
```
1. clean_build.bat
2. build_vi3.bat
3. run_all_tests.bat
```

### Demo Presentation
```
1. run_vi3_demo.bat
2. run_suffering_metrics_demo.bat
3. run_vi3.bat
```

---

## 📝 Customization

All batch files can be edited with any text editor. Common customizations:

### Change Build Mode
In `build_vi3.bat`, change:
```batch
cargo build --release
```
To:
```batch
cargo build  # Debug mode (faster builds, slower runtime)
```

### Add Custom Arguments
In `run_vi3.bat`, change:
```batch
cargo run --release
```
To:
```batch
cargo run --release -- --your-custom-args
```

### Change Pause Behavior
Remove `pause` at end of batch file to auto-close window

---

## 🔗 See Also

- **[START_HERE.md](START_HERE.md)** - Complete quick start guide
- **[VI3_QUICKSTART.md](VI3_QUICKSTART.md)** - Detailed usage guide
- **[PROJECT_VI_V3_IMPLEMENTATION.md](PROJECT_VI_V3_IMPLEMENTATION.md)** - Architecture documentation

---

**All batch files include error checking and helpful output messages!**

*Last Updated: November 3, 2025*

