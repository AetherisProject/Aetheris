/**
 * Aether Demo - Research Tools Functionality
 * Handles research tools, input processing, and output display
 */

// Research state
const ResearchState = {
    currentTool: 'search',
    inputData: {},
    outputData: null,
    history: [],
    isRunning: false
};

// DOM Elements
const ResearchDOM = {
    toolList: null,
    toolInput: null,
    outputContent: null,
    runBtn: null,
    currentToolName: null,
    toolDescriptionTitle: null,
    toolDescriptionText: null,
    welcomeResearch: null,
    historyPanel: null,
    historyList: null
};

// Tool configurations
const ToolConfigs = {
    search: {
        name: 'Web Search',
        description: 'Search the web and get AI-powered summaries of the results.',
        icon: '&#128269;',
        inputForm: 'input-search',
        process: processSearch
    },
    summarize: {
        name: 'Summarize',
        description: 'Condense long text into a concise summary.',
        icon: '&#128196;',
        inputForm: 'input-summarize',
        process: processSummarize
    },
    translate: {
        name: 'Translate',
        description: 'Translate text between 100+ languages.',
        icon: '&#127757;',
        inputForm: 'input-translate',
        process: processTranslate
    },
    ocr: {
        name: 'OCR',
        description: 'Extract text from images using optical character recognition.',
        icon: '&#128247;',
        inputForm: 'input-ocr',
        process: processOCR
    },
    analyze: {
        name: 'Analyze',
        description: 'Analyze documents and extract insights.',
        icon: '&#128200;',
        inputForm: 'input-analyze',
        process: processAnalyze
    },
    'code-review': {
        name: 'Code Review',
        description: 'Review code for bugs, performance, and style issues.',
        icon: '&#128187;',
        inputForm: 'input-code-review',
        process: processCodeReview
    },
    generate: {
        name: 'Generate',
        description: 'Generate content based on prompts.',
        icon: '&#9998;',
        inputForm: 'input-generate',
        process: processGenerate
    }
};

// Initialize research
document.addEventListener('DOMContentLoaded', () => {
    initializeResearchDOM();
    initializeResearchEventListeners();
    selectTool('search');
});

/**
 * Initialize research DOM references
 */
function initializeResearchDOM() {
    ResearchDOM.toolList = document.getElementById('tool-list');
    ResearchDOM.toolInput = document.getElementById('tool-input');
    ResearchDOM.outputContent = document.getElementById('output-content');
    ResearchDOM.runBtn = document.getElementById('run-btn');
    ResearchDOM.currentToolName = document.getElementById('current-tool-name');
    ResearchDOM.toolDescriptionTitle = document.getElementById('tool-description-title');
    ResearchDOM.toolDescriptionText = document.getElementById('tool-description-text');
    ResearchDOM.welcomeResearch = document.getElementById('welcome-research');
    ResearchDOM.historyPanel = document.getElementById('history-panel');
    ResearchDOM.historyList = document.getElementById('history-list');
}

/**
 * Initialize research event listeners
 */
function initializeResearchEventListeners() {
    // Tool selection
    if (ResearchDOM.toolList) {
        ResearchDOM.toolList.querySelectorAll('.tool-item').forEach(item => {
            item.addEventListener('click', () => {
                const tool = item.dataset.tool;
                selectTool(tool);
            });
        });
    }
    
    // File upload handlers
    setupFileUploadHandlers();
}

/**
 * Set up file upload handlers
 */
function setupFileUploadHandlers() {
    // OCR file upload
    const ocrFile = document.getElementById('ocr-file');
    if (ocrFile) {
        ocrFile.addEventListener('change', (e) => handleFileUpload(e, 'ocr'));
    }
    
    // Analyze file upload
    const analyzeFile = document.getElementById('analyze-file');
    if (analyzeFile) {
        analyzeFile.addEventListener('change', (e) => handleFileUpload(e, 'analyze'));
    }
    
    // Drag and drop for upload areas
    const uploadAreas = document.querySelectorAll('.file-upload-area');
    uploadAreas.forEach(area => {
        area.addEventListener('dragover', (e) => {
            e.preventDefault();
            area.classList.add('active');
        });
        
        area.addEventListener('dragleave', () => {
            area.classList.remove('active');
        });
        
        area.addEventListener('drop', (e) => {
            e.preventDefault();
            area.classList.remove('active');
            const files = e.dataTransfer.files;
            handleDroppedFiles(files, area.id);
        });
    });
}

