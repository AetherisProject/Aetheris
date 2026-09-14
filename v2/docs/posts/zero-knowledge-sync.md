# One PWA, every platform: how the vault sync stays zero-knowledge

> **Status:** Draft for dev.to / HN
> **Author:** Aetheris Team
> **Date:** September 2026
> **Tags:** #pwa #webassembly #cryptography #sync #zeroknowledge

---

## The Problem: Sync Without Trust

I have a confession to make: I'm paranoid about my API keys.

Not just a little paranoid - **a lot** paranoid. I've seen what happens when keys get leaked. I've seen the bills. I've seen the panic.

So when I built Aetheris, I had a simple requirement: **No one but me should ever be able to read my keys. Not even the sync server.**

This is the story of how I built a **zero-knowledge sync system** for a Progressive Web App (PWA) that works across every platform.

---

## What is Zero-Knowledge?

Zero-knowledge means the server **never sees your plaintext data**. It only stores and forwards encrypted blobs. Even if someone:
- Hacks the server
- Gets a court order for the data
- Works at the hosting company

They still can't read your keys.

### Traditional vs Zero-Knowledge

```
TRADITIONAL SYNC:
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Device A   │────>│    Server    │────>│   Device B   │
│   Plaintext  │     │   Plaintext  │     │   Plaintext  │
└─────────────┘     └─────────────┘     └─────────────┘
                    ❌ Server can read everything

ZERO-KNOWLEDGE SYNC:
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Device A   │────>│    Server    │────>│   Device B   │
│  Encrypted   │     │  Encrypted   │     │  Encrypted   │
└─────────────┘     └─────────────┘     └─────────────┘
                    ✅ Server sees only encrypted blobs
```

---

## The Architecture

Aetheris uses a **client-side encryption** model with a **hosted hub** for sync:

```
┌─────────────────────────────────────────────────────────────┐
│                        Aetheris Ecosystem                         │
├─────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐      │
│  │   Desktop    │    │    Mobile    │    │     Web     │      │
│  │   (Avalonia) │    │    (PWA)     │    │    (PWA)     │      │
│  └──────┬──────┘    └──────┬──────┘    └──────┬──────┘      │
│         │                  │                  │               │
│         └──────────────────┼──────────────────┘               │
│                            │                                    │
│                            ▼                                    │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │                    Local Vault                            │  │
│  │  - Encrypted with master password (Argon2id)             │  │
│  │  - Stored locally on each device                          │  │
│  │  - Never leaves device in plaintext                       │  │
│  └─────────────────────────────────────────────────────────┘  │
│                            │                                    │
│                            ▼                                    │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │                    Sync Key                                 │  │
│  │  - Random 32-byte key                                     │  │
│  │  - Encrypted with master key                              │  │
│  │  - Used to encrypt vault for sync                         │  │
│  └─────────────────────────────────────────────────────────┘  │
│                            │                                    │
│                            ▼                                    │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │                    Encrypted Vault Blob                     │  │
│  │  - Vault encrypted with sync key                           │  │
│  │  - Sync key encrypted with master key                     │  │
│  │  - Uploaded to hosted hub                                  │  │
│  └─────────────────────────────────────────────────────────┘  │
│                            │                                    │
│                            ▼                                    │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │                    Hosted Hub                              │  │
│  │  - Stores encrypted blobs                                  │  │
│  │  - No access to plaintext keys                            │  │
│  │  - LemonSqueezy for payments                              │  │
│  └─────────────────────────────────────────────────────────┘  │
│                                                                  │
└─────────────────────────────────────────────────────────────┘
```

---

## The PWA: One App, Every Platform

### Why PWA?

| Platform | PWA Support | Native Feel | Offline | Installable |
|----------|-------------|-------------|---------|-------------|
| Desktop (Chrome) | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |
| Desktop (Edge) | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |
| Desktop (Firefox) | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |
| Desktop (Safari) | ✅ Yes | ✅ Yes | ✅ Yes | ❌ No |
| Mobile (Android) | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |
| Mobile (iOS) | ✅ Yes | ⚠️ Limited | ✅ Yes | ❌ No |
| Tablet | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |

**PWA gives us 90% of native functionality with 10% of the development effort.**

### PWA Implementation

```html
<!-- manifest.json -->
{
  "name": "Aetheris",
  "short_name": "Aetheris",
  "description": "Your LLM keys never live in .env again",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#090D16",
  "theme_color": "#6366F1",
  "icons": [
    {
      "src": "icons/icon-192x192.png",
      "sizes": "192x192",
      "type": "image/png"
    },
    {
      "src": "icons/icon-512x512.png",
      "sizes": "512x512",
      "type": "image/png"
    }
  ]
}
```

```javascript
// service-worker.js
const CACHE_NAME = 'aetheris-v2';
const ASSETS = [
  '/',
  '/index.html',
  '/css/style.css',
  '/js/main.js',
  // ...
];

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(cache => cache.addAll(ASSETS))
  );
});

self.addEventListener('fetch', (event) => {
  event.respondWith(
    caches.match(event.request)
      .then(response => response || fetch(event.request))
  );
});
```

### Responsive Design

```css
/* Mobile-first approach */
:root {
  --bg: #090D16;
  --primary: #6366F1;
  /* ... */
}

/* Desktop */
@media (min-width: 1024px) {
  .app {
    display: grid;
    grid-template-columns: 240px 1fr;
  }
}

/* Tablet */
@media (min-width: 768px) and (max-width: 1023px) {
  .app {
    display: flex;
    flex-direction: column;
  }
}

/* Mobile */
@media (max-width: 767px) {
  .app {
    font-size: 14px;
  }
  
  .sidebar {
    display: none;
  }
}
```

