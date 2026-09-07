## Context

Ver proposal.md para motivación y alcance. El repositorio contiene dotfiles y un script Bash, sin proyecto Rust ni specs anteriores. El script depende de git/rsync, no instala aplicaciones y elimina el directorio al que enlaza Zsh. Zsh contiene rutas Linuxbrew fijas, opciones de utilidades GNU e inicializaciones obligatorias de herramientas externas. Neovim contiene su propia búsqueda de Node que deberá reconciliarse con las rutas detectadas.

El usuario ha elegido Rust para aprender, TDD y ausencia de comentarios en código. El diseño necesita separar lógica comprobable de operaciones reales sobre la máquina.

## Goals / Non-Goals

**Goals:** núcleo pequeño, módulos declarativos, operaciones observables, estado recuperable y aprendizaje incremental. Las pruebas deben demostrar comportamiento sin usar el HOME real ni gestores reales salvo verificaciones expresamente ejecutadas en entornos desechables.

**Non-Goals:** framework de plugins dinámicos, ejecución paralela de gestores, intérprete general de scripts, fusión semántica universal de TOML/JSON/Lua y transacciones sobre paquetes del sistema.

## Decisions

### 1. Un paquete Rust con biblioteca y ejecutable

Separar cli, domain, application e infrastructure como módulos del mismo paquete Cargo. domain contendrá plataforma, módulo, dependencias, plan y decisiones de actualización; application coordinará casos de uso; infrastructure implementará procesos, archivos, descargas y detección.

Usar struct, enum y Result para estados y errores. Introducir traits únicamente en límites que las pruebas necesiten sustituir, especialmente ejecutor de procesos, detección y fuente del catálogo. Usar filesystem real en directorios temporales para respaldos y escritura. No incorporar un runtime asíncrono hasta justificarlo.

Alternativas: un workspace con muchos crates añade navegación prematura; una implementación monolítica dificulta probar fallos sin tocar el equipo. No escribir comentarios nuevos en Rust, pruebas ni scripts; explicar decisiones y aprendizaje en documentación. Mantener licencias y avisos legales existentes de terceros al migrar archivos.

### 2. Catálogo independiente del binario

Estructura prevista:

```
Cargo.toml
src/
tests/
modules/<id>/
  module.toml
  common/
  ubuntu/
  macos/
profiles/personal.toml
```

El manifiesto v1 define id, schema_version, descripción, restricciones de plataforma, dependencias required/optional, proveedor por plataforma, detección con argumentos y reglas de versión, mapeos de archivos, variables tipadas y verificaciones. Un perfil enumera ids sin replicar módulos.

Los proveedores iniciales serán paquetes APT, fórmula/cask Homebrew y artefactos o instaladores oficiales versionados. Cada proveedor debe modelar detección, instalación, comprobación y actualización; si una herramienta necesita comportamiento fuera de esos contratos se añade un adaptador Rust específico. No permitir comandos shell arbitrarios interpolados desde manifiestos. Los instaladores oficiales se descargan y verifican antes de ejecutarse mediante argumentos explícitos.

Descubrir todas las carpetas, validar primero el catálogo completo, construir un grafo acíclico y ordenar de forma estable. La caché reside en almacenamiento persistente del usuario. El catálogo predeterminado proviene de una release identificable del repositorio; --source permite un checkout local para editar y probar sin publicar. No incorporar el catálogo exclusivamente dentro del ejecutable: eso obligaría a recompilar por cada carpeta nueva.

### 3. Compatibilidad verificable y proveedores explícitos

Detectar Ubuntu por os-release, macOS mediante consultas nativas, arquitectura y variables de entorno. Respetar XDG_CONFIG_HOME cuando exista; rutas de estado propias bajo XDG_STATE_HOME en Ubuntu y Application Support en macOS. Resolver Homebrew desde PATH y ubicaciones conocidas, consultando su prefijo.

En Ubuntu usar APT para prerrequisitos del sistema y el proveedor declarado para cada aplicación. En macOS usar Homebrew cuando lo requiera el módulo. Homebrew también es un módulo seleccionable y una dependencia instalable. No asumir que un cask macOS funciona en Ubuntu ni cambiar de proveedor silenciosamente.