/**
 * Select a tool
 */
function selectTool(tool) {
    ResearchState.currentTool = tool;
    const config = ToolConfigs[tool];
    
    if (!config) return;
    
    // Update tool list
    if (ResearchDOM.toolList) {
        ResearchDOM.toolList.querySelectorAll('.tool-item').forEach(item => {
            item.classList.toggle('active', item.dataset.tool === tool);
        });
    }
    
    // Update header
    if (ResearchDOM.currentToolName) {
        ResearchDOM.currentToolName.textContent = config.name;
    }
    
    // Update description
    if (ResearchDOM.toolDescriptionTitle) {
        ResearchDOM.toolDescriptionTitle.textContent = config.name;
    }
    if (ResearchDOM.toolDescriptionText) {
        ResearchDOM.toolDescriptionText.textContent = config.description;
    }
    
    // Show input form
    if (ResearchDOM.toolInput) {
        Object.values(ToolConfigs).forEach(cfg => {
            const form = document.getElementById(cfg.inputForm);
            if (form) {
                form.style.display = cfg.inputForm === config.inputForm ? 'block' : 'none';
            }
        });
    }
    
    // Hide welcome screen
    if (ResearchDOM.welcomeResearch) {
        ResearchDOM.welcomeResearch.style.display = 'none';
    }
    
    // Clear output
    clearOutput();
    
    // Save state
    saveResearchState();
}

/**
 * Handle file upload
 */
function handleFileUpload(e, tool) {
    const files = e.target.files;
    if (files.length === 0) return;
    
    handleFiles(files, tool);
    
    // Reset input
    e.target.value = '';
}

/**
 * Handle dropped files
 */
function handleDroppedFiles(files, areaId) {
    const toolMap = {
        'ocr-upload': 'ocr',
        'analyze-upload': 'analyze'
    };
    
    const tool = toolMap[areaId];
    if (tool) {
        handleFiles(files, tool);
    }
}

/**
 * Handle files for a specific tool
 */
function handleFiles(files, tool) {
    const fileListId = `${tool}-file-list`;
    const fileList = document.getElementById(fileListId);
    
    if (!fileList) return;
    
    // Clear previous files
    fileList.innerHTML = '';
    
    // Store files in state
    ResearchState.inputData.files = Array.from(files);
    
    // Display files
    Array.from(files).forEach(file => {
        const fileItem = document.createElement('div');
        fileItem.className = 'file-item';
        fileItem.innerHTML = `
            <span class="file-icon">${getFileIcon(getFileType(getFileExtension(file.name)))}</span>
            <span class="file-name">${escapeHtml(file.name)}</span>
            <span class="file-size">${formatFileSize(file.size)}</span>
            <span class="file-remove" onclick="removeFile('${tool}', '${file.name}')">&times;</span>
        `;
        fileList.appendChild(fileItem);
    });
}

/**
 * Remove file from list
 */
function removeFile(tool, filename) {
    if (!ResearchState.inputData.files) return;
    
    ResearchState.inputData.files = ResearchState.inputData.files.filter(
        file => file.name !== filename
    );
    
    const fileListId = `${tool}-file-list`;
    const fileList = document.getElementById(fileListId);
    if (fileList) {
        const fileItem = fileList.querySelector(`.file-item .file-name:contains("${filename}")`);
        // Simple removal - in production, use a better selector
        const items = fileList.querySelectorAll('.file-item');
        items.forEach(item => {
            const nameSpan = item.querySelector('.file-name');
            if (nameSpan && nameSpan.textContent === filename) {
                item.remove();
            }
        });
    }
}

/**
 * Run the current tool
 */
