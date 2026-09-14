/**
 * Aether Demo - Main Application Logic
 * Handles navigation, state management, and common utilities
 */

// Application state
const AppState = {
    currentPage: 'index',
    currentModel: null,
    chatHistory: [],
    researchHistory: [],
    files: [],
    settings: {
        temperature: 0.7,
        maxTokens: 2048,
        stream: true,
        theme: 'dark'
    }
};

// DOM Elements
const DOM = {
    sidebar: null,
    mobileSidebarToggle: null,
    mobileFiltersToggle: null,
    filtersPanel: null,
    uploadModal: null
};

// Initialize the application
document.addEventListener('DOMContentLoaded', () => {
    initializeDOM();
    initializeNavigation();
    initializeTheme();
    loadState();
    
    // Initialize Puter client
    const client = getPuterClient();
    if (!client.isInitialized) {
        client.init().then(() => {
            
            // Update UI with model count
            updateModelCount();
        });
    } else {
        updateModelCount();
    }
});

/**
 * Initialize DOM references
 */
function initializeDOM() {
    DOM.sidebar = document.getElementById('sidebar');
    DOM.mobileSidebarToggle = document.getElementById('mobile-sidebar-toggle');
    DOM.mobileFiltersToggle = document.getElementById('mobile-filters-toggle');
    DOM.filtersPanel = document.getElementById('filters-panel');
    DOM.uploadModal = document.getElementById('upload-modal');
    
    // Set up mobile toggle visibility
    updateMobileToggles();
    
    // Add resize listener
    window.addEventListener('resize', debounce(updateMobileToggles, 250));
}

/**
 * Update mobile toggle visibility based on screen size
 */
function updateMobileToggles() {
    const isMobile = window.innerWidth <= 1024;
    
    if (DOM.mobileSidebarToggle) {
        DOM.mobileSidebarToggle.style.display = isMobile ? 'block' : 'none';
    }
    
    if (DOM.mobileFiltersToggle) {
        DOM.mobileFiltersToggle.style.display = isMobile ? 'block' : 'none';
    }
    
    // Close panels on mobile when resizing to desktop
    if (!isMobile) {
        closeSidebar();
        closeFilters();
    }
}

/**
 * Initialize navigation
 */
function initializeNavigation() {
    // Update active nav link
    const currentPath = window.location.pathname.split('/').pop();
    const navLinks = document.querySelectorAll('.nav-link');
    
    navLinks.forEach(link => {
        const href = link.getAttribute('href');
        const pageName = href.split('.')[0];
        const currentPage = currentPath.split('.')[0];
        
        if (pageName === currentPage) {
            link.classList.add('active');
            AppState.currentPage = pageName;
        } else {
            link.classList.remove('active');
        }
    });
}

/**
 * Initialize theme
 */
function initializeTheme() {
    const savedTheme = localStorage.getItem('aether-theme') || 'dark';
    AppState.settings.theme = savedTheme;
    applyTheme(savedTheme);
}

/**
 * Apply theme
 */
function applyTheme(theme) {
    const html = document.documentElement;
    
    if (theme === 'light') {
        html.classList.add('light-theme');
        html.classList.remove('dark-theme');
    } else {
        html.classList.remove('light-theme');
        html.classList.add('dark-theme');
    }
    
    localStorage.setItem('aether-theme', theme);
    AppState.settings.theme = theme;
}

/**
 * Toggle theme
 */
function toggleTheme() {
    const newTheme = AppState.settings.theme === 'dark' ? 'light' : 'dark';
    applyTheme(newTheme);
}

/**
 * Load application state from localStorage
 */
function loadState() {
    const savedState = localStorage.getItem('aether-state');
    if (savedState) {
        try {
            const state = JSON.parse(savedState);
            Object.assign(AppState, state);
        } catch (error) {
            
        }
    }
}

/**
 * Save application state to localStorage
 */
function saveState() {
    try {
        localStorage.setItem('aether-state', JSON.stringify(AppState));
    } catch (error) {
        
    }
}

/**
 * Navigation functions
 */
