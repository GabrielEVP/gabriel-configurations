## 1. Base de trabajo y compatibilidad

- [x] 1.1 Inventariar los diez módulos iniciales, dependencias activas, rutas personales, fuentes, secretos y referencias a carpetas actuales; registrar destinos y diferencias Ubuntu/macOS sin exponer valores sensibles.
- [x] 1.2 Verificar documentación oficial de proveedores y fijar matriz de versiones/arquitecturas y recetas por módulo, incluyendo prerrequisitos de Homebrew y restricciones gráficas.
- [x] 1.3 Crear paquete Cargo con biblioteca y CLI, fijar toolchain/dependencias y configurar fmt, clippy y test; documentar convención sin comentarios nuevos y evidencia red/green/refactor por incremento.

## 2. Detección y catálogo mediante TDD

- [x] 2.1 Red/green/refactor: detectar Ubuntu/macOS, versión y arquitectura mediante fixtures; rechazar otros sistemas sin mutaciones.
- [x] 2.2 Red/green/refactor: resolver HOME, XDG y prefijo Homebrew con rutas con espacios y ejecutables fuera de PATH.
- [x] 2.3 Red/green/refactor: cargar module.toml versionado y descubrir un nuevo módulo sin cambiar el ejecutable.
- [x] 2.4 Red/green/refactor: rechazar duplicados, esquemas desconocidos, dependencias ausentes y ciclos con errores localizables.
- [x] 2.5 Red/green/refactor: validar proveedores, restricciones, variables, escapes de rutas/enlaces y colisiones de destinos.

## 3. Planificación y comandos básicos

- [x] 3.1 Red/green/refactor: generar orden estable del grafo, deduplicar dependencias y distinguir presente, ausente, insuficiente e incompatible.
- [x] 3.2 Red/green/refactor: implementar setup interactivo, selección explícita con install, perfil personal y --all con exclusiones visibles.
- [x] 3.3 Red/green/refactor: implementar --dry-run sin descargas, caché, archivos, elevación ni inicialización de estado; cubrir catálogo local ausente.
- [x] 3.4 Red/green/refactor: implementar ayuda, versión, códigos 0/1/2, --yes y rechazo de setup sin selección en entornos no interactivos.

## 4. Proveedores y ejecución

- [x] 4.1 Red/green/refactor: crear ejecutor sustituible de procesos con argumentos separados, control de errores y redacción de secretos.
- [x] 4.2 Red/green/refactor: implementar proveedor APT y elevación acotada usando procesos simulados, incluyendo privilegios denegados.
- [x] 4.3 Red/green/refactor: implementar instalación/detección de Homebrew y proveedores fórmula/cask, actualizando PATH del proceso sin duplicar instalaciones.
- [x] 4.4 Red/green/refactor: implementar descarga verificada y extracción confinada para artefactos/instaladores versionados; cubrir corrupción y rutas de archivo maliciosas.
- [x] 4.5 Red/green/refactor: verificar versiones tras instalar, bloquear dependientes fallidos, continuar ramas independientes y replanificar al reintentar.

## 5. Composición y primera aplicación

- [x] 5.1 Red/green/refactor: componer common con ajustes Ubuntu/macOS y plantillas explícitas; probar prioridad, variables ausentes y valores que necesitan escape.
- [x] 5.2 Red/green/refactor: respaldar archivos/enlaces existentes y copiar únicamente destinos gestionados en almacenamiento persistente, preservando archivos ajenos.
- [x] 5.3 Red/green/refactor: implementar estado versionado, bloqueo exclusivo y rechazo de esquemas incompatibles antes de mutaciones.
- [x] 5.4 Red/green/refactor: implementar diario por módulo, preparación previa, commit, rollback y recuperación con fallos inyectados entre escrituras.
- [x] 5.5 Red/green/refactor: completar un recorrido vertical con tmux, desde selección hasta verificación y segunda ejecución sin cambios.

## 6. Actualizaciones y restauración

