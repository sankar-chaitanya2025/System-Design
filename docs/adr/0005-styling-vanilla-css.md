# ADR-0005: Vanilla CSS with design tokens; remove Tailwind

- **Status:** Accepted
- **Date:** 2026-10-26
- **Resolves:** Review finding R9 (styling portion)

## Context

The scaffold lists `tailwindcss`, `postcss` and `autoprefixer` in `devDependencies` and puts `@tailwind` directives in `globals.css`, but has no `tailwind.config` or `postcss.config`, so Tailwind never actually runs. The canvas is drawn by React Flow plus custom overlays whose colours come from CSS custom properties.

## Decision

1. Use **vanilla CSS** with a token layer of CSS custom properties (colour, spacing, radius, typography, motion) in `apps/web/src/styles/tokens.css`, imported by `globals.css`.
2. Write component styles as plain CSS files co-located with components (or CSS Modules if name collisions become a problem).
3. Remove `tailwindcss`, `postcss` and `autoprefixer` from dependencies, and remove the `@tailwind` directives and Tailwind utility classes from `index.html`.
4. Self-host fonts (Inter, JetBrains Mono) because COEP `require-corp` blocks third-party font CDNs (see ADR-0002).

## Consequences

- No build-time CSS tooling. Vite handles plain CSS natively.
- Canvas renderers and DOM components share one set of tokens through CSS variables.
- Contributors who are used to Tailwind lose utility classes.
