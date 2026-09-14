# LemonSqueezy Checkout Setup for Aetheris

> **Status:** Configuration Guide
> **Last Updated:** September 2026
> **Owner:** W7-Market Workstream

---

## Overview

Aetheris uses **LemonSqueezy** as the payment processor and merchant of record. This handles:
- ✅ EU VAT compliance (no Steuerberater nightmare)
- ✅ Payment processing (credit cards, PayPal, etc.)
- ✅ License key generation and validation
- ✅ Subscription management
- ✅ Customer support portal

---

## Pricing Model

| Tier | Price | LemonSqueezy Product ID | Contents |
|------|-------|-------------------------|----------|
| Core | €0 | N/A (Open Source) | vault, gateway, hub, CLI, PWA |
| Sync | €5/month or €49/year | `sync-monthly` / `sync-yearly` | hosted hub, multi-device, backups |
| Desktop Pro | €89 lifetime | `desktop-pro` | Avalonia app, SSH terminal, SFTP |
| Compliance | Custom | `compliance` (future) | audit exports, DPA, SSO |

---

## Setup Instructions

### 1. Create LemonSqueezy Account

1. Go to [https://www.lemonsqueezy.com](https://www.lemonsqueezy.com)
2. Sign up for an account
3. Complete business verification
4. Set up your store (Aetheris)

### 2. Configure Products

#### Sync Tier - Monthly
- **Product Name:** Aetheris Sync - Monthly
- **Price:** €5.00
- **Currency:** EUR
- **Billing Interval:** Monthly
- **Product ID:** `sync-monthly`
- **Description:** Multi-device sync with hosted hub. Zero-knowledge architecture.

#### Sync Tier - Yearly
- **Product Name:** Aetheris Sync - Yearly
- **Price:** €49.00 (17% discount)
- **Currency:** EUR
- **Billing Interval:** Yearly
- **Product ID:** `sync-yearly`
- **Description:** Multi-device sync with hosted hub. Zero-knowledge architecture. Save 17% vs monthly.

#### Desktop Pro
- **Product Name:** Aetheris Desktop Pro
- **Price:** €89.00
- **Currency:** EUR
- **Billing Interval:** One-time
- **Product ID:** `desktop-pro`
- **Description:** Full-featured desktop application with SSH terminal and SFTP support. Lifetime license.

### 3. Configure License API

1. Go to **Settings > API** in LemonSqueezy dashboard
2. Generate a new API key
3. Store it securely (use Aetheris vault!)
4. Enable **License API** access

### 4. Set Up Webhooks

Configure webhooks for real-time notifications:

| Event | URL | Purpose |
|-------|-----|---------|
| `order_created` | `https://api.aetheris.dev/webhooks/lemonsqueezy` | Activate licenses |
| `subscription_created` | `https://api.aetheris.dev/webhooks/lemonsqueezy` | Activate sync access |
| `subscription_cancelled` | `https://api.aetheris.dev/webhooks/lemonsqueezy` | Deactivate sync access |
| `subscription_resumed` | `https://api.aetheris.dev/webhooks/lemonsqueezy` | Reactivate sync access |
| `license_key_created` | `https://api.aetheris.dev/webhooks/lemonsqueezy` | Store license key |

### 5. Configure Checkout Links

#### Sync Monthly
```
https://aetheris.lemonsqueezy.com/checkout/buy/<sync-monthly-variant-id>
```

#### Sync Yearly
```
https://aetheris.lemonsqueezy.com/checkout/buy/<sync-yearly-variant-id>
```

#### Desktop Pro
```
https://aetheris.lemonsqueezy.com/checkout/buy/<desktop-pro-variant-id>
```

---

## Integration Code

### License Validation (C#)

```csharp
using System;
using System.Net.Http;
using System.Text.Json;
using System.Threading.Tasks;

public class LemonSqueezyLicenseValidator
{
    private readonly string _apiKey;
    private readonly HttpClient _httpClient;
    
    public LemonSqueezyLicenseValidator(string apiKey)
    {
        _apiKey = apiKey;
        _httpClient = new HttpClient();
        _httpClient.DefaultRequestHeaders.Add("Authorization", $