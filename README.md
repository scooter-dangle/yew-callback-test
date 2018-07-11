Yew Callback Test
=================

To build:

```sh
docker build --tag $(basename $PWD) .
docker run --interactive --tty --rm --volume $PWD:/src --workdir /src $(basename $PWD) make web_app
```

Serve generated contents of `./site` via a typical static server, e.g., with
Ruby:

```sh
ruby -run -e httpd site/
```

If you have `rack` and `puma` available, you can serve the correct MIME types
for Wasm via `make serve`.

See https://github.com/DenisKolodin/yew/issues/316 for discussion on what
motivated this test project.
