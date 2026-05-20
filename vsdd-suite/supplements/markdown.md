# Markdown Content Supplement

These dimensions supplement the standard IAR domain reviews for markdown (`.md`) content authored inside VSDD projects and the [VSDD Suite](../README.md) itself. During each domain review of a markdown artifact — `DESIGN.md`, `TODO.md`, `PROCESS.md`, `CHANGELOG.md`, per-domain index files, per-session review-log files, suite-development entries, [primers](../primers/), [domain prompts](../domains/), other [supplements](../supplements/), [`FINDINGS-INDEX.md`](../suite-development/FINDINGS-INDEX.md), or any other `.md` artifact — apply the relevant section below in addition to the standard dimensions for that domain.

**Authored:** [Review 79](../suite-development/review-log/2026-05-20-suite-review.md#review-79--2026-05-20-1730z) Finding 4 (2026-05-20) — directed by the operator alongside the anchor-link convention work (Review 79 Finding 3) and the [Technical Writer](../domains/role/TECHNICAL-WRITER-REVIEW.md) Dim 13 registration. The Suite's primary deliverables are markdown documents; the absence of a markdown supplement until now meant cross-domain markdown discipline (TW + [Documentation Reviewer](../domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) + [Quality Engineer](../domains/role/QUALITY-ENGINEER-REVIEW.md) + [Platform Engineer](../domains/role/PLATFORM-ENGINEER-REVIEW.md) + [Security](../domains/role/SECURITY-REVIEW.md) + [Accessibility](../domains/role/ACCESSIBILITY-REVIEW.md) + [UX](../domains/role/UX-REVIEW.md) + [Solution Architect](../domains/role/SOLUTION-ARCHITECT-REVIEW.md) + [Localization](../domains/role/LOCALIZATION-REVIEW.md)) lived implicitly across role prompts.

**Scope.** This supplement covers markdown as the content medium for VSDD project artifacts and suite content. It is NOT a markdown-syntax tutorial — it assumes baseline familiarity with [CommonMark](https://commonmark.org/) and [GitHub-Flavored Markdown (GFM)](https://github.github.com/gfm/). When the supplement names a markdown feature (tables, task lists, fenced code blocks, anchors), it means the GFM rendering as canonicalized at [GitHub](https://github.com/) — the suite's primary render target.

**Multi-domain authoring note:** the sections below were drafted with the relevant role-domain perspectives in mind — TW (the primary owner of documentation quality + AI-session-independence + the Dim 13 inline-reference-navigability lens), Doc Reviewer (cold-reader pair), QE (markdown-as-artifact: link validation, anchor validation, dead-link detection), SA (information architecture and heading-hierarchy discipline), PE (pre-commit hooks + CI-renderable validation), Security (rendered-HTML attack surface, referrer leakage), Accessibility (heading hierarchy, alt text, descriptive link text), UX (reader scanning patterns, line length), Localization (per-file translation workflows). The supplement is forward-looking against the 2026 markdown tooling ecosystem ([markdownlint](https://github.com/DavidAnson/markdownlint), [lychee](https://github.com/lycheeverse/lychee), [prettier](https://prettier.io/), [pandoc](https://pandoc.org/)).

---

## Baseline standards

- **GitHub is the canonical render target.** Per operator declaration ([Review 80](../suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 2): "Most of my markdown files are intended to be read on GitHub." The applicable style authority is the [GitHub Docs Style Guide](https://github.com/github/docs/blob/main/content/contributing/style-guide-and-content-model/style-guide.md). Where GitHub's guide overlaps with general markdown discipline, follow GitHub; where this supplement specifies a stricter / suite-specific convention, follow the suite. See [§ GitHub render-target conventions](#github-render-target-conventions) below for the codified subset.
- **[CommonMark](https://commonmark.org/) + GFM is the floor.** All `.md` files in VSDD projects and the suite render through GitHub's renderer first. CommonMark defines the core syntax; GFM layers tables, task lists, autolinks, strikethrough, and heading anchors on top. Features outside CommonMark + GFM (Pandoc extensions, MyST roles, kramdown blocks) are out of scope for forward-facing suite content — they break on GitHub's renderer, which is where readers land.
- **GFM tables** — pipe-delimited tables with header-separator row. Long cells should be hard-wrapped only if the table is hand-edited (the rendered output ignores in-cell line breaks anyway); machine-generated tables can be one-line-per-row.
- **GFM task lists** — `- [ ]` / `- [x]` checkboxes. Used pervasively in `TODO.md` and `manual-tests/layer-N.md` per the [Review 74](../suite-development/review-log/2026-05-20-suite-review.md) manual-test split convention.
- **GFM autolinks** — bare URLs render as links. Prefer explicit `[descriptive text](URL)` over autolinks in prose (per the anti-pattern list below); autolinks are acceptable inside code blocks and tables where descriptive labeling would add noise.
- **GFM heading anchors** — GitHub auto-generates anchor IDs from heading text by lowercasing, replacing spaces with hyphens, and stripping most punctuation. The anchor for `## Naming and identifier discipline (Review 78 Finding 4)` is `#naming-and-identifier-discipline-review-78-finding-4`. The mapping is mechanical: lowercase → strip punctuation that isn't `-` or `_` → spaces become `-`. This is the contract the [anchor-link convention](../suite-development/suite-development.md#anchor-link-convention-for-cross-references-review-79-finding-3) relies on.
- **UTF-8, LF line endings, no BOM** — markdown files are UTF-8 with Unix line endings. Windows CRLF and BOM cause inconsistent rendering across tools and break some markdown processors. [Prettier](https://prettier.io/) enforces this by default; [markdownlint](https://github.com/DavidAnson/markdownlint) does not but is paired with prettier in practice.
- **File extension is `.md`.** Not `.markdown`, not `.txt`, not `.mdown`. This matches the Filename-extension-matches-content discipline from [`bash.md`](bash.md) § Platform Engineering — editors and pre-commit hooks scope by extension; the canonical extension is what tooling expects.

---

## Technical Writer

The primary owner of markdown quality. TW is the domain where markdown discipline lives — every other domain's markdown concern is a specialization of TW's stance.

- **Anchor-link convention compliance ([Review 79](../suite-development/review-log/2026-05-20-suite-review.md#review-79--2026-05-20-1730z) Finding 3)** — Does the markdown follow the [anchor-link convention for cross-references](../suite-development/suite-development.md#anchor-link-convention-for-cross-references-review-79-finding-3)? Every internal reference (G-ID, Review N, domain name, primer name, Phase name, file path, § Section reference) should be a markdown link to its anchor target. Every external reference (software, people, governing documents) should link to canonical homepage or GitHub repo on first mention per file. Subsequent same-file mentions of external links are plain text (first-mention-per-file rule). Subsequent same-file mentions of internal anchors continue to link (anchors are zero-cost to repeat). The convention's forward-only constraint ([G-89](../suite-development/FINDINGS-INDEX.md#g-89)) preserves pre-Review-79 prose unlinked; new prose authored post-2026-05-20 must follow the convention.
- **Inline-reference navigability ([TW Dim 13](../domains/role/TECHNICAL-WRITER-REVIEW.md))** — The companion review dimension. Project documentation is evaluated against the anchor-link convention at Phase 3 review time. Unlinked `G-N`, `Review N`, domain-name, primer-name, file-path mentions in forward-facing prose are findings. The detector pattern lives in TW Dim 13 itself; the convention lives in [`suite-development.md`](../suite-development/suite-development.md).
- **First-mention-per-file rule** — External links land on the first mention in each file (highest-leverage placement — the reader clicks once and is anchored on canonical source). Subsequent same-file mentions are plain text. This avoids visual noise without sacrificing discoverability. Internal anchor links land on every mention (low cost; same-page navigation).
- **Documentation accuracy** — Markdown that documents code, commands, or configuration must match the current implementation. Stale markdown is actively harmful — it misleads rather than informs. The TW-domain regression check applies in full: every claim a markdown file makes is verifiable against the current code/spec/process.
- **AI session independence ([TW Dim 10](../domains/role/TECHNICAL-WRITER-REVIEW.md))** — Knowledge required to maintain the project must live in markdown artifacts, not in AI conversation history. A `DESIGN.md` that says "we decided this in the Layer 3 session" with no rationale is a finding. The markdown is the durable surface; the session is ephemeral.
- **Heading-level discipline (no skips)** — Heading hierarchy mirrors the artifact's logical structure. H1 is the file title (one per file); H2 is the major section; H3 is the subsection; H4 is the sub-subsection. Skipping levels (H2 → H4 with no H3 between) is a finding — it breaks both screen-reader navigation (per [Accessibility](#accessibility) below) and table-of-contents generation. The exception: deeply-nested findings inside review-log entries where the natural depth is H2 (Review heading) → H3 (Finding heading) and a single helper H5 callout is acceptable.
- **Sentence-case headings** — The suite's voice is sentence-case headings (`## Per-domain guidance`, not `## Per-Domain Guidance`). Title-case is acceptable for proper nouns inside a sentence-case heading (`## VSDD whitepaper integration`).
- **Descriptive link text — NOT "click here"** — Link text describes the destination, not the action. `See the [anchor-link convention](../suite-development/suite-development.md#anchor-link-convention-for-cross-references-review-79-finding-3)` is correct; `For the convention, [click here](...)` is a finding. This dovetails with the Accessibility § Descriptive link text dimension below — screen readers announce link text out of context, so "click here" is unintelligible.
- **Code-fence language identifiers** — Every fenced code block declares its language: ` ```bash `, ` ```python `, ` ```rust `, ` ```toml `, ` ```json `, ` ```markdown `. Blocks without a language identifier break syntax highlighting on GitHub and confuse downstream tooling. Use ` ```text ` for non-language content (file trees, output dumps) when no language fits.
- **Table-of-contents discipline for long docs** — Markdown files exceeding 300 lines should declare a table of contents at the top (anchor links to the H2 sections). The suite's `suite-development.md` is the worked example. Files under 300 lines: ToC is optional; H2 section headings are themselves the navigation.

---

## Documentation Reviewer

(Active when [Technical Writer](../domains/role/TECHNICAL-WRITER-REVIEW.md) is active — paired per the [forthcoming Documentation Reviewer domain registration](../domains/role/DOCUMENTATION-REVIEWER-REVIEW.md). This section is forward-link only; until that domain registers and runs its first markdown cold-read, the section here documents the cold-reader markdown dimensions for whoever runs the pair-validation in advance of formal domain registration.)

- **Cross-reference resolution test** — Every `[text](path#anchor)` link in the markdown should actually resolve. Named failure modes: path drift after a rename (the kind of stale `GAP-ANALYSIS-LOG.md` / `bookmark-cli/` references caught in this PR's history); section-anchor drift after a rewrite (the heading text changed but the link target didn't); dead external links (especially [gist URLs](https://gist.github.com/) that move when the author changes them); line-number citations that drifted after edits. The exact test: every `[text](path)` gets opened and confirmed; failures are findings. Tool support: [lychee](https://github.com/lycheeverse/lychee) handles external link verification; markdown-link-check handles both internal and external. See § [Tooling](#tooling) below.
- **Cold-reader anchor-followability** — Given a link like `[Review 79](review-log/2026-05-20-suite-review.md#review-79--2026-05-20-1730z)`, does following the link land on the cited content? A common failure: the heading text changed (e.g., `## Review 79 — 2026-05-20 17:30Z` became `## Review 79 — Review 79`), the anchor regenerated, the inbound link stayed pointed at the old anchor. The reader clicks and lands on the file but not the section.
- **Tutorial-followability** — Markdown that documents a procedure (install steps, manual-test plans, contribution workflow) should be executable verbatim by a cold reader. Named failure modes: install instructions that assume a prior tool installed; tutorials that copy-paste code referencing undocumented module state; manual-test steps that depend on environment variables set by an earlier step but the dependency isn't named.
- **Forward-reference safety** — Can a markdown section be entered cold without reading file order? A `## Phase 5 hardening` section that references `## Phase 2a Red Gate` before Phase 2a is documented in the same file (or linked to a primer that documents it) is a forward-reference defect. The test: read each H2 section as if opened at that section without scrolling up.

---

## Quality Engineering

Markdown-as-artifact discipline — the markdown is itself a build product subject to the same correctness checks as code.

- **Markdown linting with [markdownlint](https://github.com/DavidAnson/markdownlint)** — Is `markdownlint` (or `markdownlint-cli2`) run as a pre-commit hook AND in CI? Markdownlint catches heading-level skips, trailing-whitespace, tab-vs-space inconsistency, mismatched list markers, bare URLs in prose, and a long list of common defects. The recommended ruleset is the default with project-specific carve-outs (e.g., `MD013/line-length` disabled — long lines are fine in machine-edited markdown). Suppressions (`<!-- markdownlint-disable MDxxx -->`) require a comment explaining why the rule doesn't apply.
- **Link checking with [lychee](https://github.com/lycheeverse/lychee) or [markdown-link-check](https://github.com/tcort/markdown-link-check)** — Is link validation run as a CI gate? Both tools traverse markdown files and verify every link resolves. Lychee is the modern Rust-based tool (fast, handles GitHub rate limits gracefully, supports authentication for private resources); markdown-link-check is the Node-based legacy option (slower, simpler config). Either is acceptable; the suite's recommendation is lychee for new projects. The CI gate should fail on any dead link; transient failures (rate-limit, server flakiness) should be retried, not silently ignored.
- **Anchor validation** — Internal `[text](file.md#anchor)` links should resolve to existing anchors. Lychee handles fragment validation when invoked with `--include-fragments`. Anchor drift is the highest-frequency cross-reference failure mode after a heading rename — heading text changes, anchor regenerates, inbound links break silently.
- **Round-trip rendering verification** — For markdown that ships as part of a deliverable (PyPI README, npm README, GitHub Pages content, published docs), build the deliverable locally and inspect the rendered output. Named failure modes: relative-path links that work on GitHub but break on PyPI (different base URL); HTML embedded in markdown that GitHub strips but a less-strict renderer executes; image references that load on GitHub but break on the docs site. The test: build the deliverable and open the rendered HTML in a browser; every link, image, and code block should render as intended.
- **Markdown-as-test-input** — If markdown files are parsed by project code (e.g., a review-log parser, a manual-test extractor), are there fixture tests covering the expected markdown structure? Named failure mode: a hook that grep-parses heading anchors silently misses anchors after a heading-format change because the grep pattern was tied to the old format. Treat markdown-parsing code under the same falsifiability discipline as any other parser — every parse path needs tests against representative input.
- **Test the rendered surface, not the source** — When verifying a markdown claim (e.g., "the README has an install section"), check the rendered output, not the markdown source. The source contains the heading `## Install`; the rendered output is `<h2 id="install">Install</h2>`. If the reader's experience is the rendered output, that's what the test asserts on.

---

## Solution Architect

Information architecture and cross-document consistency.

- **Heading hierarchy mirrors logical structure** — The H1 is the file's identity; H2 sections are the major divisions of that identity; H3 sections are sub-divisions within. A file whose H2 sections don't make sense without H3 context is mis-leveled — promote the H3s to H2s, or fold the H2 into a parent. Named failure mode: a `## Background` H2 section that contains four H3 subsections and no other content — the H2 is structural noise; the H3s should be the H2s. The opposite failure: a 50-section file all at H2 with no H3 grouping — the reader has no chunking signal.
- **Table-of-contents for long docs** — Files over 300 lines need a ToC. The ToC is a bulleted list at the top of the file, after the intro paragraph, linking to each H2 (and sometimes H3) anchor. ToC currency is a maintenance burden — when adding or renaming a section, update the ToC. The detector: every H2 in the file is in the ToC; every ToC entry corresponds to an existing H2.
- **Cross-document consistency** — Conventions used in one markdown file should match conventions used in sibling files. Named failure modes: half the per-domain index files use `## Reviews` and half use `## Review history`; some review-log entries use `**Finding N — Title**` and some use `### Finding N: Title`; some files capitalize "Finding" inside prose and some don't. The suite's [`suite-development.md`](../suite-development/suite-development.md) is the canonical reference for suite-wide conventions; project files should match.
- **One H1 per file** — Every `.md` file starts with one H1 (the file title). Multiple H1s confuse table-of-contents generation, screen readers, and SEO tooling. If the content naturally needs multiple H1s, the file should probably be split.
- **File-naming consistency** — Markdown file names use lowercase + hyphens (`anchor-link-convention.md`) except for canonical all-caps artifacts that the suite has standardized (`DESIGN.md`, `TODO.md`, `PROCESS.md`, `CHANGELOG.md`, `README.md`, `FINDINGS-INDEX.md`, per-domain `<DOMAIN>-REVIEW.md`). The mix is intentional — caps signals "this is a load-bearing artifact in the methodology"; lowercase signals "this is content within a category."
- **Per-domain markdown structure** — Files belonging to a categorical group should share structure. Every domain prompt under [`domains/role/`](../domains/role/) follows the same H2 sections (Current Review Prompt → Standard Evaluation Dimensions). Every supplement under [`supplements/`](../supplements/) follows the per-domain section template. Drift inside a categorical group is a finding.

---

## Platform Engineering

Pre-commit hooks, CI integration, and tool-version pinning for markdown.

- **Pre-commit framework integration** — Is the [pre-commit](https://pre-commit.com/) framework configured with markdown hooks? The suite's [`.pre-commit-config.yaml`](../../.pre-commit-config.yaml) already wires five suite-internal hooks against markdown files; the recommendation here is to add markdownlint + lychee on top. Example shape for a new project's pre-commit config:

  ```yaml
  - repo: https://github.com/igorshubovych/markdownlint-cli
    rev: v0.41.0
    hooks:
      - id: markdownlint
        args: ['--config', '.markdownlint.json']

  - repo: https://github.com/lycheeverse/lychee
    rev: v0.15.1
    hooks:
      - id: lychee
        args: ['--no-progress', '--include-fragments', '--exclude-mail']
  ```

- **CI-renderable validation** — Is the markdown rendered through the actual production renderer (GitHub, PyPI, docs.rs) as a CI step? A file that lints clean but renders broken on PyPI is a defect the lint stage can't catch. For PyPI projects: build the wheel, extract the long-description, run it through PyPI's renderer ([readme-renderer](https://github.com/pypa/readme_renderer) is the PyPA-endorsed tool). For docs sites: build the site and inspect.
- **Tool versions pinned** — Are `markdownlint-cli`, `lychee`, `prettier` versions pinned in the pre-commit config and CI? A floating version means a tool update can break the build silently. The pre-commit framework pins via `rev:`; CI workflow files pin via explicit version strings.
- **The suite's existing five hooks** — The suite ships five hooks at [`vsdd-suite/hooks/`](../hooks/) that operate on markdown ([`check-review-log-anonymization.sh`](../hooks/check-review-log-anonymization.sh), [`check-crosslink-references.py`](../hooks/check-crosslink-references.py), [`check-suite-review-preamble.py`](../hooks/check-suite-review-preamble.py), [`check-project-review-discipline.py`](../hooks/check-project-review-discipline.py), [`check-changelog-currency.py`](../hooks/check-changelog-currency.py)). New markdown hooks should follow the same patterns: pre-commit framework integration, per-file invocation, `# hook-bypass:` HTML-comment escape valve where applicable, registration in [`FINDINGS-INDEX.md`](../suite-development/FINDINGS-INDEX.md) when the hook closes a gap.
- **Prettier as the formatter** — Is [prettier](https://prettier.io/) run with `--check` on markdown? Prettier enforces consistent wrapping, list-marker style, and whitespace. Without an enforced formatter, markdown accumulates whitespace and quote drift that masks substantive diffs. The recommendation: prettier on save (in editor config) + prettier `--check` in CI.
- **GitHub-Action workflow shape** — A typical markdown CI job: checkout → install Node + Python (for tooling) → run markdownlint → run lychee → run prettier `--check`. Each step is a separate job step so a failure points at the specific tool. Cache the tool installs across runs (the pre-commit framework's GitHub Action handles this).
- **Filename-extension consistency** — Markdown files are `.md`. The suite's own [Review 76](../suite-development/SUITE-DEVELOPMENT-REVIEW.md) surfaced an analogous discipline for Python hooks ending in `.sh` "for parity" — the file extension should match the content. A `.txt` file containing markdown is a maintenance defect: editors apply plain-text rules; tools scoped by `.md` miss it; readers expect plain-text conventions.

---

## Security

Markdown is content, but markdown renders through HTML — the rendered surface has a security posture even though the source is plain text.

- **Raw HTML inside markdown** — CommonMark and GFM both allow raw HTML inside markdown. [GitHub](https://github.com/) sanitizes HTML aggressively on render (strips `<script>`, sanitizes attributes, blocks `javascript:` URLs); less-strict renderers (custom static-site generators, internal wikis, third-party preview tools) may not. Named failure mode: a markdown file with `<script>` tags that GitHub strips silently but a self-hosted MkDocs site executes. Defense: prefer pure markdown; when HTML is necessary (alignment, embed, complex tables), keep it minimal and audit it against the target renderer's sanitization rules.
- **Image `src` referrer leakage** — Embedded images (`![alt](https://third-party.example/image.png)`) cause the reader's browser to issue a referrer header to the third-party host when the markdown renders. For sensitive markdown (internal docs, paths that reveal repo structure), this leaks information. Defense: host images inside the repo where possible; for external images, prefer hosts that respect referrer-policy headers; consider `<img src="..." referrerpolicy="no-referrer">` when raw HTML is acceptable.
- **Third-party-asset data flow** — Any external resource referenced from markdown (image, link, embedded gist, badge SVG) is a third-party trust boundary. Named failure modes: status badges from a vendor that goes offline (the README breaks); gist embeds whose author retroactively changes the content (the documented behavior drifts). For long-lived markdown (governing whitepapers, foundational docs), prefer in-repo copies of external assets over hot-linked references.
- **Link-target validation** — Markdown links to external URLs should be validated against typosquatting (a `https://requets.io` link instead of `https://requests.io`). The link-check tool catches dead links but not typosquatted live links; treat external-link review as a Red Team concern adjacent to PyPI typosquatting per [`python.md`](python.md) § Red Team.
- **`<!--` HTML comments as load-bearing content** — Markdown HTML comments (`<!-- text -->`) are hidden from rendered output. If a comment carries load-bearing metadata (a hook-bypass directive, an authorship attribution, a license note), the rendered consumer can't see it. Acceptable uses: hook-bypass markers consumed by tooling ([`check-suite-review-preamble.py`](../hooks/check-suite-review-preamble.py) reads `<!-- hook-bypass: ... -->` markers per design); editor-only comments. Unacceptable uses: a `<!-- AUTHOR: someone -->` line that's the only attribution; a `<!-- LICENSE: ... -->` line that's the only license notice.
- **Markdown-injection in user-content contexts** — When markdown is rendered from user-supplied input (issue templates, comment forms, PR bodies), the user can inject markdown that affects layout, embeds remote content, or escapes the intended container. GitHub's renderer is hardened against the worst cases; custom renderers may not be. If a project ships markdown rendering, treat the renderer as a security-sensitive component and validate against [CommonMark](https://commonmark.org/) test vectors.

---

## Accessibility

Markdown's accessibility surface is heading hierarchy, image alt text, table structure, and descriptive link text. Markdown renders to HTML, and the HTML's accessibility is what end users experience.

- **Heading hierarchy without skipping levels** — Screen readers navigate by heading level. Skipping a level (H2 → H4) creates a hierarchy gap that breaks "next heading at level N" navigation. Per the [TW](#technical-writer) heading-level discipline above, H1 → H2 → H3 → H4 with no skips. [markdownlint](https://github.com/DavidAnson/markdownlint) rule `MD001/heading-increment` catches this.
- **Alt text on images** — Every image (`![alt text](path/to/image.png)`) requires meaningful alt text. Empty alt (`![]`) is acceptable ONLY for purely decorative images (status badges where the badge text is the alt-equivalent and the link target makes the meaning available). Named failure modes: `![image](...)` with the literal word "image" as alt (carries no semantic content); `![screenshot](...)` similarly; alt text that duplicates the surrounding prose without adding information. Good alt text describes the image's content or function in the context of the page.
- **Table captions and headers** — GFM tables have a header row (first row + separator row). Long tables benefit from a caption — markdown doesn't have a native caption syntax, so the convention is a prose sentence immediately before the table acting as the caption: `**Convention table — internal navigability:**` immediately before the table is the suite's standard pattern (see [`suite-development.md`](../suite-development/suite-development.md) for the canonical example).
- **Descriptive link text — NOT "click here" / "see here" / bare URLs** — Link text is announced by screen readers out of context. `[click here](URL)` is announced as just "click here" — the destination is invisible. The standard: link text describes the destination (`[the anchor-link convention](URL)`, not `[click here](URL)`). This dovetails with the [TW](#technical-writer) anchor-link-convention first-mention-per-file rule — link text on a domain-name first-mention IS the descriptive text. Markdownlint rule `MD059/descriptive-link-text` catches the common offenders.
- **List-marker consistency** — Use `-` for unordered lists (or consistently `*` or `+`); use `1.` / `2.` / `3.` for ordered lists. Mixed markers in the same list confuse some screen readers; markdownlint catches via `MD004/ul-style` and `MD029/ol-prefix`.
- **No information conveyed by formatting alone** — Bold + italic + color (when rendered) should not be the only conveyer of meaning. Named failure mode: a checklist where "complete" is signaled only by italics — a screen reader announcing the text gives no signal. Pair formatting with explicit text or use task-list checkboxes (`- [x]`) where the state is in the markup itself.

---

## UX

Reader scanning patterns and visual density.

- **F-shape reading pattern accommodation** — Readers scan markdown documents in an F-pattern: across the top heading, down the left margin, across the next prominent heading. The implications: load-bearing information goes in heading text (not buried in mid-paragraph prose); the first sentence of each paragraph carries the paragraph's claim (not the third sentence); section openings carry the section's thesis (not a meandering intro). Long blocks of unstructured prose violate the scanning pattern; bullet lists, tables, and frequent headings support it.
- **Line length** — In rendered HTML on GitHub, line length is determined by viewport width, not source line length. Source line length is a developer-experience concern (long lines in source = hard to diff). The suite's convention: source lines are wrapped at natural sentence boundaries OR not wrapped at all (one paragraph per source line). Mixed wrapping inside one file is a finding. [Prettier](https://prettier.io/) enforces a consistent wrap policy; the suite's existing markdown files use one-paragraph-per-line wrapping.
- **Whitespace between sections** — Blank lines between paragraphs, between list items where readability benefits (long-prose items), and around code fences and tables. The H2 / H3 transition gets a blank line before AND after the heading. Lack of whitespace produces wall-of-text rendering; excessive whitespace produces sparse rendering. Markdownlint rules `MD012/no-multiple-blanks` and `MD022/blanks-around-headings` enforce both directions.
- **Table density** — Tables with more than 5 columns hit a readability cliff on standard viewport widths (GitHub's rendered width is ~960px; ~190px per column at 5 columns is workable; below that, columns truncate to ellipses or wrap). Named alternatives for wide tables: split into multiple tables grouped by concern; convert to a definition-list shape (markdown supports `Term`<br>`: definition`); use a multi-row table with each "record" spanning a few rows.
- **Code-block length** — Long code blocks (100+ lines) inside markdown break the F-shape pattern (the reader scrolls past without scanning). Prefer extracting long code to a separate file referenced via link, OR collapse via GFM's `<details>` element:

  ````markdown
  <details>
  <summary>Click to expand the full example</summary>

  ```python
  # long code here
  ```

  </details>
  ````

- **Frontloaded conclusions** — A long markdown file (DESIGN.md, PROCESS.md) should open with a 1-2 paragraph summary of the file's claim. The reader who lands on the file from a deep link should know within 30 seconds whether the file is what they need. Burying the thesis below an extensive background section is a UX defect.

---

## Localization

Markdown localization is a per-file translation workflow, distinct from code-string-table localization.

- **Per-file translation, not string extraction** — Unlike code (where user-visible strings are wrapped in `_("...")` and extracted to a `.pot` template), markdown files are translated as whole files. The English source is `docs/en/getting-started.md`; the French translation is `docs/fr/getting-started.md`. Tooling: [Crowdin](https://crowdin.com/) and [Transifex](https://www.transifex.com/) both have markdown file-format support that preserves markdown structure while exposing prose for translation. The translator works on prose; the structure (headings, links, code blocks) is preserved.
- **Translation drift management** — When the English source updates, the translation needs a corresponding update. Without tooling, translations drift silently — the reader of `docs/fr/foo.md` sees content that's accurate as of last year. Crowdin / Transifex flag translations as "out of date" when the source updates; without the tooling, a manual cross-reference table tracking source-version-vs-translation-version is the fallback.
- **Date/time format conventions in dated review-log files** — The suite's review-log files use `YYYY-MM-DD HH:MMZ` UTC (Zulu) format per the [G-133](../suite-development/FINDINGS-INDEX.md#g-133) Per-review entry preamble standard. This is locale-independent by design — Zulu time and ISO 8601 date format produce the same string regardless of reader locale. User-facing markdown (project READMEs, end-user docs) should use locale-aware date formatting where the content is locale-sensitive; suite-internal audit-trail markdown stays in Zulu / ISO 8601.
- **UTF-8 with explicit declaration where relevant** — Markdown files are UTF-8. Tools like [pandoc](https://pandoc.org/) honor explicit encoding declarations; GitHub renders UTF-8 by default. Right-to-left languages (Arabic, Hebrew) require special handling at the HTML rendering layer (`<html dir="rtl">`) — markdown alone can't express direction. Projects with RTL content should pair markdown with a renderer that handles bidirectional text.
- **Pluralization and locale-sensitive prose** — Markdown prose written in English uses English plural rules. Translations into languages with non-binary plurals (Russian, Arabic, Polish) need the translator to restructure prose, not just replace words. The translation workflow's QA pass catches this; mechanical machine translation does not.
- **Link target localization** — Links inside markdown can point at locale-specific targets (`https://docs.example.com/fr/...` for the French version). When the source link points at a generic English doc, the translation should update the link to the localized version where one exists. The translation tooling does not do this automatically — it's a translator-judgment task.

---

## GitHub render-target conventions

Codified from the [GitHub Docs Style Guide](https://github.com/github/docs/blob/main/content/contributing/style-guide-and-content-model/style-guide.md) ([Review 80](../suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 2). These rules apply to forward-facing suite + reference-example content; pre-Review-80 prose is preserved per [G-89](../suite-development/FINDINGS-INDEX.md#g-89). Source-of-truth precedence: GitHub style guide → suite-development.md § Naming and identifier discipline → suite-development.md § Anchor-link convention → this supplement.

**Headings.**
- **Sentence case** — capitalize only the first word and proper nouns. The suite already follows this (per [Review 78](../suite-development/review-log/2026-05-20-suite-review.md#review-78--2026-05-20-1630z) Finding 4 — descriptive names are the primary identifier). Exception: methodology Title Case names from the [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) (Mutation Testing, Fuzz Testing, Purity Boundary Audit, Proof Execution, Red Gate, Minimal Implementation, Adversarial Refinement, Feedback Integration Loop, Convergence) — these are proper nouns by whitepaper convention.
- **Never skip heading levels** — H2 → H4 without H3 is forbidden. Captured in [§ Anti-patterns](#anti-patterns) below; enforced by [markdownlint](https://github.com/DavidAnson/markdownlint) rule `MD001`.
- **Unique H2 per page** — every H2 heading text must be unique within the file. Affects suite-development.md and review-log files where similar concerns recur — disambiguate via parenthetical (e.g., `## Resolution (Phase 1 — high-leverage entry points)`).
- **Content between heading and subheading** — every H2 has at least one paragraph before its first H3. Stack-of-empty-headings is a defect.

**Link text.**
- **Descriptive link text, not "click here" / "see this" / "more"** — already codified in [Accessibility](#accessibility) below and [TW Dim 13](../domains/role/TECHNICAL-WRITER-REVIEW.md). The link text should answer "where does this take me" without surrounding context.
- **Same link, single instance per article** — GitHub's style guide discourages repeating the same URL in one article. The suite's first-mention-per-file rule (per [suite-development.md § Anchor-link convention](../suite-development/suite-development.md#anchor-link-convention-for-cross-references-review-79-finding-3)) for external links matches this principle. Internal anchor-links remain low-cost-per-mention.

**Inclusive language.**
- **Allowlist / Denylist** (not whitelist / blacklist). **Default branch** or **main branch** (not master). **Decommission / Retire** (not kill / sunset). The suite has no historical whitelist/blacklist usage; the discipline is forward-looking.
- **Avoid regional idioms and slang.** Suite-authored prose targets a global apprentice audience; idioms that read naturally to a US-native reader may opacify for a non-native speaker.
- **Be accurate when referring to people.** Operator handles use the canonical form (`dollspace.gay` per [github.com/dollspace-gay](https://github.com/dollspace-gay), not invented variants); anonymized review-log artifacts use `<user>` / `<email>` / `<path>` placeholders per the [anonymization hook](../hooks/check-review-log-anonymization.sh).

**Voice and tense.**
- **Active voice; second person; present tense for procedures.** The suite uses second person ("you write the Red Gate test") and active voice consistently. Procedural primers (`primers/2a-red-gate.md`, `primers/2b-implementation.md`, etc.) are written in present-tense imperative ("Run the test suite. Confirm every new test fails."), matching GitHub's procedural-doc voice.
- **Past tense for retrospective content** — `CHANGELOG.md` entries, `PROCESS.md` retrospectives, review-log Findings describing already-completed work use past tense ("Authored", "Resolved", "Swept across 16 files"). Forward-looking sections in the same artifact (`PR after Phase 2 anchor-link sweep`) use the appropriate forward tense.

**Acronyms and abbreviations.**
- **Spell out on first use; abbreviate after.** VSDD ([Verified Spec-Driven Development](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00)), VDD ([Verified Development Discipline](https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25)), IAR (Iterative Adversarial Refinement), MVR (maximum viable refinement), QE (Quality Engineer), SE (Software Engineer), TW (Technical Writer), SA (Solution Architect), SO (Solution Owner), PE (Platform Engineer), DE (Data Engineer), TDD (test-driven development), GFM (GitHub-Flavored Markdown). The suite's [TW Dim 12](../domains/role/TECHNICAL-WRITER-REVIEW.md) catches missing first-use expansions at review time.
- **Acceptable suite abbreviations that include the concept word** — `Dim N`, `Layer N`, `Round N`, `Finding N`, `Phase N`. These match GitHub's "abbreviation includes the noun" pattern.

**Alerts and callouts.**
- **GFM alerts** — `> [!NOTE]`, `> [!TIP]`, `> [!IMPORTANT]`, `> [!WARNING]`, `> [!CAUTION]` per GitHub's [alert syntax](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#alerts). Use for content that genuinely warrants reader attention; over-use degrades the signal. Maximum one alert per H2 section. Suite primers and domain prompts have historically used `**Bold prefix:**` inline emphasis instead of alerts — both are valid; alerts are reserved for risk + side-effect warnings (e.g., destructive operations, version-pin mismatches).

**Code blocks.**
- **Always specify language after the fence** — ` ```rust ` / ` ```python ` / ` ```bash ` / ` ```json ` / ` ```yaml ` / ` ```toml ` / ` ```diff ` — matches GitHub's syntax-highlighter language list. Unspecified language (` ``` `) drops syntax highlighting and is an anti-pattern.
- **No command prompts (`$`, `>`) before the command itself** — `cargo test` not `$ cargo test`. The user pastes the line as-is.
- **Comment out command output, don't fence it separately** — when showing expected output inline, prefix with `# ` so the block remains executable-or-readable as a single unit.
- **Placeholders in `UPPERCASE-KEBAB-CASE` not `<angle-brackets>`** — `BRANCH-NAME` not `<branch-name>`. Avoids confusion with HTML/XML and renders cleanly in code blocks.

**Tables.**
- **Pipes at start AND end of every row** — `| col1 | col2 |` not `col1 | col2`. Improves diff readability and matches markdownlint's `MD055` rule.
- **Every cell has a value** — empty cells use `None` or `Not applicable`. The suite's `*(not applicable)*` italicized placeholder is acceptable; bare empty cells are not.
- **Left-align text columns by default** — GFM's `:---` (left), `:---:` (center), `---:` (right) syntax. The suite has no formal alignment policy; default left works for prose-content cells.
- **Tables are for tabular data, not for layout** — captured in [§ Anti-patterns](#anti-patterns).

**Alt text.**
- **Start with the graphic type** — "Screenshot of...", "Diagram showing...", "Flowchart of...". Never start with "Image of..." or "Picture of...".
- **Describe meaning, not appearance** — what the image conveys, not what it looks like.
- **40–150 characters typical range** — under 40 usually misses context; over 150 belongs in the body prose.
- **End with a period.** Screen readers pause appropriately.

**File names and paths.**
- **Kebab-case for content files** — `red-gate-test.rs` not `redGateTest.rs` or `red_gate_test.rs`. (The suite has historical exceptions for SCREAMING-CASE domain prompts (`TECHNICAL-WRITER-REVIEW.md`) and snake_case Python modules — those are appropriate to their context; new general-purpose content files default to kebab-case.)
- **Descriptive image file names** — `phase-5-hardening-surface-mapping.png` not `image1.png`.

---

## Tooling

Concrete tool list for markdown quality. Versions pin via the pre-commit framework's `rev:` field or via CI-workflow version strings; the suite's recommendation is pin-to-specific.

- **[markdownlint](https://github.com/DavidAnson/markdownlint) (and [markdownlint-cli2](https://github.com/DavidAnson/markdownlint-cli2))** — The canonical markdown linter. Catches heading-level skips, missing alt text, inconsistent list markers, bare URLs in prose, and many other defects. Configuration via `.markdownlint.json` at repo root; ruleset starts with the defaults and adds project-specific carve-outs. Suppress per-file via `<!-- markdownlint-disable MDxxx -->` with a comment explaining why.
- **[lychee](https://github.com/lycheeverse/lychee)** — The recommended link checker. Rust-based, fast, handles GitHub rate limits via `--github-token` flag, supports both internal and external links, supports fragment validation (`--include-fragments`). Configuration via `lychee.toml`. Run as pre-commit hook AND CI gate.
- **[markdown-link-check](https://github.com/tcort/markdown-link-check)** — The legacy Node-based link checker. Slower than lychee; simpler config; widely-used in older projects. Acceptable but not recommended for new projects.
- **[prettier](https://prettier.io/)** — The recommended markdown formatter. Enforces consistent wrapping, list markers, table alignment, and whitespace. Configuration via `.prettierrc` at repo root. Run as pre-commit hook with `--check` flag (fails the commit if reformatting is needed) AND with editor-on-save for the authoring experience.
- **[pandoc](https://pandoc.org/)** — The universal document converter. Use when markdown needs to convert to PDF, DOCX, HTML for non-GitHub renderers, or LaTeX. Not part of the standard CI pipeline (slow, heavyweight) but invaluable for one-off conversion tasks.
- **[readme-renderer](https://github.com/pypa/readme_renderer)** — The PyPA-endorsed tool for verifying PyPI README rendering. Run locally before publishing to PyPI: `python -m readme_renderer README.md > rendered.html` and inspect.
- **[crosslink](https://github.com/forecast-bio/crosslink)** — The suite's first-party dependency for issue tracking; relevant here because [`check-crosslink-references.py`](../hooks/check-crosslink-references.py) validates markdown citations of crosslink commands against the installed CLI. See [G-139](../suite-development/FINDINGS-INDEX.md#g-139) for the discipline.
- **Editor integration** — For real-time markdown feedback, editor plugins for [VS Code](https://code.visualstudio.com/) (markdownlint extension), Vim (vim-markdownfmt), Emacs (markdown-mode), and Sublime (SublimeLinter-markdownlint). The editor catches violations at authoring time; pre-commit catches at commit time; CI catches at merge time. Three layers of defense, all running the same ruleset.

---

## Anti-patterns

Concrete patterns to reject in markdown content.

- **Unlinked cross-references** — Inline mentions of `G-N`, `Review N`, domain names, primer names, Phase names, file paths, or external software/people/documents in forward-facing prose without a markdown link to the cited content. Per [Review 79](../suite-development/review-log/2026-05-20-suite-review.md#review-79--2026-05-20-1730z) Finding 3 — apply [the anchor-link convention](../suite-development/suite-development.md#anchor-link-convention-for-cross-references-review-79-finding-3). Pre-Review-79 prose is preserved per [G-89](../suite-development/FINDINGS-INDEX.md#g-89); new prose follows the convention.
- **Letter-coded methodology concepts** — Naming a methodology concept with a single-letter label or short letter code as the primary identifier when a descriptive name would carry the meaning. Per [Review 78](../suite-development/review-log/2026-05-20-suite-review.md#review-78--2026-05-20-1630z) Finding 4 — see [`suite-development.md`](../suite-development/suite-development.md) § Naming and identifier discipline. The canonical worked example: "Surface A" / "Surface B" / "Mode K" naming retired in favor of "property-based testing" / "mutation testing" / descriptive names.
- **Heading-level skips** — H2 → H4 with no H3 between. Breaks screen-reader navigation and table-of-contents generation. Markdownlint rule `MD001/heading-increment` catches.
- **Tables with more than 5 columns** — Readability cliff on GitHub's standard viewport. Split into multiple narrower tables, OR convert to a definition-list shape, OR use multi-row records.
- **Code fences without a language identifier** — Triple-backtick blocks with no language string. Breaks syntax highlighting and downstream tooling. Use ` ```text ` for non-code content (file trees, output dumps) when no language fits.
- **Raw URLs in prose** — `See https://example.com/docs for details` instead of `See [the docs](https://example.com/docs) for details`. Bare URLs are subject to GFM autolinking but the link text is the URL itself — useless to a screen reader and ugly in rendered output. Acceptable inside code blocks (the URL is part of the displayed example) and inside tables where descriptive labeling would add noise.
- **HTML comment-only content (`<!-- ... -->`) as load-bearing metadata** — A `<!-- AUTHOR: ... -->` line that's the only authorship attribution; a `<!-- LICENSE: ... -->` that's the only license note. The rendered reader can't see it. Acceptable when the comment is tooling-consumed AND there's a rendered equivalent (e.g., `<!-- hook-bypass: ... -->` markers consumed by [hooks](../hooks/) where the bypass is also documented in the file's prose).
- **"Click here" / "see here" / bare-URL link text** — Per the [Accessibility](#accessibility) descriptive-link-text dimension; link text should describe the destination, not the action.
- **Multiple H1s in one file** — Confuses table-of-contents generation, screen readers, and SEO. One H1 per file; everything else is H2+. If multiple H1s are natural, the file should split.
- **Stale relative paths after rename** — Markdown links with paths that no longer resolve after a rename or move. The most common documentation-rot vector. Mitigation: link-checking in CI catches at merge time; the discipline matters at authoring time.
- **Anchor drift after heading rename** — A heading rename regenerates the anchor; inbound links to the old anchor break silently. The link-check tool with fragment validation (`lychee --include-fragments`) catches this; without it, anchor drift is invisible until a reader follows the link.
- **Front-matter metadata not consumed by tooling** — YAML front matter (`---\nfoo: bar\n---` at file top) is consumed by static-site generators (Jekyll, Hugo, MkDocs). For VSDD project markdown rendered by GitHub directly, front matter is invisible — and any metadata in it is load-bearing-hidden-from-readers. Don't introduce front matter unless a downstream tool consumes it.

---

## Maintenance

When to update this supplement:

- **New markdown tooling adoption.** If a project adopts a markdown tool not listed in § [Tooling](#tooling) (e.g., [Vale](https://vale.sh/) for prose linting, [doctoc](https://github.com/thlorenz/doctoc) for table-of-contents generation), add the tool to the list and update the relevant per-domain section.
- **New markdown convention shift in suite-development.** When [`suite-development.md`](../suite-development/suite-development.md) registers a new markdown-related convention (a new anchor-link rule, a new file-naming rule, a new heading-format rule), backport the convention into the per-domain section here so domain reviewers see the convention at review time.
- **New render target.** If the suite (or a project under it) starts publishing markdown through a new renderer (a docs site, a static-site generator, a wiki), audit the supplement's § [Baseline standards](#baseline-standards) and § [Security](#security) sections for assumptions tied to GitHub's renderer that may not hold elsewhere.
- **Forward-only constraint.** This supplement is forward-facing content authored post-[Review 79](../suite-development/review-log/2026-05-20-suite-review.md#review-79--2026-05-20-1730z). Updates to the supplement follow the same anchor-link convention the supplement teaches — every new internal reference is linked; every new external first-mention is linked; the supplement exemplifies the discipline at every authoring step.
- **Domain registration.** When new domains register (e.g., [Documentation Reviewer](../domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) lands in a subsequent Review), add a per-domain section here covering that domain's markdown stance. When existing domains adopt new dimensions that touch markdown, update the corresponding section to reference the new dimension.
- **External-link rot.** External links in this supplement (to [markdownlint](https://github.com/DavidAnson/markdownlint), [lychee](https://github.com/lycheeverse/lychee), [prettier](https://prettier.io/), [pandoc](https://pandoc.org/), [crosslink](https://github.com/forecast-bio/crosslink), [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00), [VDD whitepaper](https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25), [dollspace.gay](https://github.com/dollspace-gay)) decay over time. The link-checking CI gate catches dead links; replace or update as needed.
