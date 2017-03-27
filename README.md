# Rusttracer

YAR(Yet Another Raytracer).

I like to write raytracers to learn new programming languages. They're fun and are a natural way to learn most OOP languages. Naturally I'm writing a raytracer in rust called rusttracer 🙂.. This is largely a port of my SwiftTracer.

## Running it

Make sure you have the rust toolchain installed then run

```bash
NUM_THREADS=4 cargo run
```

To benchmark the implementation first build for release with

```bash
cargo build --release
```

then run

```bash
NUM_THREADS=4 BENCHMARK=true target/release/rusttracer
```

The benchmark is fairly naive and currently just renders the scene setup in `main.rs` several times.

## Renders

This was the first render produced.

![](docs/first-render.png)

This was rendered a while later, at this point there was support for diffuse colors, specular highlights, reflection, colored lights, and super sampling. This is 2560x1440 at 4x4 super sampling

![](docs/bit-later-render.png)