function goToChat() {
    window.location.href = 'chat.html';
}

function goToModels() {
    window.location.href = 'models.html';
}

function goToResearch() {
    window.location.href = 'research.html';
}

function goToFiles() {
    window.location.href = 'files.html';
}

function goToHome() {
    window.location.href = 'index.html';
}

/**
 * Sidebar toggle functions
 */
function toggleSidebar() {
    if (DOM.sidebar) {
        DOM.sidebar.classList.toggle('visible');
    }
}

function closeSidebar() {
    if (DOM.sidebar) {
        DOM.sidebar.classList.remove('visible');
    }
}

function openSidebar() {
    if (DOM.sidebar) {
        DOM.sidebar.classList.add('visible');
    }
}

/**
 * Filters toggle functions
 */
function toggleFilters() {
    if (DOM.filtersPanel) {
        DOM.filtersPanel.classList.toggle('visible');
    }
}

function closeFilters() {
    if (DOM.filtersPanel) {
        DOM.filtersPanel.classList.remove('visible');
    }
}

function openFilters() {
    if (DOM.filtersPanel) {
        DOM.filtersPanel.classList.add('visible');
    }
}

/**
 * Modal functions
 */
function openModal(modalId) {
    const modal = document.getElementById(modalId);
    if (modal) {
        modal.classList.add('visible');
        document.body.style.overflow = 'hidden';
    }
}

function closeModal(modalId) {
    const modal = document.getElementById(modalId);
    if (modal) {
        modal.classList.remove('visible');
        document.body.style.overflow = '';
    }
}

function toggleModal(modalId) {
    const modal = document.getElementById(modalId);
    if (modal) {
        modal.classList.toggle('visible');
        document.body.style.overflow = modal.classList.contains('visible') ? 'hidden' : '';
    }
}

/**
 * Update model count in UI
 */
function updateModelCount() {
    const client = getPuterClient();
    const stats = client.getModelStats();
    
    // Update index page stats
    const statElements = document.querySelectorAll('.stat-number');
    if (statElements.length >= 3) {
        statElements[0].textContent = stats.total + '+';
        statElements[1].textContent = stats.providers.length + '+';
    }
    
    // Update models page
    const totalModelsEl = document.getElementById('total-models');
    const totalProvidersEl = document.getElementById('total-providers');
    const totalTypesEl = document.getElementById('total-types');
    
    if (totalModelsEl) {
        totalModelsEl.textContent = stats.total + '+';
    }
    if (totalProvidersEl) {
        totalProvidersEl.textContent = stats.providers.length + '+';
    }
    if (totalTypesEl) {
        totalTypesEl.textContent = stats.types.length + '+';
    }
}

/**
 * Show loading state
 */
function showLoading(elementId, message = 'Loading...') {
    const element = document.getElementById(elementId);
    if (element) {
        element.innerHTML = `
            <div class="loading-indicator">
                <div class="loading-spinner"></div>
                <span>${message}</span>
            </div>
        `;
    }
}

/**
 * Show error state
 */
function showError(elementId, message = 'An error occurred') {
    const element = document.getElementById(elementId);
    if (element) {
        element.innerHTML = `
            <div class="error-display">
                <span>&#9888;</span> ${message}
            </div>
        `;
    }
}

/**
 * Format date
 */
function formatDate(date) {
    if (!date) return 'Unknown';
    
    const d = new Date(date);
    return d.toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
    });
}

/**
 * Format file size
 */
function formatFileSize(bytes) {
    if (!bytes) return '0 B';
    
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    let size = bytes;
    let unitIndex = 0;
    
    while (size >= 1024 && unitIndex < units.length - 1) {
        size /= 1024;
        unitIndex++;
    }
    
    return `${size.toFixed(2)} ${units[unitIndex]}`;
}

/**
 * Format time
 */
function formatTime(date) {
    if (!date) return 'Unknown';
    
    const d = new Date(date);
    return d.toLocaleTimeString('en-US', {
        hour: '2-digit',
        minute: '2-digit'
    });
}

/**
 * Debounce function
 */
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

