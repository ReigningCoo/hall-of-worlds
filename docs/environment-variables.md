---
title: Server environment variables
nav_order: 2
parent: Server
---

# Server environment variables

A server is set up entirely by environment variables, and none of them are required. Yes-or-no options take `true` or `false`.

## Network

| Variable | Default | What it does |
| -------- | ------- | ------------ |
| `HOW_ADDRESS` | `0.0.0.0:24690` | The plain port's address. It only answers that connections must be secure, unless `HOW_BEHIND_PROXY` is on. |
| `HOW_TLS_ADDRESS` | `0.0.0.0:24691` | The secure port's address, for HTTPS and `wss://`. |
| `HOW_BEHIND_PROXY` | `false` | `true` also serves everything on the plain port, for a reverse proxy to reach. The secure port stays open, so the proxy can use either. |
| `HOW_DISABLE_SSL` | `false` | `true` doesn't open the secure port or set up a certificate. The server refuses to start unless `HOW_BEHIND_PROXY` is on too, and if a certificate of your own or Let's Encrypt is set. |
| `HOW_UPNP` | `false` | `true` asks this network's router to forward the secure port to the server (UPnP) while it runs, and says in the log how that went. With `HOW_LETS_ENCRYPT_DOMAIN` it forwards the router's port 443, which Let's Encrypt needs. Ignored, with a warning, when `HOW_BEHIND_PROXY` is on. In Docker it needs host networking. In the native client, if set, it decides instead of Server Options' switch. |
| `HOW_WEB` | none (`/srv/web` in Docker) | The folder holding the web client. Without it, only the game itself is served. The last few builds it served are kept in the data folder, so browsers that have one download only what changed. |

## Certificates

Without any of these, the server makes a self-signed certificate and keeps it in the data folder.

| Variable | Default | What it does |
| -------- | ------- | ------------ |
| `HOW_TLS_NAMES` | none | More names and addresses, separated by commas, for a certificate the server makes itself. For example `game.example.com,192.168.0.10`. |
| `HOW_TLS_CERTIFICATE` | none | The path of your own PEM certificate. Needs `HOW_TLS_KEY` too. |
| `HOW_TLS_KEY` | none | The path of your own PEM private key. Needs `HOW_TLS_CERTIFICATE` too. |
| `HOW_LETS_ENCRYPT_DOMAIN` | none | Get a certificate for this domain from Let's Encrypt, which accepts its terms. The domain's public port 443 must reach the secure port. Can't be used with your own certificate. |
| `HOW_LETS_ENCRYPT_EMAIL` | none | Where Let's Encrypt can write to you. Needs `HOW_LETS_ENCRYPT_DOMAIN`. |

## Game and saves

