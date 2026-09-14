/**
 * Aether - AI Magic Showcase
 * Demo Implementations
 */

// Demo Registry
const DemoRegistry = (function() {
    'use strict';
    
    const demos = {};
    
    /**
     * Register a demo
     */
    function register(name, demo) {
        demos[name] = {
            ...demo,
            name: demo.name || name,
            id: name
        };
    }
    
    /**
     * Get a demo by name
     */
    function get(name) {
        return demos[name];
    }
    
    /**
     * Get all demos
     */
    function getAll() {
        return { ...demos };
    }
    
    /**
     * Get demos by category
     */
    function getByCategory(category) {
        return Object.values(demos).filter(demo => demo.category === category);
    }
    
    /**
     * Get categories
     */
    function getCategories() {
        const categories = new Set();
        Object.values(demos).forEach(demo => {
            if (demo.category) categories.add(demo.category);
        });
        return Array.from(categories);
    }
    
    return {
        register,
        get,
        getAll,
        getByCategory,
        getCategories
    };
})();

// Email Demo
DemoRegistry.register('email', {
    name: 'Write a Professional Email',
    description: 'Just describe what you need, AI writes the perfect email',
    category: 'work',
    icon: '✉️',
    tags: ['productivity', 'business', 'communication'],
    difficulty: 'easy',
    inputPlaceholder: 'What should the email be about?',
    options: [
        { name: 'tone', label: 'Tone', type: 'select', values: ['Professional', 'Friendly', 'Urgent'] },
        { name: 'recipient', label: 'Recipient', type: 'select', values: ['Boss', 'Client', 'Colleague', 'Team'] }
    ],
    examplePrompts: [
        'Write an email to my boss asking for a raise',
        'Create a follow-up email to a client about our meeting',
        'Write a resignation letter',
        'Draft an email to schedule a team meeting'
    ],
    run: async function(prompt, options = {}, puter) {
        const client = window.PuterClient || window.puter || puter;
        if (!client) {
            throw new Error('AI client not available');
        }
        
        const tone = options.tone || 'Professional';
        const recipient = options.recipient || 'Boss';
        const fullPrompt = `Write a ${tone.toLowerCase()} email to my ${recipient.toLowerCase()} about: "${prompt}". 
Make it professional and appropriate for the workplace. Include a subject line and proper email formatting.`;
        
        const response = await client.chat(fullPrompt, { model: 'gpt-4o-mini' });
        return formatEmailResponse(response.text);
    }
});
// Logo Demo

DemoRegistry.register('logo', {
    name: 'Create a Beautiful Logo',
    description: 'Describe your business, AI creates a custom logo',
    category: 'business',
    icon: '🎨',
    tags: ['design', 'creativity', 'branding'],
    difficulty: 'easy',
    inputPlaceholder: 'What is your business name and type?',
    options: [
        { name: 'style', label: 'Style', type: 'select', values: ['Minimal', 'Modern', 'Vintage', 'Playful', 'Elegant'] }
    ],
    examplePrompts: [
        'Logo for a coffee shop called Brew Haven',
        'Modern logo for a tech startup',
        'Elegant logo for a wedding planner',
        'Fun logo for a children\'s toy store'
    ],
    run: async function(prompt, options = {}, puter) {
        const client = window.PuterClient || window.puter || puter;
        if (!client) {
            throw new Error('AI client not available');
        }
        
        const style = options.style || 'Modern';
        
        try {
            // Try to generate an actual image
            const blob = await client.generateImage(`Create a ${style.toLowerCase()} logo for a business called: ${prompt}. Professional design, clean lines, suitable for a company logo.`);
            const url = URL.createObjectURL(blob);
            return `<img src="${url}" alt="Generated logo" style="max-width: 100%; border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.3);">`;
        } catch (error) {
            // Fallback to text description
            const fullPrompt = `Create a detailed description of a ${style.toLowerCase()} logo for: "${prompt}". 
Describe the colors, shapes, typography, and overall design concept.`;
            const response = await client.chat(fullPrompt, { model: 'gpt-4o-mini' });
            return formatLogoDescription(response.text);
        }
    }
});

