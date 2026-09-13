// Aetheris Browser Extension - Chat Popup

document.addEventListener('DOMContentLoaded', () => {
    const modelSelect = document.getElementById('modelSelect');
    const promptInput = document.getElementById('promptInput');
    const sendButton = document.getElementById('sendButton');
    const conversation = document.getElementById('conversation');
    const serverStatus = document.getElementById('serverStatus');
    
    let models = [];
    let serverOnline = false;
    
    // Update server status display
    function updateServerStatus(online) {
        serverOnline = online;
        serverStatus.textContent = online ? 'Server: Online' : 'Server: Offline';
        serverStatus.className = online ? 'status online' : 'status offline';
        sendButton.disabled = !online || models.length === 0;
    }
    
    // Add message to conversation
    function addMessage(role, text, model = '') {
        const messageDiv = document.createElement('div');
        messageDiv.className = `message ${role}`;
        
        if (model && role === 'assistant') {
            const modelSpan = document.createElement('span');
            modelSpan.style.fontSize = '11px';
            modelSpan.style.color = '#888';
            modelSpan.style.marginRight = '10px';
            modelSpan.textContent = `[${model}]`;
            messageDiv.prepend(modelSpan);
        }
        
        const textNode = document.createElement('span');
        textNode.textContent = text;
        messageDiv.appendChild(textNode);
        
        conversation.appendChild(messageDiv);
        conversation.scrollTop = conversation.scrollHeight;
    }
    
    // Show loading state
    function showLoading() {
        const loadingDiv = document.createElement('div');
        loadingDiv.className = 'message system';
        loadingDiv.innerHTML = '<span class="loading"></span> Thinking...';
        conversation.appendChild(loadingDiv);
        conversation.scrollTop = conversation.scrollHeight;
        return loadingDiv;
    }
    
    // Remove loading state
    function hideLoading(loadingElement) {
        if (loadingElement) {
            conversation.removeChild(loadingElement);
        }
    }
    
    // Load models from background service
    async function loadModels() {
        try {
            const response = await chrome.runtime.sendMessage({ method: 'list_models' });
            if (response.success && response.models) {
                models = response.models;
                updateModelSelect();
                updateServerStatus(true);
            } else {
                // Fallback to default models
                models = [
                    'puter/claude-fable-5',
                    'puter/claude-fable-5.1',
                    'puter/opus-5',
                    'puter/sonnet-5'
                ];
                updateModelSelect();
                updateServerStatus(false);
            }
        } catch (error) {
            console.error('Failed to load models:', error);
            models = [
                'puter/claude-fable-5',
                'puter/claude-fable-5.1',
                'puter/opus-5',
                'puter/sonnet-5'
            ];
            updateModelSelect();
            updateServerStatus(false);
        }
    }
    
    // Update model dropdown
    function updateModelSelect() {
        modelSelect.innerHTML = '';
        
        // Add friendly labels
        const friendlyNames = {
            'puter/claude-fable-5': 'Fable 5',
            'puter/claude-fable-5.1': 'Fable 5.1',
            'puter/opus-5': 'Opus 5',
            'puter/sonnet-5': 'Sonnet 5'
        };
        
        models.forEach(model => {
            const option = document.createElement('option');
            option.value = model;
            option.textContent = friendlyNames[model] || model;
            modelSelect.appendChild(option);
        });
        
        sendButton.disabled = models.length === 0;
    }
    
    // Send chat message
    async function sendMessage() {
        const prompt = promptInput.value.trim();
        if (!prompt || !modelSelect.value) return;
        
        const model = modelSelect.value;
        const loadingElement = showLoading();
        promptInput.value = '';
        
        // Add user message
        addMessage('user', prompt);
        
        try {
            const response = await chrome.runtime.sendMessage({
                method: 'send_chat',
                model: model,
                prompt: prompt
            });
            
            hideLoading(loadingElement);
            
            if (response.success) {
                // Get friendly model name
                const friendlyNames = {
                    'puter/claude-fable-5': 'Fable 5',
                    'puter/claude-fable-5.1': 'Fable 5.1',
                    'puter/opus-5': 'Opus 5',
                    'puter/sonnet-5': 'Sonnet 5'
                };
                const friendlyName = friendlyNames[model] || model;
                addMessage('assistant', response.response, friendlyName);
            } else {
                addMessage('system', `Error: ${response.error || 'Unknown error'}`);
            }
        } catch (error) {
            hideLoading(loadingElement);
            addMessage('system', `Error: ${error.message}`);
        }
    }
    
    // Event listeners
    sendButton.addEventListener('click', sendMessage);
    
    promptInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            sendMessage();
        }
    });
    
    modelSelect.addEventListener('change', () => {
        sendButton.disabled = !modelSelect.value;
    });
    
    // Initialize
    loadModels();
    
    // Check server status periodically
    setInterval(async () => {
        try {
            const response = await chrome.runtime.sendMessage({ method: 'list_models' });
            updateServerStatus(response.success);
        } catch {
            updateServerStatus(false);
        }
    }, 30000);
});
