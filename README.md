# Discord Mafia Rich Presence

A rust program to "advertise" the discord mafia server as a discord rich presence/

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust

### Installation

1. Clone the repository

```sh
git clone https://github.com/mafia-engine/discord-mafia-rich-presence.git
cd discord-mafia-rich-presence
```

<details><summary> MacOS </summary>

2. Run the build script

```sh
./bundle_macos.sh
```

</details>

<details><summary> Windows </summary>

2. Run the build script

```sh
cargo run build
```

</details>

<details><summary> Building for Windows on MacOS </summary>

2. Install the x86_64-pc-windows-gnu toolchain

```sh
rustup target add x86_64-pc-windows-gnu
```

3. Install the mingw-w64 compiler

```sh
brew install mingw-w64
```

4. Run the build script. Remove the --release tag if you're not building for release

```sh
cargo build --release --target x86_64-pc-windows-gnu
```

</details>
