# JavaScript / TypeScript Language Supplement

These dimensions supplement the standard IAR domain reviews for JavaScript and TypeScript projects. During each domain review, apply the relevant section below in addition to the standard dimensions for that domain.

---

## Quality Engineering

- **`npm ci` in test runs** — Are tests run after `npm ci` (not `npm install`) to ensure the lock file is the source of truth?
- **Axe accessibility scanning** — Is `axe-playwright` or `axe-core` run in the test suite and configured to fail on violations? Manual review of accessibility is not sufficient — automated scanning is required for browser apps.
- **Browser test coverage** — Are tests run against an actual browser (Playwright, Cypress) for DOM-touching code? JSDOM-based tests miss rendering, focus, and layout behavior.
- **Type coverage** — Are `any` types and non-null assertions (`!`) minimized? Does `strict: true` (or equivalent) appear in `tsconfig.json`?

## Security

- **Rendering safety** — Is user-supplied content set via `.textContent` (safe) or `.innerHTML` (potentially unsafe)? Any `innerHTML` usage with user-controlled data is a finding.
- **URL injection** — Can a user save a URL that executes code when clicked? Verify `javascript:`, `data:`, `vbscript:`, and other non-http(s) protocols are rejected at input or before rendering.
- **`JSON.parse` runtime validation** — Is data from `JSON.parse` (localStorage reads, API responses, file reads) validated at runtime before being cast to a typed interface? TypeScript `as` casts are compile-time only and provide no runtime guarantee.
- **Prototype pollution** — Does any code merge user-supplied or externally-sourced objects into plain objects using `Object.assign`, spread (`{...obj}`), or property iteration? A payload containing `{"__proto__": {"isAdmin": true}}` or `{"constructor": {"prototype": {"isAdmin": true}}}` can silently modify `Object.prototype`. Mitigation patterns: validate parsed objects have no `__proto__` or `constructor` keys before merging; use `Object.create(null)` for property bags; use a schema validator that rejects prototype-polluting keys explicitly. This is distinct from XSS — the data may be structurally valid JSON and still exploit this path.
- **Dependency confusion** — Does the project use private package names that could be claimed by an attacker in the public npm registry? A package manager that resolves `@mycompany/utils` from the public registry instead of the private registry at a higher version number will install the attacker's package. Mitigation: scope packages to a private registry, audit `.npmrc` for registry configuration, and verify that `publishConfig` is set for all internal packages.
- **Content Security Policy** — Is a CSP present (meta tag or response header)? Evaluate the risk surface without one.
- **`npm audit`** — Are there known CVEs in direct or transitive dependencies? Is `npm audit` run in CI and configured to fail above the accepted risk threshold?

## Software Engineering

- **`as` casts require runtime validation** — Any TypeScript `as SomeType` cast applied to data from external sources (storage, API, user input) without a corresponding runtime validation function is a finding. The cast tells the compiler to trust you; the validation is what makes that trust warranted.
- **`any` types** — Are `any` types present in production code? Each is a type-safety hole. Flag uses that could be replaced with `unknown` + narrowing or a specific type.
- **Non-null assertions (`!`)** — Is `!` used on values that could be null or undefined in practice? Is each one justified?
- **`const` over `let`** — Is `let` used where `const` would communicate that the binding is not reassigned? `const` is not about immutability of the value but about the binding — it communicates intent.
- **Unhandled promise rejections** — Are `async` functions called without `await` or `.catch()`? Are promises dropped without handling rejection?
- **Error handling at async boundaries** — Are `try/catch` blocks present around `await` expressions that can fail (fetch, JSON.parse, storage operations)?

## Platform Engineering

- **`npm ci` in CI** — Is `npm ci` used in CI (not `npm install`) to ensure reproducible installs from the lock file?
- **`package-lock.json` committed** — Is the lock file committed and the source of truth for all installs?
- **`npm audit` in CI** — Is `npm audit` (or `npm audit --audit-level=moderate`) run in CI and configured to fail on findings above the accepted threshold?
- **Node version pinning** — Is the Node.js version pinned in `.nvmrc`, `.node-version`, or the CI workflow `node-version` field? Floating versions cause silent behavior changes.
- **TypeScript `strict` in CI** — Is `tsc --noEmit` run in CI to catch type errors? Are `strict: true` (or equivalent strictness flags) enforced?
- **Coverage enforcement** — Is test coverage measured in CI with thresholds enforced? Named configuration: Jest `coverageThreshold` in `jest.config.js`/`jest.config.ts`; Vitest `coverage.thresholds` in `vitest.config.ts`; `c8` or `nyc` for other test runners. The threshold must be configured to fail the build when missed — measuring coverage locally without a CI gate is not enforcement. Minimum 80% line coverage; 100% for exported public API functions.

## Data Engineering

- **Runtime schema validation** — Is data from external sources (localStorage, `fetch`, `JSON.parse`, URL params) validated with a runtime schema library (`zod`, `io-ts`, `valibot`) or an explicit validation function? TypeScript types disappear at runtime.
- **`JSON.parse` error handling** — Is `JSON.parse` wrapped in `try/catch`? Malformed stored data is a realistic failure mode.
- **Normalization functions** — When stored data is read, is there a normalization function that applies safe defaults for missing or unexpected fields, rather than trusting the raw object shape?
- **Date handling** — Are dates stored as ISO 8601 strings or Unix timestamps, not `Date` objects? `Date` objects do not survive `JSON.stringify`/`JSON.parse` as dates.

