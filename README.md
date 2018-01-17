# Haque (*"hack"*)

Haque is a Rust library for doing "unsafe hacks" with memory management!

## Tools

### `Man<T>`

The `Man<T>` type is a manually-managed pointer type.  It lets you ergonomically
create objects that live on the heap and pass around the reference to them
fairly easily.  It implements `AsRef` so it can be used anywhere that can.
Basically the only time you should use this is if you *really* know what you're
doing and Rust's lifetimes aren't on your side today.

Calling `.clone()` on it will do a proper clone of the underlying data, as if it
was in a `Box`.

It does *absolutely nothing* to stop you from having multiple mutable (raw)
references, so good luck with that, buddy.

**Be sure to call `.free()` on it** to make sure that you free the underlying
heap space, as if you don't then it'll leak memory.  And be sure that you don't
try to access it *after* you've freed it because *that's wrong*.  Also don't
call free on it twice.
