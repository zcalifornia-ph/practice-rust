<a id="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![Apache-2.0][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

<div align="center">

[![PRACTICE RUST Screen Shot][product-screenshot]](https://github.com/zcalifornia-ph/practice-rust)

<h3 align="center">PRACTICE RUST</h3>

<p align="center">
  <strong>A practice repository to learn Rust programming language, RustC, and Cargo.</strong>
  <br />
  Version: v0.0.4
  <br />
  Status: active learning repository with Cargo baseline and first primitive data types exercise.
  <br />
  <a href="https://github.com/zcalifornia-ph/practice-rust"><strong>Explore the repository</strong></a>
  <br />
  <br />
  <a href="https://github.com/zcalifornia-ph/practice-rust/issues">Report Bug</a>
  &middot;
  <a href="https://github.com/zcalifornia-ph/practice-rust/issues">Request Feature</a>
</p>
</div>

## Table of Contents

- [About The Project](#about-the-project)
  - [Learning Scope](#learning-scope)
  - [Built With](#built-with)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Run Primitive Data Types Exercise](#run-primitive-data-types-exercise)
- [Learning Resource](#learning-resource)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)
- [Acknowledgments](#acknowledgments)

## About The Project

`PRACTICE RUST` is a personal practice repository focused on building practical Rust fundamentals through small, iterative exercises and documentation-first learning.

Current implementation baseline:

- Cargo project scaffold added at `practice-rust/`
- Initial binary runs successfully with `cargo run` (`Hello, world!`)
- First standalone exercise added at `practice-rust/src/u1-primitive-data-types/integers.rs`
- Exercise demonstrates signed/unsigned integer usage and basic `bool`/`char` output

### Learning Scope

- Rust syntax and core language behavior
- `rustc` compiler workflow and diagnostics
- Cargo package management and project commands

### Built With

- [![Rust][Rust-lang]][Rust-url]
- [Rust Compiler (rustc)](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Getting Started

### Prerequisites

- Rust toolchain installed via `rustup`

Verify local installation:

```bash
rustc --version
cargo --version
```

### Installation

1. Clone the repository:

```bash
git clone https://github.com/zcalifornia-ph/practice-rust.git
cd practice-rust
```

2. Run the included Cargo project:

```bash
cd practice-rust
cargo run
```

Expected output:

```text
Hello, world!
```

### Run Primitive Data Types Exercise

To compile and run the first topic exercise:

```bash
cd practice-rust/src/u1-primitive-data-types
rustc integers.rs -o integers
```

Run the produced binary:

```bash
./integers
```

On Windows PowerShell, use:

```powershell
.\integers.exe
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Learning Resource

Current primary learning material:

- Rust Programming Full Course - BekBrace
  - Link: <https://www.youtube.com/watch?v=rQ_J9WH6CGk>
  - Instructor: Amir Bekhit
  - Duration: 3h 05m
  - Premiered: May 22, 2024

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Roadmap

- [x] Initialize first Cargo binary project (`practice-rust`)
- [x] Add first beginner exercise (`u1-primitive-data-types/integers.rs`)
- [ ] Add more beginner Rust exercises per topic
- [ ] Add Cargo workflow notes and command references
- [ ] Add ownership and borrowing practice modules
- [ ] Add tests and examples for collections and error handling

See open issues: <https://github.com/zcalifornia-ph/practice-rust/issues>

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Contributing

Contributions are welcome.
Please review `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, and `SECURITY.md` before opening pull requests.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## License

Distributed under Apache License 2.0.
See `LICENSE.txt` for full terms.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Contact

- Maintainer: Zildjian E. California
- Email: `zecalifornia@up.edu.ph`
- LinkedIn: <https://www.linkedin.com/in/zcalifornia>
- ORCID: <https://orcid.org/0009-0002-2357-7606>
- ResearchGate: <https://www.researchgate.net/profile/Zildjian-California>
- X (Twitter): <https://x.com/zcalifornia_>
- Repository: <https://github.com/zcalifornia-ph/practice-rust>

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Acknowledgments

- Rust documentation team: <https://www.rust-lang.org/learn>
- BekBrace learning content used in this repository context

<p align="right">(<a href="#readme-top">back to top</a>)</p>

[contributors-shield]: https://img.shields.io/github/contributors/zcalifornia-ph/practice-rust.svg?style=for-the-badge
[contributors-url]: https://github.com/zcalifornia-ph/practice-rust/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/zcalifornia-ph/practice-rust.svg?style=for-the-badge
[forks-url]: https://github.com/zcalifornia-ph/practice-rust/network/members
[stars-shield]: https://img.shields.io/github/stars/zcalifornia-ph/practice-rust.svg?style=for-the-badge
[stars-url]: https://github.com/zcalifornia-ph/practice-rust/stargazers
[issues-shield]: https://img.shields.io/github/issues/zcalifornia-ph/practice-rust.svg?style=for-the-badge
[issues-url]: https://github.com/zcalifornia-ph/practice-rust/issues
[license-shield]: https://img.shields.io/github/license/zcalifornia-ph/practice-rust.svg?style=for-the-badge
[license-url]: https://github.com/zcalifornia-ph/practice-rust/blob/main/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/zcalifornia
[product-screenshot]: repo/images/project_screen.png
[Rust-lang]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
