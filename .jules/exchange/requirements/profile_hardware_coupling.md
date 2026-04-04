---
label: "feats"
implementation_ready: true
---

## Goal

Consider splitting or renaming `Profile` to clarify whether it means "HardwareTarget" or "ConfigurationProfile". Alternatively, remove the hardware-specific naming convention from the concept of a `Profile`.

## Problem

The term `Profile` currently mixes hardware variants (`Macbook`, `MacMini`) with a `Global` configuration state. A Profile shouldn't be defined strictly as hardware constraints while also including a `global` identifier, confusing the domain noun "Profile" which usually means user-centric configuration sets instead of hardware targets.

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

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.

## Acceptance Criteria

- The core issues detailed in the problem statements are resolved.
- Required tests are written or passing after the change.
- The identified file paths in the change scope have been appropriately modified according to the goal.
