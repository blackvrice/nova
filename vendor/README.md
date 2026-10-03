# Vendored Unicode identifier data

Nova D02 fixes identifier classification to Unicode **18.0.0** using
`unicode-ident` **1.0.26**. This dependency is excluded from the Nova workspace
tests and is included unchanged so frontend builds require no registry access.

- Official source: https://github.com/dtolnay/unicode-ident
- Official archive: https://static.crates.io/crates/unicode-ident/unicode-ident-1.0.26.crate
- Archive SHA-256: `d245f478577f809a851594d02313b640fb437e0bb33866753cff937863096954`
- License: `(MIT OR Apache-2.0) AND Unicode-3.0`; the upstream license files are retained.

Changing the Unicode data version is a language compatibility decision. Do not
silently update this directory or normalize identifier source spelling.