- [x] 6.1 Red/green/refactor: implementar tabla B/L/N con altas, modificaciones y eliminaciones; bloquear el módulo completo ante conflicto.
- [ ] 6.2 Red/green/refactor: implementar config update con revisión remota verificada y --source local, preservando la revisión activa ante errores de descarga/esquema.
- [x] 6.3 Red/green/refactor: bloquear nuevas configuraciones con dependencias ausentes sin instalar aplicaciones implícitamente.
- [x] 6.4 Red/green/refactor: implementar config resolve local/incoming, revalidación de cambios concurrentes, actualización de base y rechazo de resolución implícita con --yes.
- [x] 6.5 Red/green/refactor: implementar config restore por transacción, restauración del seguimiento y conflictos por cambios posteriores sin revertir paquetes.
- [x] 6.6 Red/green/refactor: implementar upgrade mediante proveedor conocido, mostrando dependencias afectadas y rechazando instalaciones de origen indeterminado.

## 7. Migración del catálogo personal

- [ ] 7.1 Migrar Zsh con pruebas previas de renderizado/arranque por sistema: rutas Homebrew, Oh My Zsh, fzf/fd, zoxide, atuin, carapace, plugins y utilidades GNU/macOS.
- [ ] 7.2 Migrar Neovim con pruebas previas para rutas Node, dependencias activas y verificación headless; conservar recursos y licencias necesarios.
- [x] 7.3 Migrar Ghostty, Alacritty y Kitty con fixtures por plataforma, módulo de fuente y validaciones de configuración disponibles.
- [x] 7.4 Migrar Zellij y Yazi con pruebas previas de dependencias y renderizado, completando el catálogo de terminal junto con tmux.
- [x] 7.5 Migrar Zed y OpenCode con pruebas previas de preservación de campos locales, credenciales y diagnóstico de autenticación pendiente.
- [ ] 7.6 Probar adopción heredada y reparación de enlaces Zsh temporales rotos; actualizar referencias a carpetas migradas sin mantener dos fuentes de verdad.

## 8. Diagnóstico y distribución

- [x] 8.1 Red/green/refactor: implementar status y doctor de solo lectura para versiones, dependencias, archivos, enlaces y transacciones pendientes.
- [x] 8.2 Red/green/refactor: implementar self update verificado con prueba de arranque y recuperación de binario anterior, sin modificar apps/configuraciones.
- [x] 8.3 Preparar empaquetado de binarios y catálogo con checksums, metadatos de compatibilidad y pipelines de release para las combinaciones verificadas.
- [ ] 8.4 Probar antes de sustituir install.sh: máquina sin Rust/git/rsync, utilidades mínimas ausentes, arquitectura incompatible, descarga corrupta y ejecutable previo preservado.
- [x] 8.5 Integrar bootstrap con artefactos de prueba y secuencia de publicación, evitando apuntar el instalador a releases inexistentes.

## 9. Verificación y documentación

- [ ] 9.1 Ejecutar fmt, clippy y suites unitarias/integración en runners nativos Ubuntu/macOS; revisar ausencia de comentarios nuevos en código propio.
- [ ] 9.2 Ejecutar instalación real en entornos desechables de la matriz: limpia, selección parcial, --all compatible, segunda ejecución y fallo recuperable; registrar evidencia y límites.
- [ ] 9.3 Verificar integración de terminales/fuentes en entornos gráficos admitidos y Neovim/Zsh sin rutas de la otra plataforma.
- [ ] 9.4 Ejecutar aceptación de actualización local/remota, credenciales preservadas, conflicto, resolución, eliminación gestionada, caída, restauración y actualización del CLI.
- [x] 9.5 Escribir guía de uso, migración y creación de módulos con un ejemplo completo sin cambios Rust, además de notas de aprendizaje ligadas a los incrementos.
- [ ] 9.6 Cruzar todos los escenarios de specs con evidencia, validar OpenSpec y dejar preparada la implementación para revisión sin publicar automáticamente.
