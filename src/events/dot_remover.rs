use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::http::Http;
use serenity::builder::ExecuteWebhook;
use serenity::builder::CreateWebhook;
use regex::Regex;

pub struct Handler;

enum StrOrChar<T: AsRef<str>> {
    Str(T),
    Char(char),
}

// Implement PartialEq for StrOrChar
impl PartialEq<&str> for StrOrChar<&str> {
    fn eq(&self, other: &&str) -> bool {
        match self {
            StrOrChar::Str(s) => s == other,
            StrOrChar::Char(c) => c.to_string() == other.to_string(),
        }
    }
}

// Implement PartialEq for StrOrChar
impl PartialEq<char> for StrOrChar<&str> {
    fn eq(&self, other: &char) -> bool {
        match self {
            StrOrChar::Str(s) => s == &other.to_string(),
            StrOrChar::Char(c) => c == other,
        }
    }
}

// Implement PartialEq for StrOrChar
impl PartialEq<StrOrChar<&str>> for StrOrChar<&str> {
    fn eq(&self, other: &StrOrChar<&str>) -> bool {
        match self {
            StrOrChar::Str(s) => s == &other.to_string(),
            StrOrChar::Char(c) => c.to_string() == other.to_string(),
        }
    }
}

// Implement to_string for StrOrChar
impl StrOrChar<&str> {
    fn to_string(&self) -> String {
        match self {
            StrOrChar::Str(s) => s.to_string(),
            StrOrChar::Char(c) => c.to_string(),
        }
    }
}

// Implement Display for StrOrChar
impl std::fmt::Display for StrOrChar<&str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrOrChar::Str(s) => write!(f, "{}", s),
            StrOrChar::Char(c) => write!(f, "{}", c),
        }
    }
}

const DOTS: [StrOrChar<&str>; 20] = [
    StrOrChar::Char('.'),
    StrOrChar::Char('·'),
    StrOrChar::Char('․'),
    StrOrChar::Char('‧'),
    StrOrChar::Char('⋅'),
    StrOrChar::Char('・'),
    StrOrChar::Char('⸱'),
    StrOrChar::Char('ᐧ'),
    StrOrChar::Char('⏺'),
    StrOrChar::Char('●'),
    StrOrChar::Char('⚬'),
    StrOrChar::Char('⦁'),
    StrOrChar::Char('⸰'),
    StrOrChar::Char('﹒'),
    StrOrChar::Char('⸱'),
    StrOrChar::Char('⋅'),
    StrOrChar::Char('。'),
    StrOrChar::Char('．'),
    StrOrChar::Char('｡'),
    StrOrChar::Char('܂'),
];

fn remove_whitespace_from_end(text: &str, dot: &StrOrChar<&str>) -> String {
    const WHITESPACE: [StrOrChar<&str>; 18] = [
        StrOrChar::Char('\u{2000}'),
        StrOrChar::Char('\u{2001}'),
        StrOrChar::Char('\u{2003}'),
        StrOrChar::Char('\u{2004}'),
        StrOrChar::Char('\u{2005}'),
        StrOrChar::Char('\u{2006}'),
        StrOrChar::Char('\u{2007}'),
        StrOrChar::Char('\u{2008}'),
        StrOrChar::Char('\u{2009}'),
        StrOrChar::Char('\u{200A}'),
        StrOrChar::Char('\u{200B}'),
        StrOrChar::Char('\u{200C}'),
        StrOrChar::Char('\u{200D}'),
        StrOrChar::Char('\u{200E}'),
        StrOrChar::Char('\u{200F}'),
        StrOrChar::Char('\u{3164}'),
        StrOrChar::Str(r"_[ \n]*_"),
        StrOrChar::Str(r"\|\|[ \n]*\|\|"),
    ];

    let mut owned_text = text.to_owned().to_string();
    let joined_char_whitespaces: String = WHITESPACE
        .into_iter()
        .filter_map(
            |c| 
                if matches!(c, StrOrChar::Char(_)) {
                    Some(c.to_string())
                } else {
                    None
            }
        ).collect();

    let str_whitespaces_list: Vec<String> = WHITESPACE
        .into_iter()
        .filter_map(
            |c| 
                if matches!(c, StrOrChar::Str(_)) {
                    Some(c.to_string())
                } else {
                    None
                }
        ).collect();

    let joined_str_whitespaces = str_whitespaces_list.join("|");

    let re = Regex::new(
        &format!(r"{dot}([{joined_char_whitespaces} `]|({joined_str_whitespaces}))*", 
            joined_char_whitespaces=joined_char_whitespaces,
            joined_str_whitespaces=joined_str_whitespaces,
            dot=dot,
        )).unwrap();

    // count number of the ` in the first match
    let occurances = re.find_iter(&text).next();
    if occurances.is_some() {
        owned_text = re
            .replace_all(
                &owned_text, 
                dot.to_string().to_owned() + 
                &("`".repeat(
                    occurances
                        .unwrap()
                        .as_str()
                        .matches('`')
                        .count()
                    )
                ))
                .to_string();
    }
    
    return owned_text.to_string();
}


