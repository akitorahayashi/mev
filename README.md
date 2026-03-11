# mev

macOS development environment provisioning CLI.

Rust-first architecture with embedded Ansible assets.

## Prerequisites

1. Xcode Command Line Tools
   ```sh
   xcode-select --install
   ```
2. Homebrew
   ```sh
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   ```
3. pipx
   ```sh
   brew install pipx
   pipx ensurepath
   ```
4. ansible (via pipx)
   ```sh
   pipx install ansible
   ```

## Installation

```sh
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/akitorahayashi/mev/main/install.sh)"
```

## Verification

```sh
mev --version
mev list
```

## Documentation

- [Documentation Index](docs/README.md): Central index routing to specific documentation areas, such as Usage and Architecture.
- [Contributing](CONTRIBUTING.md): Development guidelines, coding standards, and testing procedures for contributors.
