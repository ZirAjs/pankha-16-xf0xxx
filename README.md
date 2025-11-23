# pankha-16-xf0xxx

This is a fork of [pankha](https://github.com/VulnX/pankha), modified to work with the `HP OMEN by HP Gaming Laptop 16-xf0xxx` series.

> ⚠️ **DISCLAIMER**: This tool has only been tested on **my** omen-16 xf0xxx device. The `EC registers` are almost certainly different on other laptop models. **Use at your own risk.**

## Changes from original pankha

- Modified the `EC register` mappings in the driver to match those of the `HP OMEN 16-xf0xxx` series. 
  - Referenced [EC dump by @realKarthikNair](https://github.com/realKarthikNair/16-xf0xxx-EC-Dumps)
  - If you are curious about the mapping, please refer to the source above.
- Modified client to see xf0xxx temperature without panic.

## Installation

See [INSTALL.md](INSTALL.md)

## 📎 Notes

- Tested only on `HP OMEN 16-xf0xxx` series.

## 🛠️ TODO

- [ ] Feature: auto-adjust fan speed based on temperature
