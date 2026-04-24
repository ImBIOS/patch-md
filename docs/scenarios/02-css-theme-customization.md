# Scenario 2: CSS Theme Customization

## Problem Statement

You're using a UI library and need to customize the color scheme. When the library updates with new component styles, you want to keep your theme changes while accepting structural updates.

## Setup

```bash
mkdir -p scenario-2/.original themes

# Original theme.css from library
cat > themes/theme.css << 'EOF'
:root {
  --primary-color: #007bff;
  --secondary-color: #6c757d;
  --background-color: #ffffff;
  --text-color: #212529;
  --border-radius: 4px;
  --spacing: 16px;
}

.button {
  background-color: var(--primary-color);
  color: white;
  border-radius: var(--border-radius);
  padding: var(--spacing);
}

.card {
  background: var(--background-color);
  border: 1px solid #dee2e6;
  border-radius: var(--border-radius);
  padding: var(--spacing);
}
EOF

cp themes/theme.css .original/
```

## Your Brand Customization

```bash
cat > themes/theme.css << 'EOF'
:root {
  --primary-color: #8b5cf6;    /* Your brand purple */
  --secondary-color: #64748b;  /* Slate gray */
  --background-color: #0f172a; /* Dark theme */
  --text-color: #f8fafc;
  --border-radius: 8px;        /* Rounder corners */
  --spacing: 20px;
}

.button {
  background-color: var(--primary-color);
  color: white;
  border-radius: var(--border-radius);
  padding: var(--spacing);
  font-weight: 600;
  transition: transform 0.2s;
}

.card {
  background: var(--background-color);
  border: 1px solid #334155;
  border-radius: var(--border-radius);
  padding: var(--spacing);
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
}
EOF
```

## Tracking with patch-md

```bash
# Initialize and capture changes
patch-md init --target "ui-library@v2.3.0" --author "design-team"

# Capture the theme customization
patch-md add themes/theme.css --original .original/theme.css
```

### Generated PATCH.md

```markdown
# PATCH.md

## Metadata

| Key | Value |
|-----|-------|
| version | 1.0 |
| target | ui-library@v2.3.0 |
| created | 2026-04-24T12:00:00Z |
| author | design-team |

## Patches

### themes/theme.css

```diff
--- a/themes/theme.css
+++ b/themes/theme.css
@@ -1,11 +1,11 @@
 :root {
--  --primary-color: #007bff;
-+  --primary-color: #8b5cf6;
--  --secondary-color: #6c757d;
-+  --secondary-color: #64748b;
--  --background-color: #ffffff;
-+  --background-color: #0f172a;
--  --text-color: #212529;
-+  --text-color: #f8fafc;
--  --border-radius: 4px;
-+  --border-radius: 8px;
--  --spacing: 16px;
-+  --spacing: 20px;
 }
```

### themes/theme.css

```diff
@@ -9,6 +9,8 @@
   background-color: var(--primary-color);
   color: white;
   border-radius: var(--border-radius);
   padding: var(--spacing);
+  font-weight: 600;
+  transition: transform 0.2s;
 }

 .card {
@@ -16,5 +18,6 @@
   border: 1px solid #334155;
   border-radius: var(--border-radius);
   padding: var(--spacing);
+  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
 }
```

```

## Simulating Library Update

```bash
# Library adds new component styles
cat > themes/theme.css << 'EOF'
:root {
  --primary-color: #007bff;
  --secondary-color: #6c757d;
  --background-color: #ffffff;
  --text-color: #212529;
  --border-radius: 4px;
  --spacing: 16px;
  --font-family: system-ui, -apple-system, sans-serif;  /* NEW */
}

.button {
  background-color: var(--primary-color);
  color: white;
  border-radius: var(--border-radius);
  padding: var(--spacing);
  cursor: pointer;
}

/* NEW: Modal component */
.modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
}

/* NEW: Form styles */
.form-input {
  border: 1px solid #ced4da;
  border-radius: var(--border-radius);
  padding: calc(var(--spacing) / 2);
}
EOF
```

## Reconciliation

```bash
# Check what changed
patch-md diff themes/theme.css

# Apply your theme over the new library
patch-md apply --force
```

### Result - Your Dark Theme Preserved

```css
:root {
  --primary-color: #8b5cf6;    /* YOUR: Brand purple */
  --secondary-color: #64748b;  /* YOUR: Slate gray */
  --background-color: #0f172a; /* YOUR: Dark theme */
  --text-color: #f8fafc;      /* YOUR: Light text */
  --border-radius: 8px;        /* YOUR: Rounder corners */
  --spacing: 20px;             /* YOUR: Larger spacing */
  --font-family: system-ui, -apple-system, sans-serif;  /* NEW from library */
}

.button {
  background-color: var(--primary-color);
  color: white;
  border-radius: var(--border-radius);
  padding: var(--spacing);
  font-weight: 600;            /* YOUR: Bold text */
  transition: transform 0.2s;  /* YOUR: Smooth animation */
  cursor: pointer;            /* NEW from library */
}

.modal {                      /* NEW from library */
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
}

.card {
  background: var(--background-color);
  border: 1px solid #334155;
  border-radius: var(--border-radius);
  padding: var(--spacing);
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
}

.form-input {
  border: 1px solid #ced4da;
  border-radius: var(--border-radius);
  padding: calc(var(--spacing) / 2);
}
```

## Key Takeaways

- All your CSS variable customizations preserved (colors, spacing, border-radius)
- Your button enhancements (font-weight, transition) kept
- Your card shadow kept
- New library components (modal, form-input) added automatically
- New `cursor: pointer` from library merged with your styles

## Commands Used

```bash
patch-md init --target "ui-library@v2.3.0" --author "design-team"
patch-md add themes/theme.css --original .original/theme.css
patch-md diff themes/theme.css
patch-md apply --force
```