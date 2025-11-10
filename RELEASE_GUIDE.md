# 🚀 Release Guide for PROJECT VI

## Quick Start: Publishing Your First Release

This guide walks you through publishing PROJECT VI to GitHub Packages.

## Prerequisites

✅ All changes committed and pushed to GitHub  
✅ GitHub repository at `ryanj97g/Project_VI`  
✅ GitHub Actions enabled in repository settings  

## Step-by-Step Release Process

### 1. Prepare the Release

Update version number in `Cargo.toml`:

```toml
[package]
name = "project-vi"
version = "0.1.0"  # ← Change this (e.g., to "0.2.0")
```

Update `CHANGELOG.md` with release notes:

```markdown
## [0.2.0] - 2025-11-10

### Added
- Feature X
- Feature Y

### Fixed
- Bug Z

### Changed
- Improvement W
```

### 2. Commit and Push Changes

```bash
# Stage the changes
git add Cargo.toml CHANGELOG.md

# Commit with descriptive message
git commit -m "Bump version to 0.2.0"

# Push to main branch
git push origin main
```

### 3. Create Git Tag

```bash
# Create annotated tag (replace version)
git tag -a v0.2.0 -m "Release v0.2.0: Brief description of changes"

# Push the tag to GitHub
git push origin v0.2.0
```

### 4. Create GitHub Release

**Option A: Via GitHub Web UI (Recommended)**

1. Go to your repository: `https://github.com/ryanj97g/Project_VI`
2. Click **Releases** (right sidebar)
3. Click **Draft a new release**
4. Fill in the form:
   - **Choose a tag:** Select `v0.2.0` (the tag you just pushed)
   - **Release title:** `PROJECT VI v0.2.0`
   - **Description:** Add release notes (can copy from CHANGELOG.md)
5. Click **Publish release**

**Option B: Via GitHub CLI**

```bash
# Install GitHub CLI if needed: https://cli.github.com/

# Create release with notes from CHANGELOG
gh release create v0.2.0 \
  --title "PROJECT VI v0.2.0" \
  --notes "$(sed -n '/## \[0.2.0\]/,/## \[/p' CHANGELOG.md | head -n -1)"
```

### 5. Watch the Magic Happen! ✨

Once you publish the release, GitHub Actions will automatically:

1. **Run Tests** (~5-10 minutes)
   - Tests on Linux, Windows, and macOS
   - Runs formatting checks
   - Runs clippy lints
   - Runs 62 test cases

2. **Build Binaries** (~10-15 minutes)
   - Builds release binary for each platform
   - Creates `.tar.gz` archives
   - Attaches to release:
     - `project-vi-v0.2.0-Linux.tar.gz`
     - `project-vi-v0.2.0-Windows.tar.gz`
     - `project-vi-v0.2.0-macOS.tar.gz`

3. **Build Docker Image** (~10-15 minutes)
   - Builds multi-stage Docker image
   - Pushes to GitHub Container Registry
   - Tagged as:
     - `ghcr.io/ryanj97g/project_vi:latest`
     - `ghcr.io/ryanj97g/project_vi:0.2.0`
     - `ghcr.io/ryanj97g/project_vi:0.2`

### 6. Monitor Progress

**View GitHub Actions:**
1. Go to repository → **Actions** tab
2. Click on the running workflow
3. Watch the progress in real-time

**Check Container Registry:**
1. Go to repository → **Packages** (right sidebar)
2. You'll see `project_vi` package
3. Click to view tags and details

## What Gets Published

### GitHub Releases
- **Source code** (automatic .zip and .tar.gz)
- **Binary packages** for Linux/Windows/macOS
- **Release notes** from your description
- **Changelog** excerpt

### GitHub Container Registry (ghcr.io)
- **Docker image** with all dependencies
- **Multiple tags** (latest, version, major.minor)
- **Metadata** (labels, description, links)

## Installation for Users

After publishing, users can install via:

**Binary Release:**
```bash
curl -LO https://github.com/ryanj97g/Project_VI/releases/latest/download/project-vi-v0.2.0-Linux.tar.gz
tar -xzf project-vi-v0.2.0-Linux.tar.gz
./vi3
```

**Docker:**
```bash
docker pull ghcr.io/ryanj97g/project_vi:latest
docker run -it ghcr.io/ryanj97g/project_vi:latest
```

**From Source:**
```bash
git clone https://github.com/ryanj97g/Project_VI.git
cd Project_VI
cargo build --release
```

