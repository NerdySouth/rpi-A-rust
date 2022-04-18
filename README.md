<p align="center">
  <a href="" rel="noopener">
![Fancy logo](./img/logo_size_invert.jpg#gh-dark-mode-only)
![Fancy logo](./img/logo_size.jpg#gh-light-mode-only)
</p>

<h3 align="center">Nox</h3>

<div align="center">

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![GitHub Issues](https://img.shields.io/github/issues/TristenSeth/nox.svg)](https://github.com/TristenSeth/nox/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/TristenSeth/nox.svg)](https://github.com/TristenSeth/nox/pulls)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)

</div>

---

<p align="center"> Nox is memory safe OS for the Rpi Zero/A+ written in Rust.
    <br> 
</p>

## 📝 Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Deployment](#deployment)
- [Usage](#usage)
- [Built Using](#built_using)
- [TODO](../TODO.md)
- [Contributing](../CONTRIBUTING.md)
- [Authors](#authors)
- [Acknowledgments](#acknowledgement)

## 🧐 About <a name = "about"></a>

Nox is a playground for OS concepts and design in Rust. Initially, we are starting with the Rpi Zero/A+ platform which is a 32bit armv6 platform. The idea is to build a fast, safe OS for doing cool things on a pi. A lot of the initial features will likely come from the work done by Dawson Engler for CS140E at Stanford. I took this class as an undergrad, and want to expand on what we worked on there and push it further with Rust.

## 🏁 Getting Started <a name = "getting_started"></a>

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See [deployment](#deployment) for notes on how to deploy the project on a live system.

### Prerequisites

- Rust nightly v1.60.0
- GCC
- GNU Arm Embedded Toolchain


### Installing

#### [__Rust:__](https://www.rust-lang.org/tools/install)

To install rust, follow the general instructions for installing the rust toolchain above.
Once you have rust installed, you will want to make sure you are on the right compiler version, and that you are using the nightly build, since some features we use in rust are not stable yet.

```
$ rustc --version
rustc 1.60.0-nightly (e789f3a3a 2022-02-11)
```

To set the default compiler version for a single project, navigate to the project directory and run the following

```
$ rustup override set nightly
```

#### __GCC/GNU Toolchain:__

MacOS: go to the [CS107E](http://cs107e.github.io/guides/install/mac/) page and follow their instructions (note: do not do the python stuff).

Ubuntu/Linux: From CS140E [labs/0-blink/README](https://github.com/dddrrreee/cs140e-22win/tree/main/labs/0-blink)

-  For [ubuntu/linux](https://askubuntu.com/questions/1243252/how-to-install-arm-none-eabi-gdb-on-ubuntu-20-04-lts-focal-fossa), ARM recently
      changed their method for distributing the tool change.   Now you
      must manually install.  As of this lab, the following works:

            wget https://developer.arm.com/-/media/Files/downloads/gnu-rm/10.3-2021.10/gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2

            sudo tar xjf gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2 -C /usr/opt/

      Then either add symlinks to these:

            sudo ln -s /usr/opt/gcc-arm-none-eabi-10.3-2021.10/bin/* /usr/bin/

      Or, cleaner, add `/usr/opt/gcc-arm-none-eabi-10.3-2021.10/bin` to your
      `path` variable in your shell configuration file (e.g., `.tchsrc`
       or `.bashrc`), save it, and `source` the configuration.  When you run:


            arm-none-eabi-gcc
            arm-none-eabi-ar
            arm-none-eabi-objdump 

      You should not get a "Command not found" error.


      You may also have to add your username to the `dialout` group.

      If gcc can't find header files, try:

           sudo apt-get install libnewlib-arm-none-eabi


## 🔧 Running the tests <a name = "tests"></a>

Testing coming soon!

### Break down into end to end tests

Explain what these tests test and why

```
Give an example
```

## 🎈 Usage <a name="usage"></a>

Coming soon!

## 🚀 Deployment <a name = "deployment"></a>

Coming Soon!

## ⛏️ Built Using <a name = "built_using"></a>

- [Rust](https://www.rust-lang.org) - High Level Language
- [Armv6 assembly](https://developer.arm.com/documentation/102438/latest://expressjs.com/) - Assembly Language

## ✍️ Authors <a name = "authors"></a>

- [@TristenSeth](https://github.com/TristenSeth) - Idea & Initial work

See also the list of [contributors](https://github.com/kylelobo/The-Documentation-Compendium/contributors) who participated in this project.

## 🎉 Acknowledgements <a name = "acknowledgement"></a>

- Special thanks to [@dddrrreee](https://github.com/dddrrreee) for a lot of the knowledge, inspiration, and
intial work on the rpi A+/Zero as well as his kindness and willingness to teach very hard things.
- Thanks to Stanford CS for the cs107e linker script
- Thanks to [@dwelch67](https://github.com/dwelch67) for initial work and hacks on the rpi platform.
