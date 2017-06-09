# Calcium
Calcium is a series of libraries aimed at improving the infrastructure and
graphical quality of game projects made in Rust. Its aims are to create various
libraries that can independently be used in various other Rust engines and game
frameworks.

The mentality behind these libraries should be to increase choice. It should
make it easier to create new engines by picking certain libraries both from
inside and outside calcium.

## Project Overview
Here's a full list of all the projects that make up calcium.

- **calcium-apr** (*stub*): Also called "caper", generic asset pipeline task
    runner. Processes assets into game-engine-ready formats, and builds data
    packages.
- **calcium-rendering**: The core rendering library, manages your window and
    generic rendering-related tasks.
- **calcium-rendering-shaders**: Vulkano generated shader binding code.
- **calcium-rendering-world3d**: 3D rendering tools, provides you with a 3D world
    representation, and a renderer to put it on screen. Has the following
    features:
    - Perspective Rendering
    - Deferred PBR Lighting
- **calcium-utils**: Misc generic game utilities. Contains the following utilities:
    - Game Loop Timer
