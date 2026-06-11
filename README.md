# esp-idf-sample

A repo created by `cargo generate esp-rs/esp-idf-template cargo` (see <tt><https://github.com/esp-rs/esp-idf-template></tt>).

..but able to use ESP-IDF 5.5.4, 6.0.1.
<!-- was: 5.4.2, 5.5.3 -->

## Requirements

- ESP32-C6 devkit

- Ubuntu Linux, with:

	```
	$ sudo apt install libssl-dev pkg-config python3-venv
	```

	>Note: Full `esp-idf-template` [prerequisites](https://github.com/esp-rs/esp-idf-template?tab=readme-ov-file#prerequisites) are longer; we're trimming to the set that actually seemed to be needed.

- Rust with:

	```
	$ cargo install --locked ldproxy
	```

<!--
sudo apt-get install git wget flex bison gperf python3 python3-pip python3-venv cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0
-->

## Steps

```
$ cargo build --release -vv
```

<!--
```
$ file target/riscv32imac-esp-espidf/debug/abc
/home/ubuntu/target/riscv32imac-esp-espidf/debug/abc: ELF 32-bit LSB executable, UCB RISC-V, RVC, soft-float ABI, version 1 (SYSV), statically linked, with debug_info, not stripped
```
-->

### Confirm `sdkconfig.default` was followed

```
$ cat ~/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-df770abcade46a94/out/sdkconfig | grep MAIN_TASK_STACK
CONFIG_ESP_MAIN_TASK_STACK_SIZE=8000
CONFIG_MAIN_TASK_STACK_SIZE=8000
```

8000 is, indeed, what we have in `sdkconfig.default`!

>Note: The author has a setup where an account-global `~/target` is used. Yours may be local to the work folder. To find the right file, use: 
>
>```
>$ find ~/target -name sdkconfig
>```

### To flash

```
$ espflash flash --monitor ~/target/riscv32imac-esp-espidf/release/abc
[...]
I (349) abc: Hello, world!
I (359) main_task: Returned from app_main()
```

>Press Ctrl-C to end the process.

### To clean up

```
$ rm -rf ~/.espressif
$ cargo clean
```


## Advanced

### Changing esp-idf versions

Clear caches, first:

- `cargo clean`

>Note: Build output might prompt you to run `idf.py fullclean`. DON'T! It's not even available. Do the above, instead.

<p />

>Note: You likely don't need to `rm -rf ~/.espressif`: different ESP-IDF versions vacate different paths within it, and seem to be able to co-exist. If in doubt, wipe it!

<!-- hidden
<p />

>Note: Setting the ESP-IDF version *should* be possible also in `Config.toml` (`...`), but the author didn't get that to work. Edit `.cargo/config.toml`, instead.
-->

|version|status|comments|
|---|---|---|
|5.5.4|✅|works|
|6.0.1|👺|fails|

<!-- older versions; not actively tested any more (since Jun'26)
|5.4.2|♻️|default of `esp-idf-svc` 0.52|
|5.3.3|♻️|default of `esp-idf-template` / `esp-idf-svc` 0.51|
-->

### Testing with plain `esp-idf-sys`

The author experimented with some setups, and managed to get a `#![no_std]` set up to work, with `esp-idf-sys` <sup>`|1|`</sup>:

|branch|uses|`std`|comments|
|---|---|---|---|
|`main`|`esp-idf-svc`|yes|works|
|`sys`|`esp-idf-sys`|yes|works|
|`sys-core`|`esp-idf-sys`|no|does not build: `error: linking with ldproxy failed`|
|`sys-core2`|`esp-idf-sys`|partly|works|

<small>
`|1|`: This was with version 5.5.4.
</small>

>NOTE: The author has since moved to full `std` (and `esp-idf-svc`) in his application level use of ESP-IDF. The non-main branches are likely going to be left behind.

The last option is interesting, because it shows how your **application** can be `#![no_std]`, while having `esp-idf-sys` *still* work and launch your code! In that branch, `esp-idf-sys` is without the `"std"` feature, but the runtime has:

```
build-std = ["std", "panic_abort"]
```

This seems to do it for `ldproxy`, so that it creates a runnable binary.

**Sizes**

|branch|Total Image Size [B]|
|---|---|
|`main`|377 136|
|`sys`|369 728|
|`sys-core`|n/a|
|`sys-core2`|149 600|

Total Image Size from `espflash flash` output.

<!-- e.g. 
>```
>App/part. size:    377,136/4,128,768 bytes, 9.13%
>```
-->


## References

- [`embuild`](https://github.com/esp-rs/embuild) (GitHub)

	Contains the `ldproxy` tool.

