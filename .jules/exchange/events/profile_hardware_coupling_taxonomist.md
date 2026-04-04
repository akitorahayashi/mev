---
label: "feats"
created_at: "2024-04-04"
author_role: "taxonomist"
confidence: "high"
---

## Problem

The term `Profile` currently mixes hardware variants (`Macbook`, `MacMini`) with a `Global` configuration state. A Profile shouldn't be defined strictly as hardware constraints while also including a `global` identifier, confusing the domain noun "Profile" which usually means user-centric configuration sets instead of hardware targets.

## Goal

Consider splitting or renaming `Profile` to clarify whether it means "HardwareTarget" or "ConfigurationProfile". Alternatively, remove the hardware-specific naming convention from the concept of a `Profile`.

## Context

"One Concept, One Preferred Term". The domain noun "Profile" is overloaded to mean both "The hardware being provisioned" (via `validate_hardware_profile`) and "A global configuration scope".

## Evidence

- path: "src/domain/profile.rs"
  loc: "line 9: `pub enum Profile { Macbook, MacMini, Global }`"
  note: "Mixes hardware targets with global scope."

- path: "src/domain/profile.rs"
  loc: "line 25: `fn is_hardware_profile(self) -> bool { matches!(self, Self::Macbook | Self::MacMini) }`"
  note: "Explicitly defines hardware within a Profile."

## Change Scope

- `src/domain/profile.rs`
