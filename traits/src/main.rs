
struct Tweet {
    username: String,
    content: String,
    timestamp: u64,
}

struct Comment {
    username: String,
    content: String,
    timestamp: u64,
    tweet_id: u64,
}

trait Summarizable {
    fn summarize(&self) -> String;
}

impl Summarizable for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summarizable for Comment {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("user1"),
        content: String::from("Hello, world!"),
        timestamp: 1625251200,
    };

    let comment = Comment {
        username: String::from("user2"),
        content: String::from("Nice tweet!"),
        timestamp: 1625251260,
        tweet_id: 1,
    };

    summarise_things(&tweet);
    summarise_things(&comment);
}


fn summarise_things<T>(item: &T) where T: Summarizable {
    println!("Summary: {}", item.summarize());
}