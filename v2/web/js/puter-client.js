/**
 * Puter Client Wrapper
 * Provides a clean interface to Puter.js API
 * Part of Aether Demo - Free AI Model Access
 */

class PuterClient {
    constructor() {
        this.puter = window.puter;
        this.models = [];
        this.isInitialized = false;
        this.currentModel = null;
    }

    /**
     * Initialize the Puter client
     */
    async init() {
        try {
            // Check if Puter is available
            if (!this.puter) {
                console.error('Puter.js not loaded');
                return false;
            }

            // List available models
            this.models = await this.listModels();
            this.isInitialized = true;
            console.log(`Puter client initialized with ${this.models.length} models`);
            return true;
        } catch (error) {
            console.error('Failed to initialize Puter client:', error);
            return false;
        }
    }

    /**
     * List all available AI models
     */
    async listModels() {
        try {
            const models = await this.puter.ai.listModels();
            return models || [];
        } catch (error) {
            console.error('Failed to list models:', error);
            // Return a fallback list of known models
            return this.getFallbackModels();
        }
    }

    /**
     * Get fallback model list when API fails
     */
    getFallbackModels() {
        return [
            {
                id: 'gpt-4o',
                name: 'GPT-4o',
                provider: 'OpenAI',
                type: 'text',
                description: 'OpenAI\'s most advanced model with multimodal capabilities and real-time reasoning.',
                capabilities: ['chat', 'generate', 'analyze', 'reason'],
                pricing: 'paid',
                contextLength: 128000,
                maxTokens: 128000
            },
            {
                id: 'gpt-4o-mini',
                name: 'GPT-4o Mini',
                provider: 'OpenAI',
                type: 'text',
                description: 'A smaller, faster version of GPT-4o with lower cost.',
                capabilities: ['chat', 'generate', 'analyze'],
                pricing: 'paid',
                contextLength: 128000,
                maxTokens: 128000
            },
            {
                id: 'claude-3-5-sonnet',
                name: 'Claude 3.5 Sonnet',
                provider: 'Anthropic',
                type: 'text',
                description: 'Anthropic\'s intelligent and helpful AI assistant with advanced reasoning capabilities.',
                capabilities: ['chat', 'code', 'reason', 'analyze'],
                pricing: 'paid',
                contextLength: 200000,
                maxTokens: 4096
            },
            {
                id: 'claude-3-haiku',
                name: 'Claude 3 Haiku',
                provider: 'Anthropic',
                type: 'text',
                description: 'Fast and efficient model for quick responses.',
                capabilities: ['chat', 'code', 'reason'],
                pricing: 'paid',
                contextLength: 200000,
                maxTokens: 4096
            },
            {
                id: 'gemini-1.5-pro',
                name: 'Gemini 1.5 Pro',
                provider: 'Google',
                type: 'text',
                description: 'Google\'s advanced multimodal model with long context window.',
                capabilities: ['chat', 'generate', 'analyze', 'reason'],
                pricing: 'paid',
                contextLength: 1048576,
                maxTokens: 8192
            },
            {
                id: 'gemini-1.5-flash',
                name: 'Gemini 1.5 Flash',
                provider: 'Google',
                type: 'text',
                description: 'Fast and efficient model for quick interactions.',
                capabilities: ['chat', 'generate', 'analyze'],
                pricing: 'paid',
                contextLength: 1048576,
                maxTokens: 8192
            },
            {
                id: 'grok-2',
                name: 'Grok 2',
                provider: 'xAI',
                type: 'text',
                description: 'xAI\'s advanced reasoning model with real-time knowledge.',
                capabilities: ['chat', 'generate', 'reason'],
                pricing: 'paid',
                contextLength: 128000,
                maxTokens: 8192
            },
            {
                id: 'grok-1.5',
                name: 'Grok 1.5',
                provider: 'xAI',
                type: 'text',
                description: 'Improved version with better reasoning and knowledge.',
                capabilities: ['chat', 'generate', 'reason'],
                pricing: 'paid',
                contextLength: 128000,
                maxTokens: 8192
            },
            {
                id: 'mistral-large-2',
                name: 'Mistral Large 2',
                provider: 'Mistral',
                type: 'text',
                description: 'Mistral\'s flagship model with exceptional performance in reasoning and coding tasks.',
                capabilities: ['chat', 'code', 'math', 'reason'],
                pricing: 'paid',
                contextLength: 128000,
                maxTokens: 4096
            },
            {
                id: 'mistral-small',
                name: 'Mistral Small',
                provider: 'Mistral',
                type: 'text',
                description: 'Fast and efficient model for quick responses.',
                capabilities: ['chat', 'code', 'reason'],
                pricing: 'paid',
                contextLength: 32000,
                maxTokens: 4096
            },
            {
                id: 'deepseek-chat',
                name: 'DeepSeek Chat',
                provider: 'DeepSeek',
                type: 'text',
                description: 'DeepSeek\'s advanced chat model with reasoning capabilities.',
                capabilities: ['chat', 'code', 'reason'],
                pricing: 'paid',
                contextLength: 32000,
                maxTokens: 4096
            },
            {
                id: 'deepseek-coder',
                name: 'DeepSeek Coder',
                provider: 'DeepSeek',
                type: 'code',
                description: 'Specialized model for code generation and understanding.',
                capabilities: ['code', 'generate', 'analyze'],
                pricing: 'paid',
                contextLength: 16000,
                maxTokens: 4096
            },
            {
                id: 'llama-3.1-70b',
                name: 'Llama 3.1 70B',
                provider: 'Meta',
                type: 'text',
                description: 'Meta\'s open-source model with strong performance across tasks.',
                capabilities: ['chat', 'generate', 'code', 'reason'],
                pricing: 'free',
                contextLength: 128000,
                maxTokens: 8192
            },
            {
                id: 'llama-3.1-8b',
                name: 'Llama 3.1 8B',
                provider: 'Meta',
                type: 'text',
                description: 'Smaller, faster version of Llama 3.1.',
                capabilities: ['chat', 'generate', 'code'],
                pricing: 'free',
                contextLength: 128000,
                maxTokens: 8192
            },
            {
                id: 'command-r-plus',
                name: 'Command R+',
                provider: 'Cohere',
                type: 'text',
                description: 'Cohere\'s advanced model with strong reasoning and tool use.',
                capabilities: ['chat', 'generate', 'reason'],
                pricing: 'paid',
                contextLength: 128000,
                maxTokens: 4096
            },
            {
                id: 'command-r',
                name: 'Command R',
                provider: 'Cohere',
                type: 'text',
                description: 'Cohere\'s efficient model for various tasks.',
                capabilities: ['chat', 'generate'],
                pricing: 'paid',
                contextLength: 128000,
                maxTokens: 4096
            },
            {
                id: 'flux-1-dev',
                name: 'FLUX.1 [Dev]',
                provider: 'Black Forest Labs',
                type: 'image',
                description: 'State-of-the-art text-to-image model with incredible detail and prompt adherence.',
                capabilities: ['generate', 'edit', 'high-res'],
                pricing: 'paid',
                contextLength: 0,
                maxTokens: 0
            },
            {
                id: 'flux-1-schnell',
                name: 'FLUX.1 [Schnell]',
                provider: 'Black Forest Labs',
                type: 'image',
                description: 'Fast version of FLUX.1 for quick image generation.',
                capabilities: ['generate', 'high-res'],
                pricing: 'paid',
                contextLength: 0,
                maxTokens: 0
            }
        ];
    }

