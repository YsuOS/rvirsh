# rvirsh

`rvirsh` is a Rust-based virtual machine management tool built using the `libvirt-rs` library. It reimplements the functionality of the `virsh` command, providing simple and efficient virtualization operations in Rust.

## Installation

1. Ensure that Rust is installed on your system. If not, install it from the [official Rust website](https://www.rust-lang.org/tools/install).

2. Clone the `rvirsh` repository:

   ```bash
   git clone https://github.com/your-username/rvirsh.git
   ```

3. Navigate to the project directory and build it using Cargo:

   ```bash
   cd rvirsh
   cargo build --release
   ```

4. Install the binary:

   ```bash
   cargo install --path .
   ```

## Usage

`rvirsh` provides commands similar to those found in `virsh`. Below are some basic examples. Please run `rvirsh help` to see which commands are supported.

### List all virtual machines

```bash
rvirsh list --all
```

### Start a virtual machine

```bash
rvirsh start <vm-name>
```

### Shut down a virtual machine

```bash
rvirsh shutdown <vm-name>
```

### Display information about a virtual machine

```bash
rvirsh dominfo <vm-name>
```

### Show detailed configuration of a virtual machine

```bash
rvirsh dumpxml <vm-name>
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

3. Run tests: (Not introduced yet)

   ```bash
   cargo test
   ```

## Dependencies

- [`libvirt-rs`](https://gitlab.com/ryasuoka/libvirt-rust/-/tree/dev?ref_type=heads) - Rust bindings for the Libvirt API. Note that rvirsh uses some features not implemented in upstream `libvirt-rs`. So it depends on my personal `libvirt-rs`.
## Contributing

Please report any bugs or feature requests via the GitHub issue tracker. Pull requests are welcome!
