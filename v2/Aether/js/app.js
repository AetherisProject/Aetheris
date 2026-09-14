/**
 * Aether - AI Magic Showcase
 * Main Application Logic
 */

// Global state
let puter = null;
let currentDemo = null;
let isRunning = false;
let modelsLoaded = false;

// DOM elements
let demoInput, demoOutput, demoSubmit, modelSelect, demoSelect;

// Demo configurations
const demos = {
    email: {
        name: 'Write a Professional Email',
        description: 'Just describe what you need, AI writes the perfect email',
        category: 'work',
        icon: '✉️',
        promptTemplate: (input) => `Write a professional email about: ${input}`
    },
    logo: {
        name: 'Create a Beautiful Logo',
        description: 'Describe your business, AI creates a custom logo',
        category: 'business',
        icon: '🎨',
        promptTemplate: (input) => `Create a logo design for: ${input}. Make it professional and modern.`
    },
    summarize: {
        name: 'Summarize Document',
        description: 'Upload any document, AI extracts the key points',
        category: 'productivity',
        icon: '📄',
        promptTemplate: (input) => `Summarize the following text: ${input}`
    },
    code: {
        name: 'Write Code',
        description: 'Describe what you want, AI writes the code',
        category: 'development',
        icon: '💻',
        promptTemplate: (input) => `Write code for: ${input}. Include comments and explanations.`
    },
    translate: {
        name: 'Translate',
        description: 'Break language barriers instantly',
        category: 'communication',
        icon: '🌍',
        promptTemplate: (input) => `Translate the following to Spanish, French, and German: ${input}`
    },
    social: {
        name: 'Generate Social Media Posts',
        description: 'Create engaging content for any platform',
        category: 'marketing',
        icon: '📱',
        promptTemplate: (input) => `Create 3 engaging social media posts about: ${input}`
    },
    trip: {
        name: 'Plan My Perfect Trip',
        description: 'Get personalized itineraries with one request',
        category: 'travel',
        icon: '✈️',
        promptTemplate: (input) => `Create a detailed 7-day travel itinerary for: ${input}. Include activities, restaurants, and accommodations.`
    },
    learn: {
        name: 'Learn Anything',
        description: 'Get personalized lessons on any topic',
        category: 'education',
        icon: '🎓',
        promptTemplate: (input) => `Explain ${input} in a way that a 10-year-old can understand.`
    },
    data: {
        name: 'Analyze My Data',
        description: 'Upload data, AI finds insights and creates charts',
        category: 'business',
        icon: '📊',
        promptTemplate: (input) => `Analyze the following data and provide insights: ${input}`
    },
    meal: {
        name: 'Create a Meal Plan',
        description: 'Get personalized recipes based on your needs',
        category: 'health',
        icon: '🍽️',
        promptTemplate: (input) => `Create a 7-day meal plan for: ${input}. Include recipes and shopping lists.`
    }
};

