# Branding Integration Guide

> **Purpose:** Complete integration guide for all branding improvements into README.md, website, and general styling
> **Status:** ✅ All 7 branding improvements implemented and ready for integration

---

## 🎯 Overview

This guide provides step-by-step instructions for integrating all 7 branding improvements into:

- **README.md files** (all projects)
- **Website** (reasonkit-site)
- **General styling** (CSS, components, documentation)

---

## ✅ Completed Branding Improvements

### 1. ✅ Anti-Hype Copywriting Linter

**Location:** `.vale.ini` + `.vale/styles/ReasonKit/`
**Status:** Ready for CI/CD integration

### 2. ✅ High-Contrast Accessibility Mode

**Location:** `reasonkit-site/main.css` (lines 7051+)
**Status:** Implemented and active

### 3. ✅ Automated Brand-to-Web CI/CD Pipeline

**Location:** `reasonkit-core/.github/workflows/brand-assets.yml`
**Status:** Ready for GitHub Actions setup

### 4. ✅ ReasonUI Component Spec

**Location:** `reasonkit-core/brand/REASONUI_COMPONENT_SPEC.md`
**Status:** Complete specification ready for implementation

### 5. ✅ Motion Design Physics Guidelines

**Location:** `reasonkit-core/brand/MOTION_DESIGN_GUIDELINES.md`
**Status:** Complete guidelines ready for CSS implementation

### 6. ✅ Community Badge System

**Location:** `reasonkit-core/brand/COMMUNITY_BADGES.md` + `brand/badges/*.svg`
**Status:** Assets created, ready for README integration

### 7. ✅ Interactive 3D Asset Strategy

**Location:** `reasonkit-core/brand/3D_ASSET_STRATEGY.md`
**Status:** Complete specification ready for WebGL implementation

---

## 📋 Integration Checklist

### README.md Integration

#### Step 1: Add Community Badge

Add to the top of each project's README.md:

```markdown
[![Reasoned By ReasonKit](https://raw.githubusercontent.com/reasonkit/reasonkit-core/main/brand/badges/reasoned-by.svg)](https://reasonkit.sh)
```

**Files to update:**

- [ ] `reasonkit-core/README.md`
- [ ] `reasonkit-mem/README.md`
- [ ] `reasonkit-web/README.md`
- [ ] `README.md` (umbrella)

#### Step 2: Add Badge Section

Add a "Built With" or "Powered By" section:

````markdown
## 🏷️ Badge

If you use ReasonKit in your project, add our badge:

```markdown
[![Reasoned By ReasonKit](https://raw.githubusercontent.com/reasonkit/reasonkit-core/main/brand/badges/reasoned-by.svg)](https://reasonkit.sh)
```
````

See [Community Badges](brand/COMMUNITY_BADGES.md) for all variants.

````

#### Step 3: Reference Branding Resources

Add to documentation section:

```markdown
## 🎨 Branding

- [Brand Playbook](brand/BRAND_PLAYBOOK.md) - Complete brand guidelines
- [Component Spec](brand/REASONUI_COMPONENT_SPEC.md) - UI component system
- [Motion Guidelines](brand/MOTION_DESIGN_GUIDELINES.md) - Animation system
- [3D Assets](brand/3D_ASSET_STRATEGY.md) - WebGL integration guide
````

---

### Website Integration (reasonkit-site)

#### Step 1: Enable High-Contrast Mode

**Status:** ✅ Already implemented in `main.css`

The high-contrast mode is automatically enabled when users have `prefers-contrast: high` set in their system preferences.

**Verify:**

```bash
# Check CSS implementation
grep -A 20 "HIGH-CONTRAST INDUSTRIAL MODE" reasonkit-site/main.css
```

#### Step 2: Integrate ReasonUI Components

**Location:** `reasonkit-core/brand/REASONUI_COMPONENT_SPEC.md`

**Implementation Steps:**

1. Create component library in `reasonkit-site/src/components/reasonui/`
2. Implement 4 core components:
   - `TraceNode.tsx` - Reasoning chain visualization
   - `ConfidenceMeter.tsx` - Confidence gauge
   - `LogStream.tsx` - Terminal-like logs
   - `StatusToggle.tsx` - Industrial switches
3. Import and use in documentation pages

**Example:**

```tsx
import { TraceNode, ConfidenceMeter } from '@/components/reasonui';

<TraceNode step={step} confidence={0.85} />
<ConfidenceMeter value={0.85} format="radial" />
```

#### Step 3: Apply Motion Design Guidelines

**Location:** `reasonkit-core/brand/MOTION_DESIGN_GUIDELINES.md`

**Implementation Steps:**

1. Add motion utilities to `reasonkit-site/src/styles/motion.css`
2. Implement three Kinetic Profiles:
   - Snap-to-Grid transitions
   - Data Flow animations (Cyan→Green)
   - Error Glitch effects
3. Apply to interactive elements

**Example:**

```css
/* Snap-to-Grid */
.element {
  transition: transform 0.1s cubic-bezier(0.4, 0, 0.2, 1);
}