async function runTool() {
    if (ResearchState.isRunning) return;
    
    const tool = ResearchState.currentTool;
    const config = ToolConfigs[tool];
    
    if (!config || !config.process) return;
    
    // Collect input data
    collectInputData(tool);
    
    // Validate input
    if (!validateInput(tool)) {
        showToast('Please provide required input', 'error');
        return;
    }
    
    // Set running state
    ResearchState.isRunning = true;
    if (ResearchDOM.runBtn) {
        ResearchDOM.runBtn.disabled = true;
        ResearchDOM.runBtn.innerHTML = '<span class="btn-icon">&#8634;</span> Processing...';
    }
    
    // Clear output
    clearOutput();
    showLoading('output-content', 'Processing...');
    
    try {
        // Process with the tool
        const result = await config.process(ResearchState.inputData);
        
        // Display result
        displayOutput(result);
        
        // Save to history
        saveToHistory(tool, ResearchState.inputData, result);
    } catch (error) {
        console.error('Tool error:', error);
        showError('output-content', error.message || 'Failed to process request');
        showToast('Failed to process request', 'error');
    } finally {
        ResearchState.isRunning = false;
        if (ResearchDOM.runBtn) {
            ResearchDOM.runBtn.disabled = false;
            ResearchDOM.runBtn.innerHTML = '<span class="btn-icon">&#9654;</span> Run';
        }
        saveResearchState();
    }
}

/**
 * Collect input data for the current tool
 */
function collectInputData(tool) {
    ResearchState.inputData = {};
    
    switch (tool) {
        case 'search':
            ResearchState.inputData.query = document.getElementById('search-query')?.value || '';
            ResearchState.inputData.engine = document.getElementById('search-engine')?.value || 'google';
            ResearchState.inputData.results = parseInt(document.getElementById('search-results')?.value || '10');
            break;
            
        case 'summarize':
            ResearchState.inputData.text = document.getElementById('summarize-text')?.value || '';
            ResearchState.inputData.length = document.getElementById('summary-length')?.value || 'medium';
            break;
            
        case 'translate':
            ResearchState.inputData.text = document.getElementById('translate-text')?.value || '';
            ResearchState.inputData.sourceLanguage = document.getElementById('source-language')?.value || 'auto';
            ResearchState.inputData.targetLanguage = document.getElementById('target-language')?.value || 'en';
            break;
            
        case 'ocr':
            // Files already handled
            ResearchState.inputData.language = document.getElementById('ocr-language')?.value || 'auto';
            break;
            
        case 'analyze':
            // Files already handled
            ResearchState.inputData.type = document.getElementById('analysis-type')?.value || 'summary';
            break;
            
        case 'code-review':
            ResearchState.inputData.code = document.getElementById('code-text')?.value || '';
            ResearchState.inputData.language = document.getElementById('code-language')?.value || 'auto';
            ResearchState.inputData.focus = document.getElementById('review-focus')?.value || 'general';
            break;
            
        case 'generate':
            ResearchState.inputData.prompt = document.getElementById('generate-prompt')?.value || '';
            ResearchState.inputData.type = document.getElementById('generate-type')?.value || 'article';
            break;
    }
}

/**
 * Validate input for the current tool
 */
function validateInput(tool) {
    switch (tool) {
        case 'search':
            return ResearchState.inputData.query?.trim() !== '';
            
        case 'summarize':
            return ResearchState.inputData.text?.trim() !== '';
            
        case 'translate':
            return ResearchState.inputData.text?.trim() !== '';
            
        case 'ocr':
            return ResearchState.inputData.files?.length > 0;
            
        case 'analyze':
            return ResearchState.inputData.files?.length > 0;
            
        case 'code-review':
            return ResearchState.inputData.code?.trim() !== '';
            
        case 'generate':
            return ResearchState.inputData.prompt?.trim() !== '';
            
        default:
            return false;
    }
}

/**
 * Process search
 */
async function processSearch(data) {
    const client = getPuterClient();
    
    const prompt = `Perform a web search for: ${data.query}. 
    Use ${data.engine} search engine and return ${data.results} results. 
    Provide a comprehensive summary of the findings.`;
    
    const response = await client.chat(prompt, {
        temperature: 0.3,
        maxTokens: 4096
    });
    
    return {
        type: 'text',
        content: response.message?.content || response,
        query: data.query,
        engine: data.engine,
        results: data.results
    };
}

