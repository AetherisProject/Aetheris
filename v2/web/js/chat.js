/**
 * Aether Demo - Chat Functionality
 * Handles chat interface, model selection, and messaging
 */

// Chat state
const ChatState = {
    messages: [],
    currentModel: null,
    isStreaming: false,
    isLoading: false,
    conversationId: null
};

// DOM Elements
const ChatDOM = {
    modelList: null,
    messagesContainer: null,
    promptInput: null,
    sendBtn: null,
    currentModelName: null,
    currentModelAvatar: null,
    currentModelStatus: null,
    welcomeMessage: null,
    settingsPanel: null,
    temperatureSlider: null,
    temperatureValue: null,
    maxTokensSlider: null,
    maxTokensValue: null,
    streamToggle: null,
    streamStatus: null
};

// Initialize chat
document.addEventListener('DOMContentLoaded', () => {
    initializeChatDOM();
    initializeChatEventListeners();
    loadModels();
    loadChatState();
});

/**
 * Initialize chat DOM references
 */
function initializeChatDOM() {
    ChatDOM.modelList = document.getElementById('model-list');
    ChatDOM.messagesContainer = document.getElementById('messages-container');
    ChatDOM.promptInput = document.getElementById('prompt-input');
    ChatDOM.sendBtn = document.getElementById('send-btn');
    ChatDOM.currentModelName = document.getElementById('current-model-name');
    ChatDOM.currentModelAvatar = document.getElementById('current-model-avatar');
    ChatDOM.currentModelStatus = document.getElementById('current-model-status');
    ChatDOM.welcomeMessage = document.getElementById('welcome-message');
    ChatDOM.settingsPanel = document.getElementById('settings-panel');
    ChatDOM.temperatureSlider = document.getElementById('temperature-slider');
    ChatDOM.temperatureValue = document.getElementById('temperature-value');
    ChatDOM.maxTokensSlider = document.getElementById('max-tokens-slider');
    ChatDOM.maxTokensValue = document.getElementById('max-tokens-value');
    ChatDOM.streamToggle = document.getElementById('stream-toggle');
    ChatDOM.streamStatus = document.getElementById('stream-status');
}

/**
 * Initialize chat event listeners
 */
function initializeChatEventListeners() {
    // Input events
    if (ChatDOM.promptInput) {
        ChatDOM.promptInput.addEventListener('input', handleInputChange);
        ChatDOM.promptInput.addEventListener('paste', handlePaste);
        ChatDOM.promptInput.addEventListener('keydown', handleKeyDown);
    }
    
    // Settings events
    if (ChatDOM.temperatureSlider) {
        ChatDOM.temperatureSlider.addEventListener('input', (e) => {
            const value = parseFloat(e.target.value);
            ChatDOM.temperatureValue.textContent = value.toFixed(1);
            AppState.settings.temperature = value;
        });
    }
    
    if (ChatDOM.maxTokensSlider) {
        ChatDOM.maxTokensSlider.addEventListener('input', (e) => {
            const value = parseInt(e.target.value);
            ChatDOM.maxTokensValue.textContent = value;
            AppState.settings.maxTokens = value;
        });
    }
    
    // Model search
    const modelSearch = document.getElementById('model-search');
    if (modelSearch) {
        modelSearch.addEventListener('input', (e) => {
            filterModels(e.target.value);
        });
    }
    
    // Auto-scroll to bottom when messages change
    const messagesObserver = new MutationObserver(() => {
        scrollToBottom();
    });
    if (ChatDOM.messagesContainer) {
        messagesObserver.observe(ChatDOM.messagesContainer, {
            childList: true,
            subtree: true
        });
    }
}

/**
 * Load models from Puter client
 */
async function loadModels() {
    const client = getPuterClient();
    
    try {
        // Ensure client is initialized
        if (!client.isInitialized) {
            await client.init();
        }
        
        const models = client.models;
        renderModelList(models);
        
        // Set default model if none selected
        if (!ChatState.currentModel && models.length > 0) {
            setCurrentModel(models[0].id);
        }
    } catch (error) {
        console.error('Failed to load models:', error);
        showError('model-list', 'Failed to load models. Please refresh the page.');
    }
}

/**
 * Render model list
 */
