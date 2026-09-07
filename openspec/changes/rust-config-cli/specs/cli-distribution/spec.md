## Purpose

Entregar un CLI utilizable sin entorno Rust, con comandos claros, diagnóstico y actualizaciones independientes de las aplicaciones y sus configuraciones.

## ADDED Requirements

### Requirement: Prebuilt installation
install.sh SHALL detectar sistema y arquitectura, descargar el artefacto compatible de una versión publicada, verificar su checksum y colocarlo en una ruta persistente del usuario. MUST funcionar sin Rust, Cargo, git ni rsync instalados, declarando y comprobando las utilidades mínimas de descarga y extracción.

#### Scenario: Fresh supported machine
- **WHEN** se ejecuta install.sh con las utilidades mínimas disponibles y una release compatible
- **THEN** instala gabriel y ofrece setup sin compilar código

#### Scenario: Invalid download
- **WHEN** falla la descarga o el checksum no coincide
- **THEN** no reemplaza un ejecutable existente y muestra un error

### Requirement: Separate executable updates
self update SHALL actualizar únicamente el ejecutable a una release compatible y verificada, conservando el ejecutable previo hasta completar la sustitución. Una incompatibilidad de esquema MUST detectarse antes de modificar configuraciones o estado.

#### Scenario: Failed self update
- **WHEN** el nuevo ejecutable no supera la comprobación básica de versión y arranque
- **THEN** se conserva o recupera la versión anterior utilizable

### Requirement: Command contract and diagnostics
El CLI SHALL ofrecer setup, install, status, doctor, config update, config resolve, config restore, upgrade y self update, con ayuda y versión. status y doctor MUST ser de solo lectura. doctor SHALL comprobar ejecutables, requisitos de versión, dependencias, destinos, enlaces, variables y operaciones pendientes; MUST distinguir integraciones pendientes de autenticación de fallos de instalación.

#### Scenario: Broken configuration
- **WHEN** doctor encuentra un enlace roto o una dependencia obligatoria ausente
- **THEN** identifica módulo, causa y acción de recuperación sin modificar archivos

### Requirement: Automation outcomes
Los comandos SHALL devolver 0 en éxito, 1 en fallo operativo o parcial y 2 para argumentos, catálogo o compatibilidad inválidos. --yes SHALL omitir confirmaciones ordinarias únicamente para acciones explícitamente seleccionadas; MUST NOT omitir restricciones del sistema ni resolver conflictos.

#### Scenario: Noninteractive conflict
- **WHEN** config update --yes encuentra cambios divergentes
- **THEN** conserva archivos locales, muestra los conflictos y devuelve 1 sin esperar entrada

### Requirement: Versioned catalog compatibility
El ejecutable SHALL declarar las versiones de esquema de catálogo y estado que comprende. Las actualizaciones SHALL rechazar versiones incompatibles antes de aplicar cambios, indicando si requiere self update.

#### Scenario: New module schema
- **WHEN** el catálogo descargado requiere un esquema más reciente
- **THEN** permanece activa la revisión anterior y se informa cómo actualizar el CLI

