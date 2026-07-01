# Autorok
[![Download](https://img.shields.io/badge/Download-GitHub-blue?style=for-the-badge&logo=github)](https://github.com/GoodDay360/autorok/releases/latest)

**Autorok** is a Rust-powered auto-configuration tool built on top of [zrok](https://zrok.io/) that makes peer-to-peer connectivity effortless even without a public IP address.

## Why Autorok?

Normally, two devices can't talk to each other directly unless at least one has a public IP. That's a dealbreaker if you're behind NAT, on a home network, or just don't want to mess with port forwarding.

Autorok solves this by automatically handling zrok's setup and configuration for you, letting you punch through that limitation with zero manual tunnel configuration.

## What can you do with it?

- **Share a website** running on your local machine with anyone, instantly.
- **Host a game server** (e.g. Minecraft) and let friends join no public IP, no router config needed.
- **Connect peers directly** for any service that normally requires public network access.
- **Backend Mode** support: udp, tcp



## How It Works

Autorok comes with a simple interactive CLI that handles all the zrok setup for you:

<img width="283" height="215" alt="image" src="https://github.com/user-attachments/assets/6a764776-2ddc-4715-a66e-d2cc2c57347a" />

Getting a peer-to-peer connection running takes just four steps:

### 1. Get an environment token
Grab an environment token from [https://api.zrok.io/](https://api.zrok.io/). 

<img width="924" height="297" alt="image" src="https://github.com/user-attachments/assets/17b437d2-fff4-417e-8fd4-c7b1c5a5d6b7" />


This token is what links your environment so others using the same one can join your services.
After that enable your token in autorok.

<img width="291" height="54" alt="image" src="https://github.com/user-attachments/assets/0ad8924b-d864-46e2-b6b2-35005276cd06" />

### 2. Host creates a service
The host defines what they want to expose, like a website or a game server, and creates a service for it through the CLI.
<img width="428" height="259" alt="image" src="https://github.com/user-attachments/assets/1de294d5-61f8-4587-8e59-0231585a5056" />

### 3. Host shares the service
Once created, the host shares the service.
<img width="769" height="257" alt="image" src="https://github.com/user-attachments/assets/c677142a-8aed-4437-a9d4-75f03c7ccae7" />

### 4. Everyone else joins
Anyone using the same environment token can simply join that shared service and connect directly. No public IP required on either end.

<img width="791" height="244" alt="image" src="https://github.com/user-attachments/assets/6e96e574-2486-418c-88f6-1dd1b5123990" />


That's it. No router settings, no dynamic DNS, no fiddling with firewalls. Just get a token, create, share, and join.

# Build
- Required `Rust` and `Cargo` to compile.
```
cargo build --release
```