function renderModelList(models) {
    if (!ChatDOM.modelList) return;
    
    // Group models by provider
    const modelsByProvider = {};
    models.forEach(model => {
        const provider = model.provider || 'Other';
        if (!modelsByProvider[provider]) {
            modelsByProvider[provider] = [];
        }
        modelsByProvider[provider].push(model);
    });
    
    let html = '';
    
    // Sort providers alphabetically
    const sortedProviders = Object.keys(modelsByProvider).sort();
    
    sortedProviders.forEach(provider => {
        const providerModels = modelsByProvider[provider];
        
        // Sort models by name
        providerModels.sort((a, b) => a.name.localeCompare(b.name));
        
        html += `
            <div class="model-group">
                <div class="model-group-header">
                    <span class="model-group-title">${provider}</span>
                </div>
                ${providerModels.map(model => createModelItemHtml(model)).join('')}
            </div>
        `;
    });
    
    ChatDOM.modelList.innerHTML = html;
    
    // Add click handlers
    document.querySelectorAll('.model-item').forEach(item => {
        item.addEventListener('click', () => {
            const modelId = item.dataset.modelId;
            setCurrentModel(modelId);
        });
    });
}

/**
 * Create HTML for a model item
 */
function createModelItemHtml(model) {
    const providerClass = getProviderClass(model.provider);
    const typeClass = getTypeClass(model.type);
    const isActive = ChatState.currentModel?.id === model.id;
    
    return `
        <div class="model-item ${isActive ? 'active' : ''}" data-model-id="${model.id}">
            <div class="model-avatar ${providerClass}">${getModelInitials(model.name)}</div>
            <div class="model-info">
                <div class="model-name">${escapeHtml(model.name)}</div>
                <div class="model-provider">${escapeHtml(model.provider)}</div>
            </div>
            <span class="model-type ${typeClass}">${model.type}</span>
        </div>
    `;
}

/**
 * Get provider CSS class
 */
function getProviderClass(provider) {
    if (!provider) return '';
    return provider.toLowerCase().replace(/\s+/g, '-');
}

/**
 * Get type CSS class
 */
function getTypeClass(type) {
    if (!type) return '';
    return `type-${type.toLowerCase()}`;
}

/**
 * Get model initials
 */
function getModelInitials(name) {
    if (!name) return 'AI';
    const words = name.split(/[\s-]/);
    if (words.length >= 2) {
        return (words[0][0] + words[1][0]).toUpperCase();
    }
    return name.slice(0, 2).toUpperCase();
}

/**
 * Filter models by search query
 */
function filterModels(query) {
    if (!ChatDOM.modelList) return;
    
    const client = getPuterClient();
    const models = client.models;
    
    if (!query) {
        renderModelList(models);
        return;
    }
    
    const searchLower = query.toLowerCase();
    const filtered = models.filter(model => 
        model.name.toLowerCase().includes(searchLower) ||
        model.id.toLowerCase().includes(searchLower) ||
        model.provider.toLowerCase().includes(searchLower) ||
        model.description.toLowerCase().includes(searchLower)
    );
    
    renderModelList(filtered);
}

/**
 * Set current model
 */
function setCurrentModel(modelId) {
    const client = getPuterClient();
    const model = client.getModel(modelId);
    
    if (model) {
        ChatState.currentModel = model;
        client.setCurrentModel(modelId);
        
        // Update UI
        updateCurrentModelDisplay(model);
        
        // Update model items
        document.querySelectorAll('.model-item').forEach(item => {
            item.classList.toggle('active', item.dataset.modelId === modelId);
        });
        
        // Save state
        saveChatState();
        
        // Show welcome message if no messages
        if (ChatState.messages.length === 0) {
            showWelcomeMessage();
        }
    }
}

/**
 * Update current model display
 */
function updateCurrentModelDisplay(model) {
    if (!model) return;
    
    if (ChatDOM.currentModelName) {
        ChatDOM.currentModelName.textContent = model.name;
    }
    
    if (ChatDOM.currentModelAvatar) {
        ChatDOM.currentModelAvatar.textContent = getModelInitials(model.name);
        ChatDOM.currentModelAvatar.style.background = getProviderGradient(model.provider);
    }
    
    if (ChatDOM.currentModelStatus) {
        ChatDOM.currentModelStatus.textContent = 'Ready';
    }
}