## Red Team

- **Prototype pollution exploitation** — Does any code merge user-supplied or parsed objects into plain objects? A payload containing `{"__proto__": {"isAdmin": true}}` silently modifies `Object.prototype` if the merge is unguarded. Test by: sending a request or input containing `__proto__` or `constructor.prototype` keys and observing whether application behavior changes. Mitigation: validate parsed objects against a schema that explicitly rejects these keys before merging; use `Object.create(null)` for property bags.
- **DOM-based XSS sinks** — Enumerate every sink where attacker-controlled data reaches the DOM: `innerHTML`, `outerHTML`, `insertAdjacentHTML`, `document.write`, `eval`, `setTimeout`/`setInterval` with a string argument, `location.href = userInput`, `<script src=userInput>`. For each sink: can an attacker control the value? Can a `javascript:` URI or inline script payload reach it?
- **JWT algorithm confusion** — If the application validates JWTs client-side or server-side with a JS library: does it explicitly require the expected algorithm (`RS256`, `HS256`) and reject `alg: none`? Popular libraries (jsonwebtoken, jose) accept algorithm override by default in some versions — verify the version and configuration.
- **npm supply chain** — Does the project use private package names (`@company/util`)? Can those names be registered on the public npm registry to trigger a dependency confusion attack? Check `.npmrc` for registry configuration and verify `publishConfig.registry` is set for any internal packages. Check for typosquatted dependencies (`lod-ash` vs `lodash`).
- **localStorage/sessionStorage as persistence injection surface** — Can an attacker pre-seed localStorage with a crafted payload (via XSS on any origin, or physical access to the browser) that the application reads and trusts at startup? If the application reads from storage and uses the value without runtime validation, a poisoned storage value is an attack vector — even if the XSS that planted it was on a different page.

## Performance Engineer

- **Bundle size analysis** — Use `webpack-bundle-analyzer`, `source-map-explorer`, or `rollup-plugin-visualizer` to identify the largest contributors to bundle size. Flag dependencies that are large relative to their use in the project. Named failure modes: importing an entire utility library for one function (`import _ from 'lodash'` instead of `import debounce from 'lodash/debounce'`); multiple versions of the same library in the bundle (use `npm ls <package>` to detect).
- **V8 profiling** — Use the Chrome DevTools Performance panel (or `--prof` with Node.js) to identify hot functions and GC pressure. Named signals: frequent garbage collection pauses visible as sawtooth memory curves; functions appearing at the top of the flame graph that should not be hot.
- **Web Vitals as performance contract** — For user-facing browser apps, the performance budget should be expressed in Core Web Vitals: Largest Contentful Paint (LCP) ≤2.5s, Interaction to Next Paint (INP) ≤200ms, Cumulative Layout Shift (CLS) ≤0.1. Lighthouse measures these under simulated conditions. A project with no Web Vitals baseline has no measurable performance contract.
- **Event delegation efficiency** — Are event listeners registered per list item rather than on the parent container? A list that adds one `click` listener per item at render time creates O(n) listeners. Using event delegation (one listener on the parent, `event.target` to identify the item) reduces this to O(1) regardless of list size.

## Solution Architect

- **VSDD purity boundary** — Are validation and transformation functions pure (no side effects, no storage reads, no DOM access, no `Date.now()`, no `Math.random()`)? Is effectful code (localStorage, fetch, DOM manipulation, event handling) isolated in a thin shell that calls pure functions for logic? In a browser app without a build framework, this means validating inputs and computing derived data in plain functions that receive their arguments, and separating the code that reads from and writes to the DOM. A function that both validates a bookmark and writes it to storage conflates two concerns that should be independently testable.
- **Module organization** — For larger apps, is code organized by concern (e.g., storage, validation, rendering, state) rather than by file type? Are module boundaries enforced by convention or tooling?
- **State flow** — Is the direction of state flow consistent and predictable? In a browser app, the pattern is: user action → update model → re-render from model. A codebase that reads DOM state as its source of truth during updates is fragile. Flag any pattern where the DOM is used as mutable storage.
- **Event handling coupling** — Are event handlers thin (call a function, handle the result) or thick (contain business logic inline)? Thick handlers are not unit-testable and entangle behavior with DOM concerns.
- **Circular dependencies** — Are there circular imports between modules (`A` imports `B`, `B` imports `A`)? A circular import in a JS module tree can cause one module to receive `undefined` for values that have not been initialized yet — a silent initialization-order bug that is difficult to diagnose. Tool: `madge --circular` for detection. Any circular dependency is a finding; the fix is typically to extract the shared dependency into a third module.

## Technical Writer

