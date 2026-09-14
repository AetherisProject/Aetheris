// offline-queue.js - Queue actions for offline sync
window.AetherisOffline = window.AetherisOffline || {};

const OFFLINE_QUEUE_KEY = 'aetheris:offlineQueue';

function getQueue() {
    try {
        const stored = localStorage.getItem(OFFLINE_QUEUE_KEY);
        return stored ? JSON.parse(stored) : [];
    } catch {
        return [];
    }
}

function saveQueue(queue) {
    try {
        localStorage.setItem(OFFLINE_QUEUE_KEY, JSON.stringify(queue));
    } catch {
        // localStorage full or not available
    }
}

// Queue an action for offline sync
window.AetherisOffline.queue = function(action, data) {
    const queue = getQueue();
    queue.push({ 
        action: action, 
        data: data, 
        timestamp: Date.now() 
    });
    saveQueue(queue);
};

// Flush the offline queue (sync with hub)
window.AetherisOffline.flush = async function() {
    const queue = getQueue();
    if (queue.length === 0) return;
    
    // For now, just clear the queue
    // In a real implementation, this would sync with the hub
    // by calling the appropriate API endpoints
    saveQueue([]);
};

// Auto-flush on online event
window.addEventListener('online', function() {
    setTimeout(window.AetherisOffline.flush, 1000);
});

// Tab visibility listener for auto-lock
function setupTabVisibilityListener() {
    if (window.AetherisVaultState && typeof window.AetherisVaultState.lock === 'function') {
        document.addEventListener('visibilitychange', function() {
            if (document.hidden) {
                window.AetherisVaultState.lock();
            }
        });
    }
}

// Initialize on DOM load
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', setupTabVisibilityListener);
} else {
    setupTabVisibilityListener();
}
