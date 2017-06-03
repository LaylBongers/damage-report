# Cobalt
Cobalt is a rendering engine and toolset for creating games in Rust.

## Project Overview
Here's a full list of all the projects that make up cobalt.

- **cobalt-apr** (*stub*): Also called "caper", generic asset pipeline task
    runner. Processes assets into game-engine-ready formats.
- **cobalt-rendering**: The core rendering library, manages your window and
    generic rendering-related tasks.
- **cobalt-rendering-shaders**: Vulkano generated shader binding code.
- **cobalt-rendering-world3d**: 3D rendering tools, provides you with a 3D world
    representation, and a renderer to put it on screen. Has the following
    features:
    - Perspective Rendering
    - Basic Lighting
- **cobalt-utils**: Misc generic game utilities. Contains the following utilities:
    - Game Loop Timer
