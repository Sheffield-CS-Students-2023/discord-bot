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
    for whitespace in WHITESPACE.iter() {
        let re = Regex::new(
            &format!(r"{dot}([ `]*{char})*", 
                dot=dot, 
                char=whitespace
            )).unwrap();
        // count number of the ` in the first match
        let occurances = re.find_iter(&text).next();
        if occurances != None {
            // print!("{:?} occurances with char {}", occurances.unwrap().as_str(), whitespace);
            owned_text = re
                .replace_all(
                    &text, 
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
    }

    return owned_text.to_string();
}


// If the dot is found, return the text without dot
pub fn find_if_dot(text: &str) -> Option<String> {
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

	let joined_dots: String = DOTS.into_iter().map(|c| c.to_string()).collect();

    let re = Regex::new(
        &format!(r"([{joined_dots}] )|([{joined_dots}]$)", 
            joined_dots=joined_dots
        )).unwrap();
    if re.find_iter(&text).count() > 1 {
        return None; // If the text has more than a single sentence the last dot does not need to be removed.
    }

    // Loop through all possible dots
    for dot in DOTS.iter() {
        let replacement =StrOrChar::Str(r"\.");
        let new_text = Regex::new(&format!(r"\\+{dot}", dot=dot))
            .unwrap()
            .replace_all(
                &remove_whitespace_from_end(
                    &text, 
                    if dot == &'.' { &replacement } else { &dot }
                ), 
                "."
            )
            .to_string();
        // If the dot is found, return the text without dot
        if new_text.ends_with(|x| *dot == x) && 
            !new_text.ends_with(&format!("{}{}{}", dot, dot, dot)) {
            print!("Found dot with char {}", dot);
            // Check if second to last character is a dot
            if new_text.len() > 1 && 
                new_text.chars().nth(new_text.len() - 2).is_some_and(|x| dot == &x) {
                // If it is, remove both dots
                return new_text.strip_suffix(&format!("{}{}", dot, dot)).map(|s| s.to_string());
            }
            return new_text.strip_suffix(&format!("{}",dot)).map(|s| s.to_string());
            // Note: This is using strip_suffix instead of indexing
            // because of a sneaky taktic found by Jimmy where
            // if the dot is a multiple byte character, this
            // will error if indexing is used.
        } else {
            #[allow(unused_parens)] // Cleaner with them in my opinion
            if  (
                    Regex::new(&format!(r"{dot} *`$", dot=dot))
                        .unwrap()
                        .captures(&new_text)
                        .is_some() && // If there is a dot followed by `
                    new_text.matches('`').count() == 2 && // if there are exactly two ``
                    Regex::new(&format!(r"{dot}{dot}{dot} *`$", dot=dot))
                        .unwrap()
                        .captures(&new_text)
                        .is_none() // if there are not three dots followed by `
                ) {
                print!("Found two dots with char {}", dot);
                // Check if third to last character is a dot
                if new_text.len() > 2 && 
                    new_text.chars().nth(new_text.len() - 3).is_some_and(|x| dot == &x) {
                        println!("This shoudln't happen");
                    // If it is, remove both dots
                    return Some(
                        Regex::new(&format!(r"{dot}{dot} *`$", dot=dot))
                        .unwrap()
                        .replace_all(&new_text, "`")
                        .to_string()
                    );
                }
                println!("This shoud happen");
                return Some(
                    Regex::new(&format!(r"{dot} *`$", dot=dot))
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
        
        // Loop through all possible markdown
        for dot in DOTS.iter() {
            // Loop through possible similar characters
            for mark in MARKDOWN.iter() {
                // If the dot is found, return the text without dot
                let replacement: &StrOrChar<&str> = &StrOrChar::Str(r"\.") ;
                let new_text = Regex::new(&format!(r"\\+{dot}", dot=dot))
                    .unwrap()
                    .replace_all(
                        &remove_whitespace_from_end(
                            &text, 
                            if dot == &'.' { &replacement } else { &dot }
                        ), 
                        "."
                    )
                    .to_string();
                let re = Regex::new(
                    &format!(
                        r"{mark}+{dot}{{1,2}}{mark}+",
                        mark=mark, 
                        dot=if dot == &'.' { &replacement } else { &dot }
                    )
                ).unwrap();
                if re.find_iter(&new_text).count() > 0 {
                    print!("Found dot with char {} and markdown {}", dot, mark);
                    let new_text = re.replace_all(&new_text, "").to_string();
                    return Some(new_text);
                }
            }
        }
    }
    None // Message is dot free!! 
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
        if text != None {
            print!("Found dot in message: {}", text.clone().unwrap());
            // Create a webhook in current channel
            let webhook = msg.channel_id
                .create_webhook(&ctx.http, CreateWebhook::new("dot!"))
                .await.expect("Could not create webhook.");

            let nullable_name = match &msg.member{
                Some(member) => &member.nick,
                None => &None
                };
            
            // Doing this again is the move here instead of doing it all
            // in a bulk because member.nick can be None
            // as well as msg.author.global_name
            let name = match nullable_name {
                Some(name) => name.clone(),
                None => match &msg.author.global_name {
                    Some(name) => name.clone(),
                    None => msg.author.name.clone()
                }
            };

            let http = Http::new("");
            let builder = ExecuteWebhook::new()
                .content(&text.unwrap())
                .username(&name)
                .avatar_url(&msg.author.avatar_url().unwrap());
            webhook.execute(&http, false, builder).await
                .expect("Could not execute webhook.");

            msg.delete(&ctx.http).await.expect("Could not delete message.");
            webhook.delete(&http).await.expect("Could not delete webhook.");
        }
    }
}
