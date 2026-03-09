# Which version of `esp-idf` to use?

- If you use `esp-idf-svc = "0.51"` (latest release, 8-Mar-26), you can climb up to 5.3.x

- If you use `master`, we can note that:

	- `esp-idf-svc` claims ["compatibility with ESP-IDF v.5.4x and v5.5.x](https://github.com/esp-rs/esp-idf-svc/blob/master/CHANGELOG.md?plain=1#L50)
	- `esp-idf-svc` itself [uses `v5.4.2`](https://github.com/esp-rs/esp-idf-svc/blob/master/.cargo/config.toml#L28) - and sets it via `.cargo/config.toml`.

	- `esp-idf-sys` (`master`) likewise states ["Compatibility with ESP-IDF v5.4.X, v5.5.x"](https://github.com/esp-rs/esp-idf-sys/blob/master/CHANGELOG.md?plain=1#L23)

There are some pointers that claim the version could be given as a section in `Cargo.toml`:

```
[package.metadata.esp-idf-sys]
esp_idf_version = "v5.5"
```

The author **did not get this to work** - and there's no real down side in providing it in `.cargo/config.toml`.


## Which version to use?

I don't know. I'll go with the 5.4.2 that `esp-idf-svc` itself uses. Seems like a safe bet.

## Track

- [ ] `esp-idf-svc` could make a new release?