// Demo types for the interactive demo
const demoTypes = {
    chat: {
        name: 'Chat with AI',
        run: async (prompt, model) => {
            if (!puter) throw new Error('Puter not loaded');
            const response = await puter.ai.chat(prompt, { model });
            return response.text || response.message?.content || String(response);
        }
    },
    write: {
        name: 'Write Something',
        run: async (prompt, model) => {
            if (!puter) throw new Error('Puter not loaded');
            const response = await puter.ai.chat(`Write: ${prompt}`, { model });
            return response.text || response.message?.content || String(response);
        }
    },
    generate: {
        name: 'Generate Image',
        run: async (prompt, model) => {
            if (!puter) throw new Error('Puter not loaded');
            try {
                const blob = await puter.ai.txt2img(prompt);
                const url = URL.createObjectURL(blob);
                return `<img src="${url}" alt="Generated image" style="max-width: 100%; border-radius: 8px; margin-top: 1rem; box-shadow: 0 4px 6px rgba(0,0,0,0.3);">`;
            } catch (error) {
                // Fallback for models that don't support image generation
                const response = await puter.ai.chat(`Describe what an image of "${prompt}" would look like in vivid detail.`, { model });
                return `<div style="padding: 1rem; background: var(--surface); border-radius: 8px; color: var(--text-secondary);">
                    <p><strong>Image Description:</strong></p>
                    <p>${response.text || response.message?.content || String(response)}</p>
                    <p style="margin-top: 1rem; font-size: 0.875rem;">Note: Image generation not available for this model. Here's a description instead.</p>
                </div>`;
            }
        }
    },
    summarize: {
        name: 'Summarize Text',
        run: async (prompt, model) => {
            if (!puter) throw new Error('Puter not loaded');
            const response = await puter.ai.chat(`Summarize the following text in 3-5 bullet points:\n\n${prompt}`, { model });
            return response.text || response.message?.content || String(response);
        }
    },
    translate: {
        name: 'Translate',
        run: async (prompt, model) => {
            if (!puter) throw new Error('Puter not loaded');
            const response = await puter.ai.chat(`Translate the following text to Spanish, French, and German:\n\n${prompt}`, { model });
            return response.text || response.message?.content || String(response);
        }
    },
    code: {
        name: 'Write Code',
        run: async (prompt, model) => {
            if (!puter) throw new Error('Puter not loaded');
            const response = await puter.ai.chat(`Write clean, well-commented code for: ${prompt}`, { model });
            return `<pre style="background: var(--surface); padding: 1rem; border-radius: 8px; overflow-x: auto; font-family: var(--font-mono); font-size: 0.875rem;">${escapeHtml(response.text || response.message?.content || String(response))}</pre>`;
        }
    }
};

// Available models (will be populated from Puter)
let availableModels = [
    { id: 'gpt-4o-mini', name: 'GPT-4o Mini (Fast & Smart)' },
    { id: 'claude-3-haiku', name: 'Claude 3 Haiku (Quick & Helpful)' },
    { id: 'gemini-1.5-flash', name: 'Gemini 1.5 Flash (Creative)' },
    { id: 'llama-3.1-70b', name: 'Llama 3.1 70B (Open Source)' },
    { id: 'mistral-large', name: 'Mistral Large (Precise)' }
];

/**
 * Initialize the application
 */
async function init() {
    // Get DOM elements
    demoInput = document.getElementById('demo-input');
    demoOutput = document.getElementById('demo-output');
    demoSubmit = document.getElementById('demo-submit');
    modelSelect = document.getElementById('model-select');
    demoSelect = document.getElementById('demo-select');
    
    // Load Puter
    await loadPuter();
    
    // Set up event listeners
    setupEventListeners();
    
    // Auto-focus input
    if (demoInput) {
        demoInput.focus();
    }
    
    // Enable submit button when input has content
    if (demoInput && demoSubmit) {
        demoInput.addEventListener('input', () => {
            demoSubmit.disabled = !demoInput.value.trim();
        });
    }
    
    // Load models
    await loadModels();
    
    // Add scroll animations
    setupScrollAnimations();
    
    console.log('Aether app initialized');
}

/**
 * Load Puter.js
 */
async function loadPuter() {
    return new Promise((resolve) => {
        if (window.puter) {
            puter = window.puter;
            resolve();
            return;
        }
        
        const script = document.createElement('script');
        script.src = 'https://js.puter.com/v2/';
        script.onload = () => {
            puter = window.puter;
            console.log('Puter loaded successfully');
            resolve();
        };
        script.onerror = () => {
            console.error('Failed to load Puter');
            // Create a mock puter for demo purposes
            puter = {
                ai: {
                    chat: async (prompt, options) => ({
                        text: `Mock response: I received your request: "${prompt}". In a real implementation, this would be processed by the AI model.`
                    }),
                    txt2img: async (prompt) => {
                        throw new Error('Image generation not available in mock mode');
                    }
                }
            };
            resolve();
        };
        document.head.appendChild(script);
    });
}

