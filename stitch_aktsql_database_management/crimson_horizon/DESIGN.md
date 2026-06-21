---
name: Crimson Horizon
colors:
  surface: '#121414'
  surface-dim: '#121414'
  surface-bright: '#383939'
  surface-container-lowest: '#0d0e0f'
  surface-container-low: '#1b1c1c'
  surface-container: '#1f2020'
  surface-container-high: '#292a2a'
  surface-container-highest: '#343535'
  on-surface: '#e3e2e2'
  on-surface-variant: '#e9bcb5'
  inverse-surface: '#e3e2e2'
  inverse-on-surface: '#303031'
  outline: '#b08781'
  outline-variant: '#5f3f3a'
  surface-tint: '#ffb4a8'
  primary: '#ffb4a8'
  on-primary: '#690000'
  primary-container: '#e60000'
  on-primary-container: '#fff7f5'
  inverse-primary: '#c00000'
  secondary: '#c9c6c5'
  on-secondary: '#313030'
  secondary-container: '#4a4949'
  on-secondary-container: '#bab8b7'
  tertiary: '#c8c6c5'
  on-tertiary: '#313030'
  tertiary-container: '#737272'
  on-tertiary-container: '#fbf8f7'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#ffdad4'
  primary-fixed-dim: '#ffb4a8'
  on-primary-fixed: '#410000'
  on-primary-fixed-variant: '#930100'
  secondary-fixed: '#e5e2e1'
  secondary-fixed-dim: '#c9c6c5'
  on-secondary-fixed: '#1c1b1b'
  on-secondary-fixed-variant: '#474646'
  tertiary-fixed: '#e5e2e1'
  tertiary-fixed-dim: '#c8c6c5'
  on-tertiary-fixed: '#1c1b1b'
  on-tertiary-fixed-variant: '#474746'
  background: '#121414'
  on-background: '#e3e2e2'
  surface-variant: '#343535'
typography:
  headline-lg:
    fontFamily: Inter
    fontSize: 32px
    fontWeight: '700'
    lineHeight: 40px
    letterSpacing: -0.02em
  headline-md:
    fontFamily: Inter
    fontSize: 24px
    fontWeight: '600'
    lineHeight: 32px
  body-md:
    fontFamily: Inter
    fontSize: 14px
    fontWeight: '400'
    lineHeight: 20px
  code-md:
    fontFamily: JetBrains Mono
    fontSize: 13px
    fontWeight: '400'
    lineHeight: 18px
  label-sm:
    fontFamily: JetBrains Mono
    fontSize: 11px
    fontWeight: '500'
    lineHeight: 16px
spacing:
  unit: 4px
  container-padding: 16px
  gutter: 12px
  density-compact: 4px
  density-comfortable: 12px
---

## Brand & Style

The design system is engineered for high-performance database management, drawing inspiration from the efficiency and precision of the Unix philosophy and the striking visual identity of the Akt. The brand personality is authoritative, clandestine, and utility-obsessed. It targets technical power users who prioritize speed of thought and data density over decorative flourishes.

The visual style is a fusion of **Modern Minimalism** and **Technical Brutalism**. It utilizes high-contrast interfaces, sharp geometric containers, and a rigid adherence to functional hierarchy. The emotional response is one of total control—a "Dawn of Database Management" where complex operations feel surgical and decisive.

## Colors

The palette is anchored by "Crimson Red," used strictly for primary actions, critical states, and brand highlights. 

**Dark Mode (Default):**
Uses "Deep Black" (#0A0A0A) for the primary canvas to minimize eye strain and maximize contrast. Surfaces are layered using subtle shifts in gray (#1A1A1A) to maintain a flat, technical aesthetic without relying on traditional shadows.

**Light Mode:**
Reverts to a sterile "Clean White" (#FFFFFF) and "Slate Gray" (#F5F5F5) foundation. Crimson Red remains the high-alert accent, ensuring brand continuity while providing a high-legibility environment for daylight operations.

**Semantic Utility:**
- **Success:** Emerald Green (reserved for query completions).
- **Warning:** Deep Amber.
- **Error/Critical:** Crimson Red.

## Typography

This design system employs a dual-font strategy to separate UI navigation from data processing. 

- **Inter (Sans-serif):** Used for all structural UI elements, navigation, and headers. It provides a neutral, professional tone that keeps the interface modern.
- **JetBrains Mono (Monospace):** The workhorse for all data-heavy contexts, including SQL editors, cell values, and metadata labels. 

Typography is scaled for high density. Line heights are kept tight to allow more information per screen inch, mirroring the efficiency of a terminal environment.

## Layout & Spacing

The layout follows a **Fixed Grid** model for sidebar-driven navigation and a **Fluid Grid** for data tables and editors. 

- **Sidebar:** Fixed at 240px for primary navigation.
- **Main Canvas:** Uses a 12-column system with tight 12px gutters.
- **Data Density:** Spacing is strictly based on a 4px baseline grid. Elements in the database view use "Compact" spacing (4px/8px) to maximize the "above-the-fold" data visibility.

**Mobile Reflow:**
For the rare mobile use case, the 12-column grid collapses to a single column, and the sidebar is hidden behind a crimson-accented drawer menu. High-density tables switch to a horizontal scroll or card-stack view.

## Elevation & Depth

This design system rejects soft shadows and ambient light. Depth is communicated through **Tonal Layers** and **Bold Outlines**.

- **Level 0 (Base):** Deep Black (#0A0A0A) for the workspace background.
- **Level 1 (Panels):** Slate Gray (#1A1A1A) with a 1px solid border (#333333).
- **Level 2 (Popovers/Tooltips):** Lighter Gray (#2A2A2A) with a Crimson Red 2px top-border to indicate "active" focus.

Interactive elements do not "lift" off the page; instead, they change border color or background saturation to signal state changes. This maintains the "flat terminal" feel of the Unix philosophy.

## Shapes

The shape language is strictly **Sharp (0px)**. 

Every UI element—from buttons and input fields to large container cards—features 90-degree corners. This evokes a sense of technical precision and structural rigidity. The only exception to the "no curves" rule is the Crimson Red cloud-inspired brand mark, which acts as a visual disruptor against the clinical sharpness of the interface.

## Components

**Buttons:** 
Sharp corners. Primary buttons are solid Crimson Red with White text. Secondary buttons are ghost-style with a 1px white or gray border. No gradients or shadows.

**Input Fields:**
Dark backgrounds with a 1px bottom-border only. When focused, the border turns Crimson Red. Errors are indicated by a solid red left-border.

**Data Tables:**
The core component. Minimal padding, Zebra striping using #111111 on the Deep Black base. Column headers use Monospace font in all-caps.

**Chips/Badges:**
Used for status (e.g., "Connected," "Indexing"). These are small, rectangular blocks with solid color fills and Monospace text.

**Navigation Rail:**
A slim 64px vertical bar on the far left, containing sharp, minimal line icons. Active states are indicated by a high-saturation Crimson Red vertical line.

**Query Editor:**
Full-width monospaced environment with syntax highlighting that utilizes the Crimson and Slate palette for a bespoke, branded coding experience.