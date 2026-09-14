// argon2.js - Argon2id wrapper using libsodium.js
// Load libsodium.js from CDN
const sodiumScript = document.createElement('script');
sodiumScript.src = 'https://cdnjs.cloudflare.com/ajax/libs/sodium.js/0.7.13/sodium.min.js';
sodiumScript.async = true;
document.head.appendChild(sodiumScript);

// Queue of callbacks waiting for sodium to load
const sodiumReadyCallbacks = [];
let sodiumReady = false;

sodiumScript.onload = () => {
    sodiumReady = true;
    while (sodiumReadyCallbacks.length > 0) {
        const cb = sodiumReadyCallbacks.shift();
        cb();
    }
};

// Ensure sodium is loaded before using it
function ensureSodium(callback) {
    if (sodiumReady) {
        callback();
    } else {
        sodiumReadyCallbacks.push(callback);
    }
}

// Argon2id hashing
// params: { timeCost, memoryCost, parallelism, hashLength, salt, secret }
async function argon2id(password, salt, timeCost = 3, memoryCost = 65536, parallelism = 1, hashLength = 32) {
    return new Promise((resolve, reject) => {
        ensureSodium(async () => {
            try {
                // Convert string password to Uint8Array
                const passwordBytes = new TextEncoder().encode(password);
                const saltBytes = new TextEncoder().encode(salt);
                
                const hashBytes = await window.sodium.crypto_pwhash(
                    hashLength,
                    passwordBytes,
                    saltBytes,
                    timeCost,
                    memoryCost,
                    parallelism,
                    window.sodium.crypto_pwhash_ALG_ARGON2ID13
                );
                
                // Convert to hex string
                const hashHex = window.sodium.to_hex(hashBytes);
                resolve(hashHex);
            } catch (error) {
                reject(error);
            }
        });
    });
}

// Derive a key from password using Argon2id
async function deriveKey(password, salt, keyLength = 32) {
    return argon2id(password, salt, 3, 65536, 1, keyLength);
}

// Export for JS interop
export { argon2id, deriveKey, ensureSodium };

// Also expose on window for direct access
window.AetherisCrypto = window.AetherisCrypto || {};
window.AetherisCrypto.argon2id = argon2id;
window.AetherisCrypto.deriveKey = deriveKey;
