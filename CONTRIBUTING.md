# Contributing Guide

Contributions are absolutely welcome!

## Contact Information

In order of likelihood that I will actionably receive your contact, my
information is:

- Email: [self@myrrlyn.dev](mailto:self@myrrlyn.dev)
- GitHub: [@myrrlyn](//github.com/myrrlyn)
- Twitter: [@myrrlyn](//twitter.com/myrrlyn)
- Mastodon: [@myrrlyn@cybre.space](//cybre.space/myrrlyn)
- Reddit: [/u/myrrlyn](//reddit.com/u/myrrlyn)

I am not active on any IRC channels at this time. I am on Discord in the Rust
channel, so you may be able to reach me there, but I don’t know offhand how to
give out Discord profile links. I have a very consistent username scheme and so
anywhere you see my name, it’s *probably* me and I’ll *probably* respond to it.

## Preconditions

Be able to make a Rust project compile.

Be comfortable using `U+0009 CHARACTER TABULATION` as your indentation setting.

## Contributing

If you have a patch you think is worth inspecting right away, opening a pull
request without prelude is fine, although I would certainly appreciate an
accompanying explanation of what the patch does and why.

If you have questions, bugs, suggestions, or other contributions of any kind
that do not immediately touch the codebase, you can reach me informally to talk
about them or open an issue.

I will do my best to respond to all contacts in a timely manner.

## Workflow

This project uses a `Justfile` to contain its workflows. You can install the
`just` tool from Cargo (`cargo install just`), or from any of the sources listed
at https://github.com/casey/just. If you run `just loop dev` in a separate
terminal while you work, or run `just dev` as your editor’s on-save event, you
should be all set.
