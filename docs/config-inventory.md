# Configuration inventory

Initial inventory of versioned sources; it does not include credential values. `config` means XDG_CONFIG_HOME when set and each tool's default destination otherwise.

| Module | Current source | Expected destination | Requirements and differences |
|---|---|---|---|
| nvim | nvim/ | config/nvim/ | git, Node/npm, parser compiler, fzf/fd/rg search, clipboard. Mason declares jdtls, vtsls, tailwindcss, biome, lua_ls, gopls, angularls, jsonls, marksman, and taplo. Avoid fixed Node paths and WSL references. |
| ghostty | ghostty/ | config/ghostty/ | IosevkaTerm Nerd Font, temas y shaders; validar teclas y opciones admitidas por plataforma. |
| alacritty | alacritty/ | config/alacritty/ | Misma fuente, opciones de ventana y modificadores por sistema. |
| kitty | kitty/ | config/kitty/ | Misma fuente y compatibilidad de acciones de teclado, incluido clipboard_export. |
| zsh | zsh/zshrc, zprofile, zshenv | home/.zshrc, .zprofile, .zshenv | Homebrew, Oh My Zsh, fzf, fd, bat, zoxide, atuin, carapace, and syntax-highlighting/autosuggestions plugins; EDITOR nvim. |
| tmux | tmux/tmux.conf | home/.tmux.conf | TPM y plugins declarados; clip en Linux debe sustituirse por backend X11/Wayland detectado, pbcopy en macOS. |
| zellij | zellij/config.kdl | config/zellij/config.kdl | Validación KDL y compatibilidad con la versión instalada; eliminar dependencia de referencias históricas a home del autor. |
| yazi | yazi/theme.toml | config/yazi/theme.toml | Yazi y fuente; separar herramientas opcionales de preview de dependencias del tema. |
| zed | zed/ | config/zed/ | Fuente, agentes/context servers, disponibilidad gráfica; preservar configuración local de servicios. |
| opencode | opencode/ | config/opencode/ | Integraciones MCP y plugins que invocan procesos externos; preservar credenciales y campos locales. |

## Personalizations and local data

Zsh contains PROJECT_PATHS and a Parsun session function that requires personal directories and project tools. Keep this preference configurable and check directories first. Docker aliases and conditional Bun, NVM, PNPM, and SDKMAN loading do not imply installing every ecosystem. Enable integrations only when their requirements are present or selected.

Neovim searches for Node in macOS prefixes and user managers, while lazy.lua assigns node_host_prog again. Unify both locations. java.lua prefers SDKMAN; accept a detected compatible Java. Claude integrations require an executable and authentication and must be diagnosed separately.

The opencode/config.json and opencode/opencode.json files contain MCP configuration with environment fields; do not copy values into this inventory. opencode.json also contains provider, lsp, and plugin settings. Review local Zed keys using the same rule. Never print diffs of sensitive files.

## References and migration

install.sh and README contain the current folder paths. opencode/skills/sync-gabriel-config/SKILL.md lists synchronization destinations and must be updated during migration using the relevant editing skill. The installer links Zsh to /tmp/gabriel-configurations-install and removes that clone; back up broken links too.

Keep required Neovim resources, including dictionaries and licenses, until their use is verified; do not exclude spell files automatically because of size. This inventory records observed dependencies; versions and recipes are validated separately before declaring compatibility.
