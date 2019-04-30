# Rust Web

Experiments for productive Rust web development.

## Reloading

Use [watchexec](https://github.com/watchexec/watchexec) to restart the server when any `src` files change.

- :-) Static files are compiled into release the same as in production.
- :-( Page needs to be reloaded to see changes.

## Docker

Ensures set up of things like `watchexec` are portable.
