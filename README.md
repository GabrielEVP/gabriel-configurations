# gabriel-configurations

Personal dotfiles for Codex, ghostty, neovim, opencode, zed, zsh, zellij, and tmux.

## Contents

```
gabriel-configurations/
├── codex/              # Codex settings, personal skills and execution rules
├── ghostty/
│   ├── config          # Terminal configuration (font, colors, keybindings)
│   └── themes/         # Ghostty theme presets
├── nvim/               # Neovim config based on LazyVim + custom plugins
├── opencode/
│   ├── config.json     # MCP servers configuration (tokens required)
│   ├── opencode.json   # Main opencode configuration
│   ├── tui.json        # TUI theme selection
│   ├── agents/         # Custom AI agents
│   ├── themes/         # OpenCode themes
│   ├── plugins/        # OpenCode plugins
│   ├── profiles/       # Execution profiles
│   ├── prompts/        # Prompt templates
│   └── skills/         # Skill files for SDD workflows
├── tmux/               # Tmux config (.tmux.conf)
├── zed/                # Zed editor config (keymaps, settings, themes, tasks)
├── zellij/             # Zellij config (config.kdl)
├── zsh/                # Zsh config (.zshrc, .zprofile, .zshenv)
└── install.sh          # Cross-platform automated installation (macOS + Linux)
```

## Codex

See [codex/README.md](codex/README.md) for the configuration backup and manual restore instructions.

## Quick Install

```bash
bash <(curl -fsSL https://raw.githubusercontent.com/GabrielEVP/gabriel-configurations/main/install.sh)
```

Or clone and run manually:

```bash
git clone https://github.com/GabrielEVP/gabriel-configurations.git
cd gabriel-configurations
chmod +x install.sh
./install.sh
```

> **Note:** Existing config directories are backed up automatically before installation.

## Rust CLI (in development)

The new `gabriel` CLI supports Ubuntu and macOS, discovers modules from `modules/`, adapts configuration paths per platform, and preserves local edits with recoverable state.

```bash
cargo run -- setup --all --dry-run
cargo run -- install tmux --dry-run
cargo run -- status
cargo run -- doctor
cargo run -- config update --yes
cargo run -- config resolve nvim --file init.lua --use incoming --yes
cargo run -- upgrade nvim --yes
```

Add a tool by creating `modules/<id>/module.toml` and its `common/`, `ubuntu/`, and `macos/` files. Use `--source <checkout>` to test another catalog. Package installation requires explicit `--yes`; configuration-only modules can be applied after previewing the plan.

GitHub Actions verifies formatting, Clippy, and the full test suite on Ubuntu 24.04 and macOS 15.

## Post-Install: Add Your Tokens

After installation, edit `~/.config/opencode/opencode.json` and add your API tokens:

| Token | Description |
|---|---|
| `GITHUB_PERSONAL_ACCESS_TOKEN` | GitHub Personal Access Token |
| `GITLAB_TOKEN` | GitLab Personal Access Token |
| `FIGMA_ACCESS_TOKEN` | Figma API Key |
| `NOTION_TOKEN` | Notion Integration Token |

## Requirements

- **ghostty** — [ghostty.org](https://ghostty.org)
- **neovim** >= 0.10
- **opencode** — [opencode.ai](https://opencode.ai)
- **zed** — [zed.dev](https://zed.dev)
- **zellij** — [zellij.dev](https://zellij.dev) (optional)
- **tmux** — [github.com/tmux/tmux](https://github.com/tmux/tmux) (optional)
- **IosevkaTerm NF** font — [nerdfonts.com](https://www.nerdfonts.com/font-downloads)
- `curl` and `sha256sum` or `shasum` (for the bootstrap script)

## Neovim

Based on [LazyVim](https://www.lazyvim.org/) with:
- `blink.cmp` — completion engine
- `fzf-lua` — fuzzy finder
- `oil.nvim` — file manager
- `nvim-dap` — debugger
- `trouble.nvim` — diagnostics list
- `neotest` — test runner
- And more in `lua/plugins/`

## License

MIT

## Herdr

See [herdr/README.md](herdr/README.md) for the layout, custom mods and keybindings backup.
