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

## Data Engineering

- **Runtime schema validation** — Is data from external sources (localStorage, `fetch`, `JSON.parse`, URL params) validated with a runtime schema library (`zod`, `io-ts`, `valibot`) or an explicit validation function? TypeScript types disappear at runtime.
- **`JSON.parse` error handling** — Is `JSON.parse` wrapped in `try/catch`? Malformed stored data is a realistic failure mode.
- **Normalization functions** — When stored data is read, is there a normalization function that applies safe defaults for missing or unexpected fields, rather than trusting the raw object shape?
- **Date handling** — Are dates stored as ISO 8601 strings or Unix timestamps, not `Date` objects? `Date` objects do not survive `JSON.stringify`/`JSON.parse` as dates.
