# Install

## Download

Get latest release from [GitHub Releases](https://github.com/davelet/redis-egui-client/releases).

## Build from Source

```bash
git clone https://github.com/davelet/redis-egui-client.git
cd redis-egui-client
cargo run
```

## Build Release

```bash
cargo build --release
```

Executable: `target/release/rudist`

## Important: Keychain/Keyring Access

> ⚠️ **IMPORTANT**: Rudist uses the system keychain (macOS Keychain, Windows Credential Manager, or Linux Secret Service) to securely store Redis connection passwords.

### First Launch

On **first launch**, your system will prompt you to enter your **login password** to allow Rudist to access the keychain. This is a **one-time permission** request.

- **macOS**: You'll see a system dialog asking for your login password
- **Windows**: You may see a UAC prompt or credential request
- **Linux**: A keyring unlock dialog may appear

### Why This Is Needed

- Connection passwords are stored encrypted in the system keychain
- This ensures your Redis credentials remain secure
- Without this permission, Rudist cannot save or retrieve connection passwords

### What If I Deny?

If you deny keychain access, you can still use Rudist, but:
- Passwords won't be saved between sessions
- You'll need to enter passwords manually each time you connect