/**
 * Load available models from Puter
 */
async function loadModels() {
    try {
        if (!puter || !puter.ai) return;
        
        // Try to get available models
        // Note: Puter.js v2 might have a models() method
        if (puter.ai.models) {
            const models = await puter.ai.models();
            availableModels = models.map(m => ({
                id: m.id || m.model,
                name: m.name || m.id || m.model
            }));
        }
        
        // Update model selector
        updateModelSelector();
        modelsLoaded = true;
    } catch (error) {
        console.log('Could not load models:', error);
        // Use default models
        updateModelSelector();
    }
}

/**
 * Update the model selector with available models
 */
function updateModelSelector() {
    if (!modelSelect) return;
    
    // Clear existing options
    modelSelect.innerHTML = '';
    
    // Add default models
    availableModels.forEach(model => {
        const option = document.createElement('option');
        option.value = model.id;
        option.textContent = model.name;
        modelSelect.appendChild(option);
    });
}

/**
 * Set up event listeners
 */
function setupEventListeners() {
    // Demo submit
    if (demoSubmit) {
        demoSubmit.addEventListener('click', runDemo);
    }
    
    // Allow Enter key to submit
    if (demoInput) {
        demoInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter' && !e.shiftKey && !demoSubmit.disabled) {
                e.preventDefault();
                runDemo();
            }
        });
    }
    
    // Demo select change - update placeholder
    if (demoSelect) {
        demoSelect.addEventListener('change', () => {
            const demoType = demoSelect.value;
            const demo = demoTypes[demoType];
            if (demo && demoInput) {
                demoInput.placeholder = getPlaceholderForDemoType(demoType);
            }
        });
    }
}

/**
 * Get placeholder text for demo type
 */
function getPlaceholderForDemoType(type) {
    const placeholders = {
        chat: 'Ask me anything... (e.g., "What is artificial intelligence?")',
        write: 'What would you like me to write? (e.g., "A poem about the ocean")',
        generate: 'Describe the image you want to create... (e.g., "A sunset over mountains")',
        summarize: 'Paste or type the text you want summarized...',
        translate: 'Type the text you want translated...',
        code: 'Describe the code you need... (e.g., "A Python function to sort a list")'
    };
    return placeholders[type] || 'Describe what you want to create...';
}

/**
 * Run the selected demo
 */
async function runDemo() {
    if (isRunning || !demoInput || !demoOutput || !demoSubmit) return;
    
    const prompt = demoInput.value.trim();
    if (!prompt) return;
    
    isRunning = true;
    const model = modelSelect?.value || 'gpt-4o-mini';
    const demoType = demoSelect?.value || 'chat';
    const demo = demoTypes[demoType];
    
    if (!demo) {
        showError('Invalid demo type selected');
        return;
    }
    
    // Show loading state
    toggleLoadingState(true);
    demoOutput.innerHTML = '<div class="loading"><i class="fas fa-spinner fa-spin"></i><br>Generating with AI...</div>';
    
    try {
        const result = await demo.run(prompt, model);
        
        // Format the result
        let formattedResult = `<div class="result">`;
        if (demoType === 'chat' || demoType === 'write' || demoType === 'summarize' || demoType === 'translate') {
            formattedResult += `<h4><i class="fas fa-robot"></i> AI Response:</h4>`;
        } else if (demoType === 'generate') {
            formattedResult += `<h4><i class="fas fa-image"></i> Generated Image:</h4>`;
        } else if (demoType === 'code') {
            formattedResult += `<h4><i class="fas fa-code"></i> Generated Code:</h4>`;
        }
        formattedResult += result + `</div>`;
        
        demoOutput.innerHTML = formattedResult;
        
        // Scroll to output
        demoOutput.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
        
    } catch (error) {
        console.error('Demo error:', error);
        showError(`Error: ${error.message || String(error)}`);
    } finally {
        isRunning = false;
        toggleLoadingState(false);
    }
}

/**
 * Toggle loading state
 */
