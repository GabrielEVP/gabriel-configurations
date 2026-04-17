# gabriel-configurations

Personal dotfiles for ghostty, neovim, and opencode.

## Contents

```
gabriel-configurations/
├── ghostty/
│   ├── config          # Terminal configuration (font, colors, keybindings, shader)
│   └── shaders/        # GLSL shaders collection (cursor effects, backgrounds, etc.)
├── nvim/               # Neovim config based on LazyVim + custom plugins
├── opencode/
│   ├── config.json     # MCP servers configuration (tokens required — see below)
│   ├── opencode.json   # Engram MCP configuration
│   ├── tui.json        # TUI theme selection
│   ├── agents/         # Custom AI agents (gabriel, gabriel-backend, gabriel-frontend, etc.)
│   ├── themes/         # Custom themes (gabriel-theme, gentleman)
│   ├── plugins/        # Engram plugin
│   └── skills/         # Skill files for specialized workflows
└── install.sh          # Automated installation script
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

> **Note:** Any existing `~/.config/ghostty`, `~/.config/nvim`, or `~/.config/opencode` directories will be backed up automatically before installation.

## Post-Install: Add Your Tokens

After installation, edit `~/.config/opencode/config.json` and replace the placeholders:

| Placeholder | Description |
|---|---|
| `YOUR_GITHUB_TOKEN` | GitHub Personal Access Token |
| `YOUR_GITLAB_TOKEN` | GitLab Personal Access Token |
| `YOUR_FIGMA_API_KEY` | Figma API Key |
| `YOUR_NOTION_TOKEN` | Notion Integration Token |

Also verify the `github-mcp-server` binary path matches your system:

```json
"command": ["/home/linuxbrew/.linuxbrew/bin/github-mcp-server", "stdio"]
```

## Requirements

- **ghostty** — [ghostty.org](https://ghostty.org)
- **neovim** >= 0.10
- **opencode** — [opencode.ai](https://opencode.ai)
- **IosevkaTerm NF** font — [nerdfonts.com](https://www.nerdfonts.com/font-downloads)
- `git`, `rsync` (for the install script)

## Themes

### gabriel-theme
Transparent background with Kanagawa-inspired palette. Best with a dark wallpaper.

### gentleman
Dark navy background (`#06080f`) with a refined color palette. Standalone, no wallpaper needed.

Both themes use black (`#000000`) for input fields and expandable code blocks.

## Neovim

Based on [LazyVim](https://www.lazyvim.org/) with additional plugins:
- `avante.nvim` — AI assistant
- `copilot.nvim` + `copilot-chat.nvim` — GitHub Copilot integration
- `blink.cmp` — completion engine
- `fzf-lua` — fuzzy finder
- `oil.nvim` — file manager
- `obsidian.nvim` — Obsidian vault integration
- `nvim-dap` — debugger
- And more in `lua/plugins/`

## License

MIT
