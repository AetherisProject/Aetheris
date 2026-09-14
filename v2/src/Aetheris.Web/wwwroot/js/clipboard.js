// clipboard.js - Fallback clipboard implementation
window.AetherisClipboard = window.AetherisClipboard || {};

window.AetherisClipboard.copy = function(text) {
    // Try modern clipboard API first
    if (navigator.clipboard && navigator.clipboard.writeText) {
        return navigator.clipboard.writeText(text);
    }
    
    // Fallback for older browsers
    const textarea = document.createElement('textarea');
    textarea.value = text;
    textarea.style.position = 'fixed';
    textarea.style.opacity = '0';
    textarea.style.pointerEvents = 'none';
    textarea.style.zIndex = '-1000';
    document.body.appendChild(textarea);
    textarea.select();
    
    try {
        const success = document.execCommand('copy');
        if (!success) {
            console.warn('Clipboard copy failed');
        }
        return Promise.resolve(success);
    } catch (err) {
        console.error('Clipboard copy error:', err);
        return Promise.reject(err);
    } finally {
        document.body.removeChild(textarea);
    }
};

// Also expose on AetherisCrypto for convenience
window.AetherisCrypto = window.AetherisCrypto || {};
window.AetherisCrypto.copyToClipboard = window.AetherisClipboard.copy;
