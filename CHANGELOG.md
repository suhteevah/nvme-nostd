# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-02

### Added

- Initial release
- NVMe 1.4 controller register definitions with volatile MMIO access
- Submission Queue Entry (64-byte) and Completion Queue Entry (16-byte) structures
- QueuePair with phase-bit tracking, doorbell management, and spin-poll completion
- Admin commands: Identify Controller, Identify Namespace, Create I/O CQ/SQ,
  Set/Get Features, Set Number of Queues
- I/O commands: Read, Write, Flush with automatic PRP list construction
- High-level `NvmeController` with full initialization sequence
- `NvmeDisk` namespace handle for sector-level I/O
- `NvmeBlockDevice` byte-level wrapper with automatic sector alignment and
  read-modify-write for unaligned access
- PCI class/subclass/progif constants for NVMe device detection
- Comprehensive NVMe status code constants
- Verbose logging via the `log` crate