// Summarize Demo
DemoRegistry.register('summarize', {
    name: 'Summarize Document',
    description: 'Upload any document, AI extracts the key points',
    category: 'productivity',
    icon: '📄',
    tags: ['analysis', 'efficiency', 'reading'],
    difficulty: 'easy',
    inputPlaceholder: 'Paste your document text or describe what you want summarized',
    options: [
        { name: 'length', label: 'Summary Length', type: 'select', values: ['Short (1 paragraph)', 'Medium (3-5 paragraphs)', 'Detailed (Full summary)'] },
        { name: 'focus', label: 'Focus On', type: 'select', values: ['Key Points', 'Action Items', 'Decisions', 'Recommendations'] }
    ],
    examplePrompts: [
        'Summarize the key points of artificial intelligence',
        'Extract the main ideas from this long article',
        'Create a brief summary of the Industrial Revolution'
    ],
    run: async function(prompt, options = {}, puter) {
        const client = window.PuterClient || window.puter || puter;
        if (!client) {
            throw new Error('AI client not available');
        }
        
        const lengthMap = { 'Short (1 paragraph)': '1 paragraph', 'Medium (3-5 paragraphs)': '3-5 paragraphs', 'Detailed (Full summary)': 'detailed' };
        const focus = options.focus || 'Key Points';
        const length = options.length || 'Medium (3-5 paragraphs)';
        const fullPrompt = `Summarize the following text with a focus on ${focus.toLowerCase()}. Make it ${lengthMap[length]}: ${prompt}`;
        
        const response = await client.chat(fullPrompt, { model: 'gpt-4o-mini' });
        return formatSummaryResponse(response.text);
    }
});

// Code Demo
DemoRegistry.register('code', {
    name: 'Write Code',
    description: 'Describe what you want, AI writes the code',
    category: 'development',
    icon: '💻',
    tags: ['programming', 'development', 'automation'],
    difficulty: 'medium',
    inputPlaceholder: 'What do you want to create? (e.g., a contact form, a calculator, etc.)',
    options: [
        { name: 'type', label: 'Type', type: 'select', values: ['Website', 'Script', 'Function', 'App', 'Game'] },
        { name: 'language', label: 'Language', type: 'select', values: ['HTML/CSS/JavaScript', 'Python', 'JavaScript', 'TypeScript', 'React'] }
    ],
    examplePrompts: [
        'Create a Python function to calculate Fibonacci sequence',
        'Write a JavaScript function to sort an array',
        'Create a simple HTML/CSS contact form',
        'Write a SQL query to find the top 10 customers'
    ],
    run: async function(prompt, options = {}, puter) {
        const client = window.PuterClient || window.puter || puter;
        if (!client) {
            throw new Error('AI client not available');
        }
        
        const type = options.type || 'Website';
        const language = options.language || 'HTML/CSS/JavaScript';
        const fullPrompt = `Write ${language} code for a ${type.toLowerCase()} that: "${prompt}". Include comments explaining how it works.`;
        
        const response = await client.chat(fullPrompt, { model: 'gpt-4o-mini' });
        return formatCodeResponse(response.text);
    }
});

// Translate Demo
DemoRegistry.register('translate', {
    name: 'Translate',
    description: 'Break language barriers instantly',
    category: 'communication',
    icon: '🌍',
    tags: ['language', 'international', 'localization'],
    difficulty: 'easy',
    inputPlaceholder: 'Enter text to translate',
    options: [
        { name: 'targetLanguage', label: 'Target Language', type: 'select', values: ['Spanish', 'French', 'German', 'Chinese', 'Japanese', 'Arabic', 'Russian', 'Portuguese', 'Italian', 'Dutch'] },
        { name: 'style', label: 'Style', type: 'select', values: ['Formal', 'Casual', 'Technical', 'Literary'] }
    ],
    examplePrompts: [
        'Hello, how are you today?',
        'I would like to order a coffee, please',
        'Where is the nearest train station?',
        'Thank you for your help'
    ],
    run: async function(prompt, options = {}, puter) {
        const client = window.PuterClient || window.puter || puter;
        if (!client) {
            throw new Error('AI client not available');
        }
        
        const targetLanguage = options.targetLanguage || 'Spanish';
        const style = options.style || 'Formal';
        const fullPrompt = `Translate the following text to ${targetLanguage} in a ${style.toLowerCase()} style: "${prompt}"`;
        
        const response = await client.chat(fullPrompt, { model: 'gpt-4o-mini' });
        return formatTranslationResponse(response.text);
    }
});

