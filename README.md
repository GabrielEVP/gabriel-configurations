# gabriel-configurations

Personal dotfiles for ghostty, neovim, opencode, zed, and zsh.

## Contents

```
gabriel-configurations/
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
│   └── skills/         # Skill files for SDD workflows
├── zed/                # Zed editor config (keymaps, settings, themes, tasks)
├── zsh/                # Zsh config (.zshrc, .zprofile, .zshenv)
└── install.sh          # Cross-platform automated installation (macOS + Linux)
```

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
- **IosevkaTerm NF** font — [nerdfonts.com](https://www.nerdfonts.com/font-downloads)
- `git`, `rsync` (for the install script)

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