function toggleLoadingState(loading) {
    if (!demoSubmit) return;
    
    const buttonText = demoSubmit.querySelector('.button-text');
    const buttonLoading = demoSubmit.querySelector('.button-loading');
    
    if (buttonText) buttonText.style.display = loading ? 'none' : 'inline';
    if (buttonLoading) buttonLoading.style.display = loading ? 'inline' : 'none';
    demoSubmit.disabled = loading;
}

/**
 * Show error message
 */
function showError(message) {
    if (!demoOutput) return;
    demoOutput.innerHTML = `
        <div class="error">
            <i class="fas fa-exclamation-triangle"></i>
            <p><strong>Error:</strong> ${escapeHtml(message)}</p>
            <p>Please try again or select a different demo.</p>
        </div>
    `;
}

/**
 * Select a demo from the showcase
 */
function selectDemo(demoKey) {
    const demo = demos[demoKey];
    if (!demo) return;
    
    // Use the new modal approach
    openDemoModal(demoKey);
}

/**
 * Get sample prompt for demo
 */
function getSamplePrompt(demoKey) {
    const samples = {
        email: 'Write an email to my boss asking for a raise',
        logo: 'Create a logo for a coffee shop called Brew Haven',
        summarize: 'Summarize the key points of artificial intelligence',
        code: 'Create a Python function to calculate Fibonacci sequence',
        translate: 'Hello, how are you today?',
        social: 'Create social media posts about a new product launch',
        trip: 'Plan a 7-day trip to Japan',
        learn: 'Explain quantum computing to a 10-year-old',
        data: 'Analyze sales data and provide insights',
        meal: 'Create a vegetarian meal plan for a week'
    };
    return samples[demoKey] || '';
}

/**
 * Show random demo
 */
function showRandomDemo() {
    const demoKeys = Object.keys(demos);
    const randomDemo = demoKeys[Math.floor(Math.random() * demoKeys.length)];
    openDemoModal(randomDemo);
}

/**
 * Scroll to demos section
 */
function scrollToDemos() {
    const demoShowcase = document.getElementById('demo-showcase');
    if (demoShowcase) {
        demoShowcase.scrollIntoView({ behavior: 'smooth' });
    }
}

/**
 * Filter demos by category
 */
function filterDemos(category) {
    // Scroll to demo showcase
    scrollToDemos();
    
    // Highlight demos of this category
    const demoCards = document.querySelectorAll('.demo-card');
    demoCards.forEach(card => {
        const cardCategory = card.querySelector('.category')?.textContent?.toLowerCase();
        if (cardCategory === category) {
            card.style.borderColor = 'var(--primary)';
            card.style.boxShadow = 'var(--shadow-glow)';
        } else {
            card.style.borderColor = 'var(--border)';
            card.style.boxShadow = 'none';
        }
    });
    
    // Scroll to first matching demo
    const firstMatching = Array.from(demoCards).find(card => 
        card.querySelector('.category')?.textContent?.toLowerCase() === category
    );
    if (firstMatching) {
        firstMatching.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
    }
}

/**
 * Set up scroll animations
 */
function setupScrollAnimations() {
    const animatedElements = document.querySelectorAll('.demo-card, .step, .category-card, .testimonial-card');
    
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('animated');
            }
        });
    }, {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    });
    
    animatedElements.forEach(el => {
        el.classList.add('animate-on-scroll');
        observer.observe(el);
    });
}

/**
 * Escape HTML to prevent XSS
 */
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', init);

// Also run init if already loaded
if (document.readyState !== 'loading') {
    init();
}

// Export for other modules
window.AetherApp = {
    init,
    runDemo,
    selectDemo,
    showRandomDemo,
    scrollToDemos,
    filterDemos,
    demos,
    demoTypes
};

// ============================================
// DEMO MODAL FUNCTIONS
// ============================================

let currentDemoConfig = null;
let currentDemoOptions = {};

/**
 * Open demo modal with configuration
 */