- **TypeDoc / JSDoc generation** — Is TypeDoc (TypeScript) or JSDoc (JavaScript) configured to generate API documentation from inline comments? Is `npx typedoc` or equivalent run in CI? Named checks: `typedoc.json` or `jsdoc.json` present and committed; entry points configured; output format set (`html` for browsing, `json` for tooling). A project that claims documented APIs but has no generation config produces documentation only in the developer's head.
- **TSDoc / JSDoc comment completeness** — Are all exported functions, types, and classes documented with `/** ... */` block comments? Named checks: `@param` with description for each parameter; `@returns` describing the return value and its type; `@throws` for functions that can throw; `@example` for non-obvious usage. An exported function with no doc comment is an undocumented contract. Verify with `typedoc --treatWarningsAsErrors` or eslint-plugin-jsdoc.
- **README code examples accuracy** — Are code examples in README or documentation tested against the current implementation? Named failure modes: a README example calling a function that was renamed; an import path that no longer exists; an example that would throw on execution. Consider `ts-snippet`, `remark-validate-links`, or a dedicated examples test file that imports and calls the documented patterns.
- **`@deprecated` markers** — Are deprecated exports marked with `@deprecated` and a migration note? A caller who imports a deprecated export should receive an IDE warning and documentation pointing to the replacement. Silent removal of an export with no prior deprecation annotation is a breaking change without a migration path.
- **Locale of generated docs** — If the project targets non-English speakers or is evaluated in an international context: are doc comments written in the appropriate language? Mixed-language documentation (English code, non-English comments) should be flagged if inconsistent.

## Localization

- **`Intl.*` API usage** — Are dates, numbers, and relative times formatted with `Intl.DateTimeFormat`, `Intl.NumberFormat`, `Intl.RelativeTimeFormat`, and `Intl.PluralRules` rather than hardcoded format strings or `.toLocaleString()` called without an explicit locale? The `Intl` APIs are the standard library for locale-aware formatting; calling them with an explicit `locale` parameter produces consistent output across environments and test contexts. `toLocaleString()` without a locale depends on the environment default, which varies.
- **i18next / react-i18next configuration** — If i18next is used: are all translatable strings defined in namespaced JSON resource files? Is `lng` and `fallbackLng` configured explicitly? Are pluralization keys using the i18next plural suffix convention (`_one`, `_other`, etc.) or the `Intl.PluralRules` integration? A missing plural key silently renders the singular form for all quantities in languages with complex plural rules.
- **Missing key handling** — Is a fallback strategy defined for missing translation keys? Named failure modes: i18next returns the key string verbatim when a translation is missing — users see `"app.title.main"` instead of a fallback string; missing keys are not reported to a monitoring service; the `missingKeyHandler` is not configured in production. Test by deliberately using an undefined key and verifying the fallback behavior is correct.
- **Locale injection in tests** — Are locale-sensitive functions called with an explicit locale in tests rather than relying on `process.env.LANG` or `navigator.language`? A test that passes in one CI environment and fails in another due to locale differences is a locale-injection failure. The fix is to pass `locale: 'en-US'` (or the expected locale) explicitly to `Intl.*` constructors in test assertions.

---

## Three-audience lens

The JS/TS supplement covers JavaScript- and TypeScript-specific dimensions across 10+ domain perspectives. Per the [Three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) ([Review 92 Finding 2](../suite-development/review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) cascade-applied for per-language supplements with broad domain-perspective coverage):

- **Suite developers** (contributors extending JS/TS-specific dimensions) read this supplement to understand which JS/TS idioms / failure modes the methodology teaches as canonical + how to extend per-domain sections when the ecosystem changes (new TC39 stage-4 proposals; new TypeScript versions; new framework conventions; new build-tool patterns). Each per-domain section under `## <Domain>` is the extension surface; new dimensions land in the relevant per-domain section with the canonical TC39 / TypeScript / framework authority cited + the named failure mode named.
- **Suite users** (project teams applying VSDD to a JS/TS project) read this supplement alongside the domain prompt when running each domain's IAR cycle. The per-domain sections (`## Software Engineering`, `## Quality Engineering`, `## Security`, etc.) are the JS/TS-specific add-ons to the corresponding domain's standard dimensions. When authoring a per-domain review-log entry, declare via the `**Supplements applied:**` preamble field (per [Review 91 Finding 2](../suite-development/review-log/2026-05-23-suite-review.md#r91-f2)) which section(s) of this supplement informed the round.
- **AI agents** (parallel cold-session reviewers + main-session orchestrators) read this supplement as the JS/TS-specific failure-mode catalog. Per-domain sections are H2-anchored; agent grep idiom for SE JS/TS-specific failure modes: `awk '/^## Software Engineering/,/^## /' vsdd-suite/supplements/javascript-typescript.md` returns the section's full content. Each named failure mode is a substantive defect-class to assess; absent named-failure-mode coverage in a per-domain section is itself a methodology gap.

The companion review dimensions per audience map: SO scopes which JS/TS idioms are spec-promised (suite-developer); Documentation Reviewer audits the supplement's clone-and-follow fidelity (suite-user); AI Engineer audits the supplement's per-domain coverage + cite-verify discipline at finding-authoring time (agent); TW audits the supplement's prose for cold-reader readability across all per-domain sections (cross-audience).
