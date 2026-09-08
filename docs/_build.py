#!/usr/bin/env python3
"""Aetheris Documentation HTML Generator — converts Markdown to styled HTML."""

import os
import sys
import re
from pathlib import Path

try:
    import markdown
except ImportError:
    os.system(f"{sys.executable} -m pip install markdown pygments")
    import markdown

DOCS_DIR = Path(__file__).parent

PAGES = {
    "index.md": "index.html",
    "getting-started.md": "getting-started.html",
    "architecture.md": "architecture.html",
    "security.md": "security.html",
    "features.md": "features.html",
    "terminal.md": "terminal.html",
    "vault.md": "vault.html",
    "api-keys.md": "api-keys.html",
    "sync.md": "sync.html",
    "authentication.md": "authentication.html",
    "proactive-engine.md": "proactive-engine.html",
    "browser-extension.md": "browser-extension.html",
    "mobile.md": "mobile.html",
    "web-app.md": "web-app.html",
    "development.md": "development.html",
    "api-reference.md": "api-reference.html",
    "configuration.md": "configuration.html",
    "i18n.md": "i18n.html",
    "design-system.md": "design-system.html",
    "debugging.md": "debugging.html",
    "deployment.md": "deployment.html",
}

NAV_HTML = """
<nav class="sidebar">
    <div class="sidebar-header">
        <a href="index.html" class="logo">
            <img src="assets/icons/logo.svg" alt="Aetheris" class="logo-icon">
            <span class="logo-text">Aetheris</span>
        </a>
        <div style="margin-top: 0.5rem;">
            <a href="brand.html" style="display:inline-block;font-size:0.75rem;padding:0.2rem 0.5rem;border-radius:9999px;background:rgba(6,182,212,0.15);border:1px solid rgba(6,182,212,0.3);color:#06b6d4;text-decoration:none;font-family:'JetBrains Mono',monospace;">Brand Portal &rarr;</a>
        </div>
    </div>
    <div class="sidebar-search">
        <input type="search" placeholder="Search docs..." id="search">
    </div>
    <ul class="nav-section"><li class="nav-section-title">Brand &amp; System</li>
        <li><a href="brand.html">Web Brand Showcase</a></li>
        <li><a href="design-system.html">Design Tokens &amp; Spec</a></li></ul>
    <ul class="nav-section"><li class="nav-section-title">Getting Started</li>
        <li><a href="getting-started.html">Quick Start</a></li></ul>
    <ul class="nav-section"><li class="nav-section-title">Architecture</li>
        <li><a href="architecture.html">Overview</a></li>
        <li><a href="security.html">Security</a></li>
        <li><a href="features.html">Features</a></li></ul>
    <ul class="nav-section"><li class="nav-section-title">Platform</li>
        <li><a href="terminal.html">Terminal</a></li>
        <li><a href="vault.html">Vault</a></li>
        <li><a href="api-keys.html">API Keys</a></li>
        <li><a href="sync.html">Sync</a></li>
        <li><a href="authentication.html">Auth</a></li></ul>
    <ul class="nav-section"><li class="nav-section-title">Client</li>
        <li><a href="browser-extension.html">Browser</a></li>
        <li><a href="mobile.html">Mobile</a></li>
        <li><a href="web-app.html">Web App</a></li></ul>
    <ul class="nav-section"><li class="nav-section-title">Advanced</li>
        <li><a href="proactive-engine.html">Proactive</a></li>
        <li><a href="api-reference.html">API Ref</a></li>
        <li><a href="configuration.html">Config</a></li>
        <li><a href="i18n.html">i18n</a></li></ul>
    <ul class="nav-section"><li class="nav-section-title">Dev</li>
        <li><a href="development.html">Setup</a></li>
        <li><a href="debugging.html">Debug</a></li>
        <li><a href="deployment.html">Deploy</a></li></ul>
</nav>
"""

HEADER = """<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1.0">
<meta name="theme-color" content="#6366f1">
<link rel="stylesheet" href="styles.css">
<link rel="icon" href="assets/icons/favicon.svg">
<link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@300;400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
<title>{title} — Aetheris</title>
</head>
<body><div class="docs-wrapper">
{nav}
"""

FOOTER = """
</div>
<footer class="footer">
<p>&copy; 2026 Aetheris (Merlin Tribukait). Published on <a href="https://aether.merlin-tribukait.com">aether.merlin-tribukait.com</a> under MIT License.</p>
<p><a href="https://github.com/merlin-tribukait/Aetheris">GitHub</a> | <a href="brand.html">Brand Portal</a> | <a href="security.html">Security</a></p>
</footer>
<script src="script.js"></script>
</body></html>
"""


def extract_title(md):
    m = re.search(r"^#\s+(.+)$", md, re.MULTILINE)
    return m.group(1).strip() if m else "Aetheris"


def build_page(md_file, html_file):
    md = md_file.read_text(encoding="utf-8") if md_file.exists() else f"# {md_file.stem}\n\nComing soon.\n"
    html = markdown.markdown(md, extensions=["fenced_code", "codehilite", "tables", "toc"])
    title = extract_title(md)
    full = HEADER.format(title=title, nav=NAV_HTML) + '<main class="content"><div class="content-inner">' + html + "</div></main>\n" + FOOTER
    html_file.write_text(full, encoding="utf-8")


def build_all():
    print("Building Aetheris documentation...")
    for md_name, html_name in PAGES.items():
        build_page(DOCS_DIR / md_name, DOCS_DIR / html_name)
        print(f"  {md_name} -> {html_name}")
    print(f"Done. Built {len(PAGES)} pages.")


if __name__ == "__main__":
    build_all()
