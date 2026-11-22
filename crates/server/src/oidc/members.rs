//! The bot asking Discord who is on the server's Discord server, on a thread
//! of its own so the game never waits: the game asks by Discord user id
//! (`Logins::ask_member`) and takes the answers each tick
//! (`Logins::member_answers`). Rate limits are waited out a few times, then
//! answered as `Unknown`. Ids are never logged.

use std::collections::BTreeSet;
use std::sync::Mutex;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use super::http::Http;
use super::member::{MemberSettings, Membership, Read, read};

/// How many times one question waits out a rate limit before giving up.
const TRIES: usize = 4;

/// The longest a rate limit is waited out at once.
const MOST_WAIT: Duration = Duration::from_secs(10);

/// Questions for the bot's thread, and its answers.
pub(super) struct Members {
    asks: Mutex<Sender<u64>>,
    answers: Mutex<Receiver<(u64, Membership)>>,
}

impl Members {
    /// Starts the bot's thread, asking Discord at `site` (`DISCORD_SITE`, or a
    /// fake one in tests). It ends when this is dropped.
    pub(super) fn start(http: Http, site: String, settings: MemberSettings) -> Self {
        let (asks, asked) = mpsc::channel::<u64>();
        let (answer, answers) = mpsc::channel();
        thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("a tokio runtime");
            while let Ok(first) = asked.recv() {
                // Each user once, however often they were asked meanwhile.
                let users: BTreeSet<u64> = std::iter::once(first).chain(asked.try_iter()).collect();
                for user in users {
                    let found = runtime.block_on(ask(&http, &site, &settings, user));
                    if answer.send((user, found)).is_err() {
                        return;
                    }
                }
            }
        });
        Self {
            asks: Mutex::new(asks),
            answers: Mutex::new(answers),
        }
    }

    pub(super) fn ask(&self, user: u64) {
        // The thread lives as long as this.
        let _ = self.asks.lock().expect("never poisoned").send(user);
    }

    pub(super) fn answers(&self) -> Vec<(u64, Membership)> {
        let answers = self.answers.lock().expect("never poisoned");
        answers.try_iter().collect()
    }
}

/// Asks Discord whether `user` is on the server, waiting out rate limits.
async fn ask(http: &Http, site: &str, settings: &MemberSettings, user: u64) -> Membership {
    let site = site.trim_end_matches('/');
    let url = format!("{site}/api/v10/guilds/{}/members/{user}", settings.server);
    let authorization = format!("Bot {}", settings.bot_token);
    for _ in 0..TRIES {
        match http.get_status(&url, &authorization).await {
            Ok((status, body)) => match read(status, &body) {
                Read::Done(found) => return found,
                Read::Wait(millis) => {
                    tokio::time::sleep(Duration::from_millis(millis).min(MOST_WAIT)).await;
                }
            },
            Err(error) => return Membership::Unknown(format!("Discord: {error}")),
        }
    }
    Membership::Unknown("Discord kept asking the bot to wait".into())
}
