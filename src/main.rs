mod process_message;
use process_message::process_message;
use frankenstein::{AsyncApi, UpdateContent};
use frankenstein::AsyncTelegramApi;
use frankenstein::GetUpdatesParams;

static TOKEN: &str = "1738132292:AAFqt_SGEEdfAH4mZOOfVAK5npssmg8DkP8";
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
                    if let UpdateContent::Message(message) = update.content {
                        let api_clone = api.clone();
                        tokio::spawn (async move{
                            process_message(message, api_clone).await;
                        }) ;

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