    /**
     * Get models filtered by criteria
     */
    getModels(options = {}) {
        const { 
            provider, type, pricing, capability, 
            search, sortBy, sortOrder, 
            page = 1, limit = 20 
        } = options;

        let filtered = [...this.models];

        // Filter by provider
        if (provider && provider.length > 0) {
            filtered = filtered.filter(model => 
                provider.includes(model.provider.toLowerCase())
            );
        }

        // Filter by type
        if (type && type.length > 0) {
            filtered = filtered.filter(model => 
                type.includes(model.type.toLowerCase())
            );
        }

        // Filter by pricing
        if (pricing && pricing.length > 0) {
            filtered = filtered.filter(model => 
                pricing.includes(model.pricing.toLowerCase())
            );
        }

        // Filter by capability
        if (capability && capability.length > 0) {
            filtered = filtered.filter(model => {
                if (!model.capabilities) return false;
                return capability.some(cap => 
                    model.capabilities.map(c => c.toLowerCase()).includes(cap.toLowerCase())
                );
            });
        }

        // Filter by search
        if (search) {
            const searchLower = search.toLowerCase();
            filtered = filtered.filter(model => 
                model.name.toLowerCase().includes(searchLower) ||
                model.id.toLowerCase().includes(searchLower) ||
                model.description.toLowerCase().includes(searchLower) ||
                model.provider.toLowerCase().includes(searchLower)
            );
        }

        // Sort
        let sorted = [...filtered];
        switch (sortBy) {
            case 'name':
                sorted.sort((a, b) => a.name.localeCompare(b.name));
                break;
            case 'provider':
                sorted.sort((a, b) => a.provider.localeCompare(b.provider));
                break;
            case 'type':
                sorted.sort((a, b) => a.type.localeCompare(b.type));
                break;
            case 'popularity':
                // Simple popularity heuristic
                sorted.sort((a, b) => {
                    const popularity = {
                        'gpt-4o': 10, 'claude-3-5-sonnet': 9, 'gemini-1.5-pro': 8,
                        'grok-2': 7, 'mistral-large-2': 6, 'flux-1-dev': 5
                    };
                    return (popularity[b.id] || 0) - (popularity[a.id] || 0);
                });
                break;
            default:
                sorted.sort((a, b) => a.name.localeCompare(b.name));
        }

        if (sortOrder === 'desc') {
            sorted.reverse();
        }

        // Paginate
        const start = (page - 1) * limit;
        const end = start + limit;
        const paginated = sorted.slice(start, end);

        return {
            models: paginated,
            total: filtered.length,
            page,
            limit,
            totalPages: Math.ceil(filtered.length / limit)
        };
    }