// Social Media Demo
DemoRegistry.register('social', {
    name: 'Generate Social Media Posts',
    description: 'Create engaging content for any platform',
    category: 'marketing',
    icon: '📱',
    tags: ['social media', 'content creation', 'marketing'],
    difficulty: 'easy',
    inputPlaceholder: 'What do you want to promote?',
    options: [
        { name: 'platform', label: 'Platform', type: 'select', values: ['Instagram', 'Twitter/X', 'LinkedIn', 'Facebook', 'TikTok'] },
        { name: 'tone', label: 'Tone', type: 'select', values: ['Professional', 'Fun', 'Inspirational', 'Promotional', 'Educational'] },
        { name: 'count', label: 'Number of Posts', type: 'select', values: ['1', '3', '5'] }
    ],
    examplePrompts: [
        'Create social media posts about a new product launch',
        'Write engaging Instagram captions for a travel blog',
        'Create Twitter posts announcing a company milestone'
    ],
    run: async function(prompt, options = {}, puter) {
        const client = window.PuterClient || window.puter || puter;
        if (!client) {
            throw new Error('AI client not available');
        }
        
        const platform = options.platform || 'Instagram';
        const tone = options.tone || 'Professional';
        const count = parseInt(options.count) || 3;
        const fullPrompt = `Create ${count} ${tone.toLowerCase()} social media posts for ${platform} about: "${prompt}". Each post should be engaging and appropriate for the platform.`;
        
        const response = await client.chat(fullPrompt, { model: 'gpt-4o-mini' });
        return formatSocialMediaResponse(response.text);
    }
});

// Trip Planning Demo
DemoRegistry.register('trip', {
    name: 'Plan My Perfect Trip',
    description: 'Get personalized itineraries with one request',
    category: 'travel',
    icon: '✈️',
    tags: ['travel', 'planning', 'itinerary'],
    difficulty: 'easy',
    inputPlaceholder: 'Where do you want to go and for how long?',
    options: [
        { name: 'budget', label: 'Budget', type: 'select', values: ['Budget', 'Mid-range', 'Luxury'] },
        { name: 'interests', label: 'Interests', type: 'multiselect', values: ['Adventure', 'Relaxation', 'Culture', 'Food', 'Nature', 'History'] }
    ],
    examplePrompts: [
        'Plan a 7-day trip to Japan',
        'Create a weekend itinerary for New York City',
        'Plan a 2-week European vacation'
    ],
    run: async function(prompt, options = {}, puter) {
        const client = window.PuterClient || window.puter || puter;
        if (!client) {
            throw new Error('AI client not available');
        }
        
        const budget = options.budget || 'Mid-range';
        const interests = options.interests ? options.interests.join(', ') : 'Adventure, Culture, Food';
        const fullPrompt = `Create a detailed ${budget.toLowerCase()} travel itinerary for a trip to ${prompt} with interests in ${interests}. Include day-by-day activities, estimated costs, and recommendations.`;
        
        const response = await client.chat(fullPrompt, { model: 'gpt-4o-mini' });
        return formatTripResponse(response.text);
    }
});

// Learning Demo
DemoRegistry.register('learn', {
    name: 'Learn Anything, Fast',
    description: 'Get personalized lessons on any topic',
    category: 'education',
    icon: '🎓',
    tags: ['learning', 'education', 'tutoring'],
    difficulty: 'easy',
    inputPlaceholder: 'What do you want to learn about?',
    options: [
        { name: 'level', label: 'Your Level', type: 'select', values: ['Beginner (5 years old)', 'Beginner', 'Intermediate', 'Advanced'] },
        { name: 'format', label: 'Format', type: 'select', values: ['Simple Explanation', 'Step-by-Step Tutorial', 'Q&A Session', 'Summary with Examples'] }
    ],
    examplePrompts: [
        'Explain quantum computing to a 10-year-old',
        'Teach me the basics of Python programming',
        'Explain how blockchain technology works'
    ],
    run: async function(prompt, options = {}, puter) {
        const client = window.PuterClient || window.puter || puter;
        if (!client) {
            throw new Error('AI client not available');
        }
        
        const level = options.level || 'Beginner';
        const format = options.format || 'Simple Explanation';
        const fullPrompt = `Teach me about ${prompt} as if I'm a ${level}. Use a ${format.toLowerCase()} format. Make it engaging and easy to understand.`;
        
        const response = await client.chat(fullPrompt, { model: 'gpt-4o-mini' });
        return formatLearningResponse(response.text);
    }
});

