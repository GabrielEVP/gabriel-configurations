# Configuración de Codex

Respaldo de los archivos de configuración de `~/.codex`:

- `config.toml`: modelo, preferencias, proyectos de confianza, plugins y servidores MCP.
- `skills/`: las 52 skills personales con sus referencias y recursos.
- `rules/`: reglas de ejecución.

No incluye credenciales (`auth.json`), conversaciones, bases de datos, cachés ni
skills integradas o plugins descargados automáticamente por la aplicación.
La selección de plugins está en `config.toml`; su instalación y autenticación se
realizan desde Codex. Tampoco incluye el estado interno de la interfaz de la app.

## Restaurar

Cierra Codex antes de restaurar. Desde la raíz de este repositorio:

```bash
codex_target="${CODEX_HOME:-$HOME/.codex}"
if [ -e "$codex_target" ]; then
  cp -a "$codex_target" "${codex_target}.backup.$(date +%Y%m%d%H%M%S)"
fi
mkdir -p "$codex_target"
cp codex/config.toml "$codex_target/config.toml"
rsync -a codex/skills/ "$codex_target/skills/"
rsync -a codex/rules/ "$codex_target/rules/"
```

El archivo conserva las rutas originales de este Mac (`/Users/gabrielvargas`,
`/Applications/ChatGPT.app`) y referencias a componentes de la aplicación.
En otra máquina revisa esas rutas, proyectos de confianza y servidores MCP antes
de abrir Codex. Inicia sesión de nuevo y habilita o instala los plugins necesarios.
