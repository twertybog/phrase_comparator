mod phrases;
use phrases::read_phrases_file;
use frankenstein::AsyncApi;
use frankenstein::Message;
use frankenstein::User;
use frankenstein::SendMessageParams;
use frankenstein::AsyncTelegramApi;
use rand;
use rand::Rng;

pub async fn process_message(message: Message, api: AsyncApi) {
    let phrases = match read_phrases_file("phrases.txt"){
        Ok(phrases) => phrases,
        Err(error) => {
            eprintln!("{}", error);
            vec![]
        }
    };
    let mut chat_message = message.text.as_deref().unwrap_or_else(|| "Null").to_lowercase();
    let from = message.from.unwrap_or_else(||
        User { id: 0, is_bot: false, first_name: String::from(""), last_name: None, username: None,
            language_code: None, can_join_groups: None, can_read_all_group_messages: None,
            supports_inline_queries: None }).id;
    chat_message.insert(0, ' ');
    chat_message.push(' ');
    'inner: for i in phrases{
        //println!("{}", chat_message);
        if chat_message.contains(&i) && from == user_id{
            
            let send_message_params = SendMessageParams::builder()
            .chat_id(message.chat.id)
            .text(format!("{}%", rand::thread_rng().gen_range(1..101)))
            .reply_to_message_id(message.message_id)
            .build();
            if let Err(err) = api.send_message(&send_message_params).await {
                println!("Failed to send message: {:?}", err);
            }
            break 'inner;
        }
    }  
}