/**
 * Copy text to clipboard
 */
async function copyToClipboard(text) {
    try {
        await navigator.clipboard.writeText(text);
        return true;
    } catch (error) {
        // Fallback for older browsers
        const textarea = document.createElement('textarea');
        textarea.value = text;
        textarea.style.position = 'fixed';
        textarea.style.opacity = '0';
        document.body.appendChild(textarea);
        textarea.select();
        const success = document.execCommand('copy');
        document.body.removeChild(textarea);
        return success;
    }
}

/**
 * Show toast notification
 */
function showToast(message, type = 'info', duration = 3000) {
    // Create toast container if it doesn't exist
    let container = document.querySelector('.toast-container');
    if (!container) {
        container = document.createElement('div');
        container.className = 'toast-container';
        document.body.appendChild(container);
    }
    
    // Create toast
    const toast = document.createElement('div');
    toast.className = `toast toast-${type}`;
    toast.innerHTML = `
        <span class="toast-icon">${getToastIcon(type)}</span>
        <span class="toast-message">${message}</span>
        <span class="toast-close" onclick="this.parentElement.remove()">&times;</span>
    `;
    
    container.appendChild(toast);
    
    // Auto-remove after duration
    setTimeout(() => {
        toast.classList.add('hiding');
        setTimeout(() => toast.remove(), 300);
    }, duration);
    
    return toast;
}

/**
 * Get toast icon based on type
 */
function getToastIcon(type) {
    switch (type) {
        case 'success': return '&#10003;';
        case 'error': return '&#9888;';
        case 'warning': return '&#9888;';
        default: return '&#8505;';
    }
}

/**
 * Escape HTML to prevent XSS
 */
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

/**
 * Sanitize HTML for display
 */
function sanitizeHtml(html) {
    // Basic sanitization - replace script tags
    return html
        .replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, '')
        .replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*/gi, '')
        .replace(/javascript:/gi, '')
        .replace(/on\w+=/gi, '');
}

/**
 * Format markdown for display
 */
