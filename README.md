# Jam

Jam is a command line tool used to pack images into spritesheets and
easily reference them in code, including Roblox games.

## Features

- Generates Luau or Typescript code so you can use them in your game
- Bring in files, or import from Material Symbols, Font Awesome, etc.

## Later:tm:

- Non-square sprites

```toml
[spritesheets.icons.imagegen]
output_dir = "assets/spritesheets/icons"
size_xy = 512
sprites_per_row = 6

[spritesheets.icons.codegen]
output = "src/spritesheets/icons"
luau = true

[spritesheets.icons.sprites]
dropdown = { material_symbols = "arrow_drop_down", style = "filled" }
heartbroken = { material_symbols = "broken_heart", style = "filled" }
close = { material_symbols = "close", style = "filled" }

[spritesheets.logos.imagegen]
output_dir = "assets/spritesheets/logos"
size_xy = 1280
sprites_per_row = 3

[spritesheets.logos.codegen]
output = "src/spritesheets/logos"
luau = true

[spritesheets.logos.sprites]
etoh = { file = "assets/logos/etoh" }
tct = { file = "assets/logos/etoh" }
wth = { file = "assets/logos/etoh" }
```

To install dependencies:

```bash
bun install
```

To run:

```bash
bun run src/index.ts
```

This project was created using `bun init` in bun v1.2.2. [Bun](https://bun.sh) is a fast all-in-one JavaScript runtime.
