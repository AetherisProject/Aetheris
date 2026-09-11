const CACHE = 'aetheris-v2-shell-v1';
const SHELL = ['/', '/index.html', '/css/app.css', '/manifest.webmanifest'];

self.addEventListener('install', (event) => {
    event.waitUntil(caches.open(CACHE).then((c) => c.addAll(SHELL)).then(() => self.skipWaiting()));
});

self.addEventListener('activate', (event) => {
    event.waitUntil(
        caches.keys()
            .then((keys) => Promise.all(keys.filter((k) => k !== CACHE).map((k) => caches.delete(k))))
            .then(() => self.clients.claim()));
});

// Network-first for API calls, cache-first for the app shell.
self.addEventListener('fetch', (event) => {
    const url = new URL(event.request.url);
    if (url.pathname.startsWith('/v1/')) return; // gateway/hub calls: always live
    event.respondWith(
        caches.match(event.request).then((cached) => cached || fetch(event.request)));
});
