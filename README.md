[![Crates.io](https://img.shields.io/crates/v/bevy_tailwind.svg)](https://crates.io/crates/bevy_tailwind)
[![Docs](https://docs.rs/bevy_tailwind/badge.svg)](https://docs.rs/bevy/latest/bevy_tailwind/)

## Features

- Generate smart and efficient code (just like hand written) at compile time. Leaving no runtime overhead.
- Support autocomplete with [TailwindCSS IntelliSense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss).
- Support `clsx` object syntax for conditional applying style.
- Support both creation and mutation.

## Installation

```bash
cargo add bevy_tailwind
```

## Configure autocomplete

- Install [TailwindCSS IntelliSense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)
- Create `tailwind.config.js` in your project (recommend in the root folder) and add the following content

```js
module.exports = {
  content: ['./**/*.rs'],
};
```

- Add the following settings to your `settings.json` in vscode

```json
{
  "tailwindCSS.experimental.classRegex": [
    [
      "tw!\\(([^)(]*(?:\\([^)(]*(?:\\([^)(]*(?:\\([^)(]*\\)[^)(]*)*\\)[^)(]*)*\\)[^)(]*)*)\\)",
      "\"(.*?)\""
    ]
  ],
  "tailwindCSS.includeLanguages": {
    "rust": "html"
  }
}
```

## How does it work?

`bevy_tailwind` will generate corresponding components for given tailwind classes. For example

```rust
tw!("size-full bg-white z-10 border border-red-500");
```

will generate the following components `Node, BackgroundColor, ZIndex, BorderColor`

When using object syntax, it will apply priority when generating code. Consider the following example, all `flex`, `block` and `grid` is applied to the `display` property, but the latter will have higher priority. So the order is `grid`, `block`, `flex`.

```rust
tw!("flex", {
    "block": some_cond,
    "grid": some_other_cond
});
```

And here is the generated code

```rust
{
    let __class__cond_0 = some_cond;
    let __class__cond_1 = some_other_cond;
    (bevy::ui::Node {
        display: if __class__cond_1 {
            bevy::ui::Display::Grid
        } else {
            if __class__cond_0 {
                bevy::ui::Display::Block
            } else {
                bevy::ui::Display::Flex
            }
        },
        ..Default::default()
    })
}
```

Now consider more complex example. `pl` has higher priority than `px`, `p` and the latter one will overwrite. So the order for padding left is `pl-4`, `pl-3`(from `px-3` but this will be overwrite by `pl-4`), `pl-2`, `pl-5`(from `p-5`), `pl-1`(from `p-1`).

```rust
tw!("p-1", {
    "pl-2": true,
    "px-3 pl-4": true,
    "p-5": true
});
```

And here is the generated code (for padding left only, other properties are ommitted to reduce code size for this example). You can see the code is for `pl-3` from `px-3` is not generated because it will always be overwritten by `pl-4`

```rust
{
    let __class__cond_0 = true;
    let __class__cond_1 = true;
    let __class__cond_2 = true;
    (bevy::ui::Node {
        padding: bevy::ui::UiRect {
            left: if __class__cond_1 {
                bevy::ui::Val::Px(16f32)
            } else {
                if __class__cond_0 {
                    bevy::ui::Val::Px(8f32)
                } else {
                    if __class__cond_2 {
                        bevy::ui::Val::Px(20f32)
                    } else {
                        bevy::ui::Val::Px(4f32)
                    }
                }
            },
            ..Default::default()
        },
        ..Default::default()
    })
}
```

The same rules will also be applied for mutation syntax.

```rust
tw!(&mut node, "p-1", {
    "pl-2": true,
    "px-3 pl-4": true,
    "p-5": true
});
```

```rust
{
    let mut __comp = &mut node;
    let __class__cond_0 = true;
    let __class__cond_1 = true;
    let __class__cond_2 = true;
    if __class__cond_1 {
        __comp.padding.left = bevy::ui::Val::Px(16f32);
    } else {
        if __class__cond_0 {
            __comp.padding.left = bevy::ui::Val::Px(8f32);
        } else {
            if __class__cond_2 {
                __comp.padding.left = bevy::ui::Val::Px(20f32);
            } else {
                __comp.padding.left = bevy::ui::Val::Px(4f32);
            }
        }
    }
    __comp
}

```

## Troubleshooting

If you encounter weird rendering issues, you can check the generated code by using RustAnalyzer's Expand Macro feature or [cargo expand](https://github.com/dtolnay/cargo-expand).

## Bevy compatibility

| `bevy_tailwind` | `bevy` |
| --------------- | ------ |
| 0.1.0           | 0.15.0 |

## TailwindCSS compatibility

Most of TailwindCSS classes are supported except [arbitrary values](https://tailwindcss.com/docs/adding-custom-styles#using-arbitrary-values). See all supported classes [here](https://github.com/notmd/bevy_tailwind/blob/main/macros/tests/pass/src/lib.rs)
