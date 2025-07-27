# Springroll

Spritesheets as code command line tool.

> [!WARNING]
> This is my first actual Rust project. I struggled making this, so PRs are very
> welcome :)

## Features

- Configure everything with a `Springroll.toml` file
- Use your own images, or source from Material Symbols, Font Awesome, Luicide,
  Fluent, etc. with more to come
- Assortment of outputs: PNGs, Luau, TypeScript, JSON, TOML, YAML, etc. with
  decent formatting and more to come
- Customize spritesheet size and sprites per row
- Alpha bleeding for images

## Coming Later™

- Spritesheets and sprites that aren't squares, will probably allow the user to
  specify the width/height of sprites
- Blazingly fast

## Configuration

Spritesheets are declared in the `[spritesheets]` object:

```TOML
[spritesheets.icons.spritegen]
spritesheet_size = 512
sprites_per_row = 6

[[spritesheets.icons.outputs]]
# ...

[spritesheets.icons.sprites]
# ...
```

- `spritegen`:
- `outputs`:
- `sprites`:

### Sources

#### Path

#### Material Symbols

#### Font Awesome

#### Fluent

#### Luicide

### Outputs

#### Images

#### Luau

#### TypeScript

#### TypeScript Declarations (`d.ts`)

#### JSON

#### TOML

#### YAML

## License

Springroll is licensed under the Mozilla Public License 2.0.
