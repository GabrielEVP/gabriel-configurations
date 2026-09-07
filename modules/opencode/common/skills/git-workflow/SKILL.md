---
name: git-workflow
description: "Trigger: git, rebase, merge, cherry-pick, bisect, reflog, conflict, hooks, submodule, stash, blame. Advanced Git — interactive rebase, conflict resolution, git bisect, cherry-pick, hooks, submodules, reflog, advanced troubleshooting."
license: Apache-2.0
metadata:
  author: gabriel
  version: "1.0"
---

# Advanced Git

## Interactive Rebase

```bash
# Rebase last N commits
git rebase -i HEAD~3

# Rebase from a specific commit (exclusive)
git rebase -i <commit-hash>^

# Rebase onto another branch
git rebase -i --onto main feature-branch~5 feature-branch

# Abort rebase (if something goes wrong)
git rebase --abort

# Skip a commit during rebase
git rebase --skip

# Continue after resolving conflicts
git rebase --continue

# Autosquash (combine fixup commits)
# First: commit with --fixup or --squash
git commit --fixup=<commit-hash>
# Then rebase with autosquash
git rebase -i --autosquash HEAD~5
```

### Rebase vs Merge Decision

| Scenario | Use |
|----------|-----|
| Private feature branch | Rebase (clean history) |
| Shared branch | Merge (preserve collaboration context) |
| Updating feature branch with main | Rebase (avoid merge commits) |
| Integrating a PR | Merge or squash merge |
| Multiple devs on same branch | Merge (rebase would rewrite shared history) |

## Conflict Resolution

```bash
# Check which files have conflicts
git status
git diff              # shows conflict markers

# List conflicted files
git diff --name-only --diff-filter=U

# Use a merge tool
git mergetool

# Accept one side for all conflicts in a file
git checkout --ours <file>
git checkout --theirs <file>

# After resolving conflicts manually
git add <resolved-file>
git rebase --continue   # if in rebase
git merge --continue    # if in merge

# Show conflict base (common ancestor)
git checkout --conflict=diff3 <file>
# Shows: <<<<<<< ours | ||||||| base | ======= theirs | >>>>>>>
```

## Cherry-Pick

```bash
# Pick a single commit into current branch
git cherry-pick <commit-hash>

# Pick multiple commits
git cherry-pick <hash1> <hash2> <hash3>

# Pick a range of commits
git cherry-pick <start-hash>^..<end-hash>

# Cherry-pick with no commit (stage changes only)
git cherry-pick -n <commit-hash>

# Cherry-pick but edit commit message
git cherry-pick -e <commit-hash>

# Abort cherry-pick
git cherry-pick --abort

# Continue after resolving conflicts
git cherry-pick --continue
```

## Git Bisect (Find the Commit That Introduced a Bug)

```bash
# Start bisect
git bisect start

# Mark current commit as bad
git bisect bad

# Mark a known-good commit
git bisect good <known-good-commit>

# Git will checkout a midpoint. Test it, then:
git bisect good   # if the bug isn't present
git bisect bad    # if the bug is present

# Repeat until git shows the first bad commit

# Skip a commit that can't be tested
git bisect skip

# Run bisect automatically with a script
git bisect run <script>  # script exits 0=good, 1=bad

# Reset bisect when done
git bisect reset

# Bisect with specific range
git bisect start <bad-commit> <good-commit>
```

## Reflog (Safety Net)

```bash
# Show all HEAD movements
git reflog

# Show reflog for a specific branch
git reflog <branch-name>

# Recover a deleted commit
git reflog
# Find the commit hash, then:
git cherry-pick <hash>
# or create a branch at that point:
git branch recovered-branch <hash>

# Recover after a bad reset
git reflog
# Find the state before reset, then:
git reset --hard HEAD@{<n>}

# Show reflog with dates
git reflog --date=iso

# Clean reflog entries older than N days
git reflog expire --expire=30.days --all
```

## Stash Advanced

```bash
# Stash with message
git stash push -m "WIP: refactoring auth module"

# Stash only specific files
git stash push -m "config changes" src/config/*

# Stash including untracked files
git stash push -u

# Stash including ignored files too
git stash push -a

# List stashes
git stash list

# Show stash content
git stash show -p stash@{0}

# Apply a specific stash (keeps in stash list)
git stash apply stash@{2}

# Pop a stash (applies and removes from list)
git stash pop stash@{1}

# Create a branch from a stash
git stash branch <new-branch> stash@{0}

# Remove a specific stash
git stash drop stash@{1}

# Clear all stashes
git stash clear
```

## Hooks (Local)

```bash
# Hook locations (inside .git/hooks/)
.git/hooks/
├── pre-commit          # Before commit message editor
├── prepare-commit-msg  # Before commit message is finalized
├── commit-msg          # After commit message is created
├── post-commit         # After commit
├── pre-push            # Before push
├── pre-rebase          # Before rebase
├── post-merge          # After merge
└── post-checkout       # After checkout

# Make a hook executable
chmod +x .git/hooks/pre-commit

# Skip hooks for a single commit
git commit --no-verify

# Skip hooks for push
git push --no-verify

# Example pre-commit hook (lint check)
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/sh
npm run lint
if [ $? -ne 0 ]; then
  echo "Lint failed. Commit aborted."
  exit 1
fi
EOF
chmod +x .git/hooks/pre-commit

# Alternative: using husky (Node.js)
npx husky init
npx husky add .husky/pre-commit "npm run lint"
```

