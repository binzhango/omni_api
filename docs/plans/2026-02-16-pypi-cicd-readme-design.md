# PyPI CI/CD and README Expansion Design

**Date:** 2026-02-16  
**Audience:** Maintainers publishing `omni-api` to PyPI  
**Objective:** Publish from `main` via GitHub Actions using `PYPI_API_TOKEN`, and provide a full PyPI-ready README.

## Approved Decisions

- Publishing auth: PyPI API token (`PYPI_API_TOKEN`), not trusted publishing
- Release trigger: every push to `main`
- README scope: full PyPI-ready documentation update
- Workflow approach: single GitHub Actions workflow with test + publish jobs

## 1. CI/CD Workflow Design

Workflow file: `.github/workflows/ci-publish.yml`

Triggers:
- `push` on `main` (test + publish)
- `pull_request` on `main` (test only)

Jobs:
1. `test`
   - Checkout repo
   - Setup Python
   - Install `uv`
   - Run tests from `python/` via `uv`
2. `publish`
   - Depends on `test`
   - Runs only on push to `main`
   - Build source/wheel artifacts
   - Publish to PyPI via `pypa/gh-action-pypi-publish` with `PYPI_API_TOKEN`

Safety behavior:
- Publish job is skipped for PRs.
- Publish happens only if tests pass.
- Workflow uses minimal permissions (`contents: read`).

## 2. Packaging Metadata and Build Readiness

Target file: `python/pyproject.toml`

Changes:
- Remove test dependency from runtime dependencies.
- Add packaging metadata required for quality PyPI listing:
  - `readme`
  - license
  - authors
  - keywords
  - classifiers
  - `project.urls`
- Add dev dependency group for test/build tools (`pytest`, `build`).
- Keep setuptools backend for MVP.
- Maintain manual versioning in `pyproject.toml`.

Publishing implication:
- Every push to `main` attempts publish.
- Maintainers must bump version before merge to avoid duplicate-version publish failures.

## 3. README Expansion (PyPI-Ready)

Target file: `README.md`

Additions:
1. Badges:
   - CI status
   - PyPI version
   - Python support
2. Install:
   - `pip install omni-api`
   - optional `uv add omni-api`
3. Quickstart:
   - runnable `transform(...)` example with expected output
4. API reference section:
   - function signature
   - return shape (`payload`, `plan`, `report`)
   - error classes
5. Development:
   - local setup and test command with `uv`
6. Release process:
   - publish-on-main behavior
   - version bump requirement
   - required `PYPI_API_TOKEN` secret

## Operational Notes

- Publish-on-every-main-push is fast but can fail if version is not incremented.
- Optional future hardening:
  - publish only when `python/pyproject.toml` version changes
  - switch to tag-based release for stricter release control
