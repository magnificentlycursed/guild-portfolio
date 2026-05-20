# JSON Language Supplement

JSON is the suite's default data-interchange and configuration format: [`issue-tracker-cli/`](../../issue-tracker-cli/) persists issues as JSON; per-project [crosslink](https://github.com/forecast-bio/crosslink) command output is JSON; tooling configuration (`package.json`, `tsconfig.json`, `.prettierrc.json`, GitHub Actions matrix payloads) is JSON or a JSON dialect. This supplement covers JSON as a language: schema design, validation, security, performance, and operational discipline. The canonical reference is [the JSON spec (RFC 8259)](https://www.rfc-editor.org/rfc/rfc8259); this supplement does not restate the grammar, it documents the dimensions to review when JSON is the chosen format. What this supplement is NOT: a tutorial on writing JSON, a guide to choosing JSON vs other formats at architecture time (that's a Solution Architect concern handled at project scaffold), or a substitute for the host-language supplement that owns the parser code ([Python](python.md), [Rust](rust.md), [JavaScript/TypeScript](javascript-typescript.md), [Bash](bash.md)).

**Authored:** Review 80 (2026-05-20) — the suite uses JSON pervasively (issue-tracker persistence, crosslink command output, all per-language tool configs) but had no language-level review dimensions for it. This supplement closes that gap. The companion artifact is the dimension set itself: when a domain review encounters JSON-defined artifacts, the reviewer applies the relevant section below in addition to the standard domain dimensions.

