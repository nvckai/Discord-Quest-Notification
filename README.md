# Discord Notification Quest Rust Version
receive notifications when new discord quests are released. filter for orb quests or monitor all quest types.


## 1) Install Rust (via curl)
- Open PowerShell.
```powershell
curl https://sh.rustup.rs -sSf | sh -s -- -y
```

- Close and reopen PowerShell, then verify:

```powershell
rustc -V
cargo -V
```

If both commands print versions, Rust and Cargo are installed correctly.

## 2) Clone the Repository
Pick your target folder, then run:
```powershell
git clone https://github.com/nvckai/Discord-Quest-Notification.git 
cd Discord-Quest-Notification
```

## 3) Create and Fill the .env File
Create `.env` and fill these variables:

```env
# Discord auth token (required). Must not be empty.
DISCORD_AUTH_TOKEN=your_discord_token_here

# Webhook URL for sending notifications (required)
# Supports: discord.com, discordapp.com, ptb.discord.com, canary.discord.com
DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/xxxxxxxx/yyyyyyyy

# Polling interval in seconds (optional, default: 300, min: 30, max: 86400)
POLLING_INTERVAL_SEC=300

# Discord regions (optional, default: en-US). Comma-separated.
# Example: en-US,id,da,de,fr,ja,zh-CN
DISCORD_REGIONS=en-US

# Post previous quests on initial run (optional, default: false)
# Set to "true" to post all existing quests when app starts
PREVIOUS_QUEST=false

# Discord x-super-properties header (optional, has default value)
# See "How to Get SUPER_PROPERTIES" section below for instructions
SUPER_PROPERTIES=ewogICJvcyI6ICJXaW5kb3dzIi...
```

### How to Get DISCORD_AUTH_TOKEN
1. Open Discord in your web browser
2. Press `F12` to open Developer Tools
3. Go to the **Console** tab
4. Type this and press Enter:
   ```javascript
   (webpackChunkdiscord_app.push([[''],{},e=>{m=[];for(let c in e.c)m.push(e.c[c])}]),m).find(m=>m?.exports?.default?.getToken!==void 0).exports.default.getToken()
   ```
5. Copy the token (without quotes)

### How to Get SUPER_PROPERTIES
1. Open Discord in your web browser
2. Press `F12` to open Developer Tools
3. Go to the **Network** tab
4. Refresh the page or click on any channel
5. Look for a request to `discord.com/api/v9/` in the Network tab
6. Click on that request
7. Go to the **Headers** section
8. Find `x-super-properties` in the Request Headers
9. Copy the entire value (it's a long base64 encoded string)

**Note:** If you don't set `SUPER_PROPERTIES`, the app will use a default value that should work in most cases.

### Configuration Notes:
- `DISCORD_AUTH_TOKEN` and `DISCORD_WEBHOOK_URL` must be valid.
- `POLLING_INTERVAL_SEC` controls how often the app checks quests (minimum 30 seconds to avoid rate limiting).
- `DISCORD_REGIONS` can contain one or more regions. If empty, `en-US` is used.
- `PREVIOUS_QUEST=true` will post all existing quests on startup (useful for initial setup or testing).
- `SUPER_PROPERTIES` is optional but recommended for best compatibility.

## 4) Run the App
From the project folder, run:
```powershell
cargo run
```

To stop the app, press `Ctrl+C`. The app will perform a graceful shutdown.

## Project Structure (Quick)
- `src/main.rs`: Main loop and processing.
- `src/shutdown.rs`: Signal handling for graceful shutdown.
- `src/config/mod.rs`: Loads configuration from `.env`.
- `src/handlers/*`: Quest checking/processing logic.
- `src/communication/*`: Communication integration (e.g., Discord).
- `Cargo.toml`: Project metadata and dependencies.

Created with ❤️ by **Ph1on** 🌸

> [!NOTE]  
> Join KH1EV Community : https://discord.gg/kh1ev
