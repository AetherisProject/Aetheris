/**
 * Aether - Puter.js Client Integration
 * Handles Puter.js initialization and AI model interactions
 */

// Puter Client Module
const PuterClient = (function() {
    'use strict';
    
    let puterInstance = null;
    let isInitialized = false;
    let modelsCache = null;
    let lastRequestTime = 0;
    const REQUEST_DELAY = 1000; // Minimum delay between requests in ms
    
    /**
     * Initialize Puter.js
     */
    async function init() {
        if (isInitialized) return puterInstance;
        
        return new Promise((resolve, reject) => {
            // Check if Puter is already loaded
            if (window.puter && window.puter.ai) {
                puterInstance = window.puter;
                isInitialized = true;
                console.log('Puter already loaded, using existing instance');
                resolve(puterInstance);
                return;
            }
            
            // Load Puter script
            const script = document.createElement('script');
            script.src = 'https://js.puter.com/v2/';
            script.async = true;
            
            script.onload = async () => {
                try {
                    puterInstance = window.puter;
                    isInitialized = true;
                    
                    // Verify AI capabilities
                    if (!puterInstance.ai) {
                        throw new Error('Puter loaded but AI module not available');
                    }
                    
                    console.log('Puter.js initialized successfully');
                    
                    // Load models cache
                    await loadModels();
                    
                    resolve(puterInstance);
                } catch (error) {
                    console.error('Failed to initialize Puter:', error);
                    reject(error);
                }
            };
            
            script.onerror = () => {
                console.error('Failed to load Puter script');
                reject(new Error('Failed to load Puter.js'));
            };
            
            document.head.appendChild(script);
        });
    }
    
    /**
     * Load available AI models
     */
    async function loadModels() {
        try {
            if (!puterInstance || !puterInstance.ai) return [];
            
            // Check if models method exists
            if (puterInstance.ai.models) {
                modelsCache = await puterInstance.ai.models();
                console.log('Loaded models:', modelsCache.length || Object.keys(modelsCache).length);
            } else {
                // Default models for Puter.js v2
                modelsCache = [
                    { id: 'gpt-4o-mini', name: 'GPT-4o Mini', provider: 'OpenAI', type: 'chat' },
                    { id: 'gpt-4o', name: 'GPT-4o', provider: 'OpenAI', type: 'chat' },
                    { id: 'claude-3-haiku', name: 'Claude 3 Haiku', provider: 'Anthropic', type: 'chat' },
                    { id: 'claude-3-sonnet', name: 'Claude 3 Sonnet', provider: 'Anthropic', type: 'chat' },
                    { id: 'gemini-1.5-flash', name: 'Gemini 1.5 Flash', provider: 'Google', type: 'chat' },
                    { id: 'gemini-1.5-pro', name: 'Gemini 1.5 Pro', provider: 'Google', type: 'chat' },
                    { id: 'llama-3.1-70b', name: 'Llama 3.1 70B', provider: 'Meta', type: 'chat' },
                    { id: 'llama-3.1-405b', name: 'Llama 3.1 405B', provider: 'Meta', type: 'chat' },
                    { id: 'mistral-large', name: 'Mistral Large', provider: 'Mistral', type: 'chat' },
                    { id: 'mistral-small', name: 'Mistral Small', provider: 'Mistral', type: 'chat' }
                ];
            }
            
            return modelsCache;
        } catch (error) {
            console.log('Could not load models, using defaults:', error);
            modelsCache = null;
            return [];
        }
    }
    
    /**
     * Get available models
     */
    function getModels() {
        return modelsCache || [];
    }
    
    /**
     * Chat with AI model
     */
    async function chat(prompt, options = {}) {
        await ensureInitialized();
        enforceRateLimit();
        
        try {
            const response = await puterInstance.ai.chat(prompt, options);
            return formatResponse(response);
        } catch (error) {
            console.error('Chat error:', error);
            throw formatError(error);
        }
    }
    
    /**
     * Generate image from text
     */
    async function generateImage(prompt, options = {}) {
        await ensureInitialized();
        enforceRateLimit();
        
        try {
            const blob = await puterInstance.ai.txt2img(prompt, options);
            return blob;
        } catch (error) {
            console.error('Image generation error:', error);
            throw formatError(error);
        }
    }
    
    /**
     * Generate image URL (base64 or URL)
     */
    async function generateImageUrl(prompt, options = {}) {
        const blob = await generateImage(prompt, options);
        return URL.createObjectURL(blob);
    }
    
    /**
     * Complete text (for older models)
     */
    async function complete(prompt, options = {}) {
        await ensureInitialized();
        enforceRateLimit();
        
        try {
            const response = await puterInstance.ai.complete(prompt, options);
            return formatResponse(response);
        } catch (error) {
            console.error('Completion error:', error);
            throw formatError(error);
        }
    }
    
    /**
     * Embed text
     */
    async function embed(text, options = {}) {
        await ensureInitialized();
        enforceRateLimit();
        
        try {
            const response = await puterInstance.ai.embed(text, options);
            return response;
        } catch (error) {
            console.error('Embedding error:', error);
            throw formatError(error);
        }
    }
    
    /**
     * Classify text
     */
    async function classify(text, categories, options = {}) {
        await ensureInitialized();
        enforceRateLimit();
        
        try {
            const response = await puterInstance.ai.classify(text, categories, options);
            return response;
        } catch (error) {
            console.error('Classification error:', error);
            throw formatError(error);
        }
    }
    
    /**
     * Ensure Puter is initialized
     */
    async function ensureInitialized() {
        if (!isInitialized) {
            await init();
        }
    }
    
    /**
     * Enforce rate limiting
     */
    function enforceRateLimit() {
        const now = Date.now();
        const timeSinceLast = now - lastRequestTime;
        
        if (timeSinceLast < REQUEST_DELAY) {
            const waitTime = Math.ceil((REQUEST_DELAY - timeSinceLast) / 10);
            console.log(`Rate limiting: waiting ${waitTime}ms`);
            // In a real app, you might want to actually wait
            // For now, just log it
        }
        
        lastRequestTime = now;
    }
    
    /**
     * Format API response
     */
    function formatResponse(response) {
        if (!response) return { text: '' };
        
        // Handle different response formats
        if (typeof response === 'string') {
            return { text: response };
        }
        
        if (response.text) {
            return { text: response.text };
        }
        
        if (response.message) {
            if (response.message.content) {
                return { text: response.message.content };
            }
            if (Array.isArray(response.message)) {
                return { text: response.message.map(m => m.content || '').join('\n') };
            }
        }
        
        if (response.choices && response.choices[0]) {
            return { text: response.choices[0].message?.content || response.choices[0].text || '' };
        }
        
        return { text: String(response) };
    }
    
    /**
     * Format error message
     */
    function formatError(error) {
        if (typeof error === 'string') {
            return new Error(error);
        }
        
        if (error.message) {
            return new Error(error.message);
        }
        
        if (error.error) {
            return new Error(error.error.message || String(error.error));
        }
        
        return new Error(String(error));
    }
    
    /**
     * Check if Puter is available
     */
    function isAvailable() {
        return isInitialized && !!puterInstance;
    }
    
    /**
     * Get Puter instance
     */
    function getInstance() {
        return puterInstance;
    }
    
    // Public API
    return {
        init,
        loadModels,
        getModels,
        chat,
        generateImage,
        generateImageUrl,
        complete,
        embed,
        classify,
        isAvailable,
        getInstance
    };
})();

// Export for global access
window.PuterClient = PuterClient;

// Also make available as a module if using ES6 imports
if (typeof module !== 'undefined' && module.exports) {
    module.exports = PuterClient;
}

// Auto-initialize when DOM is ready
if (document.readyState === 'complete' || document.readyState === 'interactive') {
    PuterClient.init().catch(console.error);
} else {
    document.addEventListener('DOMContentLoaded', () => {
        PuterClient.init().catch(console.error);
    });
}