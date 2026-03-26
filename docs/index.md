# Index (Staging Area)

The index is the staging area: it stores exactly what the next commit should contain.

It is distinct from:

- working directory (what is currently on disk)
- HEAD commit (what was last committed)

## File Location and Format

Rusty Git index file:

```text
.rustygit/index
```

Entry format:

```text
<hash> <path>
```

Example:

```text
a1b2c3... src/main.rs
d4e5f6... README.md
```

## How Rusty Git Uses the Index

- `add <file>` updates a single index entry.
- `add .` walks working files recursively and updates index entries for non-ignored files.
- `rm <file>` removes entry from index and stages a deletion.
- `commit` reads the index and writes a tree/commit snapshot.
- `restore <file>` copies content from index back to working directory.
- `reset` mixed mode replaces index with the target commit tree.

## Why Git Needs an Index

Without the index, commit would always snapshot all working files.
The index lets users stage exact intent before creating history.
