---
layout: default
title: "password vault"
date: 2025-11-23
---
<div style="margin: 40px 0;"></div>
[< volver](/)
<div style="margin: 40px 0;"></div>
# 🔐 Password Vault

Gestor de contraseñas que construí en Ruby usando cifrado militar-grade AES-256. Arquitectura offline-first: sin cloud, sin telemetría, control total sobre tus secretos.
Construí este password manager en Ruby porque necesitaba algo que pudiera auditar completamente. 
<div style="margin: 10px 0;"></div>
Implementé cifrado AES-256-CBC con PBKDF2 key derivation, usando la biblioteca OpenSSL de Ruby. La arquitectura es simple: zero-knowledge, offline-first, sin dependencias externas. 
Elegí Ruby por su stdlib robusta para criptografía y porque quería demostrar que no necesitas frameworks complejos para manejar secretos de forma segura.


<img src="/assets/vault.png" alt="passwordmanager" style="max-width: 90%; width: 400px; display: block; margin: 40px auto; filter: drop-shadow(0 0 30px #ff1493);">


## → el problema

Las contraseñas débiles son la puerta de entrada más común para ataques. Los gestores comerciales están bien, pero ¿confías realmente en que tus secretos estén en la nube de alguien más?

## → la solución

Un password manager local, encriptado, de código abierto. Sin cloud, sin telemetría, sin bullshit. Tus contraseñas viven en tu máquina, cifradas con AES-256-CBC.

## → características

**Seguridad**
- Cifrado AES-256-CBC (estándar militar)
- Derivación de clave PBKDF2 con 100,000 iteraciones
- Salt y IV aleatorios por cada cifrado
- Master password que nunca se almacena

**Funcionalidad**
- Agregar/obtener/listar contraseñas
- Generador de contraseñas fuertes
- Copy to clipboard automático (macOS)
- Export de vault encriptado
- Interface CLI con colores

## → cómo funciona

### Encriptación

Cuando creas tu vault, el sistema:

1. **Deriva una clave** desde tu master password usando PBKDF2
   - 100,000 iteraciones SHA-256
   - Salt aleatorio de 16 bytes
   
2. **Cifra tus datos** con AES-256-CBC
   - IV (Initialization Vector) aleatorio
   - Cada entrada es un objeto JSON encriptado
   
3. **Almacena** en `~/.password_vault.enc`
   - Formato: `[salt][iv][encrypted_data]`
   - Base64 encoded

### Uso diario
```bash
ruby vault.rb

# Primera vez: crea master password
# Luego: unlock con tu master password

[1] Add password
[2] Get password  
[3] List all
[4] Generate strong password
[5] Delete password
[0] Exit
```

## → instalación
```bash
# Clonar
git clone
https://github.com/medusahra/password-vault.git
cd password-vault

# Instalar dependencia
gem install colorize

# Ejecutar
ruby vault.rb
```

## → arquitectura

**Componentes principales:**

- `encrypt_data()` - Cifrado AES-256-CBC con PBKDF2
- `decrypt_data()` - Descifrado con validación
- `generate_password()` - Generador criptográficamente seguro
- `save_vault()` - Persistencia encriptada

**Stack:**
- Ruby stdlib (OpenSSL, JSON, SecureRandom)
- Colorize gem (estética terminal)

## → seguridad

**✓ Lo que hace bien:**
- Cifrado fuerte (AES-256)
- Key derivation robusta (PBKDF2)
- Salt/IV aleatorios
- No almacena master password

**⚠ Consideraciones:**
- Si pierdes la master password, perdiste todo (no hay recovery)
- Local only (no sync entre dispositivos)
- Clipboard puede ser vulnerable a keyloggers

## → filosofía

Este proyecto no pretende competir con 1Password o Bitwarden. Es una herramienta educativa y funcional para entender:

- Cómo funciona la encriptación simétrica
- Key derivation functions
- Gestión segura de secretos
- CLI design

Y de paso, tener un vault que puedas auditar línea por línea.

*"The only system you can truly trust is one you can read and understand."*

---
<div style="margin: 20px 0;"></div>

**Código:** [github.com/medusahra/password-vault](https://github.com/medusahra/password-vault)

**Stack:** Ruby · OpenSSL · AES-256-CBC

**Inspiración:** OWASP security principles, cypherpunk ethics, offline-first security
<div style="margin: 10px 0;"></div>
---

<div style="margin: 40px 0;"></div>




[< volver al inicio](/)

