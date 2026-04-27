# Localization Review (i18n / L10n)

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate whether the application is prepared to support multiple languages and locales — either now or in the future. Internationalization (i18n) is the architectural work of making localization possible: separating user-visible strings from code, handling locale-sensitive formatting, and accommodating text expansion. Localization (L10n) is the work of providing content in a specific locale. This review evaluates i18n readiness; L10n content itself is out of scope unless a specific locale is being validated.

This domain applies to any user-facing application that may be used by people who speak languages other than the implementation language, or who have locale-specific formatting expectations (dates, numbers, currency, addresses). For applications explicitly scoped to a single locale in DESIGN.md, this domain evaluates whether that scope is correctly enforced and whether the architecture allows the scope to be revisited.

## Current Review Prompt

**Scope:** All user-visible strings, date/time handling, number formatting, layout, and cultural assumptions.

Read DESIGN.md for stated locale scope and i18n requirements. Then read all source files with focus on user-visible content, formatting functions, validation logic, and layout code.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), **dismissed** (no action taken, rationale required), **accepted scope** (application is intentionally single-locale; finding is a known limitation explicitly documented in DESIGN.md), or **hallucinated** (the adversary invented a problem that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal).

**Coordination:** Flag findings that overlap with UX (layout flexibility, text overflow), SE (string handling, encoding), SA (i18n architecture decisions), and PE (locale testing in CI).

**Sycophancy check:** An agent generates strings, validation rules, and format assumptions based on its training distribution, which is heavily weighted toward English and US locale conventions. It will not flag its own locale assumptions because they are invisible to it — they are the default. The adversary must surface every place where a locale assumption is baked in and evaluate whether it is intentional and documented or accidental and fragile.

**Language and interface supplement:** Consult `../../lang/` for language-specific i18n library recommendations and conventions.

## Standard Evaluation Dimensions

1. **String externalization** — Are user-visible strings stored in a translatable resource file (JSON, YAML, `.po`, `Localizable.strings`, etc.) rather than hardcoded in application code? A string literal in a UI-rendering function is not translatable without a code change. Named check: search the codebase for string literals that would appear in the UI. Each one that is not in a resource file is a finding. For projects explicitly scoped to a single language: is that decision documented, and does the architecture make future externalization feasible without a full rewrite?

2. **Date and time formatting** — Are dates and times displayed using locale-appropriate formats? Named failure modes: hardcoded format strings like `MM/DD/YYYY` (US-centric); `toLocaleDateString()` called without an explicit locale (produces inconsistent output across environments); ISO 8601 strings displayed directly to users; relative time labels ("2 days ago") that are grammatically correct only in English. The correct pattern is to store timestamps as UTC epoch values and format for display using the user's locale setting.

3. **Number, currency, and unit formatting** — Are numbers formatted with locale-appropriate decimal separators and grouping? Named failure modes: hardcoded `.` as decimal separator (a comma in many locales); no grouping separator for large numbers; currency values formatted without a currency symbol or with a hardcoded `$`; unit values that are only meaningful in one measurement system. Use `Intl.NumberFormat`, `Intl.PluralRules`, or equivalent locale-aware formatting APIs rather than string concatenation.

4. **Text direction** — Does the layout support right-to-left (RTL) languages such as Arabic, Hebrew, and Persian? Named failure modes: CSS that hardcodes `left` and `right` directional properties instead of `start` and `end` (logical properties); text alignment that assumes LTR; icons that convey direction (arrows, progress indicators) that are not mirrored for RTL. For a single-locale English application, this dimension may be noted as deferred; for any application expected to support RTL locales, it is a design-time decision, not an afterthought.

5. **Text expansion tolerance** — Does the UI accommodate strings that are 30–100% longer than the English original? German strings average 30% longer than English equivalents; Finnish, Hungarian, and some technical terms in other languages can be 2–3x longer. Named failure modes: fixed-width buttons with English-length text that overflow or truncate in other languages; modal titles that are clipped; table columns with a fixed character width. Test by replacing English strings with a synthetic long string (e.g., "AAAAAAAAAAAAAAAAAAAAAA") and verifying the layout does not break.

6. **Plural rules** — Does the application handle plural forms correctly? English has two plural forms (1 item, N items). Arabic has six. Russian, Polish, and Czech have three. Named failure modes: `count + " items"` without a plural rules library; `count === 1 ? "item" : "items"` hardcoded for English two-form logic; error messages with quantities that read grammatically in English and incorrectly in languages with different rules. Use `Intl.PluralRules` or a dedicated i18n library for any user-visible quantity.

7. **Locale-sensitive validation** — Are validation rules locale-aware where appropriate? Named failure modes: phone number validation that only accepts US formats; postal code validation hardcoded for a specific country's format; name validation that rejects characters from non-Latin scripts; address form fields that assume a US address structure (state, ZIP). If validation is intentionally single-locale, that must be documented in DESIGN.md — not silently encoded in a regex.

8. **Character encoding** — Is Unicode handled correctly throughout the stack? Named failure modes: byte-length assumptions on strings that may contain multi-byte characters; `string.length` used where codepoint count or grapheme cluster count is needed (emoji, combined characters); string truncation that splits multi-byte sequences; storage that silently converts to a narrower encoding.

9. **Cultural neutrality** — Are colors, icons, metaphors, imagery, and content culturally appropriate across the target locales? Named concerns: red used to mean success in some cultures and failure in others; left-hand gestures or thumbs-up that carry different meanings across cultures; date conventions that are ambiguous (01/02/03 is interpreted differently in US, UK, and ISO contexts); content that assumes a specific legal or cultural context (e.g., "first name / last name" name order).

10. **Locale testing strategy** — Is there a mechanism to test the application in a non-English locale without requiring full translation? Named approaches: a pseudo-localization pass that replaces strings with accented equivalents and expanded length ([ÃÃÃ ŜŦŖĨÑĜ ÃÃÃ]); an explicit `lang` attribute on the `<html>` element that can be toggled; a locale selector in development mode. Without a testing mechanism, i18n failures are discovered only when a real localization is attempted.

---

Review entries are logged in `iterative-adversarial-refinement/LOCALIZATION-REVIEW.md` inside the project being reviewed.