function openDemoModal(demoKey) {
    const demo = DemoRegistry.get(demoKey);
    if (!demo) return;
    
    currentDemoConfig = demo;
    currentDemoOptions = {};
    
    // Set modal content
    const modal = document.getElementById('demo-modal');
    const title = document.getElementById('demo-modal-title');
    const description = document.getElementById('demo-modal-description');
    const input = document.getElementById('demo-modal-input');
    const optionsContainer = document.getElementById('demo-modal-options');
    
    title.textContent = demo.name;
    description.textContent = demo.description;
    input.placeholder = demo.inputPlaceholder || 'Enter your request...';
    input.value = '';
    
    // Clear previous options
    optionsContainer.innerHTML = '';
    
    // Add options if they exist
    if (demo.options && demo.options.length > 0) {
        demo.options.forEach(option => {
            const optionGroup = document.createElement('div');
            optionGroup.className = 'demo-option-group';
            
            const label = document.createElement('label');
            label.textContent = option.label;
            label.htmlFor = `demo-option-${option.name}`;
            
            if (option.type === 'select') {
                const select = document.createElement('select');
                select.id = `demo-option-${option.name}`;
                select.name = option.name;
                
                option.values.forEach(value => {
                    const opt = document.createElement('option');
                    opt.value = value;
                    opt.textContent = value;
                    select.appendChild(opt);
                });
                
                // Set default value
                if (option.values.length > 0) {
                    currentDemoOptions[option.name] = option.values[0];
                }
                
                select.addEventListener('change', () => {
                    currentDemoOptions[option.name] = select.value;
                });
                
                optionGroup.appendChild(label);
                optionGroup.appendChild(select);
            } else if (option.type === 'multiselect') {
                const multiselect = document.createElement('div');
                multiselect.className = 'demo-multiselect';
                
                option.values.forEach(value => {
                    const checkboxLabel = document.createElement('label');
                    const checkbox = document.createElement('input');
                    checkbox.type = 'checkbox';
                    checkbox.name = option.name;
                    checkbox.value = value;
                    checkbox.id = `demo-option-${option.name}-${value}`;
                    
                    const span = document.createElement('span');
                    span.textContent = value;
                    
                    checkboxLabel.appendChild(checkbox);
                    checkboxLabel.appendChild(span);
                    multiselect.appendChild(checkboxLabel);
                    
                    // Add event listener
                    checkbox.addEventListener('change', () => {
                        if (!currentDemoOptions[option.name]) {
                            currentDemoOptions[option.name] = [];
                        }
                        const options = currentDemoOptions[option.name];
                        if (checkbox.checked) {
                            if (!options.includes(value)) {
                                options.push(value);
                            }
                        } else {
                            const index = options.indexOf(value);
                            if (index > -1) {
                                options.splice(index, 1);
                            }
                        }
                    });
                });
                
                optionGroup.appendChild(label);
                optionGroup.appendChild(multiselect);
            }
            
            optionsContainer.appendChild(optionGroup);
        });
    }
    
    // Show modal
    modal.classList.add('active');
    input.focus();
    
    // Disable body scroll
    document.body.style.overflow = 'hidden';
}

/**
 * Close demo modal
 */
function closeDemoModal() {
    const modal = document.getElementById('demo-modal');
    modal.classList.remove('active');
    
    // Enable body scroll
    document.body.style.overflow = '';
    
    // Reset current demo
    currentDemoConfig = null;
    currentDemoOptions = {};
}

/**
 * Run demo from modal
 */
async function runDemoFromModal() {
    if (!currentDemoConfig) return;
    
    const input = document.getElementById('demo-modal-input');
    const runButton = document.getElementById('demo-modal-run');
    const prompt = input.value.trim();
    
    if (!prompt) {
        showError('Please enter a request');
        return;
    }
    
    // Show loading state
    runButton.disabled = true;
    
    try {
        // Close the input modal
        closeDemoModal();
        
        // Show loading in results modal
        showDemoLoading();
        
        // Run the demo
        const result = await currentDemoConfig.run(prompt, currentDemoOptions, puter);
        
        // Show results
        showDemoResults(result, currentDemoConfig.name);
        
    } catch (error) {
        showError(`Error: ${error.message}`);
        closeResultsModal();
    } finally {
        runButton.disabled = false;
    }
}