// Data Analysis Demo
DemoRegistry.register('data', {
    name: 'Analyze My Data',
    description: 'Upload data, AI finds insights and creates charts',
    category: 'business',
    icon: '📊',
    tags: ['analytics', 'data', 'insights'],
    difficulty: 'medium',
    inputPlaceholder: 'What do you want to know about your data?',
    options: [
        { name: 'analysisType', label: 'Analysis Type', type: 'select', values: ['Summary Statistics', 'Trends Over Time', 'Key Insights', 'Visualization Ideas', 'Anomalies'] }
    ],
    examplePrompts: [
        'Analyze sales data and provide insights',
        'Find patterns in customer behavior data',
        'Create a summary report from survey results'
    ],
    run: async function(prompt, options = {}, puter) {
        const client = window.PuterClient || window.puter || puter;
        if (!client) {
            throw new Error('AI client not available');
        }
        
        const analysisType = options.analysisType || 'Key Insights';
        const fullPrompt = `Analyze this data with a focus on ${analysisType.toLowerCase()}: ${prompt}. Provide insights, observations, and recommendations.`;
        
        const response = await client.chat(fullPrompt, { model: 'gpt-4o-mini' });
        return formatDataResponse(response.text);
    }
});

// Meal Planning Demo
DemoRegistry.register('meal', {
    name: 'Create a Meal Plan',
    description: 'Get personalized recipes based on your needs',
    category: 'health',
    icon: '🍽️',
    tags: ['nutrition', 'cooking', 'health'],
    difficulty: 'easy',
    inputPlaceholder: 'Dietary preferences, allergies, or health goals',
    options: [
        { name: 'mealsPerDay', label: 'Meals per Day', type: 'select', values: ['1', '2', '3'] },
        { name: 'days', label: 'Number of Days', type: 'select', values: ['1', '3', '7'] },
        { name: 'cuisine', label: 'Cuisine Type', type: 'select', values: ['Any', 'Italian', 'Mexican', 'Asian', 'Mediterranean', 'Vegetarian', 'Vegan', 'Keto', 'Gluten-Free'] }
    ],
    examplePrompts: [
        'Create a vegetarian meal plan for a week',
        'Plan healthy meals for weight loss',
        'Create a meal plan for a family of 4'
    ],
    run: async function(prompt, options = {}, puter) {
        const client = window.PuterClient || window.puter || puter;
        if (!client) {
            throw new Error('AI client not available');
        }
        
        const mealsPerDay = options.mealsPerDay || '3';
        const days = options.days || '7';
        const cuisine = options.cuisine || 'Any';
        const fullPrompt = `Create a ${days}-day meal plan with ${mealsPerDay} meals per day. Cuisine: ${cuisine}. Dietary needs: ${prompt}. Include recipes, ingredients, and nutritional information.`;
        
        const response = await client.chat(fullPrompt, { model: 'gpt-4o-mini' });
        return formatMealPlanResponse(response.text);
    }
});

// ============================================
// Response Formatters
// ============================================

function formatEmailResponse(text) {
    return `<div class="email-response">
        <div class="email-header" style="background: var(--surface); padding: 1rem; border-radius: 8px 8px 0 0; border-bottom: 1px solid var(--border);">
            <i class="fas fa-envelope"></i> <strong>Generated Email</strong>
        </div>
        <div class="email-body" style="background: var(--surface); padding: 1rem; border-radius: 0 0 8px 8px; white-space: pre-wrap;">
            ${escapeHtml(text)}
        </div>
    </div>`;
}

function formatLogoDescription(text) {
    return `<div class="logo-description">
        <div class="logo-header" style="background: var(--surface); padding: 1rem; border-radius: 8px 8px 0 0; border-bottom: 1px solid var(--border);">
            <i class="fas fa-palette"></i> <strong>Logo Design Concept</strong>
        </div>
        <div class="logo-body" style="background: var(--surface); padding: 1rem; border-radius: 0 0 8px 8px; white-space: pre-wrap;">
            ${escapeHtml(text)}
        </div>
        <div class="note" style="margin-top: 1rem; font-size: 0.875rem; color: var(--text-muted);">
            <i class="fas fa-info-circle"></i> Note: Full image generation available in Aetheris
        </div>
    </div>`;
}

function formatSummaryResponse(text) {
    return `<div class="summary-response">
        <div class="summary-header" style="background: var(--surface); padding: 1rem; border-radius: 8px 8px 0 0; border-bottom: 1px solid var(--border);">
            <i class="fas fa-list-ul"></i> <strong>Summary</strong>
        </div>
        <div class="summary-body" style="background: var(--surface); padding: 1rem; border-radius: 0 0 8px 8px; white-space: pre-wrap;">
            ${escapeHtml(text)}
        </div>
    </div>`;
}

