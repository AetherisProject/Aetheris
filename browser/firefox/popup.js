// Aetheris Firefox Extension - Popup
// Handles popup UI and basic operations

// Check if vault is unlocked
function checkVaultStatus() {
    chrome.storage.local.get(['vaultUnlocked'], function(result) {
        const statusElement = document.getElementById('status');
        if (result.vaultUnlocked) {
            statusElement.textContent = 'Unlocked';
            statusElement.style.color = '#4CAF50';
        } else {
            statusElement.textContent = 'Locked';
            statusElement.style.color = '#F44336';
        }
    });
}

// Unlock vault
function unlockVault() {
    chrome.tabs.create({
        url: chrome.runtime.getURL('unlock.html')
    });
}

// Generate password
function generatePassword() {
    chrome.tabs.create({
        url: chrome.runtime.getURL('generate.html')
    });
}

// Auto-fill
function autoFill() {
    chrome.tabs.query({active: true, currentWindow: true}, function(tabs) {
        chrome.tabs.sendMessage(tabs[0].id, {action: 'autofill'});
    });
}

// Settings
function openSettings() {
    chrome.tabs.create({
        url: chrome.runtime.getURL('options.html')
    });
}

// Event listeners
document.getElementById('unlock').addEventListener('click', unlockVault);
document.getElementById('generate').addEventListener('click', generatePassword);
document.getElementById('autofill').addEventListener('click', autoFill);
document.getElementById('settings').addEventListener('click', openSettings);

// Initialize
checkVaultStatus();