function formatMarkdown(text) {
    // Simple markdown formatting
    return text
        .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
        .replace(/\*(.*?)\*/g, '<em>$1</em>')
        .replace(/`(.*?)`/g, '<code>$1</code>')
        .replace(/\n\n/g, '</p><p>')
        .replace(/\n/g, '<br>')
        .replace(/^# (.*?)$/gm, '<h1>$1</h1>')
        .replace(/^## (.*?)$/gm, '<h2>$1</h2>')
        .replace(/^### (.*?)$/gm, '<h3>$1</h3>')
        .replace(/^- (.*?)$/gm, '<li>$1</li>')
        .replace(/^> (.*?)$/gm, '<blockquote>$1</blockquote>');
}

/**
 * Highlight code blocks
 */
function highlightCode(code, language = 'javascript') {
    // Simple syntax highlighting based on language
    const languages = {
        javascript: ['function', 'const', 'let', 'var', 'if', 'else', 'for', 'while', 'return'],
        python: ['def', 'class', 'import', 'from', 'if', 'else', 'for', 'while', 'return'],
        html: ['div', 'span', 'class', 'id', 'href', 'src', 'style'],
        css: ['color', 'background', 'font', 'margin', 'padding', 'display']
    };
    
    const keywords = languages[language] || [];
    let highlighted = code;
    
    keywords.forEach(keyword => {
        const regex = new RegExp(`\b${keyword}\b`, 'gi');
        highlighted = highlighted.replace(regex, `<span class="keyword">${keyword}</span>`);
    });
    
    return `<pre><code class="language-${language}">${highlighted}</code></pre>`;
}

/**
 * Get file extension from filename
 */
function getFileExtension(filename) {
    return filename.split('.').pop().toLowerCase();
}

/**
 * Get file type from extension
 */
function getFileType(extension) {
    const imageExtensions = ['png', 'jpg', 'jpeg', 'gif', 'webp', 'svg', 'bmp'];
    const audioExtensions = ['mp3', 'wav', 'ogg', 'm4a', 'aac'];
    const videoExtensions = ['mp4', 'webm', 'mov', 'avi', 'mkv'];
    const documentExtensions = ['pdf', 'doc', 'docx', 'txt', 'md', 'rtf'];
    const codeExtensions = ['js', 'ts', 'py', 'java', 'cpp', 'c', 'go', 'rust', 'php', 'html', 'css', 'json'];
    
    if (imageExtensions.includes(extension)) return 'image';
    if (audioExtensions.includes(extension)) return 'audio';
    if (videoExtensions.includes(extension)) return 'video';
    if (documentExtensions.includes(extension)) return 'document';
    if (codeExtensions.includes(extension)) return 'code';
    
    return 'other';
}

/**
 * Get file icon based on type
 */
function getFileIcon(type) {
    switch (type) {
        case 'image': return '&#128247;';
        case 'audio': return '&#128664;';
        case 'video': return '&#128249;';
        case 'document': return '&#128196;';
        case 'code': return '&#128187;';
        case 'folder': return '&#128193;';
        default: return '&#128196;';
    }
}

/**
 * Get MIME type from extension
 */
function getMimeType(extension) {
    const mimeTypes = {
        // Images
        png: 'image/png',
        jpg: 'image/jpeg',
        jpeg: 'image/jpeg',
        gif: 'image/gif',
        webp: 'image/webp',
        svg: 'image/svg+xml',
        
        // Audio
        mp3: 'audio/mpeg',
        wav: 'audio/wav',
        ogg: 'audio/ogg',
        m4a: 'audio/mp4',
        aac: 'audio/aac',
        
        // Video
        mp4: 'video/mp4',
        webm: 'video/webm',
        mov: 'video/quicktime',
        avi: 'video/x-msvideo',
        mkv: 'video/x-matroska',
        
        // Documents
        pdf: 'application/pdf',
        txt: 'text/plain',
        md: 'text/markdown',
        doc: 'application/msword',
        docx: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
        
        // Code
        js: 'application/javascript',
        ts: 'application/typescript',
        py: 'text/x-python',
        html: 'text/html',
        css: 'text/css',
        json: 'application/json'
    };
    
    return mimeTypes[extension] || 'application/octet-stream';
}

/**
 * Check if file is previewable
 */
function isPreviewable(type) {
    const previewableTypes = ['image', 'audio', 'video', 'document', 'code', 'text'];
    return previewableTypes.includes(type);
}

/**
 * Get provider color
 */
function getProviderColor(provider) {
    const colors = {
        openai: '#00A67E',
        anthropic: '#FFC700',
        google: '#4285F4',
        xai: '#FF6B35',
        mistral: '#FF69B4',
        deepseek: '#00D4AA',
        meta: '#1877F2',
        cohere: '#3B82F6',
        blackforest: '#7C3AED'
    };
    
    return colors[provider.toLowerCase()] || '#6366F1';
}

/**
 * Export functions for use in other modules
 */
window.AppState = AppState;
window.DOM = DOM;
window.showLoading = showLoading;
window.showError = showError;
window.formatDate = formatDate;
window.formatFileSize = formatFileSize;
window.formatTime = formatTime;
window.copyToClipboard = copyToClipboard;
window.showToast = showToast;
window.escapeHtml = escapeHtml;
window.sanitizeHtml = sanitizeHtml;
window.formatMarkdown = formatMarkdown;
window.highlightCode = highlightCode;
window.getFileExtension = getFileExtension;
window.getFileType = getFileType;
window.getFileIcon = getFileIcon;
window.getMimeType = getMimeType;
window.isPreviewable = isPreviewable;
window.getProviderColor = getProviderColor;
window.toggleSidebar = toggleSidebar;
window.closeSidebar = closeSidebar;
window.openSidebar = openSidebar;
window.toggleFilters = toggleFilters;
window.closeFilters = closeFilters;
window.openFilters = openFilters;
window.openModal = openModal;
window.closeModal = closeModal;
window.toggleModal = toggleModal;
window.toggleTheme = toggleTheme;
window.updateModelCount = updateModelCount;
