mod phrases;
use phrases::read_phrases_file;
use frankenstein::AsyncApi;
use frankenstein::AsyncTelegramApi;
use frankenstein::GetUpdatesParams;
use frankenstein::Message;
use frankenstein::SendMessageParams;
use rand;
use rand::Rng;

static TOKEN: &str = "";
#[tokio::main]
async fn main() {
    
    let api = AsyncApi::new(TOKEN);

    let update_params_builder =
        GetUpdatesParams::builder().allowed_updates(vec!["message".to_string()]);
    let mut update_params = update_params_builder.clone().build();

    loop{
        let result = api.get_updates(&update_params).await;

        //println!("result: {:?}", result);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let Some(message) = update.message {
                        let api_clone = api.clone();
                        tokio::spawn(async move {
                            process_message(message, api_clone).await;
                        });

                        update_params = update_params_builder
                            .clone()
                            .offset(update.update_id + 1)
                            .build();
                    }
                }
            }
            Err(error) => {
                println!("Failed to get updates: {:?}", error);
            }
        }
    }
    
}
async fn process_message(message: Message, api: AsyncApi) {
    let phrases = match read_phrases_file("phrases.txt"){
        Ok(phrases) => phrases,
        Err(error) => {
            eprintln!("{}", error);
            vec![]
        },
    };
    let mut chat_message = message.text.as_deref().unwrap().to_lowercase();
    chat_message.insert(0, ' ');
    chat_message.push(' ');
    for i in phrases{
        //println!("{}", chat_message);
        if chat_message.contains(&i){
            
            let send_message_params = SendMessageParams::builder()
            .chat_id(message.chat.id)
            .text(format!("{}%", rand::thread_rng().gen_range(1..101)))
            .reply_to_message_id(message.message_id)
            .build();
            if let Err(err) = api.send_message(&send_message_params).await {
                println!("Failed to send message: {:?}", err);
            }
        }
    }  
}
