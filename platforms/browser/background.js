// Aetheris Browser Extension - Background Service Worker
// Handles LLM chat requests to local FastAPI server

const BASE_URL = 'http://127.0.0.1:8008';

// Cache for available models
let modelsCache = null;

// Fetch available models from the server
async function fetchModels() {
  try {
    const response = await fetch(`${BASE_URL}/models`);
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }
    const data = await response.json();
    modelsCache = data.data || [];
    return modelsCache;
  } catch (error) {
    console.error('Failed to fetch models:', error);
    // Return default models if server is down
    return [
      'puter/claude-fable-5',
      'puter/claude-fable-5.1',
      'puter/opus-5',
      'puter/sonnet-5'
    ];
  }
}

// Send chat request to the server
async function sendChat(model, prompt) {
  try {
    const response = await fetch(`${BASE_URL}/chat/completions`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        model: model,
        messages: [{ role: 'user', content: prompt }],
      }),
    });

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${await response.text()}`);
    }

    const data = await response.json();
    
    if (data.choices && data.choices.length > 0) {
      return data.choices[0].text;
    }
    
    throw new Error('No response text in choices');
  } catch (error) {
    console.error('Chat request failed:', error);
    throw error;
  }
}

// Stream chat request
async function* streamChat(model, prompt) {
  try {
    const response = await fetch(`${BASE_URL}/stream`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        model: model,
        messages: [{ role: 'user', content: prompt }],
      }),
    });

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }

    const reader = response.body.getReader();
    const decoder = new TextDecoder();
    
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      
      const chunk = decoder.decode(value, { stream: true });
      const lines = chunk.split('\n').filter(l => l.trim());
      
      for (const line of lines) {
        try {
          const data = JSON.parse(line);
          if (data.text) {
            yield data.text;
          }
        } catch (e) {
          // Skip parse errors for partial chunks
        }
      }
    }
  } catch (error) {
    console.error('Stream request failed:', error);
    throw error;
  }
}

// Message handler
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  (async () => {
    try {
      switch (request.method) {
        case 'list_models':
          const models = await fetchModels();
          sendResponse({ success: true, models });
          break;

        case 'send_chat':
          const { model, prompt } = request;
          const response = await sendChat(model, prompt);
          sendResponse({ success: true, response });
          break;

        case 'stream_chat':
          // For streaming, we need to handle it differently
          // This is a simplified version - full streaming would need port-based messaging
          const { model, prompt: streamPrompt } = request;
          const response = await sendChat(model, streamPrompt);
          sendResponse({ success: true, response, streaming: false });
          break;

        default:
          sendResponse({ success: false, error: 'Unknown method' });
      }
    } catch (error) {
      sendResponse({ success: false, error: error.message });
    }
  })();
  
  // Return true to indicate we want to send a response asynchronously
  return true;
});

// Initialize
console.log('Aetheris Background Service Worker loaded');
fetchModels().then(models => {
  console.log('Available models:', models);
});