/**
 * Get provider gradient
 */
function getProviderGradient(provider) {
    const gradients = {
        openai: 'linear-gradient(135deg, #00A67E 0%, #00D4AA 100%)',
        anthropic: 'linear-gradient(135deg, #FFC700 0%, #FFD700 100%)',
        google: 'linear-gradient(135deg, #4285F4 0%, #34A853 100%)',
        xai: 'linear-gradient(135deg, #FF6B35 0%, #FF8C42 100%)',
        mistral: 'linear-gradient(135deg, #FF69B4 0%, #FF1493 100%)',
        deepseek: 'linear-gradient(135deg, #00D4AA 0%, #00A67E 100%)',
        meta: 'linear-gradient(135deg, #1877F2 0%, #2563EB 100%)',
        cohere: 'linear-gradient(135deg, #3B82F6 0%, #1D4ED8 100%)',
        blackforest: 'linear-gradient(135deg, #7C3AED 0%, #5B21B6 100%)'
    };
    
    return gradients[provider?.toLowerCase()] || 'var(--gradient-primary)';
}

/**
 * Show welcome message
 */
function showWelcomeMessage() {
    if (ChatDOM.welcomeMessage) {
        ChatDOM.welcomeMessage.style.display = 'flex';
    }
    if (ChatDOM.messagesContainer) {
        ChatDOM.messagesContainer.style.alignItems = 'center';
        ChatDOM.messagesContainer.style.justifyContent = 'center';
    }
}

/**
 * Hide welcome message
 */
function hideWelcomeMessage() {
    if (ChatDOM.welcomeMessage) {
        ChatDOM.welcomeMessage.style.display = 'none';
    }
    if (ChatDOM.messagesContainer) {
        ChatDOM.messagesContainer.style.alignItems = 'flex-start';
        ChatDOM.messagesContainer.style.justifyContent = 'flex-start';
    }
}

/**
 * Handle input change
 */
function handleInputChange(e) {
    const value = e.target.value.trim();
    if (ChatDOM.sendBtn) {
        ChatDOM.sendBtn.disabled = value === '' || ChatState.isLoading;
    }
    
    // Auto-resize textarea
    autoResizeTextarea(e.target);
}

/**
 * Auto-resize textarea
 */
function autoResizeTextarea(textarea) {
    textarea.style.height = 'auto';
    textarea.style.height = Math.min(textarea.scrollHeight, 200) + 'px';
}

/**
 * Handle paste
 */
function handlePaste(e) {
    // Allow pasting
}

/**
 * Handle key down
 */
function handleKeyDown(e) {
    // Enter to send (Shift+Enter for new line)
    if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        if (!e.target.value.trim() || ChatState.isLoading) return;
        sendMessage();
    }
    
    // Escape to clear
    if (e.key === 'Escape') {
        e.target.value = '';
        handleInputChange(e);
    }
}

/**
 * Send message
 */
async function sendMessage() {
    if (!ChatDOM.promptInput) return;
    
    const prompt = ChatDOM.promptInput.value.trim();
    if (!prompt || ChatState.isLoading) return;
    
    // Clear input
    ChatDOM.promptInput.value = '';
    handleInputChange({ target: ChatDOM.promptInput });
    
    // Check if model is selected
    if (!ChatState.currentModel) {
        showToast('Please select a model first', 'error');
        return;
    }
    
    // Hide welcome message
    hideWelcomeMessage();
    
    // Add user message
    addMessage({
        role: 'user',
        content: prompt,
        timestamp: new Date()
    });
    
    // Set loading state
    ChatState.isLoading = true;
    if (ChatDOM.sendBtn) {
        ChatDOM.sendBtn.disabled = true;
    }
    if (ChatDOM.currentModelStatus) {
        ChatDOM.currentModelStatus.textContent = 'Thinking...';
    }
    
    // Get settings
    const settings = AppState.settings;
    
    try {
        const client = getPuterClient();
        
        // Check if streaming
        const stream = settings.stream && ChatDOM.streamToggle?.classList.contains('active');
        
        if (stream) {
            // Streaming chat
            await streamChat(prompt, settings);
        } else {
            // Non-streaming chat
            const response = await client.chat(prompt, {
                model: ChatState.currentModel.id,
                temperature: settings.temperature,
                maxTokens: settings.maxTokens
            });
            
            addMessage({
                role: 'assistant',
                content: response.message?.content || response,
                timestamp: new Date(),
                model: ChatState.currentModel.name
            });
        }
    } catch (error) {
        console.error('Chat error:', error);
        showToast('Failed to get response. Please try again.', 'error');
        addMessage({
            role: 'assistant',
            content: 'Sorry, I encountered an error processing your request. Please try again.',
            timestamp: new Date(),
            error: true
        });
    } finally {
        ChatState.isLoading = false;
        if (ChatDOM.sendBtn) {
            ChatDOM.sendBtn.disabled = false;
        }
        if (ChatDOM.currentModelStatus) {
            ChatDOM.currentModelStatus.textContent = 'Ready';
        }
        saveChatState();
    }
}

