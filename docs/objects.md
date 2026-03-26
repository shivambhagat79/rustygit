# Git Objects in Rusty Git

Rusty Git stores content in `.rustygit/objects` using SHA-1 object hashes.

## Blob

Blob objects store file contents.

Format:

```text
blob <size>\0<file-bytes>
```

Used by:

- `add` and `add .` when staging files
- tree construction when writing snapshots

## Tree

Tree objects represent directories.

Each entry contains:

- mode (`100644` file or `40000` directory)
- name
- object hash (raw 20-byte SHA-1)

Trees are built recursively from index paths. Nested paths produce subtree objects.

## Commit

Commit objects represent repository snapshots and history links.

A commit stores:

- `tree <hash>`: root snapshot pointer
- `parent <hash>`: previous commit (if any)
- author/committer metadata
- commit message

Commit objects are immutable; references (HEAD/branches) are what move.

## Object Immutability

If an object with the same hash already exists, Rusty Git does not rewrite it.
This matches Git's content-addressed object model.
