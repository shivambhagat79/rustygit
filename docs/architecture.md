# Architecture

## High-Level Data Flow

```text
Working Directory
        |
        | add / add . / commit -a / rm / restore
        v
      Index (.rustygit/index)
        |
        | commit
        v
   Tree + Commit Objects
        |
        v
Object Store (.rustygit/objects)

References (.rustygit/HEAD, refs/heads/*)
control which commit is current.
```

## Core Design

Rusty Git separates concerns by keeping user-facing command behavior in `src/commands` and reusable primitives in `src/utils`.

- `src/commands/*`:
  - Implements CLI actions and repository state transitions.
  - Examples: `add`, `commit`, `checkout`, `reset`, `restore`, `rm`.
- `src/utils/*`:
  - Shared functionality: hashing, parsing, ignore matching, safety checks, tree walking, index helpers.

This keeps command code focused on "what state changes" while utility code handles "how data is represented and traversed".

## State Model

Rusty Git uses the standard 3-state model:

- Working directory: current filesystem content.
- Index: planned snapshot for next commit.
- HEAD commit: last saved snapshot pointed to by current ref.

Typical transitions:

- `add` / `add .`: working directory -> index
- `commit`: index -> commit/tree objects + ref update
- `restore`: index -> working directory
- `reset` (mixed): commit tree -> index + ref move

## Module Notes

- `commands/object.rs`: blob formatting, object hashing, object persistence.
- `commands/tree.rs`: recursive tree assembly from index entries.
- `commands/commit.rs`: commit object creation, parent linking, `-a` auto-stage behavior.
- `commands/checkout.rs`: commit/branch restoration to working directory with overwrite safety checks.
- `commands/reset.rs`: HEAD/ref movement and index replacement (mixed mode).
- `commands/status.rs`: computes staged/modified/deleted/untracked categories using HEAD, index, and working directory maps.
- `utils/index.rs`: index file read/write helpers.
- `utils/parse.rs`: blob/tree/commit object parsing.
- `utils/safety_checks.rs`: unsafe checkout prevention.

## Why This Structure Works

- Minimal shared mutable state: each command derives maps from disk and writes explicit outputs.
- Easy testing: commands can be exercised in temp repositories without mocks.
- Extensible: future features (merge, hard reset, staged hunks) can build on existing index/object/ref layers.
