# esp-idf-sample

A repo created by `cargo generate esp-rs/esp-idf-template cargo` (see [`https://github.com/esp-rs/esp-idf-template`](https://github.com/esp-rs/esp-idf-template).

- Aiming to run with ESP-IDF v5.5


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
$ cargo build
```

creates:

```
$ file target/riscv32imac-esp-espidf/debug/abc
/home/ubuntu/target/riscv32imac-esp-espidf/debug/abc: ELF 32-bit LSB executable, UCB RISC-V, RVC, soft-float ABI, version 1 (SYSV), statically linked, with debug_info, not stripped
```

### To flash
```
$ espflash flash --monitor target/riscv32imac-esp-espidf/debug/abc
[...]
I (349) abc: Hello, world!
I (359) main_task: Returned from app_main()
```

