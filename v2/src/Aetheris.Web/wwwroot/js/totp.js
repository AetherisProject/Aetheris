// totp.js - RFC 6238 TOTP implementation
// Base32 decoding for TOTP secrets
function base32Decode(secret) {
    const alphabet = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567';
    let buffer = 0;
    let bufferSize = 0;
    const result = [];
    
    for (let i = 0; i < secret.length; i++) {
        const char = secret[i].toUpperCase();
        const index = alphabet.indexOf(char);
        if (index === -1) continue;
        
        buffer = (buffer << 5) | index;
        bufferSize += 5;
        
        if (bufferSize >= 8) {
            bufferSize -= 8;
            result.push((buffer >>> bufferSize) & 0xFF);
        }
    }
    
    return new Uint8Array(result);
}

// HMAC-SHA1 implementation using WebCrypto API
async function hmacSha1(key, message) {
    // Use WebCrypto if available
    if (window.crypto && window.crypto.subtle) {
        const cryptoKey = await window.crypto.subtle.importKey(
            'raw',
            key,
            { name: 'HMAC', hash: 'SHA-1' },
            false,
            ['sign']
        );
        
        const signature = await window.crypto.subtle.sign(
            'HMAC',
            cryptoKey,
            message
        );
        
        return new Uint8Array(signature);
    }
    
    // Fallback to sodium if available
    if (window.sodium && window.sodium.ready) {
        return await window.sodium.crypto_auth_hmacsha1(message, key);
    }
    
    throw new Error('No HMAC-SHA1 implementation available');
}

// Generate TOTP code (RFC 6238)
// secret: Base32-encoded secret key
// Returns: 6-digit TOTP code
window.AetherisTotp = window.AetherisTotp || {};

window.AetherisTotp.generate = async function(secret) {
    try {
        const key = base32Decode(secret);
        const epoch = Math.floor(Date.now() / 1000);
        const timeBytes = new Uint8Array(8);
        
        // Big-endian encoding of time (RFC 6238 uses floor(unix_time / 30) for 30-second windows)
        const timeWindow = Math.floor(epoch / 30);
        for (let i = 7; i >= 0; i--) {
            timeBytes[i] = timeWindow & 0xFF;
            timeWindow >>>= 8;
        }
        
        const hmac = await hmacSha1(key, timeBytes);
        
        // Dynamic truncation (RFC 4226)
        const offset = hmac[hmac.length - 1] & 0x0F;
        const truncated = (
            ((hmac[offset] & 0x7F) << 24) |
            (hmac[offset + 1] << 16) |
            (hmac[offset + 2] << 8) |
            hmac[offset + 3]
        );
        
        // Modulo to get 6-digit code (RFC 6238)
        const code = truncated % 1000000;
        return code.toString().padStart(6, '0');
    } catch (error) {
        console.error('TOTP generation error:', error);
        return null;
    }
};

// Also expose on AetherisCrypto for convenience
window.AetherisCrypto = window.AetherisCrypto || {};
window.AetherisCrypto.generateTotp = window.AetherisTotp.generate;