**Multi-domain authoring note:** the sections below were drafted with the relevant role-domain perspectives — SA (PRIMARY — schema design, key naming, nesting depth, JSON Schema, versioning), QE (validation tooling, round-trip tests, canonical forms), Security (untrusted-parse hardening, redaction), DE (migration patterns, format choice JSON/JSONL/Parquet), PE-perf (streaming parsers, large-file handling), PE-platform (pre-commit hooks, CI validation), TW (documentation-adjacent discipline since JSON has no comments), Localization (i18n string tables). The supplement is forward-looking against the 2026 JSON ecosystem ([JSON Schema](https://json-schema.org/) 2020-12 as the dominant validation dialect; [ajv](https://ajv.js.org/) for JS/TS; [jsonschema](https://python-jsonschema.readthedocs.io/) for Python; [serde_json](https://docs.rs/serde_json/) for Rust).

**Scope of this supplement:** any `.json` file in the repo, any JSON payload produced or consumed by project code, and any JSON dialect declared explicitly (JSON5, JSONC). Out of scope: YAML, TOML, XML, MessagePack — adjacent serialization formats each warrant their own supplement if the suite ever ships substantial artifacts in them.

---

## Baseline standards

- **[RFC 8259](https://www.rfc-editor.org/rfc/rfc8259) strict compliance** — Pure `.json` files MUST parse under a strict RFC 8259 parser. No trailing commas. No unquoted keys. No single-quoted strings. No comments. If the file needs any of those features, rename the file to `.jsonc` (JSON with comments) or `.json5` ([JSON5](https://json5.org/)) and declare the dialect explicitly so consumers select the right parser. A `.json` file that requires a lax parser is a finding — the extension is a contract with consumer code, not a hint.
- **UTF-8 encoding, no BOM** — JSON files are UTF-8 by default per RFC 8259 §8.1. A leading UTF-8 BOM (`EF BB BF`) is permitted by some parsers but is NOT part of the JSON spec; many strict parsers reject it. Save files as UTF-8 without BOM. Detector: `file file.json` reports `UTF-8 Unicode text` (clean) vs `UTF-8 Unicode (with BOM) text` (defect).
- **No comments in pure JSON** — RFC 8259 has no comment syntax. If documentation must live with the data, use a sibling `.md` file (preferred — see the Technical Writer section below), a `"$comment"` or `"_comment"` key (works in `JSON Schema` natively via `$comment`; ad-hoc in general JSON), or move to JSONC explicitly. The anti-pattern: `// comment` lines in `.json` files that one team's parser tolerates and another's rejects.
- **No trailing commas** — `{"a": 1, "b": 2,}` is invalid JSON. Trailing commas are convenient for diffs (last-line additions don't touch the prior line) but break strict parsers. If the diff convenience matters, switch to JSON5 / JSONC explicitly; don't smuggle the feature into a `.json` file.
- **Key naming — pick one style per file/system** — `camelCase`, `snake_case`, or `kebab-case` are all valid; mixing styles within one document or one tightly-coupled set of documents is a finding. Convention by ecosystem: JavaScript/TypeScript tooling uses camelCase (`package.json`, `tsconfig.json`); Python ecosystems often use snake_case; HTTP API conventions tend toward kebab-case for header-like fields and camelCase for body fields. The project-level choice belongs in the schema documentation; once made, enforce uniformly.
- **Indentation — 2 or 4 spaces, never tabs** — JSON does not standardize whitespace, but tabs render inconsistently across editors and tools; spaces are universal. 2 spaces is the JavaScript-ecosystem default (matched by [Prettier](https://prettier.io/)); 4 spaces is the Python-ecosystem default (matched by `json.dumps(indent=4)`). Pick one per repo; enforce via formatter.
- **LF line endings** — Unix line endings (`LF`, `\n`) only. CRLF (`\r\n`) renders identically to most viewers but produces noise in diffs and breaks line-counting tools that count `\n`. Enforce via `.gitattributes` (`*.json text eol=lf`) and the pre-commit `end-of-file-fixer` hook.
- **One trailing newline** — Like all text files, JSON files end with a single trailing newline. POSIX defines a text file as ending in `\n`; tools that read the last line break on files lacking it.

---

## Solution Architect

PRIMARY DOMAIN for JSON review — schema design is the load-bearing concern.

- **Schema design upfront** — Before writing the first JSON document, sketch the schema: what are the top-level keys, what are their types, what are the nested shapes, what's optional vs required, what's the cardinality of arrays? A JSON document authored ad-hoc and consumed by code that drifts to match it is a maintenance debt that compounds. The schema should be expressible in [JSON Schema](https://json-schema.org/) even if the project doesn't commit to runtime validation — the act of writing the schema surfaces design questions the prose can hide.
- **Key naming conventions documented** — The repo's chosen key-naming convention (camelCase / snake_case / kebab-case) is documented in the schema file or the project README. A new contributor adding a key should be able to look up the convention without reading every existing file. Mixed-style keys in one document are a finding (see Anti-patterns).
- **Nesting depth limit** — Keep nesting to 4 levels or fewer. A 7-level-deep JSON document is unreadable, painful to write a JSONPath/jq expression against, and expensive to validate. When the data wants more depth, the schema wants a redesign — usually splitting deeply-nested structure into separate documents linked by reference, or flattening to a record-per-entity model. Named worked example: a config file where `services > web > database > pool > connection > timeout > retries.max` is 7 deep; the redesign promotes `services.web.database` to a top-level `databases` map keyed by service name.
- **Reference vs. embedding** — When the same sub-document appears in multiple places, prefer reference-by-ID over duplication. Named pattern: instead of embedding the full `user` object inside every `comment`, store `user.id` in the comment and the full user in a top-level `users` map. Duplication accumulates inconsistency (one copy edited, the other isn't); references stay consistent by construction.
- **JSON Schema as schema documentation** — For any JSON document that crosses a process boundary (API request/response, persistence file, config file shared across tools), is a [JSON Schema](https://json-schema.org/) (`*.schema.json`) committed alongside? The schema serves three purposes: documentation (a reader learns the shape from the schema), validation (runtime check that produced/consumed data matches), and tooling (IDE auto-complete, generated TypeScript/Rust types). The 2020-12 draft is the current stable dialect; older `draft-07` documents are widely supported but lag on advanced features.
- **Schema versioning strategy** — When the schema evolves, how do consumers know? Three patterns: (a) a top-level `"version": "1.0.0"` field in every document, consumers branch on version; (b) a `"$schema"` URL pointing at a versioned schema URL, consumers fetch+validate; (c) implicit versioning by filename or location, no in-document marker (acceptable only for tightly-coupled internal use). Pattern (a) is the most common for persistence files; pattern (b) is the standard for API payloads with public schema URLs.
- **Backward-compatible evolution** — Adding an optional field with a default is backward-compatible (old consumers ignore unknown keys; new consumers handle absence). Renaming a field, changing a field's type, or making an optional field required is breaking. Document breaking changes in CHANGELOG and increment the schema version. Named failure mode: a field renamed from `userId` to `user_id` "to match the new convention" breaks every consumer that didn't update simultaneously.
- **Identifier discipline in keys** — Keys are part of the schema's public surface. A typo in a key (`"colour"` vs `"color"`) is a breaking change to fix later. Schema review should treat key names with the same care as function names in a language API.

---

## Quality Engineering

- **[JSON Schema](https://json-schema.org/) for runtime validation** — Is every JSON document loaded across a trust boundary validated against a schema before use? Named pattern: a config loader that calls `json.load(open(path))` and accesses `config["database"]["url"]` is one typo away from a `KeyError` at runtime; the validated form calls a schema-validator first and the access is type-safe. Tooling: [ajv](https://ajv.js.org/) for JavaScript/TypeScript (fast, draft 2020-12 support); [jsonschema](https://python-jsonschema.readthedocs.io/) for Python (most common, modest performance) or `fastjsonschema` (compiled validators, faster); [jsonschema](https://docs.rs/jsonschema/) crate for Rust.
- **Schema-document tests** — The schema itself is testable. Author positive examples (documents that MUST validate) and negative examples (documents that MUST fail validation with named error paths) and run them in CI. A schema authored without negative tests over-permits silently — it's "correct" for the example documents and accepts garbage for everything else.
- **Round-trip serialization tests** — For every parse/serialize pair the project owns, is there a test that loads a known document, re-serializes it, and asserts equality (modulo whitespace and key order)? Named failure mode: a reader that accepts `{"created": "2026-05-20"}` as a date but a writer that emits `{"created": "2026-05-20T00:00:00Z"}` — the reader-writer cycle isn't a fixed point, and round-trip-through-storage corrupts the canonical form.
- **Canonical form for diff stability** — When JSON is committed to version control, is a canonical form enforced (sorted keys, consistent indentation, no trailing whitespace)? Without canonicalization, two semantically-identical documents produce noisy diffs (key order changes, indentation changes) that mask substantive changes. Tools: [Prettier](https://prettier.io/) (no key sort by default — opt in via plugin); `jq --sort-keys`; `json.dumps(obj, sort_keys=True, indent=2)` in Python.
- **Property-based testing for schema-derived types** — If the project generates code from a JSON Schema (TypeScript types via `json-schema-to-typescript`; Rust types via `schemars`; Python types via `datamodel-code-generator`), are the generated types property-tested against the schema? A schema-to-type generator that produces types accepting documents the schema rejects (or vice versa) is a silent contract drift.
- **Coverage of optional-field combinatorics** — JSON Schemas with many optional fields have combinatorial state. A document with 10 optional fields has 1024 (2^10) presence combinations; tests exercising only "all present" and "all absent" miss 1022 of them. Property-based tests with shrinking are the standard tool — generate documents with random subsets of optionals present, assert invariants.

---

## Security

- **Untrusted JSON parsing — depth limit** — A JSON document with arbitrary nesting depth can cause stack overflow in recursive-descent parsers. Some parsers (Python's `json`, Rust's `serde_json`) have configurable depth limits; some don't. For any parse of attacker-controlled bytes, enforce a depth limit explicitly. Named attack: a JSON document of `[[[[[[[[[...]]]]]]]]]` 100,000 levels deep crashes a naïve parser. Defense: pre-scan for depth before parsing, or use a parser with a configurable maximum depth.
- **Untrusted JSON parsing — string length limit** — A JSON document containing a 1-GB string causes memory exhaustion. Limit the maximum allowed string length when parsing untrusted input. Detection: cap the input bytes BEFORE parsing (`content[:MAX_BYTES]`); reject documents that decode to a string exceeding the cap.
- **Untrusted JSON parsing — overall document size limit** — Cap the bytes accepted at the trust boundary. A 10-MB JSON document is suspicious for most application use cases; a 10-GB one is an attack. Web servers, CLI tools, and IPC endpoints all need an upper bound enforced at read time.
- **Prototype pollution (JavaScript-specific)** — In JavaScript, `Object.assign(target, JSON.parse(untrusted))` can pollute `Object.prototype` if the untrusted input contains `"__proto__"`, `"constructor"`, or `"prototype"` keys. Defense: use `Object.create(null)` for the target, or use a parser with an explicit reviver that strips dangerous keys, or use a library like [secure-json-parse](https://github.com/fastify/secure-json-parse). The exposure is real across the Node.js ecosystem and has shipped as multiple CVEs.
- **Never `eval()` JSON** — `eval("(" + jsonString + ")")` was a legacy "fast parse" pattern in old JavaScript. It is code injection — the input string is executed, not parsed. Modern code uses `JSON.parse`; any `eval`-based JSON parsing in 2026 is a critical security defect.
- **Sensitive-data redaction in logs** — When JSON containing credentials, tokens, PII, or other secrets is logged, are sensitive fields redacted? Named pattern: a request logger that calls `JSON.stringify(request.body)` writes passwords to the log. Defense: a redaction layer that walks the document and replaces known-sensitive keys (`password`, `token`, `apiKey`, `ssn`, `creditCard`) with `"[REDACTED]"` before serialization to the log sink. Maintain the redaction allowlist alongside the schema.
- **Hash-collision DoS in parsers** — Some JSON parsers use hash maps internally; a crafted document with many keys hashing to the same bucket can degrade parse time to O(n²). Modern parsers (Python 3.x with SipHash, Rust's `serde_json` with `HashMap` randomization) are resistant; legacy parsers may not be. Audit the parser version when accepting attacker-controlled JSON at high volume.

---

## Data Engineering

- **Schema migration patterns** — When a persisted JSON schema evolves, how does the project migrate existing documents? Three patterns: (a) eager migration — on first read, detect old version, write back new version; (b) lazy migration — readers handle both versions in code, writers always emit new version; (c) batch migration — a one-shot migration script rewrites all existing documents. Pattern (a) is the standard for long-lived persistence stores; pattern (b) suits short-lived caches; pattern (c) suits one-time format conversions.
- **Data lifecycle — read/write paths symmetric** — Per [G-126](../FINDINGS-INDEX.md#g-126), the validator on the create path and the validator on the load path must apply the same constraint set. JSON-specific instance: a writer that emits `{"tags": []}` for the empty case but a reader that treats `"tags"` as optional and absent — the reader-writer asymmetry surfaces when an old document (without `tags`) lands in the new system.
- **JSON vs JSONL vs Parquet — format choice by workload** — JSON for hierarchical configuration and individual records (`issues/G-123.json`); [JSON Lines](https://jsonlines.org/) (JSONL) for append-only logs and large record sets streamed line-by-line (`audit-log.jsonl`); [Parquet](https://parquet.apache.org/) for analytical workloads with columnar access patterns and compression. A 10-GB JSON array is the wrong shape — it requires loading the whole document; the JSONL equivalent streams. A 10-GB JSONL log of structured records that's queried analytically is the wrong shape — Parquet would compress 10x and scan 100x faster.
- **Compression considerations** — JSON is verbose: repeated key names, whitespace, decimal-string numbers. Gzip or zstd compression typically shrinks JSON by 5–10x. For large persisted JSON, store compressed (`.json.gz` or `.json.zst`); decompress on read. For network transport, enable HTTP `Content-Encoding: gzip` / `br`. The cost: opaque storage (can't `grep` a gzipped file without decompression); the benefit: real bandwidth and disk savings.
- **JSONL streaming semantics** — JSON Lines (one JSON document per line, `\n`-separated) is the standard streaming format. Each line MUST be a complete JSON value (object or array; usually object); embedded newlines within values MUST be escaped as `\n` to preserve line semantics. Tooling: `jq -c` produces JSONL; many log-processing pipelines (Logstash, Fluentd, ClickHouse) consume it natively.
- **Numeric precision** — JSON numbers are IEEE 754 doubles in most parsers (53-bit mantissa). Integers beyond 2^53 lose precision silently. For 64-bit IDs, big-integer amounts, or high-precision decimals, encode as strings and parse with the host language's big-integer type. Named failure mode: a database ID `9007199254740993` (2^53 + 1) serialized to JSON and round-tripped through JavaScript becomes `9007199254740992`. The standard defense: schema-level annotation that this field is a string-encoded number and host code handles the conversion.

---

## Performance Engineer

- **Parser performance for large files** — Generic recommendations: in Python, `orjson` is roughly 5–10x faster than the stdlib `json` for both encoding and decoding; in JavaScript, the native `JSON.parse` is already C-optimized and hard to beat for in-memory data; in Rust, [serde_json](https://docs.rs/serde_json/) with a target type (not `Value`) is significantly faster than generic-Value parsing. Profile first — JSON parse cost is rarely the bottleneck unless documents are large or volume is high.
- **Streaming parsers for documents that don't fit in memory** — When the JSON document exceeds available memory or when only a subset is needed, use a streaming parser: [ijson](https://github.com/ICRAR/ijson) for Python (SAX-style event stream); `serde_json::Deserializer::from_reader` with `.into_iter::<T>()` for Rust streaming arrays; `JSONStream` / `stream-json` for Node.js. The streaming form processes elements one at a time without materializing the full document.
- **JSONL for streamable workloads** — When the data is naturally a sequence of records, JSONL (one JSON document per line) is dramatically more efficient than a top-level JSON array: each line parses independently, no need to find the matching closing bracket of a giant array, and partial-read recovery is trivial (the last line may be truncated; everything before it is intact).
- **Avoid materializing the full document when streaming suffices** — Named anti-pattern: `data = json.load(open("huge.json")); for item in data["items"]: process(item)` loads the entire document into memory before iterating. The streaming form `for item in ijson.items(open("huge.json"), "items.item"): process(item)` consumes O(1) memory per item. Reach for streaming when the file size approaches the host's memory budget.
- **Key-ordering cost in serialization** — `json.dumps(obj, sort_keys=True)` is meaningfully slower than `json.dumps(obj)` for large documents — sorting adds O(k log k) per dict where k is the key count. Use `sort_keys=True` for committed/canonical artifacts (diff stability); skip it for ephemeral serialization (logs, API responses).
- **Avoid repeated parse-serialize cycles** — Each parse + serialize round-trip pays the full cost of both operations. If a service receives JSON, modifies one field, and emits JSON, the naïve `JSON.parse` + modify + `JSON.stringify` form is fine for small documents; for large documents, in-place byte-level patching (carefully) avoids the cost. The cost/risk tradeoff: in-place patching is fragile; full parse-serialize is correct. Profile before optimizing.

---

## Platform Engineering

- **Pre-commit hooks for formatting** — Is [Prettier](https://prettier.io/) (or equivalent) wired as a pre-commit hook on all `.json` files? `prettier --check '**/*.json'` in CI, `prettier --write '**/*.json'` as the developer-side autofix. Without enforcement, JSON formatting drifts (indentation, quote style, trailing newlines) and masks substantive diffs.
- **Pre-commit hooks for validation** — Is [jq](https://jqlang.github.io/jq/) (or a JSON-validating tool) run against every `.json` file in CI? The minimal check `jq . file.json > /dev/null` parses the file and fails on syntax errors. Schema-aware validation is stronger: `ajv validate -s schema.json -d file.json` (JavaScript) or equivalent Python/Rust tooling. The pre-commit framework's `check-json` hook is the stock implementation of the syntax check.
- **Schema-conformance check in CI** — For projects with JSON Schemas, is schema validation run against every committed `.json` document of the relevant type? Named pattern: a workflow step that finds all `*.issue.json` files and validates each against `issue.schema.json`. A schema authored but not enforced in CI is documentation, not validation.
- **`.editorconfig` for JSON file conventions** — Is `.editorconfig` in the repo with rules for `.json` files (`indent_style = space`, `indent_size = 2`, `end_of_line = lf`, `charset = utf-8`, `insert_final_newline = true`)? Editors honor `.editorconfig` automatically; the alternative is convention drift across contributors.
- **JSON in version-control diffs** — Large JSON documents produce noisy line-based diffs. For documents under active human review (configs, manifests), structural diff tools ([`json-diff`](https://www.npmjs.com/package/json-diff), `diff --unified` with sorted-key normalization) make changes legible. Configure repo-level diff drivers in `.gitattributes` if JSON review is frequent.
- **CI tool versions pinned** — `prettier`, `ajv-cli`, `jq` versions pinned in CI workflow files. A floating tool version means "the build broke and we don't know what changed." Same discipline as other languages' tooling.

---

## Technical Writer

- **JSON has no comments — documentation lives next to the schema** — Pure JSON cannot self-document. Companion documentation patterns: a sibling Markdown file (`schema.md` next to `schema.json`); a `description` field in the JSON Schema (rendered by schema-doc generators); a `$comment` field in JSON Schema (machine-readable, ignored at validation time). The anti-pattern: leaving the JSON document undocumented and expecting readers to infer intent from the data. Prefer the schema-with-descriptions approach — it co-locates docs with the contract.
- **Meaningful key names matter more** — Without comments, key names carry the entire weight of self-description. `created_at` is better than `c`; `databaseConnectionUrl` is better than `dburl`; `maxRetryCount` is better than `mrc`. The terseness savings (a few bytes per key) are negligible compared to the readability cost. Repeated keys compress well — JSON's verbosity is a gzip's win.
- **JSON Schema `description` and `examples` fields** — When authoring JSON Schema, populate `description` (prose explanation of the field's purpose) and `examples` (sample values that demonstrate typical use) for every non-obvious field. Schema-doc generators ([json-schema-for-humans](https://github.com/coveooss/json-schema-for-humans)) render these into browsable HTML; AI-assisted IDEs surface them in autocomplete.
- **Inline docs via `_comment` keys (when JSON Schema isn't in play)** — For plain JSON config files that need inline annotations and where a sibling Markdown file isn't appropriate, the `"_comment": "..."` convention places documentation in unused keys. The leading underscore is a convention to mark the key as non-data; most consumers ignore unknown keys gracefully. The stronger alternative: switch the file to JSONC (`.jsonc`) and use real `//` comments.
- **README integration for project-level JSON** — Project READMEs SHOULD reference the JSON files the user is expected to author or edit (config files, manifest files) with a one-line description of each and a link to its schema. A repo with five undocumented `.json` files at the root is a tour-of-discovery for new contributors.

---

## Localization

- **i18n string tables in JSON** — JSON is the de facto format for client-side i18n: one file per locale (`en.json`, `fr.json`, `ja.json`), keyed by message ID. Tooling: [i18next](https://www.i18next.com/), [FormatJS](https://formatjs.io/), [gettext-style](https://www.gnu.org/software/gettext/) JSON adapters. The schema is per-tool but the JSON shape is consistent: nested namespaces map to dot-paths (`"user.profile.edit.title"` → `t("user.profile.edit.title")`).
- **Key conventions for i18n** — Message keys are part of the public API of the i18n system: renaming `signup.button` to `register.button` is a breaking change for every string-key reference in code. Keys SHOULD be hierarchical (namespaced by feature: `auth.signup.button`, `settings.profile.title`) and stable across translations. Avoid embedding the source-language text in the key — `welcome_message_for_logged_in_users` is fine; `welcome_back_friend` ages poorly when the marketing tone shifts.
- **ICU MessageFormat for plurals and gender** — Languages have non-binary plural forms (Russian: three forms; Arabic: six; Polish: three). Gender agreement varies across languages and is unrelated to English's gender system. [ICU MessageFormat](https://unicode-org.github.io/icu/userguide/format_parse/messages/) is the cross-language standard for handling these — the message string itself encodes the plural/gender selection rules, the translator owns the locale-specific logic. Named failure mode: a JSON i18n table that stores `"items_count": "{count} items"` and the consumer does `if (count === 1) "1 item" else replaceAll("{count}", count)` — the consumer's hardcoded English plural logic breaks every non-English locale.
- **Nested namespacing vs flat keys** — Hierarchical keys (`{"auth": {"signup": {"button": "..."}}}`) are more readable in the JSON file but require nested-access at consume time (`t.auth.signup.button` or `t("auth.signup.button")`). Flat keys (`{"auth.signup.button": "..."}`) are simpler at consume time but harder to scan in the JSON file. Tooling-driven choice — the i18n library's recommended convention is the right answer; mixing the two in one file is a finding.
- **UTF-8 mandatory for i18n** — Every i18n JSON file is UTF-8. Latin-1 / Windows-1252 / Shift-JIS / EUC-KR are all defects in 2026. The file's encoding is asserted by the editor's save dialog, the repo's `.gitattributes`, and the `.editorconfig`; the parser typically assumes UTF-8 by default.

---

## Tooling

The standard toolchain for JSON across the suite. Pre-commit hooks and CI gates configure these tools; this section is the inventory.

- **[jq](https://jqlang.github.io/jq/)** — Command-line JSON processor. Read, filter, transform, validate. `jq . file.json` (pretty-print + syntax-check); `jq '.issues[] | select(.status == "open")' file.json` (filter); `jq --sort-keys . file.json > sorted.json` (canonicalize). Universally available; the first tool for any ad-hoc JSON work in a shell.
- **[Prettier](https://prettier.io/)** — Opinionated formatter. `prettier --check '**/*.json'` in CI; `prettier --write '**/*.json'` as developer-side autofix. The standard for JavaScript-ecosystem repos; the standard for cross-language repos that want one formatter for `.json` + `.md` + `.yaml`.
- **[ajv](https://ajv.js.org/)** — JSON Schema validator for JavaScript/TypeScript. Fast (compiled validators), JSON Schema 2020-12 support, widely deployed. CLI: `ajv-cli` for shell use; library: `import Ajv from 'ajv'` for in-code validation.
- **[serde_json](https://docs.rs/serde_json/)** — JSON support for Rust's `serde` framework. The standard JSON library in the Rust ecosystem; performance is excellent (especially with target types vs. generic `Value`); supports streaming via `Deserializer::from_reader`.
- **[json](https://docs.python.org/3/library/json.html) (stdlib) + [orjson](https://github.com/ijl/orjson)** — Python's stdlib `json` handles every general case; `orjson` is the high-performance alternative (5–10x faster on encode/decode, native `datetime` and `uuid` support, strict UTF-8). For new Python projects with substantial JSON throughput, evaluate `orjson` early.
- **[jsonschema](https://python-jsonschema.readthedocs.io/)** — JSON Schema validator for Python. The standard tool; `fastjsonschema` is a faster alternative when validator performance matters.
- **[ijson](https://github.com/ICRAR/ijson)** — Streaming JSON parser for Python. SAX-style event API and item-iteration API. Reach for `ijson` when documents exceed available memory.

**Pre-commit hook examples.** Add to `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.6.0
    hooks:
      - id: check-json          # syntax validation
      - id: pretty-format-json  # 2-space indent, sorted keys
        args: [--autofix, --indent=2]
      - id: end-of-file-fixer
      - id: mixed-line-ending
        args: [--fix=lf]
  - repo: https://github.com/pre-commit/mirrors-prettier
    rev: v3.1.0
    hooks:
      - id: prettier
        types_or: [json, yaml, markdown]
```

For schema-aware validation, add a project-local hook that runs `ajv-cli` (or the Python/Rust equivalent) against the project's schemas.

---

## Anti-patterns

The named anti-patterns that recur in JSON review. Each is a finding when encountered; the fix is documented inline.

- **Comments in pure JSON** — `// comment` or `/* comment */` lines in a `.json` file. Strict parsers reject; lax parsers accept; the file's contract is ambiguous. Fix: switch to `.jsonc` or `.json5` explicitly, or move documentation to a sibling `.md` file, or use a `"_comment": "..."` key (least preferred — clutters the data).
- **Trailing commas** — `{"a": 1,}` or `[1, 2, 3,]`. Convenient for diffs, rejected by strict parsers. Fix: switch to JSON5 / JSONC explicitly; don't smuggle the feature into `.json`.
- **Numbers encoded as strings without schema annotation** — `{"count": "42"}` when `count` is conceptually a number. Silently breaks numeric comparison (`"10" < "9"` is `true` lexically); silently breaks arithmetic without explicit parsing. Fix: encode as a JSON number; if precision requires a string (big integers, currency), document the convention in the schema with `"type": "string", "pattern": "^[0-9]+$"` and a description noting it's a numeric string.
- **Deeply-nested structures (> 4 levels)** — A 7-level-deep document is unreadable and indicates schema-design failure. Fix: promote inner objects to top-level maps keyed by ID, or split into multiple documents linked by reference.
- **Duplicated keys in one object** — `{"a": 1, "a": 2}` is allowed by RFC 8259 (§4) but the spec says behavior is "unpredictable." Some parsers take the last value; some take the first; some error. Fix: never emit duplicates; if input has duplicates, normalize at parse time (pick a rule and document it).
- **Byte-order mark (BOM) at start of file** — `EF BB BF` prefix breaks strict parsers and many tools. Fix: re-save the file as UTF-8 without BOM; configure the editor to never write a BOM on UTF-8.
- **Mixed-style keys in one file** — `{"firstName": "...", "last_name": "...", "email-address": "..."}`. Documents the lack of a convention. Fix: pick one style per file/system; rename until consistent; document the convention.
- **Stringified JSON inside JSON** — `{"payload": "{\"key\": \"value\"}"}` where `payload` is a JSON-encoded string of another JSON object. Doubles the parsing work, doubles the escaping complexity, defeats schema validation of the inner object. Fix: inline the inner object as a nested object (`{"payload": {"key": "value"}}`), or document a clear reason for the double-encoding (e.g., the outer system treats the inner as opaque bytes).
- **Floating-point IDs / counters** — `{"userId": 1.5e10}` for a database ID. JSON's numbers are IEEE 754; large integers lose precision. Fix: encode IDs as strings (`"userId": "15000000000"`) with schema annotation.
- **Unbounded array sizes in attacker-controlled payloads** — A schema that accepts `{"items": [...]}` without `maxItems` is a DoS vector when consumed across a trust boundary. Fix: add `maxItems` (and `maxLength` for strings, `maxProperties` for objects) constraints to schemas validating untrusted input.

---

## Maintenance

This supplement is maintained alongside the suite's review of JSON-using projects.

- **Update when a new finding surfaces a missing dimension.** A review session that discovers a recurring JSON-shaped defect not covered above is the trigger to add a new section or expand an existing one. Cite the originating Review and Finding number in the section's commit message.
- **Update when the ecosystem shifts.** New stable JSON Schema dialect (2020-12 → next draft) → update tooling references. New canonical tool (`orjson` displaces alternatives; `Prettier` adds first-class JSON Schema validation) → update the Tooling section.
- **Update when the suite adopts a new convention.** If the suite-wide key-naming convention or indentation standard changes, this supplement reflects the change. Cross-link the originating suite-development Review.
- **Co-evolution with the host-language supplements.** Cross-references in [`python.md`](python.md) (DE section, pydantic schema integration), [`rust.md`](rust.md) (serde_json), and [`javascript-typescript.md`](javascript-typescript.md) (ajv, tsconfig.json) point back to the relevant sections here. When this supplement adds a dimension that's host-language-specific, mirror the cross-reference in the host-language supplement.
- **Schema versioning of the supplement itself.** This file does not carry a version field; its anchor is the file path. Substantive structural changes (section reorder, section rename) update the [`README.md`](../README.md) supplement table-of-contents and any inbound links.
