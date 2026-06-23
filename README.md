## Quickstart

1. Download `btcmap-cli` binary

```bash
curl --output btcmap-cli --location https://github.com/teambtcmap/mapctl/releases/download/preview/btcmap-cli
````

If you have a Rust environment setup, you can compile from the soruce and go to step 4

2. Make dowloaded file executable

```bash
chmod +x btcmap-cli
```

3. Add to PATH for convenience (optional)

```bash
sudo mv btcmap-cli /usr/local/bin/
````

4. Sign up or sign in

```bash
btcmap-cli signup --user <username> --password <password>
```

or

```bash
btcmap-cli signin <username> <password>
```

5. Check `help` to see all available actions

```bash
btcmap-cli help
```
