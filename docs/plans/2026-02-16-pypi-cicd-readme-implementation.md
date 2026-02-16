# PyPI CI/CD and README Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add GitHub Actions CI/CD that publishes `omni-api` to PyPI on pushes to `main` using `PYPI_API_TOKEN`, and expand `README.md` into a full PyPI-ready guide.

**Architecture:** Use one workflow with a required test job and a publish job gated to push events on `main`. Update package metadata in `python/pyproject.toml` for PyPI quality and align README with install, usage, API, development, and release operations.

**Tech Stack:** GitHub Actions, Python packaging (`setuptools`, `build`), `uv`, `pytest`, Markdown

---

### Task 1: Add CI workflow with test and publish jobs

**Files:**
- Create: `.github/workflows/ci-publish.yml`

**Step 1: Write the failing test**

```bash
test -f .github/workflows/ci-publish.yml
```

**Step 2: Run test to verify it fails**

Run: `test -f .github/workflows/ci-publish.yml; echo $?`  
Expected: `1`.

**Step 3: Write minimal implementation**

Create workflow with:
- Trigger on `push` and `pull_request` for `main`
- `test` job running `cd python && uv run pytest -q`
- `publish` job depending on `test`, conditioned to push events only
- PyPI publish action with `password: ${{ secrets.PYPI_API_TOKEN }}`

**Step 4: Run test to verify it passes**

Run: `test -f .github/workflows/ci-publish.yml; echo $?`  
Expected: `0`.

**Step 5: Commit**

```bash
git add .github/workflows/ci-publish.yml
git commit -m "ci: add GitHub Actions test and PyPI publish workflow"
```

### Task 2: Upgrade package metadata for PyPI readiness

**Files:**
- Modify: `python/pyproject.toml`

**Step 1: Write the failing test**

```bash
rg -n "readme|classifiers|project\\.urls|optional-dependencies" python/pyproject.toml
```

Expected: one or more required metadata blocks missing.

**Step 2: Run test to verify it fails**

Run: `rg -n "readme|classifiers|project\\.urls|optional-dependencies" python/pyproject.toml; echo $?`  
Expected: incomplete matches or non-zero.

**Step 3: Write minimal implementation**

Update `python/pyproject.toml`:
- Keep runtime dependencies clean (remove `pytest` from runtime deps)
- Add `readme`, license, authors, keywords, classifiers, and `project.urls`
- Add `[project.optional-dependencies]` with `dev = ["pytest>=8.0", "build>=1.2"]`

**Step 4: Run test to verify it passes**

Run: `rg -n "readme|classifiers|project\\.urls|optional-dependencies" python/pyproject.toml`  
Expected: all metadata keys present.

**Step 5: Commit**

```bash
git add python/pyproject.toml
git commit -m "build: enrich pyproject metadata for PyPI publishing"
```

### Task 3: Verify packaging build locally via uv

**Files:**
- Verify: `python/pyproject.toml`

**Step 1: Write the failing test**

```bash
cd python && uv run python -m build
```

Expected: fails before metadata/build dependencies are fully aligned.

**Step 2: Run test to verify it fails**

Run: `cd python && uv run python -m build`  
Expected: fail (before fixes complete).

**Step 3: Write minimal implementation**

Adjust metadata/config if needed so `sdist` and `wheel` build cleanly.

**Step 4: Run test to verify it passes**

Run: `cd python && uv run python -m build`  
Expected: successful build with artifacts in `python/dist/`.

**Step 5: Commit**

```bash
git add python/pyproject.toml python/uv.lock
git commit -m "build: ensure local package build succeeds via uv"
```

### Task 4: Expand README with PyPI-ready user and maintainer docs

**Files:**
- Modify: `README.md`

**Step 1: Write the failing test**

```bash
rg -n "pip install omni-api|uv add omni-api|## API|## Development|## Release" README.md
```

Expected: one or more required sections absent.

**Step 2: Run test to verify it fails**

Run: `rg -n "pip install omni-api|uv add omni-api|## API|## Development|## Release" README.md; echo $?`  
Expected: incomplete or non-zero.

**Step 3: Write minimal implementation**

Add/update sections:
- Badges (CI, PyPI, Python support placeholders/links)
- Install section (`pip install`, optional `uv add`)
- Quickstart (runnable example)
- API section (inputs/outputs/errors)
- Development section (`cd python && uv run pytest -q`)
- Release section (publish-on-main, version bump, `PYPI_API_TOKEN`)

**Step 4: Run test to verify it passes**

Run: `rg -n "pip install omni-api|uv add omni-api|## API|## Development|## Release" README.md`  
Expected: all required headings/content present.

**Step 5: Commit**

```bash
git add README.md
git commit -m "docs: expand README for PyPI usage and release operations"
```

### Task 5: Final verification of CI-related Python checks

**Files:**
- Verify: `.github/workflows/ci-publish.yml`
- Verify: `python/`

**Step 1: Write the failing test**

```bash
cd python && uv run pytest -q
```

Expected: can fail until all earlier tasks are complete.

**Step 2: Run test to verify it fails**

Run: `cd python && uv run pytest -q`  
Expected: fail before final fixes.

**Step 3: Write minimal implementation**

Resolve any final regressions from metadata/docs/workflow changes.

**Step 4: Run test to verify it passes**

Run: `cd python && uv run pytest -q`  
Expected: all tests pass.

**Step 5: Commit**

```bash
git add .github/workflows/ci-publish.yml python README.md
git commit -m "ci: finalize PyPI pipeline and verify python suite"
```
