# Hall of Worlds

A multiplayer roguelike with an infinite, procedurally generated world.

- <https://hallofworlds.com>: landing page and email (not live yet)
- <https://hallofworlds.app>: the game, running the Docker image
- <https://hallofworlds.wiki>: game documentation, from [`docs/`](docs/)

## Development

The game is written in [Rust](https://www.rust-lang.org/) using the [Bevy](https://bevy.org/) game engine. It has two parts:

- **Client** (`hall-of-worlds`): the game itself, as a native app (Windows, macOS, Linux) or WebAssembly in the browser. Native clients include the server for single player; the browser client only connects to a server.
- **Server** (`hall-of-worlds-server`): the authoritative game server, native only. Players connect over WebSockets, on the same port that serves the web client. The Docker image runs it.

### One-time setup (Windows)

1. Install the Visual Studio C++ Build Tools (with the "Desktop development with C++" workload): <https://visualstudio.microsoft.com/visual-cpp-build-tools/>
2. Install Rust: `winget install Rustlang.Rustup` (or <https://rustup.rs/>), then open a new terminal.
3. Install Trunk, the web build tool: `cargo install --locked trunk`
4. Install Visual Studio Code <https://code.visualstudio.com/> and accept the recommended extensions when you open this folder.
5. Turn on the pre-commit lint check: `cargo xtask hooks`

`rust-toolchain.toml` makes Rust download the correct compiler and WebAssembly target automatically on the first build.

On Linux you also need some system libraries, see <https://bevy.org/learn/quick-start/getting-started/setup/>.

### Running

| What                      | Command                 | VSCode                                     |
| ------------------------- | ----------------------- | ------------------------------------------ |
| Run natively              | `cargo run`             | Task "Run (native)", or F5 to debug        |
| Run the standalone server | `cargo run -p server`   |                                            |
| Run in a browser          | `trunk serve`           | Task "Serve (web)", then <http://localhost:3000/web/> |
| Release build (native)    | `cargo build --release` | Task "Release build (native)"             |
| Release build (web)       | `trunk build --release` | Task "Release build (web)", output in `dist/` |
| Test (before merging)     | `cargo xtask test`      | Task "Test"                               |
| Lint                      | `cargo clippy`          | Shown automatically in the editor         |
| Format                    | `cargo fmt`             | Automatic on save                         |

The first build downloads and compiles Bevy, which takes several minutes. Later builds are much faster.

`cargo xtask test` checks formatting, type-checks/lints the native and web builds with warnings treated as errors, and runs the tests. Run it before merging: GitHub's "Check" workflow only runs `cargo check --release` for now.

Source code is in `crates/`: `shared` holds game rules and world generation used by both sides, `client` is the game app, `server` is the game server. Images/sounds/fonts go in `crates/client/assets/`. In the game, click the map or use the arrow keys to move (see [Controls](docs/controls.md)). Build output goes in `target/` (native) and `dist/` (web), both local only.

### Docker

The `Dockerfile` builds an x86-64 image with the game server, which serves the web client and the game (WebSockets) securely on port 24691 with a self-signed certificate (or your own), and saves in `/data`. To use a certificate from Let's Encrypt, add `-e HOW_LETS_ENCRYPT_DOMAIN=game.example.com` (which accepts its terms) and publish `-p 443:24691`: the domain's public port 443 must reach this server. Behind a reverse proxy, add `-e HOW_BEHIND_PROXY=true`, which opens port 24690 for it as well (and `-e HOW_DISABLE_SSL=true` if 24691 shouldn't open at all):

```bash
docker build -t hall-of-worlds .
docker run -p 24691:24691 -p 24690:24690 -v hall-of-worlds:/data hall-of-worlds
```

It works behind an HTTPS proxy (such as Cloudflare) that passes WebSockets: nothing but that one port is needed.

Every merge to `master` publishes it as `ghcr.io/reigningcoo/hall-of-worlds:latest`.

## Credits

Claude Code (Opus 5.5) for code and placeholder 3D images. This game would not have been possible without it.

## Licence

Hall of Worlds is licensed under [Creative Commons Attribution-NonCommercial 4.0 International](https://creativecommons.org/licenses/by-nc/4.0/) (CC BY-NC 4.0): you may share and adapt it with attribution, but not for commercial purposes. See [LICENSE](LICENSE). Third-party dependencies keep their own licences.

Some artwork is by other people and used under its own licence, credited in [crates/client/assets/ATTRIBUTION.md](crates/client/assets/ATTRIBUTION.md): the monster icons are by David E. Gervais ([tiles library](http://pousse.rapiere.free.fr/tome/), CC BY 3.0).
