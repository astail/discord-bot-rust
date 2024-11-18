use dotenv::dotenv;
use std::env;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // メッセージを受信したときに呼ばれる関数
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "Hello" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "hi").await {
                println!("メッセージ送信エラー: {:?}", why);
            }
        }
    }

    // ボットが起動したときに呼ばれる関数
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{}としてログインしました", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // .envファイルの読み込み
    dotenv().ok();

    // 環境変数DISCORD_TOKENからトークンを取得
    let token = env::var("DISCORD_TOKEN_RUST").expect("トークンが設定されていません");

    // クライアントを作成
    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("クライアントの作成に失敗しました");

    // クライアントを起動
    if let Err(why) = client.start().await {
        println!("クライアントエラー: {:?}", why);
    }
}
