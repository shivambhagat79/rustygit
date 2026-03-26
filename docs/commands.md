# Commands

## init

Creates a new `.rustygit` repository structure with object store, refs, HEAD, and index.

## add

Stages file content into the index.

- `add <file>`: stage one file.
- `add .`: recursively stage all non-ignored files.

## commit

Creates a commit from the index snapshot and updates the current branch reference.

- `commit -m "msg"`: commit staged index.
- `commit -a -m "msg"`: auto-stage tracked modifications/deletions, then commit.

## status

Shows repository state by comparing:

- index vs HEAD (staged)
- working directory vs effective index (modified/deleted)
- working directory entries not in index (untracked)

## diff

Shows line-level changes between HEAD-tracked files and working directory.

## log

Traverses commit parent links from current HEAD and prints formatted history.

## branch

- `branch <name>` creates a branch at current commit.
- `branch` lists branches and marks current branch.

## checkout

Switches to branch or commit and restores working directory from target tree.
Includes safety checks to prevent overwriting local changes.

## rm

Removes file from index and working directory, staging deletion.
Refuses removal if working file differs from index to prevent accidental data loss.

## restore

Restores file in working directory from index state.
If file is not in index, removes it from working directory.

## reset

Moves HEAD to a target commit.

- `reset --soft <commit>`: move HEAD only
- `reset <commit>` (mixed): move HEAD and replace index with target commit tree

Working directory is unchanged in both modes.