/**
 * Process summarize
 */
async function processSummarize(data) {
    const client = getPuterClient();
    
    let prompt = `Summarize the following text:\n\n${data.text}\n\n`;
    
    switch (data.length) {
        case 'short':
            prompt += 'Provide a very short summary (1-2 sentences).';
            break;
        case 'medium':
            prompt += 'Provide a medium-length summary (3-5 sentences).';
            break;
        case 'long':
            prompt += 'Provide a detailed summary with key points.';
            break;
    }
    
    const response = await client.chat(prompt, {
        temperature: 0.3,
        maxTokens: 2048
    });
    
    return {
        type: 'text',
        content: response.message?.content || response,
        originalLength: data.text.length,
        summaryLength: data.length
    };
}

/**
 * Process translate
 */
async function processTranslate(data) {
    const client = getPuterClient();
    
    const sourceLang = data.sourceLanguage === 'auto' ? 'the original language' : data.sourceLanguage;
    const targetLang = getLanguageName(data.targetLanguage);
    
    const prompt = `Translate the following text from ${sourceLang} to ${targetLang}:\n\n${data.text}`;
    
    const response = await client.chat(prompt, {
        temperature: 0.2,
        maxTokens: 4096
    });
    
    return {
        type: 'text',
        content: response.message?.content || response,
        sourceLanguage: data.sourceLanguage,
        targetLanguage: data.targetLanguage
    };
}

/**
 * Process OCR
 */
async function processOCR(data) {
    const client = getPuterClient();
    
    if (!data.files || data.files.length === 0) {
        throw new Error('No files provided for OCR');
    }
    
    // For demo purposes, we'll simulate OCR
    // In a real implementation, you would upload the image and use an OCR API
    const file = data.files[0];
    
    // Simulate OCR processing
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    // For demo, return a placeholder
    return {
        type: 'text',
        content: `OCR Result:\n\n[Image: ${file.name}]\n\nText extracted from image using OCR technology.\n\nNote: This is a demo. In the full Aetheris platform, actual OCR would be performed on your images.`,
        filename: file.name,
        language: data.language
    };
}

/**
 * Process analyze
 */
async function processAnalyze(data) {
    const client = getPuterClient();
    
    if (!data.files || data.files.length === 0) {
        throw new Error('No files provided for analysis');
    }
    
    // For demo purposes, we'll simulate document analysis
    const file = data.files[0];
    
    // Simulate analysis processing
    await new Promise(resolve => setTimeout(resolve, 1500));
    
    let prompt = `Analyze the following document:\n\n[Document: ${file.name}]\n\n`;
    
    switch (data.type) {
        case 'summary':
            prompt += 'Provide a comprehensive summary of the document.';
            break;
        case 'keywords':
            prompt += 'Extract the main keywords and key phrases from the document.';
            break;
        case 'questions':
            prompt += 'Generate 5-10 questions that someone might have after reading this document.';
            break;
        case 'insights':
            prompt += 'Provide deep insights, patterns, and observations from the document.';
            break;
    }
    
    const response = await client.chat(prompt, {
        temperature: 0.5,
        maxTokens: 2048
    });
    
    return {
        type: 'text',
        content: response.message?.content || response,
        filename: file.name,
        analysisType: data.type
    };
}

/**
 * Process code review
 */
async function processCodeReview(data) {
    const client = getPuterClient();
    
    let prompt = `Review the following ${data.language} code:\n\n${data.code}\n\n`;
    
    switch (data.focus) {
        case 'bugs':
            prompt += 'Focus on finding bugs, errors, and potential issues.';
            break;
        case 'performance':
            prompt += 'Focus on performance optimizations and efficiency improvements.';
            break;
        case 'security':
            prompt += 'Focus on security vulnerabilities and potential risks.';
            break;
        case 'style':
            prompt += 'Focus on code style, readability, and best practices.';
            break;
        default:
            prompt += 'Provide a comprehensive review covering bugs, performance, security, and style.';
    }
    
    const response = await client.chat(prompt, {
        temperature: 0.3,
        maxTokens: 4096
    });
    
    return {
        type: 'text',
        content: response.message?.content || response,
        language: data.language,
        focus: data.focus
    };
}