/**
 * Show demo loading state
 */
function showDemoLoading() {
    const resultsModal = document.getElementById('demo-results-modal');
    const title = document.getElementById('demo-results-title');
    const content = document.getElementById('demo-results-content');
    
    title.textContent = 'Generating...';
    content.innerHTML = '<div class="loading"><i class="fas fa-spinner fa-spin"></i><p>AI is working its magic...</p></div>';
    
    resultsModal.classList.add('active');
    document.body.style.overflow = 'hidden';
}

/**
 * Show demo results
 */
function showDemoResults(result, demoName) {
    const resultsModal = document.getElementById('demo-results-modal');
    const title = document.getElementById('demo-results-title');
    const content = document.getElementById('demo-results-content');
    
    title.textContent = demoName || 'Results';
    
    if (typeof result === 'string') {
        content.innerHTML = result;
    } else if (result && result.type) {
        switch (result.type) {
            case 'text':
                content.innerHTML = `<div class="result"><h4>${result.title || 'Result'}</h4><div style="white-space: pre-wrap;">${result.content}</div></div>`;
                break;
            case 'image':
                const imgUrl = typeof result.content === 'string' ? result.content : URL.createObjectURL(result.content);
                content.innerHTML = `<div class="result"><h4>${result.title || 'Generated Image'}</h4><img src="${imgUrl}" alt="Generated content" style="max-width: 100%; border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.3);"></div>`;
                break;
            case 'code':
                content.innerHTML = `<div class="result"><h4>${result.title || 'Generated Code'}</h4><pre style="background: var(--surface); padding: 1rem; border-radius: 8px; overflow-x: auto; font-family: var(--font-mono); font-size: 0.875rem; white-space: pre-wrap; word-wrap: break-word;">${escapeHtml(result.content)}</pre></div>`;
                break;
            default:
                content.innerHTML = `<div class="result"><h4>${result.title || 'Result'}</h4><div style="white-space: pre-wrap;">${result.content}</div></div>`;
        }
    } else {
        content.innerHTML = `<div class="result"><h4>Result</h4><div style="white-space: pre-wrap;">${String(result)}</div></div>`;
    }
    
    resultsModal.classList.add('active');
    document.body.style.overflow = 'hidden';
}

/**
 * Close results modal
 */
function closeResultsModal() {
    const resultsModal = document.getElementById('demo-results-modal');
    resultsModal.classList.remove('active');
    document.body.style.overflow = '';
}

/**
 * Copy results to clipboard
 */
function copyResults() {
    const content = document.getElementById('demo-results-content');
    const text = content.textContent || content.innerText;
    
    if (text) {
        navigator.clipboard.writeText(text).then(() => {
            // Show success message
            const copyBtn = document.getElementById('demo-copy-btn');
            const originalText = copyBtn.innerHTML;
            copyBtn.innerHTML = '<i class="fas fa-check"></i> Copied!';
            
            setTimeout(() => {
                copyBtn.innerHTML = originalText;
            }, 2000);
        }).catch(err => {
            showError('Failed to copy: ' + err.message);
        });
    }
}


// Close modals on escape key
document.addEventListener('keydown', function(event) {
    if (event.key === 'Escape') {
        closeDemoModal();
        closeResultsModal();
    }
});

// Close modals when clicking outside
document.addEventListener('click', function(event) {
    const demoModal = document.getElementById('demo-modal');
    const resultsModal = document.getElementById('demo-results-modal');
    
    if (event.target === demoModal) {
        closeDemoModal();
    }
    
    if (event.target === resultsModal) {
        closeResultsModal();
    }
});