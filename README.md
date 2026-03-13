# esp-idf-sample

A repo created by `cargo generate esp-rs/esp-idf-template cargo` (see [`https://github.com/esp-rs/esp-idf-template`](https://github.com/esp-rs/esp-idf-template).

..but able to use ESP-IDF 5.4.2, 5.5.3.


## Requirements

- ESP32-C6 devkit

- Ubuntu Linux, with:

	```
	$ sudo apt install libssl-dev pkg-config python3-venv
	```

	>Note: Full `esp-idf-template` [prerequisites](https://github.com/esp-rs/esp-idf-template?tab=readme-ov-file#prerequisites) are longer; we're trimming to the set that actually seemed to be needed.

- Rust with:

	```
	$ cargo install ldproxy
	```

<!--
sudo apt-get install git wget flex bison gperf python3 python3-pip python3-venv cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0
-->

## Steps

```
$ cargo build --release -vv
```

>`-vv` provides more clues.

<!--
```
$ file target/riscv32imac-esp-espidf/debug/abc
/home/ubuntu/target/riscv32imac-esp-espidf/debug/abc: ELF 32-bit LSB executable, UCB RISC-V, RVC, soft-float ABI, version 1 (SYSV), statically linked, with debug_info, not stripped
```
-->

### Confirm `sdkconfig.default` was followed

```
$ cat target/riscv32imac-esp-espidf/release/build/esp-idf-sys-df770abcade46a94/out/sdkconfig | grep MAIN_TASK_STACK
CONFIG_ESP_MAIN_TASK_STACK_SIZE=8000
CONFIG_MAIN_TASK_STACK_SIZE=8000
```

8000 is, indeed, what we have in `sdkconfig.default`!


### To flash

```
$ espflash flash --monitor target/riscv32imac-esp-espidf/release/abc
[...]
I (349) abc: Hello, world!
I (359) main_task: Returned from app_main()
```

### To clean up

```
$ rm -rf ~/.espressif
$ cargo clean
```


## Advanced

### Changing esp-idf versions

Clear caches, first:

- `rm -rf ~/.embuild`
- `cargo clean`

>Note: Build output might prompt you to run `idf.py fullclean`. DON'T! It's not even available. Do the above, instead.

<!-- hidden
<p />

>Note: Setting the ESP-IDF version *should* be possible also in `Config.toml` (`...`), but the author didn't get that to work. Edit `.cargo/config.toml`, instead.
-->

|version|status|comments|
|---|---|---|
|5.5.3|✅||
|5.4.2|✅|default of `esp-idf-svc` 0.52|
|5.3.3|✅|default of `esp-idf-template` / `esp-idf-svc` 0.51|
