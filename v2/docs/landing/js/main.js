/**
 * Aetheris v2 Landing Page - Main JavaScript
 * Handles waitlist form, animations, and dynamic content
 */

// Waitlist Form Handler
document.addEventListener('DOMContentLoaded', function() {
    const waitlistForm = document.getElementById('waitlist-form');
    const waitlistSuccess = document.getElementById('waitlist-success');
    const waitlistCount = document.getElementById('waitlist-count');
    
    // Load waitlist count from localStorage
    let count = localStorage.getItem('waitlistCount') || 0;
    waitlistCount.textContent = count + '+';
    
    // Form submission
    if (waitlistForm) {
        waitlistForm.addEventListener('submit', function(e) {
            e.preventDefault();
            
            const formData = new FormData(waitlistForm);
            const data = Object.fromEntries(formData);
            
            // Validate email
            if (!data.email || !isValidEmail(data.email)) {
                showError('Please enter a valid email address');
                return;
            }
            
            // Store submission
            storeWaitlistEntry(data);
            
            // Update count
            count = parseInt(count) + 1;
            localStorage.setItem('waitlistCount', count);
            waitlistCount.textContent = count + '+';
            
            // Show success message
            waitlistForm.classList.add('hidden');
            waitlistSuccess.classList.remove('hidden');
            
            // Scroll to success message
            waitlistSuccess.scrollIntoView({ behavior: 'smooth', block: 'center' });
        });
    }
    
    // Email validation
    function isValidEmail(email) {
        const re = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return re.test(email);
    }
    
    // Store waitlist entry
    function storeWaitlistEntry(data) {
        let entries = JSON.parse(localStorage.getItem('waitlistEntries') || '[]');
        entries.push({
            ...data,
            timestamp: new Date().toISOString()
        });
        localStorage.setItem('waitlistEntries', JSON.stringify(entries));
    }
    
    // Show error message
    function showError(message) {
        // Create error element
        const errorEl = document.createElement('div');
        errorEl.className = 'error-message';
        errorEl.textContent = message;
        errorEl.style.cssText = 'color: #EF4444; font-size: 13px; margin-top: -10px; margin-bottom: 10px;';
        
        // Insert after form
        const formParent = waitlistForm.parentElement;
        const existingError = formParent.querySelector('.error-message');
        if (existingError) existingError.remove();
        
        waitlistForm.parentElement.insertBefore(errorEl, waitlistForm.nextSibling);
        
        // Remove error after 3 seconds
        setTimeout(() => {
            errorEl.remove();
        }, 3000);
    }
});

// Smooth scroll for anchor links
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function(e) {
        const href = this.getAttribute('href');
        if (href === '#') return;
        
        const target = document.querySelector(href);
        if (target) {
            e.preventDefault();
            const headerOffset = 80;
            const elementPosition = target.getBoundingClientRect().top;
            const offsetPosition = elementPosition + window.pageYOffset - headerOffset;
            
            window.scrollTo({
                top: offsetPosition,
                behavior: 'smooth'
            });
        }
    });
});

// Header scroll effect
window.addEventListener('scroll', function() {
    const header = document.querySelector('.header');
    if (window.scrollY > 100) {
        header.style.background = 'rgba(15, 13, 22, 0.95)';
    } else {
        header.style.background = 'rgba(15, 13, 22, 0.8)';
    }
});

// Intersection Observer for animations
const observerOptions = {
    threshold: 0.1,
    rootMargin: '0px 0px -50px 0px'
};

const observer = new IntersectionObserver(function(entries) {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.style.opacity = '1';
            entry.target.style.transform = 'translateY(0)';
        }
    });
}, observerOptions);

// Observe sections for scroll animations
document.querySelectorAll('.section-header, .feature-card, .workflow-step, .pricing-card, .post-card, .metric-card').forEach(el => {
    el.style.opacity = '0';
    el.style.transform = 'translateY(20px)';
    el.style.transition = 'opacity 0.6s ease, transform 0.6s ease';
    observer.observe(el);
});

// Terminal animation
document.addEventListener('DOMContentLoaded', function() {
    const terminalBody = document.querySelector('.terminal-body');
    if (terminalBody) {
        // Add typing effect to terminal
        const lines = terminalBody.querySelectorAll('.terminal-line');
        lines.forEach((line, index) => {
            line.style.opacity = '0';
            setTimeout(() => {
                line.style.opacity = '1';
                line.style.transform = 'translateX(0)';
            }, index * 300);
        });
    }
});

// Metrics update function
function updateMetrics() {
    // These would normally come from an API or GitHub
    const metrics = {
        stars: localStorage.getItem('githubStars') || '0',
        calls: localStorage.getItem('gatewayCalls') || '0',
        users: localStorage.getItem('payingUsers') || '0',
        clients: localStorage.getItem('serviceClients') || '0'
    };
    
    document.getElementById('github-stars').textContent = metrics.stars;
    document.getElementById('gateway-calls').textContent = metrics.calls;
    document.getElementById('paying-users').textContent = metrics.users;
    document.getElementById('service-clients').textContent = metrics.clients;
}

// Initialize metrics on load
document.addEventListener('DOMContentLoaded', updateMetrics);

// Service worker registration for PWA support
if ('serviceWorker' in navigator) {
    window.addEventListener('load', function() {
        // Service worker would be registered here for PWA functionality
        // navigator.serviceWorker.register('/sw.js');
    });
}

// Keyboard navigation for accessibility
document.addEventListener('keydown', function(e) {
    // Skip to main content with Ctrl+M
    if (e.ctrlKey && e.key === 'm') {
        const main = document.querySelector('main');
        if (main) {
            main.setAttribute('tabindex', '-1');
            main.focus();
        }
    }
});

// Add loading states for buttons
document.querySelectorAll('.btn').forEach(btn => {
    btn.addEventListener('click', function() {
        // Add loading state for async actions
        if (this.classList.contains('loading')) return;
        
        // For demo purposes, we'll just add a brief loading state
        if (this.textContent.includes('Join Waitlist') || 
            this.textContent.includes('Submit') ||
            this.textContent.includes('Join')) {
            const originalText = this.innerHTML;
            this.innerHTML = '<span class="loading-spinner"></span> Processing...';
            this.classList.add('loading');
            this.disabled = true;
            
            setTimeout(() => {
                this.innerHTML = originalText;
                this.classList.remove('loading');
                this.disabled = false;
            }, 2000);
        }
    });
});

// Console easter egg
console.log('%c Aetheris ', 'background: linear-gradient(135deg, #6366F1, #A855F7); color: white; font-size: 20px; font-weight: bold; padding: 10px 20px; border-radius: 5px;');
console.log('%c Your LLM keys never live in .env again ', 'color: #06B6D4; font-size: 14px;');
console.log('%c https://github.com/Aetheris/Aetheris ', 'color: #94A3B8; font-size: 12px;');