/**
 * Stream chat response
 */
async function streamChat(prompt, settings) {
    const client = getPuterClient();
    
    // Create a placeholder message
    const messageId = Date.now().toString();
    const placeholderMessage = {
        id: messageId,
        role: 'assistant',
        content: '',
        timestamp: new Date(),
        model: ChatState.currentModel.name,
        streaming: true
    };
    
    addMessage(placeholderMessage);
    
    try {
        let fullResponse = '';
        
        await client.chat(prompt, {
            model: ChatState.currentModel.id,
            temperature: settings.temperature,
            maxTokens: settings.maxTokens,
            stream: true
        }, {
            onchunk: (chunk) => {
                if (chunk.text) {
                    fullResponse += chunk.text;
                    updateMessage(messageId, { content: fullResponse });
                }
            },
            onend: () => {
                updateMessage(messageId, { 
                    content: fullResponse,
                    streaming: false
                });
            },
            onerror: (error) => {
                console.error('Stream error:', error);
                updateMessage(messageId, { 
                    content: 'Sorry, I encountered an error. Please try again.',
                    streaming: false,
                    error: true
                });
            }
        });
    } catch (error) {
        console.error('Stream chat error:', error);
        updateMessage(messageId, { 
            content: 'Sorry, I encountered an error. Please try again.',
            streaming: false,
            error: true
        });
    }
}

/**
 * Add message to chat
 */
function addMessage(message) {
    if (!ChatDOM.messagesContainer) return;
    
    const messageElement = createMessageElement(message);
    ChatDOM.messagesContainer.appendChild(messageElement);
    
    // Add to state
    ChatState.messages.push(message);
    
    // Scroll to bottom
    scrollToBottom();
}

/**
 * Update existing message
 */
function updateMessage(messageId, updates) {
    const messageElement = document.querySelector(`[data-message-id="${messageId}"]`);
    if (!messageElement) return;
    
    const message = ChatState.messages.find(m => m.id === messageId);
    if (message) {
        Object.assign(message, updates);
    }
    
    // Update content
    const contentElement = messageElement.querySelector('.message-text');
    if (contentElement && updates.content !== undefined) {
        contentElement.innerHTML = formatMessageContent(updates.content);
    }
    
    // Update streaming state
    if (updates.streaming !== undefined) {
        const typingElement = messageElement.querySelector('.typing-indicator');
        if (typingElement) {
            typingElement.style.display = updates.streaming ? 'flex' : 'none';
        }
    }
    
    // Update error state
    if (updates.error) {
        messageElement.classList.add('error');
    }
    
    scrollToBottom();
}

/**
 * Create message element
 */
function createMessageElement(message) {
    const isUser = message.role === 'user';
    const isAssistant = message.role === 'assistant';
    const isError = message.error;
    const isStreaming = message.streaming;
    const messageId = message.id || Date.now().toString();
    
    const div = document.createElement('div');
    div.className = `message ${isUser ? 'user' : 'ai'} ${isError ? 'error' : ''}`;
    div.dataset.messageId = messageId;
    
    const avatar = isUser ? '&#128100;' : '&#129302;';
    const initials = isUser ? 'YOU' : getModelInitials(message.model || 'AI');
    
    div.innerHTML = `
        <div class="message-avatar">${isUser ? avatar : initials}</div>
        <div class="message-content">
            <div class="message-bubble">
                <div class="message-text">${formatMessageContent(message.content)}</div>
                ${isStreaming ? '<div class="typing-indicator"><span></span><span></span><span></span></div>' : ''}
            </div>
            <div class="message-meta">
                <span class="message-time">${formatTime(message.timestamp)}</span>
                ${isAssistant && message.model ? `<span class="message-model">${escapeHtml(message.model)}</span>` : ''}
            </div>
        </div>
    `;
    
    return div;
}

