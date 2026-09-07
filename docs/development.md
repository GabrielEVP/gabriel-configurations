# Desarrollo

Toolchain: Rust 1.98.1, fijada en rust-toolchain.toml. El paquete contiene biblioteca y ejecutable; Cargo.lock forma parte del proyecto.

Comprobaciones:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Cada comportamiento se desarrolla con una prueba roja por funcionalidad ausente, implementación mínima y refactorización en verde. Los errores de entorno o compilador no sustituyen esa evidencia. No añadir comentarios al código propio ni a sus pruebas; explicar decisiones aquí y conservar avisos legales de terceros.

Unidades de revisión previstas: detección del equipo; catálogo y plan; proveedores; aplicación recuperable; actualizaciones; migración por módulo; distribución. Cada unidad conserva pruebas y documentación con su implementación. Se podrán agrupar en PRs encadenadas de alcance revisable cuando se solicite publicar.

## Evidencia TDD

| Incremento | Rojo | Verde | Refactor/verificación |
|---|---|---|---|
| Detección de plataforma | cargo test --test platform: 2 fallos por detección ausente | 5 pruebas pasan | cargo fmt y clippy sin advertencias |

| Rutas de usuario | 5 fallos por resolución ausente | 5 pruebas pasan | Sin creación de directorios; rutas con espacios |
| Catálogo y grafo | 5 fallos por carga/orden ausentes | 7 pruebas pasan | fmt y clippy; errores de manifiesto localizados |
| Procesos y Homebrew | 3 fallos por ejecución/consulta ausente | 4 pruebas pasan | Argumentos literales; errores sin credenciales |

| Planificación | 3 fallos por plan ausente | 4 pruebas pasan | Clasificación y exclusión transitiva |
| Composición | 3 fallos por composición ausente | 5 pruebas pasan | Plantillas, escapes y enlaces |
| Estado y actualización | 6 fallos por aplicación ausente | 8 pruebas pasan tras corregir adopción idéntica | Respaldos, bloqueo y recuperación de diario |

| CLI inicial | 3 fallos por ayuda/selección/plan ausentes | 3 pruebas pasan | dry-run sin HOME ni estado creados |

## Aprendizaje

Platform agrupa valores relacionados en una struct. OperatingSystem y Architecture son enums: impiden representar sistemas no admitidos como estados válidos. Result devuelve una plataforma válida o un error; los tests ejercitan ambas rutas sin consultar ni modificar el equipo.

## Integración continua

El usuario ha elegido GitHub Actions para verificar Ubuntu y macOS. .github/workflows/ci.yml ejecuta fmt, clippy y tests en ubuntu-24.04 y macos-15; las instalaciones y pruebas gráficas reales necesitan jobs de aceptación separados. La existencia del workflow no demuestra una ejecución remota correcta.
