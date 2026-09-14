# Aether - AI Magic Showcase

> **Your Imagination, Unlimited** - A single-page web app that wows users with live AI demos and teaches them how to use AI.

Aether is designed to **inspire non-AI users** by showing them concrete, relatable use cases of artificial intelligence. It's a bridge between curiosity and adoption, demonstrating the power of AI through interactive, hands-on examples.

## 🎯 Mission

- **WOW** users with live AI demos
- **EDUCATE** them on how it works
- **INSPIRE** them to try AI themselves
- **CONVERT** them to [Aetheris](https://github.com/AetherisProject/Aetheris) for full power

## 🚀 Quick Start

### Option 1: Open Directly

Simply open `index.html` in your web browser. No server required!

```bash
# Navigate to the Aether directory
cd Aether

# Open in browser (on macOS)
open index.html

# Open in browser (on Linux)
xdg-open index.html

# Open in browser (on Windows)
start index.html
```

### Option 2: Local Server

For best results, serve the files using a local web server:

```bash
# Using Python 3
python3 -m http.server 8000

# Using Node.js (with http-server)
npx http-server

# Using PHP
php -S localhost:8000
```

Then open `http://localhost:8000` in your browser.

### Option 3: Deploy to the Web

Deploy the entire `Aether` folder to any static hosting service:

- **Netlify**: Drag and drop the folder to Netlify
- **Vercel**: `vercel --prod`
- **GitHub Pages**: Push to a GitHub repo and enable Pages
- **Cloudflare Pages**: Connect your repo and deploy

## 📁 Project Structure

```
Aether/
├── index.html              # Main showcase page
├── css/
│   └── style.css          # Beautiful, modern styling
├── js/
│   ├── app.js             # Main app logic
│   ├── puter-client.js    # Puter.js integration
│   └── demos.js           # All demo implementations
├── assets/
│   ├── images/           # Demo images, icons
│   └── data/             # Demo data (optional)
└── README.md              # This file
```

## ✨ Features

### 10 Interactive Demo Cards

Each demo card showcases a concrete, relatable use case:

1. **✉️ Write a Professional Email** - Generate business emails instantly
2. **🎨 Create a Beautiful Logo** - Design custom logos for your business
3. **📄 Summarize Documents** - Extract key points from long texts
4. **💻 Write Code** - Generate code from plain English descriptions
5. **🌍 Translate** - Break language barriers instantly
6. **📱 Generate Social Media Posts** - Create engaging content
7. **✈️ Plan Trips** - Get personalized travel itineraries
8. **🎓 Learn Anything** - Get personalized lessons on any topic
9. **📊 Analyze Data** - Find insights and create visualizations
10. **🍽️ Create Meal Plans** - Get personalized recipes

### Interactive Demo Interface

- **Model Selector**: Choose from multiple AI models (GPT-4o Mini, Claude 3, Gemini, Llama, Mistral)
- **Demo Type Selector**: Select what you want to try (chat, write, generate, summarize, translate, code)
- **Live Input/Output**: Type your request and see AI results in real-time
- **No Sign-up Required**: Try AI instantly, no account needed

### Educational Content

- **How It Works**: Simple 3-step explanation of AI
- **Use Case Categories**: Organized by work, creativity, development, education, etc.
- **Testimonials**: Real user stories and experiences

### Conversion Path

Clear comparison between Aether (demo) and Aetheris (full platform) with a call-to-action to upgrade.

## 🎨 Design

### Color Scheme

Aether uses a modern dark theme with vibrant accent colors:

- **Primary**: `#6366F1` (Indigo)
- **Secondary**: `#A855F7` (Purple)
- **Accent**: `#06B6D4` (Cyan)
- **Background**: `#090D16` (Deep Navy)
- **Surface**: `#1E293B` (Dark Slate)

### Typography

- **Font Family**: Inter (Google Fonts)
- **Fallback**: System fonts for better performance

### Responsive Design

Fully responsive across all devices:
- Desktop (1200px+)
- Tablet (768px - 1200px)
- Mobile (< 768px)

## 🤖 AI Integration

Aether uses **[Puter.js](https://js.puter.com/)** for AI capabilities:

- **500+ AI Models**: Access to a wide variety of models
- **No API Keys**: Works directly in the browser
- **Free Tier**: Generous free usage limits
- **Easy Integration**: Simple JavaScript API

### Supported Models

- **OpenAI**: GPT-4o Mini, GPT-4o
- **Anthropic**: Claude 3 Haiku, Claude 3 Sonnet
- **Google**: Gemini 1.5 Flash, Gemini 1.5 Pro
- **Meta**: Llama 3.1 70B, Llama 3.1 405B
- **Mistral**: Mistral Large, Mistral Small
- **And many more...**

## 🛠️ Customization

### Adding New Demos

1. Add a new demo to `js/demos.js`:

```javascript
DemoRegistry.register('new-demo', {
    name: 'My New Demo',
    description: 'What this demo does',
    category: 'category',
    icon: '🎯',
    tags: ['tag1', 'tag2'],
    difficulty: 'easy',
    examplePrompts: ['prompt 1', 'prompt 2'],
    run: async function(prompt, options = {}) {
        const client = window.PuterClient || window.puter;
        const response = await client.chat(`Custom prompt: ${prompt}`, options);
        return response.text;
    }
});
```

2. Add a demo card to `index.html`:

```html
<div class="demo-card" onclick="selectDemo('new-demo')">
    <div class="card-icon">🎯</div>
    <h3>My New Demo</h3>
    <p>What this demo does</p>
    <div class="card-meta">
        <span class="category category-name">Category</span>
        <span class="try-btn">Try It →</span>
    </div>
</div>
```

### Changing Colors

Edit the CSS variables in `css/style.css`:

```css
:root {
    --primary: #6366F1;
    --secondary: #A855F7;
    --accent: #06B6D4;
    /* ... */
}
```

### Adding New Models

Update the `availableModels` array in `js/app.js`:

```javascript
const availableModels = [
    { id: 'new-model', name: 'New Model (Description)' },
    // ...
];
```

## 📊 Performance

### Optimizations

- **Lazy Loading**: Puter.js loads only when needed
- **Rate Limiting**: Prevents API abuse
- **Caching**: Models list is cached
- **Efficient DOM**: Minimal re-renders

### Bundle Size

- **Total**: ~50KB (uncompressed)
- **CSS**: ~25KB
- **JS**: ~25KB
- **No Dependencies**: Pure HTML/CSS/JS

## 🌐 Browser Support

- **Chrome**: ✅ Full support
- **Firefox**: ✅ Full support
- **Safari**: ✅ Full support
- **Edge**: ✅ Full support
- **Mobile Browsers**: ✅ Full support

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Test thoroughly
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **[Puter.js](https://js.puter.com/)** - AI capabilities in the browser
- **[Inter Font](https://rsms.me/inter/)** - Beautiful typography
- **[Font Awesome](https://fontawesome.com/)** - Icons
- **[Aetheris Project](https://github.com/AetherisProject/Aetheris)** - Full AI platform

## 📞 Contact

- **Website**: [https://github.com/AetherisProject/Aetheris](https://github.com/AetherisProject/Aetheris)
- **GitHub**: [AetherisProject](https://github.com/AetherisProject)
- **Issues**: [GitHub Issues](https://github.com/AetherisProject/Aetheris/issues)

---

**Made with ❤️ by the Aetheris Team**

*Your imagination, unlimited.*
