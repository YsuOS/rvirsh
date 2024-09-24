# rvirsh(rv)

`rvirsh` is a Rust-based virtual machine management tool built using the `libvirt-rs` library. It reimplements the functionality of the `virsh` command, providing simple and efficient virtualization operations in Rust.

To avoid using complex arguments, `rvirsh` reads default configuration from `$HOME/.config/rvirsh/default.toml` or `./default.toml` (if it does not exists).

## Installation

1. Ensure that Rust is installed on your system. If not, install it from the [official Rust website](https://www.rust-lang.org/tools/install).

2. Ensure that `libvirt-dev` or `libvirt-devel` is installed on your system. Otherwise, compile will be fail. If not, install it with the following command (in Fedora/RHEL).

   ```bash
   dnf install libvirt-devel
   ```

3. Clone the `rvirsh` repository:

   ```bash
   git clone https://github.com/your-username/rvirsh.git
   ```

4. Navigate to the project directory and build it using Cargo:

   ```bash
   cd rvirsh
   cargo build --release
   ```

5. Install the binary:

   ```bash
   cargo install --path .
   ```

## Usage

`rvirsh` provides commands similar to those found in `virsh`. Below are some basic examples. Please run `rv help` to see which commands are supported.

### List all virtual machines

```bash
rv list
```

### Start a virtual machine

```bash
rv start <vm-name>
```

### Shut down a virtual machine

```bash
rv shutdown <vm-name>
```

### Display information about a virtual machine

```bash
rv dominfo <vm-name>
```

### Show detailed configuration of a virtual machine

```bash
rv dumpxml <vm-name>
```

## Development

To set up the development environment, follow these steps:

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/rvirsh.git
   ```

2. Install the necessary dependencies:

   ```bash
   cargo build
   ```

3. Run tests:

   ```bash
   cargo test
   ```

## Dependencies

- [`libvirt-rs`](https://gitlab.com/ryasuoka/libvirt-rust/-/tree/dev?ref_type=heads) - Rust bindings for the Libvirt API. Note that rvirsh uses some features not implemented in upstream `libvirt-rs`. So it depends on my personal `libvirt-rs`.
## Contributing

Please report any bugs or feature requests via the GitHub issue tracker. Pull requests are welcome!