Durante el primer trabajo de compatibilidad se fijarán versiones mínimas concretas y recetas con documentación oficial y pruebas. El núcleo contempla x86_64 y aarch64; solo se publicarán combinaciones verificadas. Cada módulo puede declarar una matriz más estrecha. Esto permite excluir una terminal en un entorno sin los requisitos gráficos, sin ocultar la exclusión.

Alternativa descartada: usar exclusivamente brew en ambos sistemas; no elimina las diferencias de aplicaciones gráficas ni los prerrequisitos del sistema.

### 4. Plan inmutable y ejecución secuencial

El plan lista acciones de instalar gestor, instalar/actualizar paquete insuficiente, preparar archivos, respaldar, aplicar y verificar. Incluye origen del catálogo, razones, privilegios y exclusiones. --dry-run solo usa comprobaciones de lectura y no inicializa caché o estado; sin catálogo local indica cómo obtenerlo, sin descargarlo.

El ejecutor revalida condiciones antes de mutar y solicita elevación solo para el comando que la necesita. Nunca ejecuta todo el CLI como root. Tras instalar actualiza PATH del proceso y comprueba el resultado. Continúa ramas independientes cuando falla una dependencia; bloquea las dependientes y termina con código 1.

Reejecutar significa volver a observar y planificar, no confiar ciegamente en un marcador histórico de instalación. Los gestores deben recibir argumentos separados, no cadenas construidas para shell. Registrar resultados con secretos redactados.

### 5. Configuración por archivos y plantillas limitadas

common es la base; el directorio del sistema reemplaza archivos por ruta relativa. Los archivos sin marca de plantilla se copian literalmente; las plantillas solo resuelven variables declaradas, con codificación específica para el formato de destino. No usar sustituciones globales de prefijos.

Copiar archivos gestionados al destino persistente en lugar de enlazar al checkout. Esto permite usar archivos temporales hermanos y renombrados dentro del filesystem destino. Los destinos son relativos a home/config con validación de traversal y de ancestros symlink. No reemplazar directorios completos ni borrar archivos ajenos.

Zsh separará opciones GNU/macOS, inicializaciones requeridas y opcionales. Auditar rutas personales, atajos, portapapeles, fuentes y arranque de tmux; no cambiar automáticamente el shell de login ni preferencias globales no declaradas. Node y otros ejecutables deberán usar rutas detectadas compatibles.

### 6. Base aplicada, conflictos y credenciales

Mantener por archivo la base upstream renderizada, hash del último contenido gestionado, revisión, propietario módulo y metadatos. Comparar B=base, L=local, N=nuevo:

| Condición | Acción |
|---|---|
| L = N | Aceptar sin escritura |
| L = B | Aplicar N |
| N = B | Conservar L |
| Otra | Conflicto, conservar L |

Tratar ausencia como un valor para detectar altas y eliminaciones. Un conflicto bloquea la transacción completa del módulo, pero no los módulos independientes. config resolve <module> --file <ruta> --use local/incoming opera sobre una revisión pendiente y revalida que el archivo no cambió desde la detección. Al resolver local, la base pasa a N y se registra que L conserva una divergencia intencionada.

Separar secretos en archivos locales excluidos del catálogo y permisos restrictivos. Para formatos sin inclusión de archivos, usar un adaptador de formato que preserve campos locales declarados; si no puede hacerlo, bloquear con conflicto. La primera adopción de archivos de credenciales existentes no puede reemplazarlos a ciegas. Nunca mostrar diffs con valores secretos. No prometer autenticación automática de servicios.

Alternativa descartada: sobrescribir siempre con un respaldo; conservaría una copia, pero rompería silenciosamente las personalizaciones activas.

### 7. Diario y recuperación por módulo

Un bloqueo exclusivo protege todas las operaciones mutantes. Antes de tocar un destino: crear una transacción identificada, respaldar contenido/tipo/permisos/enlace sin seguirlo y persistir el diario. Preparar todos los archivos y validarlos antes del primer reemplazo. Registrar cada paso; actualizar la base y marcar commit solo al completar el módulo.

Renombrar atómicamente cada archivo no convierte varios archivos en una transacción atómica. Ante fallo se revierte el módulo; ante caída, la siguiente mutación recupera primero la transacción pendiente. Detectar divergencias posteriores antes de restaurar y exigir resolución explícita para no destruir trabajo nuevo.

