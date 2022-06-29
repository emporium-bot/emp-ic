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
.user <optional user#1234> - get your current rewards status
.register - Prints dfx command for registering a discord account
```
";

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
                let user = format!("{}#{}", msg.author.name, msg.author.discriminator);
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
                println!("{}", response);
                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, format!("```{}```", response))
                    .await
                {
                    println!("Error sending message: {:?}", e);
                }
            } else if msg.content == ".daily" {
                let user = format!("{}#{}", msg.author.name, msg.author.discriminator);
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
                println!("{}", response);
                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, format!("```{}```", response))
                    .await
                {
                    println!("Error sending message: {:?}", e);
                }
            } else if msg.content.starts_with(".user") {
                let args: Vec<&str> = msg.content.split_whitespace().collect();
                let user: String;
                if args.len() == 1 {
                    user = format!("{}#{}", msg.author.name, msg.author.discriminator);
                } else {
                    user = args[1].to_string();
                }

                print!("{} user info", user);

                let result = Command::new("bash")
          .arg("-c")
          .arg(format!(
            "cd ../ic && dfx canister --network ic call au7z2-aaaaa-aaaah-abk7a-cai get_user '(\"{}\")'",
            user
          ))
          .output()
          .expect("failed to execute process");
                let response = match str::from_utf8(&result.stdout) {
                    Ok(r) => r,
                    _ => "error",
                };
                println!("{}", response);
                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, format!("```{}```", response))
                    .await
                {
                    println!("Error sending message: {:?}", e);
                }
            } else if msg.content == ".help" {
                if let Err(e) = msg.channel_id.say(&ctx.http, HELP_TEXT).await {
                    println!("Error sending message: {:?}", e);
                }
            } else if msg.content == ".register" {
                if let Err(e) = msg
          .channel_id
          .say(
            &ctx.http,
            format!(
              "Run the following command in your terminal: ```dfx canister --network ic call au7z2-aaaaa-aaaah-abk7a-cai register '(\"{}\")'```",
              msg.author.id
            ),
          )
          .await
        {
          println!("Error sending message: {:?}", e);
        }
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