    /**
     * Get a specific model by ID
     */
    getModel(modelId) {
        return this.models.find(model => 
            model.id === modelId || model.name === modelId
        );
    }

    /**
     * Set the current model for chat
     */
    setCurrentModel(modelId) {
        const model = this.getModel(modelId);
        if (model) {
            this.currentModel = model;
            return model;
        }
        return null;
    }

    /**
     * Get the current model
     */
    getCurrentModel() {
        return this.currentModel;
    }

    /**
     * Send a chat message
     */
    async chat(prompt, options = {}) {
        try {
            const model = this.currentModel || this.models[0];
            
            const defaultOptions = {
                model: model.id,
                stream: true,
                temperature: 0.7,
                maxTokens: 2048
            };

            const chatOptions = { ...defaultOptions, ...options };
            
            // Remove stream from options for now as we handle it separately
            const { stream, ...restOptions } = chatOptions;
            
            if (stream) {
                // Streaming chat
                return this.puter.ai.chat(prompt, restOptions, {
                    stream: true,
                    onchunk: (chunk) => {
                        if (options.onchunk) {
                            options.onchunk(chunk);
                        }
                    },
                    onend: () => {
                        if (options.onend) {
                            options.onend();
                        }
                    },
                    onerror: (error) => {
                        if (options.onerror) {
                            options.onerror(error);
                        }
                    }
                });
            } else {
                // Non-streaming chat
                const response = await this.puter.ai.chat(prompt, restOptions);
                return response;
            }
        } catch (error) {
            console.error('Chat error:', error);
            throw error;
        }
    }

    /**
     * Generate text
     */
    async generateText(prompt, options = {}) {
        try {
            const model = this.currentModel || this.models.find(m => m.type === 'text') || this.models[0];
            const response = await this.puter.ai.chat(prompt, {
                model: model.id,
                ...options
            });
            return response;
        } catch (error) {
            console.error('Generate text error:', error);
            throw error;
        }
    }

    /**
     * Generate image
     */
    async generateImage(prompt, options = {}) {
        try {
            const response = await this.puter.ai.txt2img(prompt, options);
            return response;
        } catch (error) {
            console.error('Generate image error:', error);
            throw error;
        }
    }

    /**
     * Edit image
     */
    async editImage(image, prompt, options = {}) {
        try {
            const response = await this.puter.ai.img2img(image, prompt, options);
            return response;
        } catch (error) {
            console.error('Edit image error:', error);
            throw error;
        }
    }

    /**
     * Transcribe audio
     */
    async transcribeAudio(audio, options = {}) {
        try {
            const response = await this.puter.ai.audio2text(audio, options);
            return response;
        } catch (error) {
            console.error('Transcribe error:', error);
            throw error;
        }
    }

    /**
     * Generate speech
     */
    async generateSpeech(text, options = {}) {
        try {
            const response = await this.puter.ai.text2audio(text, options);
            return response;
        } catch (error) {
            console.error('Generate speech error:', error);
            throw error;
        }
    }

    /**
     * File operations
     */
    async listFiles(path = '/') {
        try {
            const files = await this.puter.fs.readdir(path);
            return files;
        } catch (error) {
            console.error('List files error:', error);
            throw error;
        }
    }

    async readFile(path) {
        try {
            const content = await this.puter.fs.readFile(path);
            return content;
        } catch (error) {
            console.error('Read file error:', error);
            throw error;
        }
    }

    async writeFile(path, content) {
        try {
            await this.puter.fs.writeFile(path, content);
            return true;
        } catch (error) {
            console.error('Write file error:', error);
            throw error;
        }
    }

    async deleteFile(path) {
        try {
            await this.puter.fs.deleteFile(path);
            return true;
        } catch (error) {
            console.error('Delete file error:', error);
            throw error;
        }
    }

