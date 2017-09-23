# Cleanup Plans
With the messy fast development, the library has become a bit of a mess, here's
a list of things that have to be done to clean everything up.

- Remove multi-window support, the maintenance cost isn't worth it.
- Instead of using the complex messy render_pass() system, just pass over a
    full set of rendering commands all at once in one structure.
- Move all raw stuff into its own modules, add trait for accessing the raw
    backend of a structure.
- Move various things in Simple2D that are consistent between multiple backends
    into the core library.
- Move all traits that aren't Raw yet to a Raw type and create an equivalent
    non-raw accessor (known: (Simple2D/World3D)Renderer, Frame)
- Rename Initializer to Context in static, as it better communicates what it is.
- Do a big cleanup pass over public facing APIs, add checklist items for making
    various things more consistent and easy to use.
