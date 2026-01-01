# Motion Design Physics Guidelines
## "Kinetic Profiles" - Industrial Motion System

> **Classification:** Animation & Motion Design System
> **Purpose:** Define "heavy, precise, industrial" motion that feels like machinery
> **Philosophy:** "Snap-to-Grid" precision over smooth easing

---

## Core Principles

1. **Snap-to-Grid** - UI elements *snap* into place like machinery, not fade
2. **Data Flow Direction** - Animations pulse in the direction of logic flow (Cyan → Green)
3. **Error Glitch** - Errors have a specific chromatic aberration "twitch" before settling
4. **Heavy Precision** - Motion feels weighty and deliberate, not floaty
5. **Accessibility First** - Respect `prefers-reduced-motion` always

---

## Kinetic Profiles

### 1. Snap-to-Grid

**Purpose:** Elements should *snap* into place like industrial machinery, not smoothly ease.

**Implementation:**
```css
/* NO easing - instant snap */
.snap-element {
  transition: transform 0ms, opacity 0ms;
  /* Or use step-end for discrete steps */
  transition: transform 150ms steps(1, end);
}

/* Active state - instant appearance */
.snap-element.is-active {
  transform: translateY(0);
  opacity: 1;
}
```

**When to Use:**
- Status toggles (on/off states)
- Modal/dialog appearances
- Tab switching
- Accordion expansions
- Any binary state change

**Avoid:**
- Smooth easing (`ease-in-out`, `cubic-bezier`)
- Fade transitions for state changes
- Floaty animations

---

### 2. Data Flow Pulse

**Purpose:** Visualize the direction of reasoning/logic flow with directional pulse animations.

**Visual Design:**
- Pulse travels from Cyan → Green (success path)
- Pulse travels from Cyan → Orange (error path)
- Direction indicates data flow direction
- Speed: 300-500ms per pulse cycle

**Implementation:**
```css
/* Data flow connection line */
.data-flow-line {
  position: relative;
  background: linear-gradient(90deg, var(--cyan), var(--green));
  height: 2px;
  overflow: hidden;
}

/* Pulse animation - left to right */
@keyframes data-flow-pulse {
  0% {
    transform: translateX(-100%);
    opacity: 0;
  }
  50% {
    opacity: 1;
  }
  100% {
    transform: translateX(100%);
    opacity: 0;
  }
}

.data-flow-line::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 20%;
  height: 100%;
  background: var(--cyan-glow);
  animation: data-flow-pulse 400ms ease-in-out infinite;
}

/* Error flow - Cyan → Orange */
.data-flow-line--error {
  background: linear-gradient(90deg, var(--cyan), var(--orange));
}

.data-flow-line--error::before {
  background: var(--orange-glow);
}
```

**When to Use:**
- Connection lines between TraceNodes
- Progress indicators
- Status propagation
- Reasoning chain visualization

**Direction Mapping:**
- **Top → Bottom**: Parent to child reasoning
- **Left → Right**: Sequential processing
- **Cyan → Green**: Success path
- **Cyan → Orange**: Error/warning path

---

### 3. Error Glitch

**Purpose:** Errors should have a specific chromatic aberration "twitch" before settling into alert state.

**Visual Design:**
- Brief chromatic aberration effect (RGB channel separation)
- Horizontal "shake" (2-3px)
- Settles into orange/red alert state
- Duration: 200-300ms total

**Implementation:**
```css
@keyframes error-glitch {
  0% {
    transform: translateX(0);
    filter: hue-rotate(0deg);
  }
  10% {
    transform: translateX(-2px);
    filter: hue-rotate(-10deg) saturate(150%);
  }
  20% {
    transform: translateX(2px);
    filter: hue-rotate(10deg) saturate(150%);
  }
  30% {
    transform: translateX(-1px);
    filter: hue-rotate(-5deg);
  }
  40% {
    transform: translateX(1px);
    filter: hue-rotate(5deg);
  }
  50% {
    transform: translateX(0);
    filter: hue-rotate(0deg);
  }
  100% {
    /* Settle into error state */
    background: var(--orange);
    border-color: var(--orange);
  }
}

.error-element {
  animation: error-glitch 250ms ease-out;
  animation-fill-mode: forwards;
}

/* Chromatic aberration effect (requires pseudo-elements) */
.error-element::before,
.error-element::after {
  content: attr(data-text);
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
}

.error-element::before {
  color: rgba(255, 0, 0, 0.5);
  transform: translateX(-1px);
  z-index: -1;
}

.error-element::after {
  color: rgba(0, 0, 255, 0.5);
  transform: translateX(1px);
  z-index: -2;
}
```