## Versioning Guidelines

PROJECT VI uses [Semantic Versioning](https://semver.org/):

**Format:** `MAJOR.MINOR.PATCH` (e.g., `0.2.1`)

- **MAJOR** (0 → 1): Breaking changes, major architecture overhauls
- **MINOR** (0.1 → 0.2): New features, non-breaking changes
- **PATCH** (0.1.0 → 0.1.1): Bug fixes, minor improvements

**Examples:**
- `0.1.0 → 0.2.0`: Added V4.5.0 Kaelic Tensor Field metrics
- `0.2.0 → 0.2.1`: Fixed memory consolidation bug
- `0.9.9 → 1.0.0`: Stable release, production-ready

## Pre-release Versions

For experimental features:

```bash
# Create pre-release tag
git tag -a v0.2.0-beta.1 -m "Release v0.2.0 Beta 1"
git push origin v0.2.0-beta.1

# Mark as pre-release in GitHub UI
# Check "This is a pre-release" checkbox
```

## Troubleshooting

### GitHub Actions Fails

**Test failures:**
```bash
# Run tests locally first
cargo test --release

# Fix any failing tests before release
```

**Build failures:**
```bash
# Test release build locally
cargo build --release

# Check for platform-specific issues
```

### Docker Build Fails

```bash
# Test Docker build locally
docker build -t project-vi:test .

# Run to verify
docker run -it project-vi:test
```

### Binary Not Found in Release

Check GitHub Actions logs:
1. Go to Actions tab
2. Click failed workflow
3. Expand "Package binary" step
4. Look for error messages

Common issue: Binary name mismatch
- Cargo.toml builds `vi3` (current name)
- Workflow expects this name
- Should work as-is

## GitHub Container Registry Permissions

If Docker push fails with permission error:

1. **Settings** → **Actions** → **General**
2. Scroll to "Workflow permissions"
3. Select **"Read and write permissions"**
4. Check **"Allow GitHub Actions to create and approve pull requests"**
5. Save

## Cleanup Old Releases

Delete old releases to save space:

```bash
# Via GitHub CLI
gh release delete v0.1.0 --yes

# Or via web UI
# Releases → Select release → Delete
```

## Best Practices

✅ **Test locally before releasing**
```bash
cargo test --release
cargo build --release
./target/release/vi3
```

✅ **Write clear release notes**
- What's new
- What's fixed
- Breaking changes
- Migration guide (if needed)

✅ **Use pre-releases for experimental features**
- Mark as pre-release in GitHub
- Get feedback before stable release

✅ **Keep CHANGELOG.md updated**
- Update with each release
- Follow [Keep a Changelog](https://keepachangelog.com/) format

✅ **Tag consistently**
- Always use `v` prefix: `v0.2.0`
- Use semantic versioning
- Create annotated tags (not lightweight)

## Next Steps

After your first release:

1. **Announce it!**
   - Create GitHub Discussion
   - Update README badges
   - Share with community

2. **Monitor feedback**
   - Watch GitHub Issues
   - Check Docker pull statistics
   - Review user questions

3. **Plan next release**
   - Update project roadmap
   - Track feature requests
   - Fix reported bugs

## Example: Complete Release Workflow

```bash
# 1. Update version
vim Cargo.toml  # Change version to 0.2.0

# 2. Update changelog
vim CHANGELOG.md  # Add release notes

# 3. Test everything
cargo test --release
cargo build --release
./target/release/vi3

# 4. Commit and tag
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to 0.2.0"
git push origin main

git tag -a v0.2.0 -m "Release v0.2.0: Kaelic Tensor Field metrics and SQLite memory"
git push origin v0.2.0

# 5. Create GitHub Release
gh release create v0.2.0 \
  --title "PROJECT VI v0.2.0" \
  --notes "$(cat CHANGELOG.md | sed -n '/## \[0.2.0\]/,/## \[/p' | head -n -1)"

# 6. Watch it build!
# Go to: https://github.com/ryanj97g/Project_VI/actions
```

## Resources

- **GitHub Actions Docs:** https://docs.github.com/actions
- **GitHub Container Registry:** https://docs.github.com/packages
- **Semantic Versioning:** https://semver.org/
- **Keep a Changelog:** https://keepachangelog.com/

---

**Ready to publish your first release?** 🚀

Follow steps 1-4 above, and GitHub Actions will handle the rest!