| Variable | Default | What it does |
| -------- | ------- | ------------ |
| `HOW_DATA` | `Hall of Worlds/server` in the user's data folder (`/data` in Docker) | Where to save. See [Saves and data](saves). |
| `HOW_SERVER_NAME` | `Hall of Worlds` | What the server calls itself, for instance when telling players it is going offline. Players who join through Connect see it on their startup menu; with the default, they see the server's address instead. |
| `HOW_PLAYER_LIMIT` | `0` | The most players at once; anyone more is told the server is full. `0` is no limit. |
| `HOW_DIFFICULTIES` | all | Which [difficulties](difficulty.md) characters may play on, as names separated by commas: `Easy`, `Normal`, `Hard`, `Ironman` (in any case). Blank or unset allows all of them. Unknown names are warned about in the log and ignored; if none is known, all are allowed. Characters on one that isn't allowed are moved to the nearest that is. In the native client, if set, it decides instead of Server Options' tickboxes. |
| `HOW_LOCKDOWN` | `false` | `true` starts the server [locked down](hosting/#roles): only staff whose role may change Server Options (Owners and Admins by default) can play. With `false`, it starts as it was left: staff turn lockdown on and off in the game, and the server remembers it. |
| `HOW_IDLE_MINUTES` | `15` | How many minutes a player may do nothing before being disconnected. `0` never disconnects anyone. |
| `HOW_STACK_GRAMS_PER_CELL` | `2000` | The most a stack of identical items may weigh, in grams, for each cell it takes up in a bag. |
| `HOW_STACK_MAX` | `99` | The most items in one stack, however light they are. |
| `HOW_MODS` | `mods` in `HOW_DATA` | The folder of this server's mods. Herbs added or changed go in `herbs/` there, as `.ron` files each holding a list of herbs written like the game's own; a herb with the same key as one of the game's replaces it. They are read when the server starts, and players are sent them as they join. How creatures behave can be changed too: `.ron` files in `behaviours/` there, each a list of races' behaviours written like the game's own (`crates/shared/src/behaviour/behaviours.ron`), an entry replacing the game's for its race; these stay on the server. A file with a mistake is left out, with the reason in the log. |

## Accounts and logging in

Players can log in through a provider's website, such as Google, or through Discord, if you set one up: see [Logging in with Google](hosting/#logging-in-with-google) and [Logging in with Discord](hosting/#logging-in-with-discord). The server never keeps anyone's email address, only a key made from it with the login secret, and logs players by their account's id.

| Variable | Default | What it does |
| -------- | ------- | ------------ |
| `HOW_GUESTS` | `true` | Whether people can play without an account. `false` lets in only those logging in through a provider; if no provider can be used, guests are let in anyway, with a warning in the log. |
| `HOW_GUEST_MODE` | `false` | `true` treats everyone as a guest, as if no provider were set up, whatever the variables below say. Handy for trying things out. |
| `HOW_OIDC_<NAME>_CLIENT_ID` | none | The client id a provider gave you, where `<NAME>` is any name you choose for it, such as `GOOGLE`. With its secret, players can log in through that provider. |
| `HOW_OIDC_<NAME>_CLIENT_SECRET` | none | The client secret that provider gave you. Keep it to yourself. |
| `HOW_OIDC_<NAME>_CLIENT_SECRET_FILE` | none | A file holding the client secret instead, such as a Docker secret. If set, it is used in place of `HOW_OIDC_<NAME>_CLIENT_SECRET`. If the file can't be read, that provider is left out. |
| `HOW_OIDC_<NAME>_ISSUER` | Google's for `GOOGLE` | The provider's address, where its OpenID Connect settings are found (`<issuer>/.well-known/openid-configuration`). Any provider that supports OpenID Connect works. It must be `https://`, except on the server's own machine. |
| `HOW_OIDC_<NAME>_LABEL` | `Google` for `GOOGLE`, otherwise the name | What players see the provider called. |
| `HOW_DISCORD_CLIENT_ID` | none | The client id of your Discord application. With its secret, players can log in through Discord. |
| `HOW_DISCORD_CLIENT_SECRET` | none | Your Discord application's client secret. Keep it to yourself. |
| `HOW_DISCORD_CLIENT_SECRET_FILE` | none | A file holding the Discord client secret instead, such as a Docker secret. If set, it is used in place of `HOW_DISCORD_CLIENT_SECRET`. If the file can't be read, Discord logins are off. |
| `HOW_DISCORD_INVITE` | none | An invitation link to your Discord server, such as `https://discord.gg/...`. Players see a Join our Discord button beside Log in with Discord, which opens it in their browser. It must be `https://`. |
| `HOW_DISCORD_ONLY` | `false` | `true` lets players in only through Discord: no other provider is offered and no guests, whatever `HOW_GUESTS` says, and logins kept from another provider must log in again through Discord. If Discord isn't set up, the server doesn't start. |
| `HOW_DISCORD_SERVER_ID` | none | The id of your Discord server (right-click it in Discord with Developer Mode on, then Copy Server ID). Only its members can play: no guests, and anyone who logged in another way must link Discord first. Needs Discord logins and `HOW_DISCORD_BOT_TOKEN`, or the server doesn't start. See [Only for your Discord's members](hosting/#only-for-your-discords-members). |
| `HOW_DISCORD_BOT_TOKEN` | none | The token of your Discord application's bot, which checks who is on your Discord server. Keep it to yourself. |
| `HOW_DISCORD_BOT_TOKEN_FILE` | none | A file holding the bot's token instead, such as a Docker secret. If set, it is used in place of `HOW_DISCORD_BOT_TOKEN`. If the file can't be read, the server doesn't start. |
| `HOW_LOGIN_URL` | `https://` and `HOW_LETS_ENCRYPT_DOMAIN`, if that is set | The address players reach the server at, such as `https://game.example.com`. Providers send players back to `/login/callback` there. Without it, providers can't be used. |
| `HOW_LOGIN_SECRET` | made on the first run | What email addresses and Discord ids are made into keys with. Servers that share players must all have the same one. Without it, the server makes one the first time and keeps it in its data folder; once kept, a different one stops the server from starting, as it would lose every account. |
| `HOW_LOGIN_SECRET_FILE` | none | A file holding the login secret instead, such as a Docker secret. If set, it is used in place of `HOW_LOGIN_SECRET`. If the file can't be read, the server doesn't start. |
| `HOW_STATS_SECRET` | made on the first run | The key `/_stats` asks for (see [Hosting](hosting/#stats-for-a-dashboard)). Without it, the server makes one the first time and keeps it in its data folder, in `stats-key.txt`. |
| `HOW_STATS_SECRET_FILE` | none | A file holding the stats key instead, such as a Docker secret. If set, it is used in place of `HOW_STATS_SECRET`. If the file can't be read, the server doesn't start. |

A provider that is set up wrongly, or can't be reached when the server starts, is left out, with a warning in the log saying why.

Every secret can be given in a file instead, by adding `_FILE` to its name. The file's contents are used, without the spaces or line ends round them, and the file wins if both are set.

## Logging

| Variable | Default | What it does |
| -------- | ------- | ------------ |
| `LOG_LEVEL` | `info` | How much to log: `error`, `warn`, `info`, `debug` or `trace`. `debug` adds a line a second for each connection's round trip, for looking into network trouble. |
| `RUST_LOG` | none | Filters the log by crate, overriding `LOG_LEVEL`. For example `server=debug,info`. |

## Docker image

Only the [Docker image](hosting/#user-and-group) reads these, before it starts the server.

| Variable | Default | What it does |
| -------- | ------- | ------------ |
| `PUID` | `1000` | The user the server runs as. The container gives it anything in `/data` that isn't its own first. Ignored when the container is started as another user (Docker's `user:`). |
| `PGID` | `1000` | The group the server runs as, and gives `/data` to, as for `PUID`. |

## Debugging

A real server can ignore these.

| Variable | Default | What it does |
| -------- | ------- | ------------ |
| `HOW_DEBUG_TOOLS` | `false` | `true` lets **anyone connected** use the [debug menu](debug-menu), every tool but World Time and Freeze Mobs, for private test servers. Without it, only accounts whose [role](hosting/#roles) allows each tool can. |
| `HOW_DEBUG_API` | `false` | Only in development builds made with `--features debug-api`, never in releases or the Docker image. `true` starts the debug API on port 15702, or give a port number; it only listens on this computer (127.0.0.1). The native client's own single-player server takes the next port. If the port is taken, the game logs it and starts without the API. |
| `HOW_DEBUG_GENERATOR` | none | `Desert`, `Continents` or `Plates` makes every world's land that way instead of its own, to try one out. A client must have the same setting to play there, as the native client's own server does. |
| `HOW_DEBUG_INN_EXIT` | none | A tile of the Hall of Worlds' walkway, `x` or `x,y` (`y` its row, 0 to 2), where every new character first comes out of [The Way Inn](the-way-inn), in place of a place picked for each, to test leaving from somewhere known. |
| `HOW_DEBUG_MEMLOG` | `false` | `true` logs the web client's memory log: a page opened with `?memlog` at the end of its address (`/web/?memlog`) sends a short line a second of how much memory the game is using, which the server logs as `memlog` lines. For finding out why a browser (such as Safari on an iPad) closes the game. Nothing about the player is in it. |
| `HOW_DEBUG_LETS_ENCRYPT_STAGING` | `false` | `true` uses Let's Encrypt's test service, whose certificates nobody trusts, to try things without its rate limits. |

## For the native client

The native client reads `HOW_DATA` too, for its single-player saves, `HOW_UPNP`, for its own server when it goes online, and `HOW_DIFFICULTIES`, for its own server's difficulties, as well as these. They have no effect on a server.

| Variable | Default | What it does |
| -------- | ------- | ------------ |
| `HOW_SERVER` | none | A server for Play to join, such as `wss://game.example.com:24691/ws`, until logging out; a server chosen in Connect comes first. Without either, Play is single player. |
| `HOW_SERVER_KEY` | none | The key fingerprint `HOW_SERVER` must have, from its log. Without it, any certificate is accepted. See [Connecting a native client](hosting/#connecting-a-native-client). |
| `HOW_PREFERENCES` | `Hall of Worlds/client/client.ron` in the user's data folder | The file keeping the player's choices between runs. |
| `HOW_SOUND` | `true` (`false` in development builds) | `false` plays no sound at all this run, whatever the volumes in Settings, Audio, which shows that. |
| `HOW_MUSIC` | `true` | `false` plays no music this run, while other sounds still play. |