function formatCodeResponse(text) {
    return `<div class="code-response">
        <div class="code-header" style="background: var(--surface); padding: 1rem; border-radius: 8px 8px 0 0; border-bottom: 1px solid var(--border);">
            <i class="fas fa-code"></i> <strong>Generated Code</strong>
        </div>
        <pre class="code-body" style="background: var(--surface); padding: 1rem; border-radius: 0 0 8px 8px; overflow-x: auto; font-family: var(--font-mono); font-size: 0.875rem; white-space: pre-wrap; word-wrap: break-word;">${escapeHtml(text)}</pre>
    </div>`;
}

function formatTranslationResponse(text) {
    return `<div class="translation-response">
        <div class="translation-header" style="background: var(--surface); padding: 1rem; border-radius: 8px 8px 0 0; border-bottom: 1px solid var(--border);">
            <i class="fas fa-language"></i> <strong>Translations</strong>
        </div>
        <div class="translation-body" style="background: var(--surface); padding: 1rem; border-radius: 0 0 8px 8px; white-space: pre-wrap;">
            ${escapeHtml(text)}
        </div>
    </div>`;
}

function formatSocialMediaResponse(text) {
    return `<div class="social-response">
        <div class="social-header" style="background: var(--surface); padding: 1rem; border-radius: 8px 8px 0 0; border-bottom: 1px solid var(--border);">
            <i class="fas fa-share-alt"></i> <strong>Social Media Posts</strong>
        </div>
        <div class="social-body" style="background: var(--surface); padding: 1rem; border-radius: 0 0 8px 8px; white-space: pre-wrap;">
            ${escapeHtml(text)}
        </div>
    </div>`;
}

function formatTripResponse(text) {
    return `<div class="trip-response">
        <div class="trip-header" style="background: var(--surface); padding: 1rem; border-radius: 8px 8px 0 0; border-bottom: 1px solid var(--border);">
            <i class="fas fa-plane"></i> <strong>Travel Itinerary</strong>
        </div>
        <div class="trip-body" style="background: var(--surface); padding: 1rem; border-radius: 0 0 8px 8px; white-space: pre-wrap;">
            ${escapeHtml(text)}
        </div>
    </div>`;
}

function formatLearningResponse(text) {
    return `<div class="learning-response">
        <div class="learning-header" style="background: var(--surface); padding: 1rem; border-radius: 8px 8px 0 0; border-bottom: 1px solid var(--border);">
            <i class="fas fa-graduation-cap"></i> <strong>Lesson</strong>
        </div>
        <div class="learning-body" style="background: var(--surface); padding: 1rem; border-radius: 0 0 8px 8px; white-space: pre-wrap;">
            ${escapeHtml(text)}
        </div>
    </div>`;
}

function formatDataResponse(text) {
    return `<div class="data-response">
        <div class="data-header" style="background: var(--surface); padding: 1rem; border-radius: 8px 8px 0 0; border-bottom: 1px solid var(--border);">
            <i class="fas fa-chart-bar"></i> <strong>Data Analysis</strong>
        </div>
        <div class="data-body" style="background: var(--surface); padding: 1rem; border-radius: 0 0 8px 8px; white-space: pre-wrap;">
            ${escapeHtml(text)}
        </div>
    </div>`;
}

function formatMealPlanResponse(text) {
    return `<div class="meal-response">
        <div class="meal-header" style="background: var(--surface); padding: 1rem; border-radius: 8px 8px 0 0; border-bottom: 1px solid var(--border);">
            <i class="fas fa-utensils"></i> <strong>Meal Plan</strong>
        </div>
        <div class="meal-body" style="background: var(--surface); padding: 1rem; border-radius: 0 0 8px 8px; white-space: pre-wrap;">
            ${escapeHtml(text)}
        </div>
    </div>`;
}

// ============================================
// Utility Functions
// ============================================

function escapeHtml(text) {
    if (!text) return '';
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// Export for global access
window.DemoRegistry = DemoRegistry;

// Make demos available globally for the main app
if (window.AetherApp) {
    window.AetherApp.demos = DemoRegistry.getAll();
}

// Auto-initialize when DOM is ready
if (document.readyState === 'complete' || document.readyState === 'interactive') {
    // Demos are ready
} else {
    document.addEventListener('DOMContentLoaded', () => {
        // Demos are ready
    });
}