---

## The Sync Protocol

### Step 1: Device Registration

```
Device → Hub: POST /devices
{
  "device_id": "abc123...",
  "device_name": "My Laptop",
  "public_key": "..."  // For future E2E messaging
}

Hub → Device: 201 Created
{
  "device_id": "abc123...",
  "created_at": "2026-09-14T10:00:00Z"
}
```

### Step 2: Vault Encryption for Sync

```csharp
// On Device A
public async Task SyncVaultAsync(string masterPassword)
{
    // 1. Derive master key from password
    var masterKey = DeriveKey(masterPassword, vaultSalt);
    
    // 2. Generate or load sync key
    var syncKey = await LoadOrCreateSyncKey(masterKey);
    
    // 3. Encrypt vault with sync key
    var encryptedVault = EncryptVault(vaultData, syncKey);
    
    // 4. Encrypt sync key with master key
    var encryptedSyncKey = EncryptSyncKey(syncKey, masterKey);
    
    // 5. Upload to hub
    await hubClient.UploadVault(
        deviceId,
        encryptedVault,
        encryptedSyncKey,
        vaultVersion
    );
}
```

### Step 3: Vault Download and Decryption

```csharp
// On Device B
public async Task DownloadVaultAsync(string masterPassword)
{
    // 1. Derive master key from password
    var masterKey = DeriveKey(masterPassword, vaultSalt);
    
    // 2. Download encrypted data from hub
    var (encryptedVault, encryptedSyncKey, version) = 
        await hubClient.DownloadVault(deviceId);
    
    // 3. Decrypt sync key with master key
    var syncKey = DecryptSyncKey(encryptedSyncKey, masterKey);
    
    // 4. Decrypt vault with sync key
    var vaultData = DecryptVault(encryptedVault, syncKey);
    
    // 5. Save to local storage
    await SaveVault(vaultData, version);
}
```

### Step 4: Conflict Resolution

```csharp
public async Task ResolveConflictsAsync()
{
    // Get all device versions
    var versions = await hubClient.GetVaultVersions(deviceId);
    
    // If there are conflicts (multiple versions)
    if (versions.Count > 1)
    {
        // Strategy 1: Last write wins (with timestamp)
        var latest = versions.OrderByDescending(v => v.Timestamp).First();
        
        // Strategy 2: Manual merge (for advanced users)
        var merged = await ManualMerge(versions);
        
        // Strategy 3: Keep both, let user choose
        await PromptUserForResolution(versions);
    }
}
```

---

## The Encryption Layers

### Layer 1: Master Key Derivation

```csharp
// Argon2id for key derivation
var salt = GenerateRandomSalt(16);
var masterKey = Argon2id.Hash(
    password: masterPassword,
    salt: salt,
    iterations: 3,
    memorySize: 65536, // 64MB
    parallelism: 4,
    hashLength: 32
);
```

**Purpose:** Convert user's password into a cryptographic key.

**Security:** Memory-hard, resistant to GPU/ASIC attacks.

### Layer 2: Sync Key Encryption

```csharp
// Generate sync key
var syncKey = GenerateRandomBytes(32);

// Encrypt with master key
var (encryptedSyncKey, nonce, tag) = XChaCha20Poly1305.Encrypt(
    plaintext: syncKey,
    key: masterKey
);
```

**Purpose:** Protect the sync key with the user's master password.

**Security:** Only someone with the master password can access the sync key.

### Layer 3: Vault Encryption

```csharp
// Encrypt vault with sync key
var (encryptedVault, nonce, tag) = XChaCha20Poly1305.Encrypt(
    plaintext: vaultData,
    key: syncKey
);
```

**Purpose:** Protect the actual vault data.

**Security:** Even if someone gets the encrypted vault from the hub, they can't decrypt it without the sync key.

---

## The Hub: Dumb Storage, Smart Security

The hub is intentionally **dumb**. It doesn't:
- ❌ See plaintext keys
- ❌ Know what's in the vault
- ❌ Have access to master passwords
- ❌ Perform any decryption

It only:
- ✅ Stores encrypted blobs
- ✅ Forwards encrypted blobs to other devices
- ✅ Manages device registrations
- ✅ Handles versioning

### Hub API

```
POST /vaults/upload
{
  "device_id": "abc123",
  "vault_data": "<base64 encrypted vault>",
  "sync_key": "<base64 encrypted sync key>",
  "version": "2",
  "timestamp": "2026-09-14T10:00:00Z"
}

GET /vaults/download?device_id=abc123
{
  "vault_data": "<base64 encrypted vault>",
  "sync_key": "<base64 encrypted sync key>",
  "version": "2",
  "timestamp": "2026-09-14T10:00:00Z"
}

GET /vaults/versions?device_id=abc123
[
  {
    "version": "2",
    "timestamp": "2026-09-14T10:00:00Z",
    "device_name": "My Laptop"
  },
  {
    "version": "1",
    "timestamp": "2026-09-13T09:00:00Z",
    "device_name": "My Desktop"
  }
]
```

### Hub Implementation

```csharp
// Simplified hub controller
[ApiController]
[Route("vaults")]
public class VaultController : ControllerBase
{
    private readonly IBlobStorage _storage;
    
    [HttpPost("upload")]
    public async Task<IActionResult> UploadVault(
        [FromBody] VaultUploadRequest request)
    {
        // Validate device
        if (!await _deviceService.ValidateDevice(request.DeviceId))
        {
            return Unauthorized();
        }
        
        // Store encrypted blob
        await _storage.SaveBlob(
            container: request.DeviceId,
            name: $