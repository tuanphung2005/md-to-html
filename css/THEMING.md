# Color Theming Guide

This project includes a comprehensive color system using CSS custom properties (variables) that makes it easy to customize the appearance.

## Quick Start

The main stylesheet includes a light theme by default:
```html
<link rel="stylesheet" href="css/main.css">
```

To use a different theme, include it after the main stylesheet:
```html
<link rel="stylesheet" href="css/main.css">
<link rel="stylesheet" href="css/themes/dark.css">
```

## Available Themes

### Built-in Themes
- **`themes/light.css`** - Clean light theme (default)
- **`themes/dark.css`** - Professional dark theme  
- **`themes/blue.css`** - Professional blue theme
- **`themes/green.css`** - Nature-inspired green theme
- **`themes/purple.css`** - Professional purple theme
- **`themes/orange.css`** - Warm orange theme

### Theme Usage Examples

```html
<!-- Light theme (default) -->
<link rel="stylesheet" href="css/main.css">

<!-- Dark theme -->
<link rel="stylesheet" href="css/main.css">
<link rel="stylesheet" href="css/themes/dark.css">

<!-- Blue theme -->
<link rel="stylesheet" href="css/main.css">
<link rel="stylesheet" href="css/themes/blue.css">

<!-- Green theme -->
<link rel="stylesheet" href="css/main.css">
<link rel="stylesheet" href="css/themes/green.css">
```

## Creating Custom Themes

copy an existing theme file (e.g., `dark.css`) and modify the CSS variables to create your own theme.

## Available Color Variables

### Primary Colors
- `--primary-text`: Main text color
- `--secondary-text`: Secondary text (headings, strong text)
- `--tertiary-text`: Tertiary text (emphasis, captions)
- `--background`: Main background color

### Text Colors
- `--text-strong`: Color for `<strong>` elements
- `--text-emphasis`: Color for `<em>` elements

### Interactive Colors
- `--link-color`: Default link color
- `--link-hover`: Link hover state
- `--link-visited`: Visited link color

### UI Colors
- `--shadow-light`: Light shadows
- `--shadow-medium`: Medium shadows
- `--shadow-dark`: Dark shadows
- `--border-light`: Light borders
- `--border-medium`: Medium borders
- `--border-dark`: Dark borders

### Component Colors
- `--code-bg`: Code background
- `--code-border`: Code border
- `--code-text`: Code text
- `--table-header-bg`: Table header background
- `--table-border`: Table borders
- `--table-stripe`: Table striping
- `--blockquote-border`: Blockquote left border
- `--blockquote-bg`: Blockquote background
- `--blockquote-text`: Blockquote text

### Status Colors
- `--success`: Success/positive state
- `--warning`: Warning/caution state
- `--error`: Error/danger state
- `--info`: Information state

## Utility Classes

The base stylesheet also includes utility classes for quick styling:

### Text Colors
- `.text-primary`, `.text-secondary`, `.text-tertiary`
- `.text-success`, `.text-warning`, `.text-error`, `.text-info`

### Background Colors
- `.bg-primary`, `.bg-code`, `.bg-table`

### Border Colors
- `.border-light`, `.border-medium`, `.border-dark`

Example usage:
```html
<div class="text-success">This text will be green</div>
<div class="bg-code">This div will have a code background</div>
```
