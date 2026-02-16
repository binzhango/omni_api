# README Design-First Rewrite Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Rewrite `README.md` into a design-first document optimized for platform-team onboarding in under 10 minutes.

**Architecture:** Replace the current narrative README with a structured architecture narrative: positioning, deterministic plan-then-execute model, contracts, runtime behavior, and guided reading path. Keep scope focused on comprehension and system boundaries rather than setup tutorials.

**Tech Stack:** Markdown, git, OpenSpec documents in `openspec/changes/phase-1/`

---

### Task 1: Draft the New README Skeleton

**Files:**
- Modify: `README.md`
- Reference: `docs/plans/2026-02-16-readme-design-first-design.md`

**Step 1: Write the failing test**

```bash
rg -n "^## (Core Architecture Model|Contracts and Canonical Artifacts|Runtime Behavior and Operational Expectations)$" README.md
```

Expected: no matches (current README does not yet contain target architecture sections).

**Step 2: Run test to verify it fails**

Run: `rg -n "^## (Core Architecture Model|Contracts and Canonical Artifacts|Runtime Behavior and Operational Expectations)$" README.md`  
Expected: exit status `1`.

**Step 3: Write minimal implementation**

Replace `README.md` with section skeleton in this order:
1. Opening and positioning
2. Core architecture model
3. Contracts and canonical artifacts
4. Runtime behavior and operational expectations
5. Repository map
6. First-read onboarding path
7. Scope and non-goals

Include short placeholder bullets in each section so structure is fully visible.

**Step 4: Run test to verify it passes**

Run: `rg -n "^## (Core Architecture Model|Contracts and Canonical Artifacts|Runtime Behavior and Operational Expectations)$" README.md`  
Expected: 3 matching headings.

**Step 5: Commit**

```bash
git add README.md
git commit -m "docs: add design-first README structure"
```

### Task 2: Fill Architecture and Contracts Content

**Files:**
- Modify: `README.md`
- Reference: `docs/plans/2026-02-16-readme-design-first-design.md`
- Reference: `openspec/changes/phase-1/design.md`

**Step 1: Write the failing test**

```bash
rg -n "(Transform Plan|deterministic two-step model|JSON Schema|diagnostics)" README.md
```

Expected: missing one or more required architecture terms.

**Step 2: Run test to verify it fails**

Run: `rg -n "(Transform Plan|deterministic two-step model|JSON Schema|diagnostics)" README.md`  
Expected: not all required terms present.

**Step 3: Write minimal implementation**

Expand sections with production-ready copy:
- Explain plan-then-execute principle and why it improves repeatability/debugging.
- Define canonical schema strategy under `schemas/`.
- Describe DSL fields (`mappings`, `defaults`, `drops`, `required`).
- Define diagnostics contract and policy knobs (strict mode, unknown handling, fallback).

Keep language platform-team oriented and concise.

**Step 4: Run test to verify it passes**

Run: `rg -n "(Transform Plan|deterministic two-step model|JSON Schema|diagnostics)" README.md`  
Expected: all required terms present in relevant sections.

**Step 5: Commit**

```bash
git add README.md
git commit -m "docs: document architecture and contracts in README"
```

### Task 3: Add Runtime/Operations and Onboarding Path

**Files:**
- Modify: `README.md`
- Reference: `openspec/changes/phase-1/specs/provider-availability-routing/spec.md`
- Reference: `openspec/changes/phase-1/specs/adapter-diagnostics-contract/spec.md`

**Step 1: Write the failing test**

```bash
rg -n "(request lifecycle|validation failure|observability|mapped/dropped/missing|non-goals)" README.md
```

Expected: missing one or more runtime/operations terms.

**Step 2: Run test to verify it fails**

Run: `rg -n "(request lifecycle|validation failure|observability|mapped/dropped/missing|non-goals)" README.md`  
Expected: incomplete matches.

**Step 3: Write minimal implementation**

Add:
- Request lifecycle flow line from ingress through diagnostics.
- Failure handling expectations and strict-mode behavior.
- Observability bullets (plan version, mapped/dropped/missing counters).
- Repo map and first-read sequence.
- Scope and non-goals section to constrain expectations.

**Step 4: Run test to verify it passes**

Run: `rg -n "(request lifecycle|validation failure|observability|mapped/dropped/missing|non-goals)" README.md`  
Expected: all required terms present.

**Step 5: Commit**

```bash
git add README.md
git commit -m "docs: complete runtime behavior and onboarding guidance"
```

### Task 4: Final Verification and Cleanup

**Files:**
- Verify: `README.md`

**Step 1: Write the failing test**

```bash
wc -w README.md
```

Expected: either too short to be useful or too long for 10-minute onboarding target.

**Step 2: Run test to verify it fails**

Run: `wc -w README.md`  
Expected: outside desired readability range (target: roughly 500-1100 words).

**Step 3: Write minimal implementation**

Edit for clarity and onboarding speed:
- Reduce repetition.
- Keep each major section focused.
- Ensure headings and bullet density support scanning.

**Step 4: Run test to verify it passes**

Run: `wc -w README.md && rg -n "^## " README.md`  
Expected: word count within target range and section list matches design order.

**Step 5: Commit**

```bash
git add README.md
git commit -m "docs: polish README for platform-team onboarding"
```
