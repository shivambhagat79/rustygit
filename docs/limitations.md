# Limitations

Rusty Git intentionally focuses on core internals and omits several advanced Git behaviors.

## Not Implemented

- Merge and rebase workflows
- Remote operations (`fetch`, `pull`, `push`)
- Packfile storage/transfer and object compression optimizations
- Conflict resolution tooling
- Partial/staged hunks and interactive staging
- Hard reset mode (`--hard`) and full worktree rewriting controls

## Practical Implications

- History is linear unless users manually create diverging branches.
- Repositories can grow quickly because objects are stored individually.
- Collaboration workflows are out of scope without remotes.

## Future Extensions

- Add commit graph and merge-base utilities for merge support.
- Implement remote protocol subset and reference negotiation.
- Introduce packfile read/write for storage efficiency.
- Expand index model for partial staging and conflict states.
- Add safer destructive operations (`reset --hard`, checkout pathspecs).
