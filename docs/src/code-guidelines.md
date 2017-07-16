# Code Guidelines
These are general guidelines for the calcium libraries. They're not required but
are best practices for consistency. Not all code follows this and where it
doesn't, pull requests are welcome to bring it in line unless there's a good
reason to keep it the way it is.

## Types
- Types that can be created using `default()` or by directly setting fields,
    should also have `new()` (or equivalent) with the most likely values that
    should be set as parameters.

## Functions
- Subject parameter first, with the exception of `self`
    - `fn render_rectangle(rectangle, renderer, target)`
    - `fn update_state(state, events)`
    - `fn handle_events(events, state)`
    - `fn render_world(&self, world, camera, renderer)`
