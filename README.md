# Autorok

**Autorok** is a Rust-powered auto-configuration tool built on top of [zrok](https://zrok.io/) that makes peer-to-peer connectivity effortless even without a public IP address.

## Why Autorok?

Normally, two devices can't talk to each other directly unless at least one has a public IP. That's a dealbreaker if you're behind NAT, on a home network, or just don't want to mess with port forwarding.

Autorok solves this by automatically handling zrok's setup and configuration for you, letting you punch through that limitation with zero manual tunnel configuration.

## What can you do with it?

- **Share a website** running on your local machine with anyone, instantly.
- **Host a game server** (e.g. Minecraft) and let friends join no public IP, no router config, no hassle.
- **Connect peers directly** for any service that normally requires public network access.


[![Download](https://img.shields.io/badge/Download-GitHub-blue?style=for-the-badge&logo=github)](https://github.com/GoodDay360/autorok/releases/latest)

## How It Works

Autorok comes with a simple interactive CLI that handles all the zrok setup for you:


# Build
- Required `Rust` and `Cargo` to compile.
```
cargo build --release
```
