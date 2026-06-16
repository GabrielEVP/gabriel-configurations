---
name: sync-gabriel-config
description: "Trigger: sync config, sincronizar, gabriel-config, sync with repo, actualizar configuraciones, memory repo. Sync ~/.config/opencode/ changes back to the gabriel-configurations GitHub repo."
license: Apache-2.0
metadata:
  author: gabrielvargas
  version: "1.0"
---

# Sync Gabriel Configurations

## Activation Contract

Activate when the user says any variation of:
- "sync config" / "sincronizar configuraciones"
- "sync with the memory repo" / "sync con el repo de memoria"
- "actualizar gabriel-config" / "update gabriel-config"
- "subir cambios al repo de configuraciones"
- "gabriel-configurations"

This skill syncs the live `~/.config/opencode/` directory back to the GitHub repo `https://github.com/GabrielEVP/gabriel-configurations.git`.

## Repo Location

- **Local clone**: `/Users/gabrielvargas/dev/gabriel-configurations`
- **Remote**: `https://github.com/GabrielEVP/gabriel-configurations.git`
- **Branch**: `main`

## What to Sync

Copy these from `~/.config/opencode/` to `dev/gabriel-configurations/opencode/`:

| Source (live) | Destination (repo) | Always? |
|---|---|---|
| `agents/*.md` | `opencode/agents/*.md` | Yes |
| `plugins/*.ts` | `opencode/plugins/*.ts` | Yes |
| `skills/*/` (each skill dir) | `opencode/skills/*/` | Yes |
| `profiles/*.json` | `opencode/profiles/*.json` | Yes |
| `prompts/*` | `opencode/prompts/*` | Yes |
| `commands/*` | `opencode/commands/*` | Yes |
| `themes/*` | `opencode/themes/*` | Yes |
| `tui-plugins/*` | `opencode/tui-plugins/*` | Yes |
| `tui.json` | `opencode/tui.json` | Yes |

## What NOT to Sync

| Item | Reason |
|---|---|
| `node_modules/` | Runtime dependency |
| `package.json`, `package-lock.json`, `bun.lock` | Runtime dependency |
| `.atl` | OpenCode internal state |
| `.gitignore` | Not versioned |
| `config.json` | Legacy file, stays in repo only |

## Hard Rules

1. **Never commit API tokens or secrets.** Before syncing, verify `opencode.json` and `config.json` in the repo do NOT contain real tokens — replace with placeholders if needed.
2. **Never delete files from the repo** that don't exist in live config — the repo may have template files that aren't in the live setup.
3. **Always check `git status`** before and after syncing.
4. **Always show the user a list of changes** before committing.
5. **Use conventional commits**: `feat(opencode):`, `fix(opencode):`, `chore(opencode):`.
6. **Always push** after committing unless the user says otherwise.

## Execution Steps

1. **Verify location**: Confirm the repo is at `~/dev/gabriel-configurations` and `origin` points to `https://github.com/GabrielEVP/gabriel-configurations.git`.
2. **Pull latest**: `git pull --rebase` to avoid conflicts.
3. **Check diff**: Run `diff -rq` between live and repo to identify new, modified, and missing files.
4. **Exclude runtime artifacts**: Filter out `node_modules/`, `package*.json`, `bun.lock`, `.atl`, `.gitignore`.
5. **Copy files**: Use `cp` for individual files, `rsync -a` for directories.
6. **Verify**: Run `git status` and `git diff --stat` to review.
7. **Show user**: Present the list of changes with Commit Confirmation protocol.
8. **Commit & push**: On user confirmation, `git add`, `git commit`, `git push`.

## References

- Repo: `https://github.com/GabrielEVP/gabriel-configurations`
- Local clone: `~/dev/gabriel-configurations`
- Live config: `~/.config/opencode/`