/* Data Flow */
.data-flow {
  animation: pulse-cyan-green 2s ease-in-out infinite;
}
```

#### Step 4: Integrate 3D Assets

**Location:** `reasonkit-core/brand/3D_ASSET_STRATEGY.md`

**Implementation Steps:**

1. Create GLB/gLTF assets from Luminous Polyhedron
2. Implement Three.js/React Three Fiber components
3. Add to hero section (slow-rotating polyhedron)
4. Create interactive Tree of Thoughts visualization

**Example:**

```tsx
import { Canvas } from "@react-three/fiber";
import { Polyhedron } from "@/components/3d/Polyhedron";

<Canvas>
  <Polyhedron rotation={[0, time * 0.5, 0]} />
</Canvas>;
```

---

### CI/CD Integration

#### Step 1: Enable Vale Linter

**Location:** `.vale.ini` + `.vale/styles/ReasonKit/`

**Add to CI workflow:**

```yaml
# .github/workflows/ci.yml
- name: Check Brand Voice
  run: |
    vale --version || npm install -g @errata-ai/vale
    vale --config=.vale.ini --minAlertLevel=warning .
```

#### Step 2: Enable Brand Assets Pipeline

**Location:** `reasonkit-core/.github/workflows/brand-assets.yml`

**Setup:**

1. Add required secrets:
   - `AWS_ACCESS_KEY_ID` (optional)
   - `AWS_SECRET_ACCESS_KEY` (optional)
   - `S3_BUCKET` (optional)
   - `REASONKIT_SITE_WEBHOOK` (optional)

2. Workflow automatically triggers on `brand/*` changes

**Manual trigger:**

```bash
gh workflow run "Brand Assets Pipeline"
```

---

## 🎨 Styling Integration

### CSS Variables (Already in main.css)

The brand colors are already defined as CSS variables:

```css
:root {
  --primary: #06b6d4; /* Cyan */
  --secondary: #a855f7; /* Purple */
  --tertiary: #ec4899; /* Pink */
  --background: #030508; /* Void Black */
  --surface: #0a0d14; /* Deep Black */
  --text: #f9fafb; /* Primary Text */
}
```

### High-Contrast Mode (Already Implemented)

```css
@media (prefers-contrast: high) {
  :root {
    --background: #000000; /* Pure Black */
    --text: #ffffff; /* Pure White */
    --border: #ffffff; /* 100% saturation */
  }
}
```

### Motion Utilities (To Add)

Create `reasonkit-site/src/styles/motion.css`:

```css
/* Snap-to-Grid */
.snap-transition {
  transition: transform 0.1s cubic-bezier(0.4, 0, 0.2, 1);
}

/* Data Flow */
@keyframes pulse-cyan-green {
  0%,
  100% {
    border-color: var(--primary);
  }
  50% {
    border-color: var(--success);
  }
}

/* Error Glitch */
@keyframes glitch {
  0%,
  100% {
    transform: translate(0);
  }
  20% {
    transform: translate(-2px, 2px);
  }
  40% {
    transform: translate(-2px, -2px);
  }
  60% {
    transform: translate(2px, 2px);
  }
  80% {
    transform: translate(2px, -2px);
  }
}
```

---

## 📝 Documentation Updates

### Update Brand Playbook

Add references to new branding improvements:

```markdown
## Branding Infrastructure

### Automated Systems

- **Vale Linter**: `.vale.ini` - Enforces brand voice programmatically
- **CI/CD Pipeline**: `brand-assets.yml` - Automated brand-to-web sync
- **High-Contrast Mode**: `main.css` - WCAG AAA accessibility

### Component Systems

- **ReasonUI**: `REASONUI_COMPONENT_SPEC.md` - UI component library
- **Motion Design**: `MOTION_DESIGN_GUIDELINES.md` - Animation system
- **3D Assets**: `3D_ASSET_STRATEGY.md` - WebGL integration

### Community Tools

- **Badges**: `COMMUNITY_BADGES.md` - Community badge system
```

---

## ✅ Verification Checklist

### README Integration

- [ ] Badge added to all README.md files
- [ ] Badge section documented
- [ ] Branding resources linked

### Website Integration

- [ ] High-contrast mode verified
- [ ] ReasonUI components implemented
- [ ] Motion guidelines applied
- [ ] 3D assets integrated

### CI/CD Integration

- [ ] Vale linter enabled in CI
- [ ] Brand assets pipeline configured
- [ ] Webhooks set up (if using)

### Documentation

- [ ] Brand playbook updated
- [ ] Integration guide complete
- [ ] Examples provided

---

## 🚀 Quick Start

### For README.md

1. Copy badge code from `COMMUNITY_BADGES.md`
2. Paste at top of README
3. Add badge section with usage instructions

### For Website

1. Import ReasonUI components
2. Apply motion utilities
3. Add 3D assets to hero section

### For CI/CD

1. Enable Vale in CI workflow
2. Configure brand-assets.yml secrets
3. Test workflow with manual trigger

---

## 📚 Reference Documents

- [Brand Playbook](BRAND_PLAYBOOK.md) - Master brand guidelines
- [Community Badges](COMMUNITY_BADGES.md) - Badge system
- [ReasonUI Component Spec](REASONUI_COMPONENT_SPEC.md) - UI components
- [Motion Design Guidelines](MOTION_DESIGN_GUIDELINES.md) - Animations
- [3D Asset Strategy](3D_ASSET_STRATEGY.md) - WebGL integration

---

**Last Updated:** 2025-01-01  
**Status:** ✅ All improvements implemented, ready for integration
