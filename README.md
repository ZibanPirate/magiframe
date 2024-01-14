# MagiFrame

a frame (with a cheesy name) that is alive like the portraits at Hogwarts

## Structure

| **Folder**           | **Why**                                                 |
| :------------------- | :------------------------------------------------------ |
| [./api](./api)       | Axum server that return nice images, in a dotted format |
| [./mobile](./mobile) | Where you register and link your MagiFrame              |
| [./frame](./frame)   | ESP-32 and an E-Paper to bring life to the frame        |

## Get Started

Please use [VSCode](https://code.visualstudio.com/), and have [Rustup](https://rustup.rs/) and [BunJS](https://bun.sh/) installed.

Then simply run VSCode **Build Task** (<kbd>Ctrl</kbd>/<kbd>⌘ Command</kbd> + <kbd>⇧ Shift</kbd> + <kbd>B</kbd>).

Additionally, you can also attach a debugger to `./api` (<kbd>F5</kbd>)