// If the dot is found, return the text without dot
pub fn find_if_dot_logic(text: &str) -> Option<String> {

	let joined_dots: String = DOTS.into_iter().map(|c| c.to_string()).collect();

    let re = Regex::new(
        &format!(r"([{joined_dots}] )|([{joined_dots}]$)", 
            joined_dots=joined_dots
        )).unwrap();
    if re.find_iter(&text).count() > 1 {
        return None; // If the text has more than a single sentence the last dot does not need to be removed.
    }

    // Loop through all possible dots
    for dot in DOTS.into_iter() {
        let replacement =StrOrChar::Str(r"\.");
        let new_text = Regex::new(
                &format!(
                    r"\\+{dot}", 
                    dot=if dot == '.' { &replacement } else { &dot }
                )
            )
            .unwrap()
            .replace_all(
                &remove_whitespace_from_end(
                    &text, 
                    if dot == '.' { &replacement } else { &dot }
                ), 
                "."
            )
            .to_string();
        // This regex is not at all useful in the detection of dots,
        // however if a dot to be removed is found and has an escape
        // character before it, it would look weird reformatted (eg. text\)
        // so this regex removes the escape character before the dot

        // If the dot is found, return the text without dot
        if new_text.ends_with(|x| dot == x) && 
            !new_text.ends_with(&format!("{}{}{}", dot, dot, dot)) {
            // Check if second to last character is a dot
            // if new_text.len() > 1 && 
            //     new_text
            //         .chars()
            //         .nth(new_text.len() - 2)
            //         .is_some_and(|x| dot == x) {
            //     // If it is, remove both dots
            //     return new_text
            //         .strip_suffix(&format!("{}{}", dot, dot))
            //         .map(|s| s.to_string());
            // }
            // Since implementing the helper function to call whis in a loop this is not needed anymore
            return new_text.strip_suffix(&format!("{}",dot)).map(|s| s.to_string());
            // Note: This is using strip_suffix instead of indexing
            // because of a sneaky taktic found by Jimmy where
            // if the dot is a multiple byte character, this
            // will error if indexing is used.
        } else {
            #[allow(unused_parens)] // Cleaner with them in my opinion
            if  (
                    Regex::new(&format!(
                            r"{dot} *`$", 
                            dot=if dot == '.' { &replacement } else { &dot }
                        ))
                        .unwrap()
                        .captures(&new_text)
                        .is_some() && // If there is a dot followed by `
                    new_text.matches('`').count() == 2 && 
                    // if there are exactly two ``
                    Regex::new(&format!(
                            r"{dot}{dot}{dot} *`$", 
                            dot=if dot == '.' { &replacement } else { &dot }
                        ))
                        .unwrap()
                        .captures(&new_text)
                        .is_none() && 
                        // if there are not three dots followed by `
                    Regex::new(
                        &format!(
                            r"`+{dot}{{1,2}}`+",
                            dot=if dot == '.' { &replacement } else { &dot }
                        )
                    ).unwrap().captures(&new_text).is_none()
                    // This is a case handled later, removing the dot here
                    // breaks the appropriate handling of this case later
            ) {
            // Check if third to last character is a dot
            if new_text.len() > 2 && 
                new_text
                    .chars()
                    .nth(new_text.len() - 3)
                    .is_some_and(|x| dot == x) {
                // If it is, remove both dots
                return Some(
                    Regex::new(&format!(
                        r"{dot}{dot} *`$", 
                        dot=if dot == '.' { &replacement } else { &dot }
                    ))
                    .unwrap()
                    .replace_all(&new_text, "`")
                    .to_string()
                );
            }
            return Some(
                Regex::new(&format!(
                        r"{dot} *`$", 
                        dot=if dot == '.' { &replacement } else { &dot }
                    ))
                    .unwrap()
                    .replace_all(&new_text, "`")
                    .to_string()
                );
            }
        }     
    
        // Create list of possible characters to surround dots that 
        // become invisible with markdown
        const MARKDOWN: [&str; 4] = [
            r"\*",
            "_",
            "~",
            "`",
        ];  

        // Loop through possible similar characters
        for mark in MARKDOWN.iter() {
            let re = Regex::new(
                &format!(
                    r"{mark}+{dot}{{1,2}}{mark}+",
                    mark=mark, 
                    dot=if dot == '.' { &replacement } else { &dot }
                )
            ).unwrap();
            if re.captures(&new_text).is_some() {
                let new_text = re.replace_all(&new_text, "").to_string();
                return Some(new_text);
            }
        }
    } 
        
    None // Message is dot free!! 
}

