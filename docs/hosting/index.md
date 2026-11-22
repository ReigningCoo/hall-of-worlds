---
title: Hosting a server
nav_order: 1
parent: Server
permalink: /hosting/
---

# Hosting a server

A server runs every world its players are in, and serves both the game and the web client, so one port is all it needs. The published Docker image is the simplest way to run one. It is x86-64 only for now.

Every setting is listed under [Server environment variables](../environment-variables).

## Docker Compose, with a self-signed certificate

Save this as [`docker-compose.yml`](docker-compose.yml):

```yaml
{% include_relative docker-compose.yml %}
```

Then start it:

```bash
docker compose up -d
```

Open `https://<your server>:24691` to play: it goes to the web client, at `/web/`. The server makes its own certificate on first starting, and keeps it in the volume with the saves. No browser trusts a certificate like that, so each player has to accept a warning once.

For a certificate that browsers trust without a warning, see the other options below.

To update, which may wipe saves while the game is at this stage (see [Saves and data](../saves)):

```bash
docker compose pull
```

```bash
docker compose up -d
```

Before 7 November 2026 the ports were 8080 and 8081. If you update from then, change both sides of `ports` in your `docker-compose.yml` (and anything in front of the server, such as a proxy's or a tunnel's origin) to 24690 and 24691, or set `HOW_ADDRESS` and `HOW_TLS_ADDRESS` to keep the old ones.

The `latest` image changes every time something is added to the game, which can be several times a day. For something steadier, use `nightly` in its place: it changes at most once a day. `nightly-<date>` (such as `nightly-2026-11-06`) keeps a particular day's. Players whose browsers last played on any of the past week's nightly images download only what has changed in the web client, not all of it.

## Other ways to secure it

Change the `environment` and `ports` of the example:

