extern crate teleborg;

#[cfg(test)]
mod tests {
    use teleborg::{Dispatcher, Updater, Bot};
    use teleborg::objects::Update;
    use teleborg::objects::inline_query_results::{InlineQueryResultArticle, InlineQueryResult};
    use teleborg::objects::input_message_content::{InputTextMessageContent,
                                                   InputLocationMessageContent,
                                                   InputContactMessageContent,
                                                   InputVenueMessageContent};
    use teleborg::{ParseMode};

    #[test]
    fn test_updater() {
        let mut dispatcher = Dispatcher::new();
        dispatcher.add_command_handler("test", test, false);
        dispatcher.add_inline_query_handler(test_inline_query);
        Updater::start(None, None, None, None, dispatcher);
    }

    fn test(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        bot.reply_to_message(&update, "It works!").unwrap();
    }

    fn test_inline_query(bot: &Bot, update: Update, _: Option<Vec<&str>>) {
        if let Some(inline_query) = update.clone().inline_query {
            let mut results: Vec<Box<InlineQueryResult>> = Vec::new();
            let content_text = InputTextMessageContent::new("Telegram rules".to_owned(),
                                                       Some(ParseMode::Text),
                                                       Some(true));
            let article_text = InlineQueryResultArticle::new("Telegram text?".to_string(),
                                                        Box::new(content_text),
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        Some("https://telegram.org/img/t_logo.png".to_string()),
                                                        None,
                                                        None);

            let content_location = InputLocationMessageContent::new(35.0, 38.0);
            let article_location = InlineQueryResultArticle::new("Telegram location?".to_string(),
                                                         Box::new(content_location),
                                                         None,
                                                         None,
                                                         None,
                                                         None,
                                                         Some("https://telegram.org/img/t_logo.png".to_string()),
                                                         None,
                                                         None);
            let content_contact = InputContactMessageContent::new(String::from("06-12345678"),
                                                                  String::from("Voider1"),
                                                                  None);
            let article_contact = InlineQueryResultArticle::new("Telegram contact?".to_string(),
                                                                Box::new(content_contact),
                                                                None,
                                                                None,
                                                                None,
                                                                None,
                                                                Some("https://telegram.org/img/t_logo.png".to_string()),
                                                                None,
                                                                None);
            let content_venue = InputVenueMessageContent::new(35.0,
                                                              38.0,
                                                              "test title".to_string(),
                                                              "test address".to_string(),
                                                              None);
            let article_venue = InlineQueryResultArticle::new("Telegram venue?".to_string(),
                                                              Box::new(content_venue),
                                                              None,
                                                              None,
                                                              None,
                                                              None,
                                                              Some("https://telegram.org/img/t_logo.png".to_string()),
                                                              None,
                                                              None);

            results.push(Box::new(article_text));
            results.push(Box::new(article_location));
            results.push(Box::new(article_contact));
            results.push(Box::new(article_venue));

            let answer_result = bot.answer_inline_query(&update, results);
            println!("{:?}", answer_result.err());
        }
    }
}
