## Purpose

Mantener configuraciones adaptadas a cada plataforma con actualizaciones que preservan cambios locales y respaldos que permiten recuperar el estado anterior.

## ADDED Requirements

### Requirement: Platform composition
El CLI SHALL componer common/ con ubuntu/ o macos/, reemplazando archivos por ruta relativa y resolviendo únicamente variables declaradas con valores detectados. No SHALL mezclar arbitrariamente contenido estructurado ni activar integraciones sin sus requisitos.

#### Scenario: Platform override
- **WHEN** common y macos contienen el mismo archivo en un equipo macOS
- **THEN** se utiliza el archivo macos y los archivos comunes restantes

#### Scenario: Missing variable
- **WHEN** una plantilla necesita una ruta que no puede resolverse
- **THEN** la aplicación del módulo se bloquea antes de escribir sus archivos

### Requirement: Initial adoption and persistent installation
La primera aplicación SHALL respaldar destinos existentes, incluidos enlaces rotos, antes de reemplazarlos. SHALL preservar archivos no gestionados y mantener todos los archivos activos fuera de directorios temporales. Los secretos MUST permanecer locales y no aparecer en logs ni diferencias mostradas.

#### Scenario: Legacy broken shell link
- **WHEN** .zshrc apunta al clon temporal eliminado del instalador anterior
- **THEN** se conserva el enlace en el respaldo y se instala una configuración persistente válida

#### Scenario: Existing credential configuration
- **WHEN** un destino contiene credenciales o está declarado local-only
- **THEN** se conserva como dato local y cualquier adopción incompatible se presenta como conflicto sin sobrescribirlo automáticamente

### Requirement: Three-way configuration updates
config update SHALL obtener una revisión identificable del catálogo o usar una ruta local explícita, validarla y comparar por archivo la base aplicada, el contenido local y el nuevo contenido. Si local coincide con base SHALL aplicar nuevo; si nuevo coincide con base SHALL conservar local; si local coincide con nuevo SHALL aceptar sin conflicto; otras divergencias MUST conservar local y registrar conflicto. No SHALL actualizar aplicaciones ni el ejecutable.

#### Scenario: Local and upstream edits
- **WHEN** usuario y catálogo cambian un mismo archivo desde la base
- **THEN** el archivo local permanece intacto y el módulo queda pendiente de resolución explícita

#### Scenario: Removed managed file
- **WHEN** una nueva revisión elimina un archivo gestionado
- **THEN** solo se elimina automáticamente si su contenido local coincide con la base, respaldándolo; si cambió localmente se conserva y se informa conflicto

#### Scenario: New dependency during config update
- **WHEN** una nueva configuración exige una dependencia ausente
- **THEN** el módulo no se aplica y se indica ejecutar setup para resolverla

### Requirement: Conflict resolution
El CLI SHALL permitir elegir explícitamente local o incoming para cada conflicto mediante config resolve, con respaldo antes de reemplazar. --yes MUST NOT resolver conflictos implícitamente.

#### Scenario: Keep local
- **WHEN** el usuario resuelve un conflicto eligiendo local
- **THEN** se conserva el archivo y se registra la nueva base upstream para futuras comparaciones

### Requirement: Recoverable writes and restoration
Las mutaciones SHALL disponer de respaldo y registro persistente antes de reemplazar archivos; el estado aplicado solo avanzará tras completar el módulo. Ejecuciones concurrentes MUST impedirse y una interrupción MUST detectarse antes de otra mutación. config restore SHALL restaurar una transacción seleccionada sin revertir paquetes del sistema.

#### Scenario: Interrupted application
- **WHEN** el proceso termina entre dos escrituras de un módulo
- **THEN** la siguiente ejecución detecta la transacción incompleta y recupera o restaura sus archivos antes de comenzar otro cambio

#### Scenario: Restore after later local edits
- **WHEN** restaurar sobrescribiría modificaciones realizadas después de la transacción
- **THEN** se informa conflicto y solo se reemplazan tras selección explícita con un respaldo adicional

