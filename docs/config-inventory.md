# Inventario de configuración

Inventario inicial de fuentes versionadas; no incluye valores de credenciales. config significa XDG_CONFIG_HOME cuando existe y el destino por defecto de cada herramienta cuando no existe.

| Módulo | Fuente actual | Destino previsto | Requisitos y diferencias |
|---|---|---|---|
| nvim | nvim/ | config/nvim/ | git, Node/npm, compilador para parsers, búsqueda fzf/fd/rg, portapapeles. Mason declara jdtls, vtsls, tailwindcss, biome, lua_ls, gopls, angularls, jsonls, marksman y taplo; Java/Go dependen de esas integraciones. Evitar rutas Node fijas y referencias WSL. |
| ghostty | ghostty/ | config/ghostty/ | IosevkaTerm Nerd Font, temas y shaders; validar teclas y opciones admitidas por plataforma. |
| alacritty | alacritty/ | config/alacritty/ | Misma fuente, opciones de ventana y modificadores por sistema. |
| kitty | kitty/ | config/kitty/ | Misma fuente y compatibilidad de acciones de teclado, incluido clipboard_export. |
| zsh | zsh/zshrc, zprofile, zshenv | home/.zshrc, .zprofile, .zshenv | Homebrew, Oh My Zsh, fzf, fd, bat, zoxide, atuin, carapace y plugins syntax-highlighting/autosuggestions; EDITOR nvim. |
| tmux | tmux/tmux.conf | home/.tmux.conf | TPM y plugins declarados; clip en Linux debe sustituirse por backend X11/Wayland detectado, pbcopy en macOS. |
| zellij | zellij/config.kdl | config/zellij/config.kdl | Validación KDL y compatibilidad con la versión instalada; eliminar dependencia de referencias históricas a home del autor. |
| yazi | yazi/theme.toml | config/yazi/theme.toml | Yazi y fuente; separar herramientas opcionales de preview de dependencias del tema. |
| zed | zed/ | config/zed/ | Fuente, agentes/context servers, disponibilidad gráfica; preservar configuración local de servicios. |
| opencode | opencode/ | config/opencode/ | Integraciones MCP y plugins que invocan procesos externos; preservar credenciales y campos locales. |

## Personalizaciones y datos locales

Zsh contiene PROJECT_PATHS y una función de sesiones Parsun que requiere directorios personales y herramientas de proyecto. Mantener esa preferencia configurable, con comprobación de directorios. Los aliases Docker y las cargas condicionadas de Bun, NVM, PNPM y SDKMAN no implican instalar automáticamente todos esos ecosistemas. Activar únicamente integraciones cuyos requisitos estén presentes o seleccionados.

Neovim busca Node en prefijos macOS y gestores de usuario, mientras lazy.lua vuelve a asignar node_host_prog. Unificar ambos puntos. java.lua prefiere SDKMAN; admitir un Java compatible detectado. Integraciones Claude requieren ejecutable/autenticación y deben diagnosticarse separadamente.

Los archivos opencode/config.json y opencode/opencode.json contienen configuración MCP con campos environment; no copiar valores al inventario. opencode.json también contiene provider, lsp y plugin. Revisar claves locales de Zed con el mismo criterio. No imprimir diffs de archivos sensibles.

## Referencias y migración

install.sh y README contienen las rutas de las carpetas actuales. opencode/skills/sync-gabriel-config/SKILL.md enumera los destinos de sincronización y deberá actualizarse al migrar, usando la skill de edición correspondiente. El instalador enlaza Zsh a /tmp/gabriel-configurations-install y elimina ese clon: respaldar incluso enlaces rotos.

Mantener recursos Neovim necesarios, incluyendo diccionarios y licencias, hasta comprobar su uso; no excluir automáticamente spell por su tamaño. El inventario registra dependencias observadas; versiones y recetas se validan separadamente antes de declarar compatibilidad.