| Setup | Environment | Ports |
| ----- | ----------- | ----- |
| Let's Encrypt | `HOW_LETS_ENCRYPT_DOMAIN: game.example.com`, which accepts Let's Encrypt's terms | `"443:24691"` |
| Your own certificate | `HOW_TLS_CERTIFICATE` and `HOW_TLS_KEY`, the paths of your PEM files as the container sees them, which must be readable by its [user](#user-and-group) | `"24691:24691"` |
| Behind a reverse proxy, unencrypted between it and this server | `HOW_BEHIND_PROXY: "true"` and `HOW_DISABLE_SSL: "true"` | `"24690:24690"` alone |
| Behind a reverse proxy, encrypted between it and this server | Any certificate above or the self-signed one, with `HOW_BEHIND_PROXY: "true"` if it should be able to use port 24690 too | `"24691:24691"`, and `"24690:24690"` if used |

- **Let's Encrypt:** the domain's public port 443 must reach this server, directly rather than through a proxy. The self-signed certificate is used until the new one arrives, with no restart needed.
- **Your own certificate:** replaced files are picked up within an hour, with no restart needed.
- **A reverse proxy:** it must pass WebSockets. Cloudflare's does. The web client's files come already compressed, and say how long they may be kept, so a proxy that caches needs no rules of its own for them. After an update, browsers that have an earlier build download only what changed, if the proxy passes compression dictionaries through (Cloudflare can).

These can't be mixed: settings that disagree stop the server from starting, and it says why.

With none of these behind a proxy, `HOW_UPNP: "true"` asks your router to forward the port for you (UPnP), if it allows that: 443 with Let's Encrypt, otherwise 24691. The log says whether it worked. In Docker this only works with `network_mode: host` in place of `ports`, as the router's answers can't reach a container on Docker's own network. Behind a proxy it is ignored, with a warning, as the proxy's port is the one to forward.

## User and group

The server runs as user and group 1000 in the container, which is usually the first user on a Linux computer, so a folder of yours mounted at `/data` just works. To run it as someone else, there are two ways:

- **`PUID` and `PGID`:** set these in `environment` to the user and group numbers you want (`id` on Linux shows yours). Each time the container starts, it first gives that user anything in `/data` that isn't theirs already, then starts the server as them. Saves from an older image are put right the same way, with nothing for you to do.
- **Docker's `user:`:** set `user: "1001:1001"` on the service (or `--user` with `docker run`), and the container never runs anything as root. It then leaves `/data` alone, so it must already belong to that user (`chown -R 1001:1001` on the folder). If it can't be written, the server doesn't start, and its log says why.

Files the server only reads, such as your own certificate or a secret's file, must be readable by the same user.

## Connecting a native client

Players choose **Connect...** on the startup menu and type the server's address, such as `game.example.com` or `game.example.com:24691`. Without a port, the client tries 24691 and then 443, so a server on either needs no port typed; it asks `/_version` to tell a Hall of Worlds server from any other web server. The client remembers the servers it has joined, in a list to pick from in Connect...; Continue on the startup menu is only ever for the player's own single-player game.

The client accepts any certificate the server has, so you can change yours whenever you like. A player who wants to be sure they've reached your server, and not an impostor, can paste its certificate (from `https://<your server>:24691/certificate`) or its key's fingerprint into the padlock beside the address. From then on, only a server with that key is accepted, and the client says plainly why if it isn't. The fingerprint is in the server's log when it starts:

```text
Listening securely on 0.0.0.0:24691: key fingerprint ...
```

If you change your key, tell players who pasted the old one, so they can paste the new one.

For testing, `HOW_SERVER` starts the client on a server straight away, using `wss://` and the path `/ws`, and `HOW_SERVER_KEY` gives the key it must have:

```bash
HOW_SERVER=wss://game.example.com:24691/ws hall-of-worlds
```

## Logging in with Google

Players can play as guests, or log in with an account through a provider's website. Google is the one we use; any provider that supports OpenID Connect works the same way.

1. In the Google Cloud console, under **APIs & Services**, set up the OAuth consent screen, then make an **OAuth client ID** of the type **Web application**.
2. Add `https://<your server>/login/callback` as an **authorised redirect URI**, using the address players reach the server at (with its port, if it isn't 443).
3. Give the server the client id and secret, and that address:

```yaml
    environment:
      HOW_OIDC_GOOGLE_CLIENT_ID: "<client id>"
      HOW_OIDC_GOOGLE_CLIENT_SECRET: "<client secret>"
      HOW_LOGIN_URL: "https://game.example.com"
```

With Let's Encrypt, `HOW_LOGIN_URL` can be left out: the domain is used.

To keep the client secret out of the Compose file, give it as a Docker secret and name its file with `HOW_OIDC_GOOGLE_CLIENT_SECRET_FILE`. `HOW_LOGIN_SECRET_FILE` does the same for the login secret.

```yaml
services:
  hall-of-worlds:
    environment:
      HOW_OIDC_GOOGLE_CLIENT_ID: "<client id>"
      HOW_OIDC_GOOGLE_CLIENT_SECRET_FILE: /run/secrets/google_client_secret
    secrets:
      - google_client_secret

secrets:
  google_client_secret:
    file: ./google_client_secret.txt
```

The file must be readable by the server's [user](#user-and-group) (`chmod 644` on it does that). If the service has `read_only: true`, Compose only allows secrets from a `file:`, as above, as it can't write `environment:` ones into the container; set `HOW_OIDC_GOOGLE_CLIENT_SECRET` itself instead if you'd rather keep them in the environment.

The server only asks Google for the player's email address, and only one Google has checked. It keeps a key made from the address with a secret of its own, never the address itself, and forgets everything else Google says. The secret is made on the first run and kept in the data folder (`login-secret.txt`); servers that share players must be given the same one with `HOW_LOGIN_SECRET`.

If the provider can't be reached when the server starts, or a setting is missing, the server says so in its log and lets players in as guests only. `HOW_GUEST_MODE: "true"` does that on purpose.

Players then choose Log in with Google or Play as Guest when they join. The page opens in their own browser, and once they have logged in there they go back to the game. Their game remembers the login, so they only see that page again after 90 days without playing, or after Change User on Character Select. Logins only go over a secure connection, so a server reached through an unsecured port offers guests only.

## Logging in with Discord

Players can also log in with their Discord account, beside Google or instead of it.

1. In the [Discord Developer Portal](https://discord.com/developers/applications), make a **New Application**.
2. Under **OAuth2**, add `https://<your server>/login/callback` as a **redirect**, as for Google, and copy the **client id** and **client secret** (Reset Secret shows a new one).
3. Give the server the client id and secret, and the address players reach it at:

```yaml
    environment:
      HOW_DISCORD_CLIENT_ID: "<client id>"
      HOW_DISCORD_CLIENT_SECRET: "<client secret>"
      HOW_DISCORD_INVITE: "https://discord.gg/<your invite>"
      HOW_LOGIN_URL: "https://game.example.com"
```

`HOW_DISCORD_INVITE` is optional: with it, players see a Join our Discord button beside Log in with Discord. Make the invitation one that never runs out. `HOW_DISCORD_CLIENT_SECRET_FILE` takes the secret from a file, as for Google.

The server only asks Discord who the player is (the `identify` scope), never for their email address. It keeps their Discord id on their account, and a key made from it with the login secret, so the same Discord account always reaches the same game account on servers sharing that secret. In the game's menus, players see their Discord name where others see their email address.

Players who logged in another way (with Google, say) can press **Link Discord** on their Characters screen and log in with Discord there. After that, either login opens the same account and characters. Linking is refused if that Discord account already has a game account of its own on the server: accounts aren't merged.

`HOW_DISCORD_ONLY: "true"` makes Discord the only way in: no other provider is offered, guests aren't let in, and anyone who logged in another way before has to log in again through Discord. Players who linked Discord keep their accounts, so once your players have linked, you can turn Google (or any other provider) off without anyone losing their characters.

### Only for your Discord's members

You can keep the server for the members of your Discord server, and give them roles in the game from their roles on Discord.

1. In the Developer Portal, open your application's **Bot** page and copy its **token** (Reset Token shows a new one). It needs no privileged intents.
2. Under **OAuth2**, **URL Generator**, tick **bot** (no permissions are needed), open the address it makes and add the bot to your Discord server.
3. With **Developer Mode** on in Discord's settings (Advanced), right-click your server and **Copy Server ID**.
4. Give the server both, beside the Discord settings above:

```yaml
    environment:
      HOW_DISCORD_SERVER_ID: "<server id>"
      HOW_DISCORD_BOT_TOKEN: "<bot token>"
```

`HOW_DISCORD_BOT_TOKEN_FILE` takes the token from a file instead. Then:

- **Who gets in:** the bot asks Discord whether a player is on your Discord server when they log in, when a kept login is used, and every 5 minutes while they play. Anyone not on it is turned away, or sent away, and told to join it and log in again. Guests aren't let in, and players who logged in another way see **Link Discord** on their Characters screen and must link before they can play.
- **When Discord can't be reached:** players are let in as they are, and checked again 5 minutes later. If Discord refuses the bot (a wrong token, or the bot not on that server), nobody new is let in until you put it right, and the server's log says why.
- **Roles from Discord:** see [Roles](#roles).

## Roles

Every account has a role, which decides what it may do beyond playing. A role can only be given by someone whose own role allows it, and never to yourself.

| Role | What it may do |
| ---- | -------------- |
| Owner | Everything, always, including lockdown. A native client's own world is its player's. |
| Admin | Everything except making Owners and Admins, including lockdown. |
| Developer | Every tool of the [debug menu](../debug-menu) except World Time and Freeze Mobs, and Debug Info's Load (going to where someone's debug string was written), and nothing of moderation or accounts. |
| Moderator | Moderating players (Moderated and Banned), finding characters by name and reading the audit log; let in when the server is full. The debug menu's tools for looking after players: Show Debug Info, Teleport Mode, Teleport to World, God Mode, Truesight, Examine Mob with Kill and Destroy, Detect Mobs, and the doors' tools. |
| Player | Playing. Every account starts as one. |
| Moderated | Playing, with limits, usually for a time. |
| Banned | Nothing: turned away when logging in, told why and until when. |
| Guest | Playing. Every guest has this role, and guests can't be given another. |

**How staff are shown:** other players see a gold laurel crown before the name of a character whose role is anything but Player, Moderated, Banned or Guest, with the role's name (or title) after it, as in "Brin (Moderator)". Names can't contain anything that looks like it. Staff can choose to be shown as a role their own may appear as instead, such as Player to play unnoticed, with **Shown As** under Settings, Gameplay. It is kept with each character. By default the Owner may appear as any role, Admins as Developer, Moderator or Player, and Developers and Moderators as Player.

**Claiming your server:** a new server has no Owner. When it starts, it writes a claim code to its log and to `owner-claim.txt` in its data folder. Log in with an account (not as a guest) and make a new character with the code as its name: you become the Owner, no character is made, and the game goes back to its menu. Connect again to play. The code works once, and a new one is made each time the server starts until it is used.

**Changing roles in the game:** staff whose role allows it have **Accounts...** in the debug menu while playing, under **Admin Tools**. Its **Find** page looks a character up by name and shows their account's role, and gives it another (one your role may give, for good or for an hour, a day, a week or 30 days, with a reason), or takes it away by giving Player. You can't change your own role, a guest's, or the role of anyone holding a role you couldn't give. **Roles** renames a role or gives it a title (by default only the Owner and Admins), and **Audit Log** shows what has been changed, newest first. Someone playing when their role changes gets the new one at once, and someone banned is sent away.

**Lockdown:** a server can be locked down, so that only staff whose role may change Server Options (by default Owners and Admins) can play on it, for instance while fixing something. While playing there, they have **Online** in the system menu: ticked while everyone may play, and unticking it locks the server down, after asking. Anyone else playing is told the server is locked down and sent away, and anyone else logging in is turned away with the same words, until Online is ticked again. The server remembers it when restarted, and each change is written to `audit.log`. `HOW_LOCKDOWN` set to `true` (see [Server environment variables](../environment-variables)) starts it locked down.

**Changing roles by hand:** roles are kept in `roles.ron` in the data folder. **Only edit it while the server is stopped**, as a running server writes over it. In it you can:

- give an account a role, by its account id (from the server's log);
- rename a role, or give it a `title` that is shown instead of its name;
- change what each role may do, turning each permission on or off. Each debug menu tool is a permission of its own, written like `Debug(GiveItem)` (Debug Info's Load is `Debug(LoadDebugString)`); a file from before they were split, with one `DebugTools` permission, is turned into them the next time the server starts;
- add roles of your own, with an id of lower case letters, digits, `-` and `_`.

**Roles from Discord:** with `HOW_DISCORD_SERVER_ID` set, `roles.ron` can also give roles by players' roles on your Discord server, in `discord_roles`: a list of Discord role ids (right-click a role in your server's settings, with Developer Mode on, and **Copy Role ID**) and the role each gives here. Each time a player is checked, they get the first in the list whose Discord role they have, or Player if none matches:

```
    discord_roles: [
        (discord: 1111111111111111111, role: "admin"),
        (discord: 2222222222222222222, role: "moderator"),
    ],
```

The Owner can't be given this way. Each change is written to `audit.log` as given by Discord. A role given by hand (in the game or in the file) is never changed by Discord, and lets its player in even if they leave your Discord server, unless it is Banned.

If the file can't be read, the server won't start, so nothing is lost. Each change the server makes keeps the previous file in `roles-backups/` (the last 10). Every role given, taken or run out is written to `audit.log`, and so is every use of a debug menu tool by anyone but a native client's own player, by account and character id (just looking, such as Examine Mob's details, is left out).

## Stats for a dashboard

A standalone server answers `/_stats` with how it is doing, as JSON, for a dashboard such as [Homepage](https://gethomepage.dev/widgets/services/customapi/):

```json
{"players":{"online":3,"total":41},"memory":{"bytes":412000000},"cpu":{"percent":2.5,"cores":8},"uptime_seconds":86400}
```

`online` is players in the game now, `total` every character on the server, `memory` and `cpu` the server's own (`percent` is a share of the whole machine), and `uptime_seconds` how long it has run.

It needs a key, in an `X-API-Key` header, as `Authorization: Bearer <key>`, or as `?key=<key>`. Without the right key it answers 404, as if it weren't there. The server makes a key on its first run and keeps it in its data folder, in `stats-key.txt`, which holds the key alone (no newline), so a dashboard can read it as a secret as it is; to choose your own, set `HOW_STATS_SECRET`, or give it as a Docker secret with `HOW_STATS_SECRET_FILE` (see the [Compose file](docker-compose.yml)). Prefer a header to `?key=`: proxies and servers often log the addresses asked for, key and all.

A Homepage service with the customapi widget:

```yaml
- Hall of Worlds:
    href: https://game.example.com/web/
    widget:
      type: customapi
      url: http://hall-of-worlds:24690/_stats
      headers:
        X-API-Key: "{{HOMEPAGE_VAR_HOW_STATS_KEY}}"
      mappings:
        - field: { players: online }
          label: Online
          format: number
        - field: { players: total }
          label: Characters
          format: number
        - field: { memory: bytes }
          label: Memory
          format: bytes
        - field: { cpu: percent }
          label: CPU
          format: percent
```

Every port answers it, the plain one too, so a dashboard on the same Docker network needs no certificate.

## Things to know

- **Anyone who can reach the server can play as a guest,** unless `HOW_GUESTS` is `false` and a provider is set up. A guest gets their own character, tied to that browser or native client.
- **A networked server never pauses.** The system menu (or a player's window going into the background) doesn't stop the world for anyone, whatever the settings, and nor does a native client's own server while Online is on: see [Pausing](../client#pausing).
- **Hosting from the game itself:** a native client can open its own world to others with Online in the system menu, on one secure port (24691 unless Server Options says otherwise), with a certificate it makes. With UPnP turned on in Server Options (or `HOW_UPNP=true`), it asks your router to forward that port to the computer while it is online, and says there how that went and the address players on the internet connect to. Otherwise, or if the router has no UPnP or refuses, forward the port yourself. Windows may ask whether the game can use the network the first time: allow it, or the router's answer can't get through. If your internet provider shares one address between many homes, Server Options says so: no forwarding gets players through then. A standalone server is better for a world that is always there.
- **Time stops only when the server does.** The server's clock is saved, and carries on from there next time. Every world's time goes by it, so a world nobody is in keeps its time going too.
- **Keep `HOW_DEBUG_TOOLS` off on a public server.** It lets anyone connected use the [debug menu](../debug-menu), all but World Time and Freeze Mobs. Give the Developer role to those who need it instead.
- **Stopping:** the server saves everyone when asked to stop (`docker compose down`, Ctrl+C or SIGTERM).
- **Health checks:** every port answers `/_healthz` with `200 ok` while the game is running, and `503` if it has stopped, even the plain port that otherwise only says to use the secure one. The Docker image checks it by itself (`docker ps` shows `healthy`), asking with the server's own `hall-of-worlds-server --health`, which needs no curl or wget. `/_version` says what the server is, as JSON, such as `{"game":"Hall of Worlds","version":"0.1.0","protocol":37}`.
