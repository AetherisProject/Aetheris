// service-worker.js v2 - Offline-first PWA with app shell caching
const CACHE_VERSION = 'aetheris-v2-shell-v2';
const APP_SHELL = [
    '/',
    '/index.html',
    '/css/app.css',
    '/manifest.webmanifest',
    '/js/argon2.js',
    '/js/crypto.js',
    '/js/totp.js',
    '/js/clipboard.js',
    '/js/offline-queue.js'
];

// API endpoints that should always go to network
const API_PREFIXES = ['/v1/', '/hub/', '/api/'];

// Runtime cache for API responses
const RUNTIME_CACHE = 'aetheris-runtime-v2';

// Install: Cache the app shell
self.addEventListener('install', (event) => {
    event.waitUntil(
        caches.open(CACHE_VERSION).then((cache) => {
            return cache.addAll(APP_SHELL).then(() => {
                // Force the waiting service worker to become active
                return self.skipWaiting();
            });
        })
    );
});

// Activate: Clean up old caches
self.addEventListener('activate', (event) => {
    event.waitUntil(
        caches.keys().then((keys) => {
            return Promise.all(
                keys.filter((key) => key !== CACHE_VERSION && key !== RUNTIME_CACHE)
                    .map((key) => caches.delete(key))
            );
        }).then(() => {
            // Take control of all clients
            return self.clients.claim();
        })
    );
});

// Fetch: Network-first for API calls, cache-first for app shell
self.addEventListener('fetch', (event) => {
    const url = new URL(event.request.url);
    
    // Always go to network for API calls
    if (API_PREFIXES.some(prefix => url.pathname.startsWith(prefix))) {
        // Network-first with runtime caching for GET requests
        if (event.request.method === 'GET') {
            event.respondWith(
                fetch(event.request).then((response) => {
                    // Clone and cache successful responses
                    if (response.ok) {
                        const responseClone = response.clone();
                        caches.open(RUNTIME_CACHE).then((cache) => {
                            cache.put(event.request, responseClone);
                        });
                    }
                    return response;
                }).catch(() => {
                    // Fallback to runtime cache if available
                    return caches.match(event.request).then((cached) => {
                        return cached || new Response('Network error', { status: 503 });
                    });
                })
            );
        }
        // For non-GET requests, always go to network
        return;
    }
    
    // Cache-first for app shell
    event.respondWith(
        caches.match(event.request).then((cached) => {
            // Return cached response if available
            if (cached) {
                return cached;
            }
            
            // Otherwise, fetch from network and cache
            return fetch(event.request).then((response) => {
                // Cache only successful responses
                if (response.ok) {
                    const responseClone = response.clone();
                    caches.open(CACHE_VERSION).then((cache) => {
                        cache.put(event.request, responseClone);
                    });
                }
                return response;
            });
        })
    );
});

// Message: Handle messages from the client
self.addEventListener('message', (event) => {
    if (event.data.action === 'skipWaiting') {
        self.skipWaiting();
    }
    
    if (event.data.action === 'cacheAppShell') {
        caches.open(CACHE_VERSION).then((cache) => {
            return cache.addAll(APP_SHELL);
        });
    }
    
    if (event.data.action === 'clearCache') {
        caches.keys().then((keys) => {
            return Promise.all(keys.map((key) => caches.delete(key)));
        });
    }
});

// Push notification support (for future use)
self.addEventListener('push', (event) => {
    const data = event.data?.json();
    if (data) {
        event.waitUntil(
            self.registration.showNotification(data.title, {
                body: data.body,
                icon: data.icon || '/icons/icon-192x192.png',
                data: data.data
            })
        );
    }
});

// Background sync support (for offline queue)
self.addEventListener('sync', (event) => {
    if (event.tag === 'sync-offline-queue') {
        event.waitUntil(
            // In a real implementation, this would sync the offline queue
            // with the hub when the network is available
            Promise.resolve()
        );
    }
});
