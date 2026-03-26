# Rusty Git

## Project Overview

Rusty Git is a simplified reimplementation of Git in Rust, built to understand Git internals by implementing the core data model directly: object storage, trees, commits, references, and the index (staging area).

The project focuses on correctness and clarity over feature completeness. The code is organized around the same state transitions used by Git:

- working directory -> index
- index -> commit
- commit graph traversal and reference updates

## Features

- Repository initialization
- Object storage (blob, tree, commit)
- SHA-1 hashing
- Add (single file and `add .` recursive staging)
- Commit (with and without `-a`)
- Branching
- Checkout with overwrite safety checks
- Log history traversal
- Status (working directory vs index vs HEAD)
- Diff (line-based output)
- Reset (`--soft` and mixed/default)
- Restore (index -> working directory)
- Remove (`rm`)
- Ignore rules (`.rustygitignore`)

## CLI Usage

```bash
rustygit init
rustygit add file.txt
rustygit add .
rustygit commit -m "message"
rustygit commit -a -m "message"
rustygit status
rustygit diff
rustygit checkout <hash-or-branch>
rustygit branch <name>
rustygit reset <commit>
rustygit reset --soft <commit>
rustygit restore file.txt
rustygit rm file.txt
rustygit log
```

## Key Concepts

- Objects:
  - Blob stores file bytes.
  - Tree stores a directory snapshot as entries of mode, name, and object hash.
  - Commit stores tree pointer, parent pointer, metadata, and message.
- Index:
  - The staging area that represents user intent for the next commit.
  - Implemented as a text file mapping `<hash> <path>`.
- HEAD:
  - Points to the current branch reference (attached) or a commit hash (detached).
- Branches:
  - Named references under `.rustygit/refs/heads/*`.

## Example Workflow

```bash
rustygit init

echo "hello" > note.txt
rustygit add note.txt
rustygit commit -m "initial"

echo "hello again" > note.txt
rustygit status
rustygit diff
rustygit add note.txt
rustygit commit -m "update note"
```

## Testing

- Unit tests for object formatting and hashing behavior.
- Integration tests for command flows (`add`, `commit`, `checkout`, `reset`, `restore`, `rm`, etc.).
- Safety/edge-case tests for checkout overwrite protection.
- Temporary directory-based isolation (`tempfile`) for deterministic repository scenarios.

Run all tests:

```bash
cargo test
```

## Limitations

- No merge support
- No remote repositories
- No packfiles or object compression pipeline
- Simplified diff implementation
- No conflict resolution

See detailed notes in [docs/limitations.md](docs/limitations.md).
