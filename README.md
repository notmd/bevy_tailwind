<!-- <img alt="Crates.io Version" src="https://img.shields.io/crates/v/bevy_remote_inspector"><img alt="docs.rs" src="https://img.shields.io/docsrs/bevy_remote_inspector"> -->

## Features

- Generate smart and efficient code (just like hand written) at compile time. Leaving no runtime overhead.
- Support autocomplete with [TailwindCSS IntelliSense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss).

## Installation

```bash
cargo add bevy_tailwind
```

## How does it work?

`bevy_tailwind` will generate corresponding components for given tailwind classes. For example

```rust
tw!("size-full bg-white z-10 border border-red-500");
```

will generate the following components `Node, BackgroundColor, ZIndex, BorderColor`

Other than that, in will generate very efficient code. For example, the following code

```rust

```

## Development

- Run the example

```bash
cargo run --p example_simple
```

- Run the web client

```bash
pnpm run dev
```

- Then open [http://localhost:1420](http://localhost:1420) in your browser.

## Bevy compatibility

| `bevy_remote_inspector` | `bevy` |
| ----------------------- | ------ |
| 0.1.0                   | 0.15.0 |
