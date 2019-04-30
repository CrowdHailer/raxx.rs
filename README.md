# Rust Web

Experiments for productive Rust web development.

## Static assets

Static assets are found in `src/web/public`.
Each new asset must be added to the lookup function in `web::web_static`.

It would be neater to have a macro that looked through public dir at compile time.

## Reloading

Use [watchexec](https://github.com/watchexec/watchexec) to restart the server when any `src` files change.

- :-) Static files are compiled into release the same as in production.
- :-( Page needs to be reloaded to see changes.
- :-/ State is lost, not a problem for development of statless app.

## Docker

Ensures set up of things like `watchexec` are portable.