config restore <transaction-id> permite volver al estado previo, incluidos destinos antes ausentes, respaldando cambios posteriores autorizados y restaurando también el estado de seguimiento. No revierte instalaciones ni upgrades del sistema. No borrar respaldos automáticamente en esta versión.

### 8. Tres canales de actualización

config update obtiene el catálogo en staging, comprueba checksum/esquema, fija revisión y aplica comparación por archivo. También acepta --source local. Una nueva dependencia obligatoria bloquea ese módulo y remite a setup; no instala software implícitamente.

upgrade opera sobre el proveedor registrado o identificado inequívocamente y las aplicaciones seleccionadas; muestra efectos sobre dependencias. self update descarga un binario compatible, verifica checksum, comprueba arranque y sustituye conservando el anterior. Cada release publica metadatos de compatibilidad de catálogo/estado. Las migraciones de estado serán explícitas, con respaldo y pruebas; una versión desconocida se rechaza.

install.sh conserva su URL como bootstrap mínimo: requiere shell y utilidades mínimas documentadas, descarga binario y catálogo de release, verifica integridad y ejecuta/ofrece setup según argumentos y disponibilidad de TTY. No requiere git, rsync ni compilador. Los checksums detectan corrupción; no sustituyen la confianza en la fuente de publicación HTTPS.

### 9. TDD, aprendizaje y evidencia

Cada incremento de comportamiento debe demostrar una prueba fallida por el comportamiento ausente, implementación mínima y refactor con suite verde. Registrar esa evidencia en el avance de tareas; no basta escribir tests después.

Orden de aprendizaje: tipos y Result con detección; colecciones/grafos con planes; serde o equivalente para manifiestos tras evaluar dependencias; traits con proveedores simulados; ownership y filesystem con transacciones. Seleccionar versiones de crates al implementar y fijar Cargo.lock, sin depender de versiones supuestas en la propuesta.

Pruebas unitarias de decisiones, integración con HOME temporal y procesos simulados, y aceptación nativa en Ubuntu/macOS. Cubrir rutas con espacios, enlaces rotos, permisos, fallos de proceso, interrupción, secretos y repetición. Formato y lint con cargo fmt y cargo clippy; cargo test para comportamiento. Instalaciones reales solo en VM o runner desechable; contenedores Linux no demuestran integración gráfica ni macOS.

## Risks / Trade-offs

- Cambios en proveedores y disponibilidad de paquetes → recetas versionadas, compatibilidad declarada y smoke tests antes de publicar.
- Gestores con efectos sobre paquetes relacionados → mostrar el alcance conocido y no prometer rollback del sistema.
- Escrituras en múltiples destinos y fallos de energía → diario persistente, recuperación y pruebas de interrupción.
- Dotfiles con secretos o rutas personales → inventario antes de migrar, separación de campos locales y bloqueo ante ambigüedad.
- Tamaño del catálogo y diccionarios Neovim → medir artefactos, conservar recursos necesarios y evitar copias redundantes en releases.
- Alcance amplio para aprender Rust → implementar cortes verticales pequeños; primer recorrido completo con un módulo antes de migrar todos.
- Actualizaciones concurrentes fuera del CLI → volver a comparar antes de cada commit/restauración y no confiar solo en el lock propio.

## Migration Plan

1. Inventariar dotfiles y scripts que los referencian; fijar matriz y recetas verificadas.
2. Crear núcleo y primer módulo sin sustituir todavía el instalador existente.
3. Migrar módulos progresivamente, comparando archivos renderizados y comportamiento en ambas plataformas.
4. Detectar instalaciones heredadas sin adoptar como propios archivos desconocidos; respaldar destinos existentes y reparar enlaces Zsh rotos.
5. Preparar releases verificadas de binario y catálogo antes de cambiar install.sh a bootstrap.
6. Documentar comandos, migración, creación de módulos, variables, conflictos y restauración.
7. Validar instalación limpia, segunda ejecución, actualización y restauración en ambos sistemas. La publicación efectiva requerirá una instrucción posterior; esta propuesta no publica ni implementa.
