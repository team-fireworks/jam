# Springroll

Spritesheets as code command line tool.

> [!WARNING]
> This is my first actual Rust project. I struggled making this, so PRs are very
> welcome :)
> 
> Documentation later :)

## Features

- Configure everything with a `Springroll.toml` file
- Use your own PNGs/SVGs, or source from Material Symbols with more to come
- Assortment of outputs: PNGs Luau, TypeScript, JSON, TOML, YAML, etc.
  decent formatting and more to come
- Customize spritesheet size and sprites per row
- Alpha bleeding for images

## Coming Later™

- Spritesheets and sprites that aren't squares, will probably allow the user to
  specify the width/height of sprites
- Support for other file extensions
- More sources
- More outputs
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

**Not yet implemented.**

#### Fluent

**Not yet implemented.**

#### Luicide

### Outputs

#### Images

#### Luau

Dumps all sprites widths, heights, spritesheet, and XY position to a usable
Luau file.

```toml
[[spritesheets.icons.outputs]]
type = "luau"
path = "icons.luau"
include_prelude_types = false
new_luau_solver = true
freeze_tables = false
type_casing = "PascalCase"
field_casing = "camelCase"
```

#### TypeScript

Dumps all sprites widths, heights, spritesheet, and XY position to a usable
TypeScript file.

```toml
[[spritesheets.icons.outputs]]
type = "luau"
path = "icons.luau"
include_prelude_types = false
new_luau_solver = true
freeze_tables = false
type_casing = "PascalCase"
field_casing = "camelCase"
```

#### TypeScript Declarations (`d.ts`)

Typings for sprites widths, heights, spritesheet, and XY position. Most useful
when paired with another output, such as Luau.

#### JSON

**Not yet implemented.**

#### TOML

**Not yet implemented.**

#### YAML

**Not yet implemented.**

## License

Springroll is licensed under the Mozilla Public License 2.0.
