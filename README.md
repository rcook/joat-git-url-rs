# git-url

[![AppVeyor status for project](https://ci.appveyor.com/api/projects/status/fxw6ppgkjjel568o?svg=true)](https://ci.appveyor.com/project/rcook/git-url)
[![AppVeyor status for master branch](https://ci.appveyor.com/api/projects/status/fxw6ppgkjjel568o/branch/master?svg=true)](https://ci.appveyor.com/project/rcook/git-url/branch/master)

_Parse Git repo URLs in Rust_

[Official home page][home]

This is intended to be a cross-platform Rust function for parsing Git repo URLs in Rust.

## Building locally

### Install Rust

* [rustup][rustup] is recommended
* Building with rustup has been tested on Linux, Windows and macOS

### Clone workspace

```bash
cd /path/to/repos
git clone https://gitlab.com/rcook/git-url.git
cd /path/to/repos/git-url
cargo build
```

## Licence

[MIT License][licence]

[home]: https://github.com/rcook/git-url
[licence]: LICENSE
[rustup]: https://rustup.rs/
