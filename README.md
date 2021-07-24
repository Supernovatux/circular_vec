# Circular Vec

This provides a struct called CircularVec. It is a fixed length Vec that provides a `next` function, which iterates through the vec. When it hits the end, instead of returning None, we just loop back to the start.

Note: This struct could use a lot of love. While I do have a need for this struct, and I use this for a personal project, this is also to become familiar with publishing a crate to crates.io. See the many ways this could be improved below.

## Future work
- This is likely not as efficient as it could be. We use Vec internally, which can grow, but we don't need that.
- We could export a version of this that _can_ change size. Shrink semantics TBA.
- This doesn't implement many traits that it could. See the traits that [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) implements for inspiration.
- The API could probably use some love, like exposing this as an iterator instead of providing a `next` function.
- Probably much much more.
