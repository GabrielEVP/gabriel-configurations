## Purpose

Preparar automáticamente las herramientas seleccionadas en Ubuntu y macOS mediante planes verificables, repetibles y recuperables ante errores.

## ADDED Requirements

### Requirement: Environment detection
El CLI SHALL detectar Ubuntu o macOS, versión, arquitectura, directorio personal, rutas de configuración, gestores y ejecutables existentes. MUST rechazar otros sistemas antes de efectuar cambios y no asumir una ruta fija de Homebrew.

#### Scenario: Existing Homebrew outside PATH
- **WHEN** Homebrew existe en una ubicación admitida pero no está en PATH
- **THEN** el CLI lo identifica y utiliza su prefijo real sin instalar una segunda copia

#### Scenario: Unsupported operating system
- **WHEN** setup se ejecuta en otra distribución Linux
- **THEN** termina con diagnóstico de incompatibilidad sin modificar el equipo

### Requirement: Selection and deterministic planning
setup SHALL ofrecer selección interactiva; install SHALL aceptar identificadores explícitos; setup --all SHALL seleccionar todos los compatibles. El plan MUST ordenar dependencias, evitar duplicados y mostrar acciones, motivos, proveedores y privilegios necesarios. Sin terminal interactiva, setup sin selección MUST fallar con instrucciones.

#### Scenario: Shared dependency
- **WHEN** dos herramientas requieren la misma dependencia
- **THEN** aparece una sola acción para ella antes de ambas herramientas

### Requirement: Read-only preview
--dry-run SHALL producir el plan sin instalar, descargar, pedir privilegios ni escribir archivos, caché o estado. SHALL usar el catálogo local disponible y comunicar datos pendientes de comprobación remota.

#### Scenario: Preview fresh machine
- **WHEN** faltan Homebrew y las herramientas seleccionadas
- **THEN** se muestran las acciones previstas sin ejecutarlas

### Requirement: Automatic dependency installation
setup SHALL instalar los gestores y paquetes ausentes necesarios y comprobar sus versiones y disponibilidad después de cada instalación. SHALL reutilizar herramientas compatibles existentes. Una versión insuficiente MUST mostrarse como actualización necesaria en el plan; no se actualizarán herramientas compatibles ajenas a esa necesidad.

#### Scenario: Missing manager
- **WHEN** un módulo requiere Homebrew y no existe
- **THEN** el CLI instala sus prerrequisitos, instala Homebrew, actualiza el entorno del proceso y continúa verificando cada paso

#### Scenario: Privileges unavailable
- **WHEN** una instalación necesita elevación que no puede obtenerse
- **THEN** el CLI informa la operación bloqueada sin declarar la herramienta instalada

### Requirement: Idempotency and partial failure
Una segunda ejecución SHALL volver a comprobar el equipo y omitir acciones satisfechas. Cuando falla una acción MUST bloquear sus dependientes, preservar configuraciones existentes y permitir completar módulos independientes; el resultado global MUST indicar fallo parcial.

#### Scenario: Dependency fails
- **WHEN** falla la instalación de una dependencia de nvim
- **THEN** no se activa su configuración, se informa la causa y un nuevo setup reevalúa lo pendiente sin repetir pasos satisfechos

### Requirement: Application upgrades are explicit
upgrade SHALL actualizar únicamente las aplicaciones seleccionadas y las dependencias que exija su proveedor, mostrando estas últimas. No SHALL aplicar nuevas configuraciones ni actualizar el CLI.

#### Scenario: Existing externally managed installation
- **WHEN** una aplicación no puede asociarse a un proveedor de actualización conocido
- **THEN** upgrade informa que no puede actualizarla automáticamente y no instala una copia alternativa