    async uploadFile(file) {
        try {
            const result = await this.puter.fs.upload(file);
            return result;
        } catch (error) {
            console.error('Upload file error:', error);
            throw error;
        }
    }

    async downloadFile(path) {
        try {
            const url = await this.puter.fs.getDownloadLink(path);
            return url;
        } catch (error) {
            console.error('Download file error:', error);
            throw error;
        }
    }

    /**
     * Research tools
     */
    async searchWeb(query, options = {}) {
        try {
            // Use chat to perform web search
            const prompt = `Perform a web search for: ${query}. Provide a comprehensive summary of the results.`;
            const response = await this.chat(prompt, options);
            return response;
        } catch (error) {
            console.error('Search error:', error);
            throw error;
        }
    }

    async summarizeText(text, options = {}) {
        try {
            const prompt = `Summarize the following text:\n\n${text}\n\nProvide a concise summary.`;
            const response = await this.chat(prompt, options);
            return response;
        } catch (error) {
            console.error('Summarize error:', error);
            throw error;
        }
    }

    async translateText(text, targetLanguage, options = {}) {
        try {
            const prompt = `Translate the following text to ${targetLanguage}:\n\n${text}`;
            const response = await this.chat(prompt, options);
            return response;
        } catch (error) {
            console.error('Translate error:', error);
            throw error;
        }
    }

    async analyzeDocument(content, analysisType = 'summary', options = {}) {
        try {
            let prompt = '';
            switch (analysisType) {
                case 'summary':
                    prompt = `Analyze and summarize the following document:\n\n${content}`;
                    break;
                case 'keywords':
                    prompt = `Extract the main keywords from the following document:\n\n${content}`;
                    break;
                case 'questions':
                    prompt = `Generate 5-10 questions based on the following document:\n\n${content}`;
                    break;
                case 'insights':
                    prompt = `Provide deep insights and analysis of the following document:\n\n${content}`;
                    break;
                default:
                    prompt = `Analyze the following document:\n\n${content}`;
            }
            
            const response = await this.chat(prompt, options);
            return response;
        } catch (error) {
            console.error('Analyze error:', error);
            throw error;
        }
    }

    async reviewCode(code, language = 'auto', focus = 'general', options = {}) {
        try {
            let prompt = `Review the following ${language} code:\n\n${code}\n\n`;
            
            switch (focus) {
                case 'bugs':
                    prompt += 'Focus on finding bugs and potential issues.';
                    break;
                case 'performance':
                    prompt += 'Focus on performance optimizations.';
                    break;
                case 'security':
                    prompt += 'Focus on security vulnerabilities.';
                    break;
                case 'style':
                    prompt += 'Focus on code style and best practices.';
                    break;
                default:
                    prompt += 'Provide a comprehensive review.';
            }
            
            const response = await this.chat(prompt, options);
            return response;
        } catch (error) {
            console.error('Code review error:', error);
            throw error;
        }
    }

    /**
     * Get model statistics
     */
    getModelStats() {
        const providers = new Set();
        const types = new Set();
        const pricing = { free: 0, paid: 0 };
        
        this.models.forEach(model => {
            providers.add(model.provider);
            types.add(model.type);
            if (model.pricing === 'free') {
                pricing.free++;
            } else {
                pricing.paid++;
            }
        });

        return {
            total: this.models.length,
            providers: Array.from(providers).sort(),
            types: Array.from(types).sort(),
            pricing
        };
    }

    /**
     * Get provider counts
     */
    getProviderCounts() {
        const counts = {};
        this.models.forEach(model => {
            const provider = model.provider.toLowerCase();
            counts[provider] = (counts[provider] || 0) + 1;
        });
        return counts;
    }

    /**
     * Get type counts
     */
    getTypeCounts() {
        const counts = {};
        this.models.forEach(model => {
            const type = model.type.toLowerCase();
            counts[type] = (counts[type] || 0) + 1;
        });
        return counts;
    }

    /**
     * Get pricing counts
     */
    getPricingCounts() {
        const counts = { free: 0, paid: 0 };
        this.models.forEach(model => {
            counts[model.pricing] = (counts[model.pricing] || 0) + 1;
        });
        return counts;
    }
}

// Initialize and export the client
let puterClient = null;

function getPuterClient() {
    if (!puterClient) {
        puterClient = new PuterClient();
    }
    return puterClient;
}

// Auto-initialize when Puter is ready
if (window.puter) {
    getPuterClient().init();
} else {
    // Wait for Puter to load
    window.addEventListener('load', () => {
        setTimeout(() => {
            if (window.puter) {
                getPuterClient().init();
            }
        }, 1000);
    });
}
