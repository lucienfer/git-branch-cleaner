# git-branch-cleaner

## Functionality
- ✅ Automatically deletes merged Git branches
- 🔍 Dry-run mode to preview deletions
- 🧠 Selection by base branch (main, develop, etc.)

## 🚀 Installation
```bash
cargo install --path .
```

## How to use
```bash
git-branch-cleaner --base main --dry-run
```
Options:
- `--base` Specify the base branch used to detect merged branches.
- `--dry-run` Display the branches that would be deleted, without actually deleting them.
- `--force` Delete branches without asking for confirmation.
- `--interactive` Choose which branches to delete one by one.

## Tests
```bash
cargo test
```