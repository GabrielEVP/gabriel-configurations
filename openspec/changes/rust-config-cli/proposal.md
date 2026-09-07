## Why

El repositorio conserva las preferencias personales, pero su instalador copia archivos sin preparar las aplicaciones y dependencias necesarias ni adaptar todas las rutas a Ubuntu y macOS. Necesitamos reproducir y mantener el entorno desde un CLI fiable, usando el proyecto para aprender Rust mediante TDD y código sin comentarios.

## What Changes

- Crear el CLI `gabriel` en Rust para seleccionar herramientas, planificar, instalar dependencias ausentes, configurar y verificar el equipo.
- Soportar exclusivamente Ubuntu y macOS, detectando versión, arquitectura, gestores y rutas reales, con compatibilidad explícita por módulo.
- Descubrir módulos declarativos `module.toml` con configuración común y ajustes por sistema; añadir una herramienta estándar no requerirá modificar Rust.
- Incorporar las configuraciones de Neovim, Ghostty, Alacritty, Kitty, Zsh, tmux, Zellij, Yazi, Zed y OpenCode, junto con sus dependencias y fuentes necesarias.
- Separar actualización de configuraciones, actualización de aplicaciones y actualización del CLI.
- Añadir ejecución idempotente, vista previa sin efectos, respaldos restaurables, detección de conflictos locales, diagnóstico y recuperación tras fallos.
- Distribuir ejecutables precompilados mediante un instalador shell mínimo, sin exigir Rust en el equipo destino.
- Desarrollar con ciclos red/green/refactor, pruebas aisladas y verificación nativa en ambas plataformas; no introducir comentarios en código de producción ni pruebas.
- **BREAKING**: trasladar las carpetas de configuración a módulos y sustituir el comportamiento del instalador actual. Conservar `install.sh` como punto de entrada al nuevo CLI y documentar la migración, incluidos los enlaces Zsh temporales rotos.

## Capabilities

### New Capabilities

- `module-catalog`: descubrimiento, validación, dependencias y compatibilidad de módulos declarativos.
- `environment-setup`: detección del equipo, selección, planificación e instalación verificable e idempotente.
- `config-lifecycle`: composición por plataforma, aplicación, actualización, conflictos, respaldos y restauración de configuraciones.
- `cli-distribution`: instalación y actualización del ejecutable, diagnóstico y contrato de comandos.

### Modified Capabilities

Ninguna; no existen especificaciones previas.

## Impact

Afecta a `install.sh`, las carpetas actuales de dotfiles, README, un nuevo proyecto Cargo, catálogo de módulos y automatización de pruebas/publicación. Los adaptadores invocarán gestores o instaladores aprobados por módulo, con privilegios solo para operaciones que los necesiten. Credenciales y personalizaciones locales deberán preservarse; autenticar servicios externos no forma parte de la instalación automática.

Quedan fuera Windows, otras distribuciones Linux, desinstalación de aplicaciones, reversión de paquetes del sistema y una interfaz gráfica. El nombre `gabriel` es provisional. Las versiones mínimas y la matriz de cada herramienta se fijarán con evidencia de instalación durante la implementación, sin prometer compatibilidad universal.
