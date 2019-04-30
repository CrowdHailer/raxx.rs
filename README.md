# Rust Web

Experiments for productive Rust web development.

## Built assets

Assets are built in a separate container for development.
The built assets are made available to the running app through a shared volume.

*Assets without a build step can simply be added to `my_app/web/fixed_assets`.*

Each new asset must be added to the lookup function in `my_app/web/web_static`.

- :-/ It would be neater to have a macro that looked through public dir at compile time.

## Reloading

Use [watchexec](https://github.com/watchexec/watchexec) to restart the server when any `src` files change.

- :-) Static files are compiled into release the same as in production.
- :-( Page needs to be reloaded to see changes.
- :-/ State is lost, not a problem for development of statless app.

## Docker

- :-) Ensures set up of things like `watchexec` are portable.
- :-) Can use a completely isolated container for asset building
- ??? Would it be quicker to build npm tools into container?
