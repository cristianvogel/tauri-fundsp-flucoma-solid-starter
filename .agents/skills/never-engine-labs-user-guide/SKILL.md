---
name: never-engine-labs-user-guide
description: Write NeverEngine Labs user guides for creative audio tools with the established voice, pacing, and visual layout style; include full UI coverage, technical context, up-to-date copyright, and open source acknowledgements from the codebase.
---

# NeverEngine Labs User Guide

## When to use
- Use for writing or revising manuals for NeverEngine Labs audio tools.
- Use when output must match prior NeverEngine Labs guide tone, rhythm, and layout.
- Use when manuals must include full user-facing feature coverage, technical background, and creative usage guidance.

## Read first
- Read `references/style-dna.md` before drafting.

## Workflow
1. Build a feature inventory from the current codebase and UI.
2. Draft a coverage matrix: each user-facing control/feature maps to one manual section.
3. Write the manual in the NeverEngine Labs voice and cadence from `references/style-dna.md`.
4. Add visuals:
   - Cover artwork in the established abstract geometric style.
   - UI illustrations/screenshots with labels and callouts.
   - Signal-flow diagrams where useful.
5. Enforce section completeness:
   - Conceptual overview
   - Installation/setup
   - Full parameter reference
   - Workflows and creative tips
   - Technical core and innovation notes
   - Troubleshooting and constraints
   - Version/license/copyright page
   - Open source acknowledgements page
6. Generate acknowledgements from code:
   - Run `scripts/generate_oss_acknowledgements.sh`.
   - Paste output into an "Open Source Acknowledgements" page.
7. Update copyright years to current release context.
8. Run a final coverage pass against the matrix and fill any gaps.

## Writing style requirements
- Audience: technically literate creators (sound designers/composers), intermediate to expert.
- Tone: authoritative, artist-engineer, explanatory without academic padding.
- Rhythm: alternate denser conceptual paragraphs with practical parameter-level instructions.
- For each feature, cover:
  - Why it exists
  - How to use it
  - Creative implications
  - Performance/quality tradeoffs
- Use exact UI naming and identifiers where relevant.

## Layout requirements
- Strong title hierarchy and generous whitespace.
- Light neutral page backgrounds with high-contrast body text.
- Restrained accent colors and geometric motifing consistent with prior guides.
- Readable line lengths, no wall-to-wall text blocks.
- Consistent page numbering.
- Callout boxes for tips, caveats, and setup-critical notes.

## Technical accuracy requirements
- Never invent controls, modes, ranges, or defaults.
- Validate values and behavior from current source code.
- If behavior is mode dependent, document per mode explicitly.
- Include real constraints and edge cases that affect production use.

## Copyright and acknowledgements requirements
- Include a dedicated copyright/license page.
- Include publication month/year and author attribution.
- Include a dedicated open source acknowledgements page generated from current code.
- If a dependency license is not discoverable, mark it as `License: Unknown (verify)`.

## Deliverables
- Manual source (markdown/doc format as requested).
- Final PDF with illustrations and polished layout.
- Coverage matrix used for completeness checks.
- Generated OSS acknowledgements markdown.
