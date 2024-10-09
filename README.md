## Quickstart (Linux)

1. Download `btcmap-cli` binary

```bash
curl --output btcmap-cli --location https://github.com/teambtcmap/mapctl/releases/download/preview/btcmap-cli
````

2. Make dowloaded file executable

```bash
chmod +x /usr/local/bin/btcmap-cli
```

3. Add to PATH for convenience (optional)

```bash
sudo mv btcmap-cli /usr/local/bin/
````

4. Login with your admin password

```bash
btcmap-cli login <password>
```

5. Check `help` to see all available actions

```bash
btcmap-cli help
```