/**
 * Process generate
 */
async function processGenerate(data) {
    const client = getPuterClient();
    
    let prompt = `Generate a ${data.type} based on the following description:\n\n${data.prompt}\n\n`;
    
    switch (data.type) {
        case 'article':
            prompt += 'Write a well-structured article with introduction, body, and conclusion.';
            break;
        case 'story':
            prompt += 'Write a compelling story with characters, plot, and resolution.';
            break;
        case 'poem':
            prompt += 'Write a beautiful poem with appropriate structure and rhythm.';
            break;
        case 'email':
            prompt += 'Write a professional email with subject, greeting, body, and closing.';
            break;
        case 'code':
            prompt += 'Write clean, well-commented code that solves the described problem.';
            break;
    }
    
    const response = await client.chat(prompt, {
        temperature: 0.7,
        maxTokens: 4096
    });
    
    return {
        type: 'text',
        content: response.message?.content || response,
        contentType: data.type
    };
}

/**
 * Display output
 */
function displayOutput(result) {
    if (!ResearchDOM.outputContent) return;
    
    ResearchState.outputData = result;
    
    let html = '';
    
    switch (result.type) {
        case 'text':
            html = `
                <div class="output-result">
                    <div class="output-text">${formatMessageContent(result.content)}</div>
                    <div class="output-actions">
                        <button class="output-action" onclick="copyOutput()">Copy</button>
                        <button class="output-action" onclick="clearOutput()">Clear</button>
                    </div>
                </div>
            `;
            break;
            
        case 'image':
            html = `
                <div class="output-result">
                    <img class="output-image" src="${result.url}" alt="Generated image" />
                    <div class="output-actions">
                        <button class="output-action" onclick="downloadOutput()">Download</button>
                        <button class="output-action" onclick="clearOutput()">Clear</button>
                    </div>
                </div>
            `;
            break;
            
        case 'code':
            html = `
                <div class="output-result">
                    <div class="output-code">${highlightCode(result.content, result.language || 'javascript')}</div>
                    <div class="output-actions">
                        <button class="output-action" onclick="copyOutput()">Copy</button>
                        <button class="output-action" onclick="clearOutput()">Clear</button>
                    </div>
                </div>
            `;
            break;
    }
    
    ResearchDOM.outputContent.innerHTML = html;
}

/**
 * Clear output
 */
function clearOutput() {
    if (ResearchDOM.outputContent) {
        ResearchDOM.outputContent.innerHTML = '';
    }
    ResearchState.outputData = null;
}

/**
 * Copy output
 */
function copyOutput() {
    if (!ResearchState.outputData) {
        showToast('No output to copy', 'error');
        return;
    }
    
    let textToCopy = '';
    
    if (typeof ResearchState.outputData.content === 'string') {
        textToCopy = ResearchState.outputData.content;
    } else if (ResearchState.outputData.url) {
        textToCopy = ResearchState.outputData.url;
    }
    
    if (textToCopy) {
        copyToClipboard(textToCopy);
        showToast('Output copied to clipboard', 'success');
    } else {
        showToast('No text content to copy', 'error');
    }
}

/**
 * Clear input
 */
function clearInput() {
    const tool = ResearchState.currentTool;
    const config = ToolConfigs[tool];
    
    if (config && config.inputForm) {
        const form = document.getElementById(config.inputForm);
        if (form) {
            const textareas = form.querySelectorAll('textarea');
            textareas.forEach(textarea => {
                textarea.value = '';
            });
            
            const selects = form.querySelectorAll('select');
            selects.forEach(select => {
                select.selectedIndex = 0;
            });
        }
    }
    
    // Clear files
    ResearchState.inputData.files = [];
    
    // Clear file lists
    document.querySelectorAll('.file-list').forEach(list => {
        list.innerHTML = '';
    });
    
    clearOutput();
}

/**
 * Save to history
 */
function saveToHistory(tool, input, output) {
    const historyItem = {
        id: Date.now().toString(),
        tool,
        input,
        output,
        timestamp: new Date()
    };
    
    ResearchState.history.unshift(historyItem);
    
    // Keep only last 50 items
    if (ResearchState.history.length > 50) {
        ResearchState.history.pop();
    }
    
    saveResearchState();
}

