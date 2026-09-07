## Purpose

Permitir ampliar el entorno mediante módulos declarativos independientes, verificando sus dependencias y compatibilidad antes de modificar el equipo.

## ADDED Requirements

### Requirement: Declarative module discovery
El CLI SHALL descubrir automáticamente las carpetas con module.toml bajo modules/. Cada manifiesto MUST declarar versión de esquema, identificador único, compatibilidad, detección, dependencias, instalación y archivos destino; los módulos solo de configuración podrán omitir instalación.

#### Scenario: Add a standard tool
- **WHEN** se añade una carpeta válida con un proveedor ya soportado
- **THEN** la herramienta aparece en la selección y puede planificarse sin recompilar ni modificar Rust

#### Scenario: Invalid catalog
- **WHEN** hay identificadores duplicados, esquema no soportado, dependencias inexistentes o ciclos
- **THEN** el CLI identifica el módulo y motivo y rechaza el plan antes de instalar o escribir configuraciones

### Requirement: Explicit platform compatibility
Cada módulo SHALL declarar proveedores y restricciones de sistema, versión, arquitectura y entorno gráfico cuando corresponda. Una selección explícita incompatible MUST fallar antes de ejecutar; --all SHALL mostrar las exclusiones y procesar solo módulos compatibles.

#### Scenario: Unsupported terminal
- **WHEN** el usuario selecciona una terminal sin proveedor compatible con su equipo
- **THEN** recibe el motivo y no comienza una instalación parcial

### Requirement: Configuration boundaries
Los módulos SHALL limitar fuentes a su carpeta y destinos al directorio personal o directorio de configuración del usuario. MUST rechazar escapes mediante rutas o enlaces, destinos compartidos entre módulos y variables desconocidas.

#### Scenario: Conflicting destination
- **WHEN** dos módulos producen el mismo archivo destino
- **THEN** el plan falla indicando ambos módulos y la ruta

### Requirement: Initial catalog completeness
El catálogo SHALL incluir nvim, ghostty, alacritty, kitty, zsh, tmux, zellij, yazi, zed y opencode, además de módulos reutilizables para las dependencias necesarias y la fuente configurada. Las integraciones con autenticación SHALL indicar el paso manual pendiente sin registrar secretos.

#### Scenario: Zsh dependencies
- **WHEN** se selecciona zsh
- **THEN** el plan incluye las dependencias requeridas por su configuración activa y distingue las integraciones opcionales desactivadas