**When to Use:**
- Error state transitions
- Validation failures
- Critical alerts
- System errors

**Avoid:**
- Overusing (only for errors, not warnings)
- Making it too subtle (should be noticeable but not jarring)

---

## Timing Functions

### Standard Timing

| Profile | Duration | Easing | Use Case |
|---------|----------|--------|----------|
| **Snap** | 0ms | `steps(1, end)` | Binary state changes |
| **Quick** | 100-150ms | `ease-out` | Micro-interactions |
| **Standard** | 200-300ms | `ease-out` | Standard transitions |
| **Heavy** | 400-500ms | `cubic-bezier(0.4, 0, 0.2, 1)` | Major state changes |
| **Data Flow** | 300-500ms | `ease-in-out` | Pulse animations |

### Custom Cubic Bezier

**Industrial Snap:**
```css
--easing-snap: cubic-bezier(0.95, 0.05, 0.8, 0.2);
/* Fast start, sharp stop */
```

**Heavy Machinery:**
```css
--easing-heavy: cubic-bezier(0.4, 0, 0.2, 1);
/* Weighty, deliberate */
```

**Data Flow:**
```css
--easing-flow: cubic-bezier(0.25, 0.46, 0.45, 0.94);
/* Smooth but purposeful */
```

---

## Animation Examples

### Status Toggle (Snap-to-Grid)

```css
.status-toggle__slider {
  transition: transform 0ms;
  transform: translateX(0);
}

.status-toggle__input:checked ~ .status-toggle__slider {
  transform: translateX(100%);
  /* Instant snap - no easing */
}
```

### TraceNode Appearance (Snap)

```css
.trace-node {
  opacity: 0;
  transform: scale(0.8);
  transition: none; /* No transition initially */
}

.trace-node.is-visible {
  opacity: 1;
  transform: scale(1);
  /* Instant appearance */
}
```

### Confidence Meter Fill (Data Flow)

```css
.confidence-meter__fill {
  stroke-dashoffset: 283; /* Full circle */
  transition: stroke-dashoffset 400ms var(--easing-flow);
}

.confidence-meter[data-value="0.85"] .confidence-meter__fill {
  stroke-dashoffset: 42; /* 85% of circle */
  /* Smooth but purposeful fill */
}
```

### Error State (Glitch)

```css
.trace-node--error {
  animation: error-glitch 250ms ease-out;
}

/* After glitch, settle into error state */
.trace-node--error {
  background: var(--orange);
  border-color: var(--orange);
}
```

---

## Reduced Motion

**Always respect user preferences:**

```css
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
  
  /* Disable all motion effects */
  .data-flow-line::before {
    animation: none !important;
  }
  
  .error-element {
    animation: none !important;
    /* Still show error state, just no glitch */
    background: var(--orange);
  }
}
```

---

## Performance Guidelines

1. **Use `transform` and `opacity`** - GPU accelerated
2. **Avoid animating `width`, `height`, `top`, `left`** - Causes reflow
3. **Use `will-change` sparingly** - Only for elements that will animate
4. **Limit simultaneous animations** - Max 3-4 per viewport
5. **Prefer CSS animations over JavaScript** - Better performance

**Example:**
```css
/* Good - GPU accelerated */
.animated-element {
  transform: translateX(100px);
  opacity: 0.5;
  transition: transform 300ms, opacity 300ms;
}

/* Bad - Causes reflow */
.animated-element {
  left: 100px;
  width: 200px;
  transition: left 300ms, width 300ms;
}
```

---

## Brand Color Animation Mapping

| Animation Type | Start Color | End Color | Direction |
|----------------|-------------|-----------|-----------|
| **Success Flow** | Cyan (`#06b6d4`) | Green (`#10b981`) | Left → Right |
| **Error Flow** | Cyan (`#06b6d4`) | Orange (`#f97316`) | Left → Right |
| **Processing** | Cyan (`#06b6d4`) | Purple (`#a855f7`) | Pulse (in-place) |
| **Warning** | Yellow (`#fbbf24`) | Orange (`#f97316`) | Pulse (in-place) |

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2026-01-01 | Initial kinetic profiles specification |

---

**"Designed, Not Dreamed" - Turn Prompts into Protocols**
*https://reasonkit.sh*