/**
 * Format message content
 */
function formatMessageContent(content) {
    if (!content) return '<p>No content</p>';
    
    // Escape HTML first
    let formatted = escapeHtml(content);
    
    // Format markdown
    formatted = formatMarkdown(formatted);
    
    // Replace newlines with paragraphs
    formatted = formatted.replace(/\n\n/g, '</p><p>');
    formatted = formatted.replace(/\n/g, '<br>');
    
    // Wrap in paragraph if not already wrapped
    if (!formatted.startsWith('<') || formatted.startsWith('<p>')) {
        formatted = `<p>${formatted}</p>`;
    }
    
    return formatted;
}

/**
 * Scroll to bottom
 */
function scrollToBottom() {
    if (ChatDOM.messagesContainer) {
        ChatDOM.messagesContainer.scrollTop = ChatDOM.messagesContainer.scrollHeight;
    }
}

/**
 * Clear chat
 */
function clearChat() {
    if (confirm('Are you sure you want to clear this chat?')) {
        ChatState.messages = [];
        if (ChatDOM.messagesContainer) {
            ChatDOM.messagesContainer.innerHTML = '';
        }
        showWelcomeMessage();
        saveChatState();
    }
}

/**
 * New chat
 */
function newChat() {
    clearChat();
    // Could also switch to a new conversation
}

/**
 * Load chat state
 */
function loadChatState() {
    const savedState = localStorage.getItem('aether-chat-state');
    if (savedState) {
        try {
            const state = JSON.parse(savedState);
            Object.assign(ChatState, state);
            
            // Restore messages
            if (ChatState.messages.length > 0) {
                hideWelcomeMessage();
                ChatState.messages.forEach(message => {
                    addMessage(message);
                });
            }
            
            // Restore model
            if (ChatState.currentModel) {
                updateCurrentModelDisplay(ChatState.currentModel);
            }
        } catch (error) {
            console.error('Failed to load chat state:', error);
        }
    }
}

/**
 * Save chat state
 */
function saveChatState() {
    try {
        localStorage.setItem('aether-chat-state', JSON.stringify(ChatState));
    } catch (error) {
        console.error('Failed to save chat state:', error);
    }
}

/**
 * Toggle settings panel
 */
function toggleSettings() {
    if (ChatDOM.settingsPanel) {
        ChatDOM.settingsPanel.classList.toggle('visible');
    }
}

/**
 * Toggle streaming
 */
function toggleStreaming() {
    if (ChatDOM.streamToggle) {
        ChatDOM.streamToggle.classList.toggle('active');
        if (ChatDOM.streamStatus) {
            ChatDOM.streamStatus.textContent = ChatDOM.streamToggle.classList.contains('active') ? '&#10003;' : '&#10007;';
        }
        AppState.settings.stream = ChatDOM.streamToggle.classList.contains('active');
    }
}

/**
 * Use quick prompt
 */
function usePrompt(prompt) {
    if (ChatDOM.promptInput) {
        ChatDOM.promptInput.value = prompt;
        handleInputChange({ target: ChatDOM.promptInput });
        ChatDOM.promptInput.focus();
    }
}

/**
 * Copy last response
 */
function copyLastResponse() {
    const messages = ChatState.messages;
    const lastAssistantMessage = messages.filter(m => m.role === 'assistant').pop();
    
    if (lastAssistantMessage) {
        copyToClipboard(lastAssistantMessage.content);
        showToast('Response copied to clipboard', 'success');
    } else {
        showToast('No response to copy', 'error');
    }
}

/**
 * Export functions
 */
window.sendMessage = sendMessage;
window.clearChat = clearChat;
window.newChat = newChat;
window.toggleSettings = toggleSettings;
window.toggleStreaming = toggleStreaming;
window.usePrompt = usePrompt;
window.copyLastResponse = copyLastResponse;
window.setCurrentModel = setCurrentModel;
window.loadModels = loadModels;
