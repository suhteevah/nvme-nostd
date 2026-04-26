# nvme-nostd

[![Crates.io](https://img.shields.io/crates/v/nvme-nostd.svg)](https://crates.io/crates/nvme-nostd)
[![Documentation](https://docs.rs/nvme-nostd/badge.svg)](https://docs.rs/nvme-nostd)
[![License](https://img.shields.io/crates/l/nvme-nostd.svg)](LICENSE-MIT)

A `#![no_std]` NVMe driver written in pure Rust. Talks directly to NVMe controllers
via PCI BAR0 memory-mapped registers using volatile MMIO. No standard library, no OS
dependencies -- just bring your own allocator and identity-mapped memory.

## Features

- **Full NVMe 1.4 initialization sequence** -- reset, admin queue setup, controller
  enable, Identify Controller/Namespace, I/O queue creation
- **Admin commands** -- Identify Controller, Identify Namespace, Create I/O CQ/SQ,
  Set/Get Features, Set Number of Queues
- **I/O commands** -- Read, Write, Flush with automatic PRP list construction for
  multi-page transfers
- **Block device abstraction** -- `NvmeBlockDevice` provides byte-level `read_bytes` /
  `write_bytes` with automatic sector alignment and read-modify-write for unaligned
  access
- **Comprehensive register definitions** -- all NVMe 1.4 controller registers (CAP,
  CC, CSTS, AQA, ASQ, ACQ) with volatile access and bitfield constants
- **Status code constants** -- full set of NVMe generic status codes and status code
  types for error handling
- **`#![no_std]` + `alloc`** -- no OS, no standard library. Works in bare-metal
  kernels, UEFI applications, and embedded systems with a heap allocator
- **Verbose logging** via the `log` crate -- every register access, command
  submission, and completion is traced

## Quick Start

```rust,ignore
use nvme_nostd::{NvmeController, NvmeBlockDevice};

// BAR0 address from PCI enumeration (must be identity-mapped)
let mut ctrl = unsafe {
    NvmeController::init(bar0_addr).expect("NVMe init failed")
};

// Get namespace 1
let disk = ctrl.namespace(1).expect("namespace not found");
println!("Disk: {} sectors x {} bytes", disk.sector_count, disk.sector_size);

// Sector-level I/O
let mut buf = [0u8; 512];
disk.read_sectors(0, 1, &mut buf, &mut ctrl).expect("read failed");

// Byte-level block device (handles alignment automatically)
let mut blk = NvmeBlockDevice::new(&mut ctrl, 1, disk.sector_count, disk.sector_size);
let mut data = [0u8; 100];
blk.read_bytes(1234, &mut data).expect("read failed");
blk.write_bytes(1234, &data).expect("write failed");
blk.flush().expect("flush failed");
```

## Requirements

- A global allocator (`#[global_allocator]`) -- queue buffers and PRP lists are
  heap-allocated
- Identity-mapped memory -- the driver uses heap pointers as physical addresses for
  DMA. Your page tables must map virtual addresses 1:1 with physical addresses (or
  you must provide a virt-to-phys translation layer)
- BAR0 must be mapped as uncacheable MMIO

## Architecture

```text
NvmeController          -- owns BAR0 registers, admin queue, controller identity
  +-- NvmeDisk          -- namespace handle (sector count, sector size, LBA formats)
  +-- NvmeBlockDevice   -- byte-level I/O with automatic sector alignment
  +-- QueuePair         -- submission + completion queue with doorbell management
```

### Modules

| Module      | Description |
|-------------|-------------|
| `registers` | BAR0 MMIO register definitions, volatile read/write, doorbell calculation |
| `queue`     | Submission/Completion queue entries, QueuePair with phase-bit tracking |
| `admin`     | Admin command opcodes, Identify parsing, queue creation, Set/Get Features |
| `io`        | I/O command opcodes, Read/Write/Flush, PRP list construction |
| `driver`    | High-level controller init, namespace probing, BlockDevice wrapper |

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please open an issue or pull request on
[GitHub](https://github.com/suhteevah/nvme-nostd).

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

## Support This Project

If you find this project useful, consider buying me a coffee! Your support helps me keep building and sharing open-source tools.

[![Donate via PayPal](https://img.shields.io/badge/Donate-PayPal-blue.svg?logo=paypal)](https://www.paypal.me/baal_hosting)

**PayPal:** [baal_hosting@live.com](https://paypal.me/baal_hosting)

Every donation, no matter how small, is greatly appreciated and motivates continued development. Thank you!