pub fn find_if_dot(text: &str) -> Option<std::string::String> {
    const RECURSION_LIMIT: usize = 10;
    let mut counter = 0;
    let mut result = find_if_dot_logic(text);

    // continue until result does not include any dot from DOTS constant
    // or the algorithm deems it illegal dot free
    while result
            .clone()
            .is_some_and(|s| DOTS.iter()
            .any(|dot| s.contains(&dot.to_string())))
            && counter < RECURSION_LIMIT 
            // This is to prevent infinite loops when the algorithm
            // is unable to remove the dot due to an exploit
        {
        result = find_if_dot_logic(&result.unwrap());
        counter += 1;
    } 

    result
}


#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, msg: Message) {
        // Check if message is from bot
        if msg.author.bot {
            return;
        }
        // TODO add excluded users
        // Figure out if it ends with a single dot and only has one 
        // sentence using regex
        let text = find_if_dot(&msg.clone().content);
        // If it does, remove the dot and send the message
        if text.is_some() {
            // Create a webhook in current channel
            let webhook = msg.channel_id
                .create_webhook(&ctx.http, CreateWebhook::new("dot!"))
                .await.expect("Could not create webhook.");



            let (name, avatar) = {
                // do this in a block to keep the lifetime of the reference 
                // returned by `Cache::member` small 
                let member = msg.member(&ctx.http).await.expect("Could not get member.");

                (member.display_name().to_string(), member.face())
            };

            let http = Http::new("");
            let builder = ExecuteWebhook::new()
                .content(&text.unwrap())
                .username(&name)
                .avatar_url(&avatar);
            webhook.execute(&http, false, builder).await
                .expect("Could not execute webhook.");

            msg.delete(&ctx.http).await.expect("Could not delete message.");
            webhook.delete(&http).await.expect("Could not delete webhook.");
        }
    }

    async fn message_update(&self, ctx: Context, _old_if_available: Option<Message>, _new: Option<Message>, event: serenity::model::event::MessageUpdateEvent) {
        // Check if message is from bot
        if event.clone().author.unwrap().bot {
            return;
        }
        // TODO add excluded users
        // Figure out if it ends with a single dot and only has one 
        // sentence using regex
        let text = find_if_dot(&event.clone().content.unwrap());
        // If it does, remove the dot and send the message
        if text.is_some() {
            // Create a webhook in current channel
            let webhook = event.clone().channel_id
                .create_webhook(&ctx.http, CreateWebhook::new("dot!"))
                .await.expect("Could not create webhook.");



            let (name, avatar) = {
                // So, this works, but I have no idea what I am doing.
                // This sentiment continues throughout this method.
                let member = match event
                            .clone()
                            .guild_id {
                            Some(guild) => Some(
                                guild
                                    .member(&ctx.http, event.clone()
                                    .author
                                    .unwrap()
                                    .id
                                ).await.expect("Could not get member.")),
                            None => None,
                };

                if member.clone().is_none() {
                    return;
                }

                (
                    member
                        .clone()
                        .unwrap()
                        .display_name()
                        .to_string(), 
                    member
                        .clone()
                        .unwrap()
                        .face()
                )
            };

            let builder = ExecuteWebhook::new()
                .content(&text.unwrap())
                .username(&name)
                .avatar_url(&avatar);
            webhook.execute(&ctx.http, false, builder).await
                .expect("Could not execute webhook.");

            // delete the message
            let message = event
                .clone()
                .channel_id
                .message(&ctx.http, event.clone().id)
                .await.expect("Could not get message.");
            message.delete(&ctx.http).await.expect("Could not delete message.");

            webhook.delete(&ctx.http).await.expect("Could not delete webhook.");
        }
    }

}
