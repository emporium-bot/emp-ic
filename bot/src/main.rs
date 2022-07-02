use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::{process::Command, str};

struct Handler;

const HELP_TEXT: &str = "
```
.help - Show this message
.ping - Ping the bot
.daily - Get a daily reward
.work - Work for money
.balance <optional user> - get your current balances, or someone else's
.register - Prints dfx command for registering your discord account
```
";

const REGISTER_TEXT: &str = "```
1. Add the emporium (au7z2-aaaaa-aaaah-abk7a-cai) canister to plug
2. Export/Import your plug identity into dfx
3. Run the following command in your terminal:
```";

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot == false {
            if msg.content == ".ping" {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                    println!("Error sending message: {:?}", why);
                }
            } else if msg.content == ".work" {
                let user = format!("{}", msg.author.id);
                println!("{} work", user);

                let result = Command::new("bash")
          .arg("-c")
          .arg(format!(
            "cd ../ic && dfx canister --network ic call au7z2-aaaaa-aaaah-abk7a-cai work '(\"{}\")'",
            user
          ))
          .output()
          .expect("failed to execute process");
                let response = match str::from_utf8(&result.stdout) {
                    Ok(r) => r,
                    _ => "error",
                };
                let re = regex::Regex::new(r#"(")[^"]+(")"#).expect("failed to compile regex");
                let formatted_resp = re.find(response).map(|x| x.as_str()).unwrap_or("");
                println!("{}\n", formatted_resp);

                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, formatted_resp.replace(r#"""#, ""))
                    .await
                {
                    println!("Error sending message: {:?}", e);
                }
            } else if msg.content == ".daily" {
                let user = format!("{}", msg.author.id);
                println!("{} daily", user);

                let result = Command::new("bash")
          .arg("-c")
          .arg(format!(
            "cd ../ic && dfx canister --network ic call au7z2-aaaaa-aaaah-abk7a-cai daily '(\"{}\")'",
            user
          ))
          .output()
          .expect("failed to execute process");
                let response = match str::from_utf8(&result.stdout) {
                    Ok(r) => r,
                    _ => "error",
                };
                let re = regex::Regex::new(r#"(")[^"]+(")"#).expect("failed to compile regex");
                let formatted_resp = re.find(response).map(|x| x.as_str()).unwrap_or("");
                println!("{}", formatted_resp);

                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, formatted_resp.replace(r#"""#, ""))
                    .await
                {
                    println!("Error sending message: {:?}", e);
                }
            } else if msg.content.starts_with(".balance") {
                let args: Vec<&str> = msg.content.split_whitespace().collect();
                let user: String;
                if args.len() == 1 {
                    user = format!("{}", msg.author.id);
                } else {
                    user = args[1].to_string().replace("<@", "").replace(">", "");
                }
                println!(".user {}", user);

                let result = Command::new("bash")
          .arg("-c")
          .arg(format!(
            "cd ../ic && dfx canister --network ic call au7z2-aaaaa-aaaah-abk7a-cai user_balance '(\"{}\")'",
            user
          ))
          .output()
          .expect("failed to execute process");
                let response = match str::from_utf8(&result.stdout) {
                    Ok(r) => r,
                    _ => "error",
                };

                let re = regex::Regex::new(r#"\{[^\{\}]+\}"#).expect("failed to compile regex");
                let formatted_resp = re
                    .find(response)
                    .map(|x| x.as_str())
                    .unwrap_or("")
                    .replace(" : nat", "")
                    .replace(r#"""#, "")
                    .replace(";", "")
                    .replace("{", "")
                    .replace("}", "");
                println!("{}", formatted_resp);

                if let Err(e) = msg.channel_id.say(&ctx.http, formatted_resp).await {
                    println!("Error sending message: {:?}", e);
                }
            } else if msg.content == ".help" {
                if let Err(e) = msg.channel_id.say(&ctx.http, HELP_TEXT).await {
                    println!("Error sending message: {:?}", e);
                }
            } else if msg.content == ".register" {
                let response = format!(
                "{}```dfx canister --network ic call au7z2-aaaaa-aaaah-abk7a-cai register '(\"{}\")'```",
                REGISTER_TEXT,
                msg.author.id
              );

                match msg
                    .author
                    .direct_message(&ctx.http, |m| m.content(&response))
                    .await
                {
                    Ok(_) => {
                        let _ = msg.react(&ctx.http, '👌');
                    }
                    Err(why) => {
                        println!("Err sending help: {:?}", why);

                        let _ = msg.reply(
                            &ctx.http,
                            "There was an error DMing you the registration instructions",
                        );
                    }
                };
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
