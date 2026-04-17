---
label: "docs"
created_at: "2024-04-17"
author_role: "annotator"
confidence: "high"
---

## Problem

Non-obvious design decisions lack background justification in inline comments. For example, `tag::resolve_tags` silently assumes that if a tag isn't a group, it must be a valid single tag without performing any validation against the actual tag catalog, deferring that constraint to callers.

## Goal

Provide background rationale for non-obvious fallback behaviors and domain constraints at the relevant code sites.

## Context

Design rationale absent from a comment block will be rediscovered at each maintenance event, producing redundant investigation cost.

## Evidence

- path: "src/domain/tag.rs"
  loc: "9"
  note: |
    Current inline comment at site: None

    Inline comment to add at site:
    // If not found in groups, assume it's a valid single tag.
    // Validation against the catalog is deferred to the orchestration layer to keep this function pure and free of I/O.

## Change Scope

- `src/domain/tag.rs`
