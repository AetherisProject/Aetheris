// Aetheris Firefox Extension - Background Script
// Handles extension lifecycle and message routing

// Extension ID
const EXTENSION_ID = 'aetheris@merlin-tribukait.com';

// Initialize extension
chrome.runtime.onInstalled.addListener(function(details) {
    if (details.reason === 'install') {
        console.log('Aetheris Firefox extension installed');
        // Set default settings
        chrome.storage.local.set({
            vaultUnlocked: false,
            autoFillEnabled: true,
            theme: 'dark'
        });
    } else if (details.reason === 'update') {
        console.log('Aetheris Firefox extension updated to version', chrome.runtime.getManifest().version);
    }
});

// Message listener
chrome.runtime.onMessage.addListener(function(request, sender, sendResponse) {
    switch (request.action) {
        case 'autofill':
            handleAutoFill(request, sender, sendResponse);
            break;
        case 'generatePassword':
            handleGeneratePassword(request, sender, sendResponse);
            break;
        case 'unlock':
            handleUnlock(request, sender, sendResponse);
            break;
        case 'lock':
            handleLock(request, sender, sendResponse);
            break;
        default:
            sendResponse({error: 'Unknown action'});
    }
    return true; // Required for async sendResponse
});

// Handle auto-fill
function handleAutoFill(request, sender, sendResponse) {
    // Get current tab
    chrome.tabs.query({active: true, currentWindow: true}, function(tabs) {
        if (tabs.length > 0) {
            chrome.tabs.sendMessage(tabs[0].id, {action: 'autofill'});
            sendResponse({success: true});
        } else {
            sendResponse({error: 'No active tab'});
        }
    });
}

// Handle password generation
function handleGeneratePassword(request, sender, sendResponse) {
    // Generate a random password
    const password = generateRandomPassword(request.length || 16);
    sendResponse({password: password});
}

// Handle unlock
function handleUnlock(request, sender, sendResponse) {
    // Verify master password
    if (request.masterPassword) {
        // In a real implementation, verify against stored hash
        chrome.storage.local.set({vaultUnlocked: true});
        sendResponse({success: true});
    } else {
        sendResponse({error: 'Master password required'});
    }
}

// Handle lock
function handleLock(request, sender, sendResponse) {
    chrome.storage.local.set({vaultUnlocked: false});
    sendResponse({success: true});
}

// Generate random password
function generateRandomPassword(length) {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()';
    let password = '';
    for (let i = 0; i < length; i++) {
        password += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return password;
}

// Content script communication
chrome.runtime.onMessageExternal.addListener(function(request, sender, sendResponse) {
    // Handle external messages (from web pages)
    if (request.action === 'autofillRequest') {
        // Verify sender is trusted
        if (sender.url.startsWith('https://aetheris.com') || sender.url.startsWith('http://localhost')) {
            handleAutoFill(request, sender, sendResponse);
        }
    }
});

// Context menu for password generation
chrome.runtime.onInstalled.addListener(function() {
    chrome.contextMenus.create({
        id: 'generatePassword',
        title: 'Generate Password with Aetheris',
        contexts: ['editable']
    });
});

// Handle context menu clicks
chrome.contextMenus.onClicked.addListener(function(info, tab) {
    if (info.menuItemId === 'generatePassword') {
        chrome.tabs.sendMessage(tab.id, {action: 'generatePassword'});
    }
});

// Keep service worker alive
setInterval(function() {
    chrome.runtime.sendMessage({action: 'ping'});
}, 30000);