# Brand Expansion Packs - Installation Guides

> **Quick reference for installing all brand expansion packs**

---

## 📦 1. ReasonTerminal (Starship Theme)

### Installation

```bash
# 1. Install Starship (if not already installed)
curl -sS https://starship.rs/install.sh | sh

# 2. Copy configuration
cp reasonkit-core/brand/expansion-packs/REASONTERMINAL_STARSHIP.toml ~/.config/starship.toml

# 3. Add to shell config
echo 'eval "$(starship init zsh)"' >> ~/.zshrc
# OR for bash:
echo 'eval "$(starship init bash)"' >> ~/.bashrc

# 4. Reload shell
source ~/.zshrc  # or ~/.bashrc
```

### Verification

Your prompt should now show:

```
┌─ /path/to/project RUST_CORE::1.74.0 ☍ main ✓ SYNC
└─>
```

---

## 📦 2. VS Code Theme - Protocol Mode

### Manual Installation

```bash
# 1. Copy extension to VS Code extensions folder
cp -r reasonkit-core/brand/expansion-packs/vscode-extension ~/.vscode/extensions/reasonkit-protocol-mode

# 2. Reload VS Code
# Command Palette (Ctrl+Shift+P / Cmd+Shift+P) → "Developer: Reload Window"

# 3. Select theme
# Command Palette → "Color Theme" → "ReasonKit Protocol Mode"
```

### Alternative: Symlink (for development)

```bash
# Create symlink for easy updates
ln -s $(pwd)/reasonkit-core/brand/expansion-packs/vscode-extension ~/.vscode/extensions/reasonkit-protocol-mode
```

---

## 📦 3. ReasonAudio (UI Sounds)

### Status

⏳ Audio files pending production

### When Available

**Web Implementation:**

```javascript
import { ReasonAudio } from "./reasonkit-audio";

const audio = new ReasonAudio();
audio.play("gigathink-start", 0.7);
```

**Rust Implementation:**

```rust
use reasonkit_audio::ReasonAudio;

let audio = ReasonAudio::new();
audio.play("gigathink-start");
```

**Location:** `reasonkit-core/brand/audio/*.wav`

---

## 📦 4. Reasoning Manifesto Poster

### Status

⏳ Image generation pending

### When Available

**Print (A2):**

- File: `reasonkit-core/brand/posters/reasoning-manifesto-a2.png`
- Size: 420 x 594 mm
- Resolution: 300 DPI

**Web Hero:**

- File: `reasonkit-core/brand/posters/reasoning-manifesto-hero.png`
- Size: 1920 x 1080 px

**Social Media:**

- File: `reasonkit-core/brand/posters/reasoning-manifesto-social.png`
- Size: 1200 x 1200 px

---

## 📦 5. 404/500/403 Error Pages

### Status

✅ Implemented

### Location

- `reasonkit-site/404.html`
- `reasonkit-site/500.html`
- `reasonkit-site/403.html`

### Integration

**Static Site:**

- Files are ready to use
- Configure web server to serve these files for error codes

**Next.js:**

```typescript
// pages/404.tsx
export { default } from "../reasonkit-site/404.html";
```

**React Router:**

```typescript
<Route path="*" element={<NotFound />} />
```

---

## 🎯 Quick Start Checklist

- [ ] Install Starship terminal theme
- [ ] Install VS Code Protocol Mode theme
- [ ] (Pending) Add ReasonAudio to project
- [ ] (Pending) Download Reasoning Manifesto poster
- [ ] Configure error pages in web server

---

## 📚 Documentation

- [ReasonAudio Spec](REASONAUDIO_SPEC.md)
- [ReasonTerminal Config](REASONTERMINAL_STARSHIP.toml)
- [VS Code Theme](vscode-extension/README.md)
- [Reasoning Manifesto Prompt](REASONING_MANIFESTO_IMAGE_PROMPT.md)
- [404 Void Page](404_VOID_PAGE.md)

---

**Last Updated:** 2025-01-01
