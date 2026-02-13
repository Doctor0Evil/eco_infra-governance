# eco_infra-governance

**eco_infra-governance** is a thin, Rust-first governance spine for AI- and infrastructure-adjacent systems that need **ABAC-style policy**, **NIST AI RMF 1.1 + NIST SP 800‑53 alignment**, and **Web5/PQC identity + audit bindings**, without pulling any OT/actuator logic into the library itself.[file:1][file:7]

It is designed as a reusable core for Virtanetv1 / VSC‑ARTEMIS style stacks, but is generic enough to embed into other domains that need typed governance metadata, human-in-the-loop (HITL) gates, and verifiable audit trails.[file:1][file:7]

---

## Goals

- Provide a **no‑actuation, all‑governance** crate that can be safely used in high‑impact environments (energy, smart‑city, critical infrastructure) as a policy and audit spine.[file:7]
- Treat AI RMF + SP 800‑53 + COSAiS overlays as **data, not prose**, with compile‑ and test‑time validation rather than ad‑hoc documentation.[file:1][file:7]
- Encode **human primacy, HITL, neurorights/species-rights, and risk‑of‑harm ceilings** directly in Rust types and guard flows.[file:1][file:7]
- Integrate cleanly with **Web5/DID + PQC** stacks and DWN‑anchored audit, while remaining voting‑ and model‑agnostic.[file:1][file:7]

---

## Features

- **Shard governance metadata**

  A canonical `ShardGovernanceMeta` struct that binds each shard / nanopolygon / route segment to:[file:1][file:7]

  - AI RMF function tags (`MAP`, `MEASURE`, `MANAGE`, `GOVERN`) and hazard class.
  - NIST SP 800‑53 families and controls (e.g., `AC-2`, `AC-3`, `AU-12`, `RA-5`).
  - FIPS‑style impact labels (CIA: Low / Moderate / High).
  - Web5/PQC profile (DID, DWN endpoints, ML‑KEM / ML‑DSA profiles).
  - Human primacy / HITL level and appeal path identifier.
  - Role constraints for AC‑style access control (role name + min signatures).

- **Typed governance traits**

  Trait interfaces such as:[file:7]

  - `HasGovernanceMeta`
  - `HasAiRmfTag`
  - `HasNist80053Controls`
  - `HasWeb5Anchor`
  - `HasHumanPrimacy`
  - `HasRoleConstraints`

  let any domain type (nanopolygon, ALN shard, route spec) “opt in” to governance without inheriting a specific base struct.

- **Governance guard**

  A `GovernanceGuard` with a `GovernanceDecision` enum (`Allowed`, `RequiresHitlGate`, `Rejected`) that:[file:1][file:7]

  - Enforces risk‑of‑harm ceilings and monotonic safety (no harmful deltas).
  - Enforces AC‑style role constraints based on stake/role sets.
  - Switches to `RequiresHitlGate` when HITL is required for the shard’s impact level.
  - Stays voting‑ and model‑agnostic (you wire in your own stake / VC / role logic).

- **HITL typestates**

  Rust typestates for HITL flows:[file:1][file:7]

  - `PendingReview<T>`
  - `ApprovedByHuman<T>`
  - `Executed<T>`

  with explicit human DID and donutloop/DWN hexstamp, so “human approval” is a type transition, not a comment.

- **Audit‑ready by design**

  Types and traits are structured so you can align your audit schema to AU‑2 / AU‑3 / AU‑12 / RA‑5, and bind events to DIDs and DWNs for tamper‑evident logging.[file:7]

---

## Architecture

The crate is intentionally small and composable. A typical module layout looks like:[file:7]

```text
src/
  lib.rs
  governancemeta.rs      # ShardGovernanceMeta and related types
  governanceguard.rs     # GovernanceGuard + GovernanceDecision
  hitl_typestate.rs      # PendingReview / ApprovedByHuman / Executed
  traits.rs              # Has* trait interfaces
  error.rs               # Error types for governance failures
```

You embed `ShardGovernanceMeta` (or just the `Has*` traits) into your own types, then call into `GovernanceGuard` before any high‑impact action or route is accepted.[file:1][file:7]

---

## Example (conceptual)

```rust
use eco_infra_governance::{
    governancemeta::{ShardGovernanceMeta, HitlLevel},
    governanceguard::{GovernanceGuard, GovernanceDecision},
    traits::HasGovernanceMeta,
};

struct NanopolygonShard {
    // your fields...
    governance: ShardGovernanceMeta,
}

impl HasGovernanceMeta for NanopolygonShard {
    fn governance_meta(&self) -> &ShardGovernanceMeta {
        &self.governance
    }
}

fn evaluate_route(shard: &NanopolygonShard) -> anyhow::Result<()> {
    let guard = GovernanceGuard::new(/* your RoH + stake providers */);

    match guard.evaluate(shard, /* roh_before */, /* roh_after */, /* caller_roles */)? {
        GovernanceDecision::Allowed(_) => {
            // safe to proceed with non‑OT logic, or queue for actuation layer
        }
        GovernanceDecision::RequiresHitlGate(pending) => {
            // surface PendingReview<T> into your HITL UI / workflow
            // and only proceed after human approval
        }
        GovernanceDecision::Rejected(reason) => {
            anyhow::bail!("governance rejection: {reason}");
        }
    }

    Ok(())
}
```

This keeps **actuation out of the crate**: you only evaluate policies, gate on HITL, and emit audit‑ready decisions.[file:7]

---

## Standards alignment

eco_infra-governance is designed to align with:[file:1][file:7]

- **NIST AI RMF 1.1** (GOVERN / MAP / MEASURE / MANAGE) via `AiRmfTag` and guard behavior.
- **NIST SP 800‑53 Rev. 5 / COSAiS overlays** via `Sp80053Ref`, `Sp80053Impact`, and coverage checks.
- **Web5 / DID / DWN** via `Web5PqcProfile` and optional `DwnAudit` / `GovernanceHooks` traits in higher layers.
- **Virtanetv1 / VSC‑ARTEMIS ALN** design patterns (nanopolygons, ALN shards, risk‑of‑harm and neurorights overlays) as a primary reference implementation.

Formal certification is out of scope for the crate itself, but the types and docs are structured so you can map fields directly to AU, AC, SC, RA families and AI RMF functions in your own compliance documentation.[file:7]

---

## When to use this crate

Use **eco_infra-governance** if you need:

- A minimal, embeddable Rust library that makes **governance and compliance first‑class**, not an afterthought.
- ABAC‑style policy evaluation with explicit **role constraints, HITL gates, and RoH ceilings**.
- A neutral governance core you can plug into eco‑infrastructure, smart‑city, or critical‑infrastructure stacks, without coupling to any particular voting or model‑serving framework.[file:1][file:7]

Do **not** use this crate directly to drive hardware or actuators; keep that in a separate OT/field layer that consumes the decisions and audit events generated here.[file:7]

---

## Status

This repository is an early, implementation‑grade spine extracted from the Virtanetv1 / Eco‑Infra governance research. Expect APIs to evolve as more deployments and standards feedback are integrated.[file:1][file:7]

Contributions, issues, and review are welcome.
