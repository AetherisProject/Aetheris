// crypto.js - XChaCha20-Poly1305 encryption/decryption wrapper using libsodium.js

// Ensure sodium is loaded
let sodiumReady = false;
const sodiumReadyCallbacks = [];

function ensureSodium(callback) {
    if (window.sodium && window.sodium.ready) {
        callback();
    } else if (sodiumReady) {
        callback();
    } else {
        sodiumReadyCallbacks.push(callback);
    }
}

// Initialize sodium if not already loaded
if (window.sodium) {
    window.sodium.onload = () => {
        sodiumReady = true;
        while (sodiumReadyCallbacks.length > 0) {
            const cb = sodiumReadyCallbacks.shift();
            cb();
        }
    };
}

// Generate a random nonce (24 bytes for XChaCha20-Poly1305)
function generateNonce() {
    return window.sodium.randombytes_buf(window.sodium.crypto_secretbox_NONCEBYTES);
}

// Generate a random key (32 bytes for XChaCha20-Poly1305)
function generateKey() {
    return window.sodium.randombytes_buf(window.sodium.crypto_secretbox_KEYBYTES);
}

// Encrypt plaintext using XChaCha20-Poly1305
// Returns { ciphertext: hex, nonce: hex }
async function encrypt(plaintext, key) {
    return new Promise((resolve, reject) => {
        ensureSodium(async () => {
            try {
                const plaintextBytes = new TextEncoder().encode(plaintext);
                const nonce = generateNonce();
                
                const ciphertext = await window.sodium.crypto_secretbox_easy(
                    plaintextBytes,
                    nonce,
                    key
                );
                
                resolve({
                    ciphertext: window.sodium.to_hex(ciphertext),
                    nonce: window.sodium.to_hex(nonce)
                });
            } catch (error) {
                reject(error);
            }
        });
    });
}

// Decrypt ciphertext using XChaCha20-Poly1305
// ciphertextObj: { ciphertext: hex, nonce: hex }
async function decrypt(ciphertextObj, key) {
    return new Promise((resolve, reject) => {
        ensureSodium(async () => {
            try {
                const ciphertext = window.sodium.from_hex(ciphertextObj.ciphertext);
                const nonce = window.sodium.from_hex(ciphertextObj.nonce);
                
                const plaintextBytes = await window.sodium.crypto_secretbox_open_easy(
                    ciphertext,
                    nonce,
                    key
                );
                
                if (!plaintextBytes) {
                    throw new Error('Decryption failed - ciphertext may be corrupted or key incorrect');
                }
                
                const plaintext = new TextDecoder().decode(plaintextBytes);
                resolve(plaintext);
            } catch (error) {
                reject(error);
            }
        });
    });
}

// Encrypt a JSON-serializable object
async function encryptObject(obj, key) {
    const json = JSON.stringify(obj);
    return encrypt(json, key);
}

// Decrypt to a JSON object
async function decryptObject(ciphertextObj, key) {
    const json = await decrypt(ciphertextObj, key);
    return JSON.parse(json);
}

// Generate a random salt
function generateSalt(length = 16) {
    return window.sodium.to_hex(window.sodium.randombytes_buf(length));
}

// Hash a string using SHA-256 (for key derivation verification)
async function sha256(str) {
    return new Promise((resolve, reject) => {
        ensureSodium(async () => {
            try {
                const bytes = new TextEncoder().encode(str);
                const hash = await window.sodium.crypto_hash_sha256(bytes);
                resolve(window.sodium.to_hex(hash));
            } catch (error) {
                reject(error);
            }
        });
    });
}

// Export for JS interop
export { 
    encrypt, 
    decrypt, 
    encryptObject, 
    decryptObject, 
    generateNonce, 
    generateKey, 
    generateSalt,
    sha256,
    ensureSodium 
};

// Also expose on window for direct access
window.AetherisCrypto = window.AetherisCrypto || {};
Object.assign(window.AetherisCrypto, {
    encrypt,
    decrypt,
    encryptObject,
    decryptObject,
    generateNonce,
    generateKey,
    generateSalt,
    sha256
});