/**
 * Show history
 */
function showHistory() {
    if (!ResearchDOM.historyPanel || !ResearchDOM.historyList) return;
    
    const isVisible = ResearchDOM.historyPanel.style.display !== 'none';
    
    if (isVisible) {
        ResearchDOM.historyPanel.style.display = 'none';
    } else {
        // Render history
        ResearchDOM.historyList.innerHTML = ResearchState.history
            .slice(0, 20)
            .map(item => {
                const config = ToolConfigs[item.tool];
                const icon = config ? config.icon : '&#128218;';
                const name = config ? config.name : item.tool;
                
                return `
                    <div class="history-item" onclick="loadHistoryItem('${item.id}')">
                        <span>${icon}</span> ${name} - ${formatDate(item.timestamp)}
                    </div>
                `;
            })
            .join('');
        
        ResearchDOM.historyPanel.style.display = 'block';
    }
}

/**
 * Load history item
 */
function loadHistoryItem(itemId) {
    const item = ResearchState.history.find(h => h.id === itemId);
    if (!item) return;
    
    // Select the tool
    selectTool(item.tool);
    
    // Restore input
    ResearchState.inputData = { ...item.input };
    
    // Restore input form values
    const config = ToolConfigs[item.tool];
    if (config && config.inputForm) {
        const form = document.getElementById(config.inputForm);
        if (form) {
            // Restore textareas
            form.querySelectorAll('textarea').forEach(textarea => {
                const fieldName = textarea.id.replace(`${item.tool}-`, '');
                if (item.input[fieldName]) {
                    textarea.value = item.input[fieldName];
                }
            });
            
            // Restore selects
            form.querySelectorAll('select').forEach(select => {
                const fieldName = select.id.replace(`${item.tool}-`, '');
                if (item.input[fieldName]) {
                    select.value = item.input[fieldName];
                }
            });
        }
    }
    
    // Display output
    displayOutput(item.output);
    
    // Hide history
    if (ResearchDOM.historyPanel) {
        ResearchDOM.historyPanel.style.display = 'none';
    }
}

/**
 * Save research state
 */
function saveResearchState() {
    try {
        localStorage.setItem('aether-research-state', JSON.stringify(ResearchState));
    } catch (error) {
        console.error('Failed to save research state:', error);
    }
}

/**
 * Load research state
 */
function loadResearchState() {
    const savedState = localStorage.getItem('aether-research-state');
    if (savedState) {
        try {
            const state = JSON.parse(savedState);
            Object.assign(ResearchState, state);
        } catch (error) {
            console.error('Failed to load research state:', error);
        }
    }
}

/**
 * Get language name from code
 */
function getLanguageName(code) {
    const languages = {
        en: 'English',
        es: 'Spanish',
        fr: 'French',
        de: 'German',
        zh: 'Chinese',
        ja: 'Japanese',
        it: 'Italian',
        pt: 'Portuguese',
        ru: 'Russian',
        ar: 'Arabic',
        hi: 'Hindi',
        ko: 'Korean',
        nl: 'Dutch',
        sv: 'Swedish',
        fi: 'Finnish',
        da: 'Danish',
        no: 'Norwegian',
        pl: 'Polish',
        tr: 'Turkish',
        th: 'Thai',
        vi: 'Vietnamese',
        id: 'Indonesian',
        ms: 'Malay',
        he: 'Hebrew',
        el: 'Greek',
        hu: 'Hungarian',
        cs: 'Czech',
        ro: 'Romanian',
        uk: 'Ukrainian',
        auto: 'Auto Detect'
    };
    
    return languages[code] || code;
}

/**
 * Export functions
 */
window.selectTool = selectTool;
window.runTool = runTool;
window.clearInput = clearInput;
window.clearOutput = clearOutput;
window.copyOutput = copyOutput;
window.showHistory = showHistory;
window.loadHistoryItem = loadHistoryItem;
window.removeFile = removeFile;
window.handleFileUpload = handleFileUpload;
window.handleDroppedFiles = handleDroppedFiles;