## Submodules

```bash
# Add a submodule
git submodule add <repository-url> <path>

# Clone a repo with submodules
git clone --recurse-submodules <repository-url>

# Init submodules after a regular clone
git submodule init
git submodule update

# Update all submodules to latest commit
git submodule update --remote

# Update a specific submodule
git submodule update --remote <path>

# Pull all submodules
git submodule foreach git pull origin main

# Show submodule status
git submodule status

# Remove a submodule
git submodule deinit <path>
git rm <path>
rm -rf .git/modules/<path>

# Commit submodule changes
# After updating submodule, commit the parent:
git add <path>
git commit -m "chore(deps): update submodule <name>"
```

## Advanced Log & Search

```bash
# Log with graph (branch topology)
git log --oneline --graph --all --decorate

# Log with diff stats
git log --stat

# Log with patches
git log -p

# Search commits by message
git log --grep="fix:"

# Search commits by author
git log --author="gabriel"

# Search commits by date range
git log --since="2024-01-01" --until="2024-06-30"

# Show commits in branch A but not in B
git log --oneline <branch-a> ^<branch-b>
git log --oneline <branch-a>..<branch-b>  # same thing

# Show commits that modified a specific file
git log --oneline -- <file-path>

# Search code across all commits
git log -S<search-string> --oneline        # added/removed string
git log -G<regex> --oneline                # match regex in diff

# Find when a specific line was last changed (blame)
git blame <file-path>
git blame -L 10,30 <file-path>            # lines 10-30
git blame -w <file-path>                  # ignore whitespace

# Find when a function was last changed
git log -L :<function-name>:<file-path>
```

## Reset & Restore

```bash
# Unstage a file (keep changes)
git reset HEAD <file>
# or modern:
git restore --staged <file>

# Discard working directory changes
git restore <file>

# Discard all uncommitted changes
git restore .

# Reset to previous commit (keep changes staged)
git reset --soft HEAD~1

# Reset to previous commit (keep changes unstaged)
git reset --mixed HEAD~1

# Reset to previous commit (discard changes — DANGEROUS)
git reset --hard HEAD~1

# Reset to a specific commit
git reset --hard <commit-hash>

# Remove untracked files
git clean -n           # dry run
git clean -fd          # force delete files and directories
```

## Worktrees (Multiple Branches at Once)

```bash
# Create a new worktree
git worktree add ../project-feature feature-branch

# Create a worktree from a specific commit
git worktree add ../project-hotfix <commit-hash>

# List worktrees
git worktree list

# Remove a worktree
git worktree remove ../project-feature

# Prune stale worktree references
git worktree prune
```

## Patch Management

```bash
# Create a patch from latest commit
git format-patch HEAD~1

# Create patches for last N commits
git format-patch HEAD~N

# Apply a patch
git apply <patch-file>

# Apply a patch from a URL
curl -L <url> | git apply

# Create a diff file
git diff > changes.patch

# Apply diff file
git apply changes.patch

# Check if patch applies cleanly
git apply --check <patch-file>

# Export commits as patch series
git format-patch -o patches/ <start-commit>..<end-commit>
```

## Useful Configuration

```bash
# Set your identity
git config --global user.name "Your Name"
git config --global user.email "your@email.com"

# Default branch name
git config --global init.defaultBranch main

# Better diff
git config --global diff.algorithm histogram
git config --global diff.colorMoved plain

# Rebase on pull (instead of merge)
git config --global pull.rebase true

# Autosetup remote tracking
git config --global push.autoSetupRemote true

# Global gitignore
git config --global core.excludesFile ~/.gitignore_global

# Pretty log
git config --global alias.lg "log --oneline --graph --all --decorate"

# Better status
git config --global alias.st "status -sb"

# Show last commit
git config --global alias.last "log -1 HEAD --stat"

# Undo last commit but keep changes
git config --global alias.undo "reset --soft HEAD~1"

# Show file from another branch
git config --global alias.show-file "!f() { git show $1:$2; }; f"
```

## Common Fixes

```bash
# Accidentally committed to wrong branch
git log --oneline -1                    # get commit hash
git reset --soft HEAD~1                 # undo commit, keep changes
git stash                               # stash changes
git checkout correct-branch
git stash pop
git commit -m "message"

# Modified a commit message
git commit --amend -m "new message"

# Forgot to add a file to last commit
git add <forgotten-file>
git commit --amend --no-edit

# Accidentally deleted a branch
git reflog
# find the commit hash, then:
git branch <branch-name> <hash>

# Recover deleted stash
git fsck --unreachable | grep commit | cut -d' ' -f3 | xargs git log --merges --no-walk --grep="WIP"

# Large file committed? Remove from git history
# Use git-filter-repo or BFG Repo-Cleaner
# git filter-repo --path <large-file> --invert-paths
```
