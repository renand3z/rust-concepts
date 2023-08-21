trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }
}

pub fn run() {
    let article = NewsArticle {
        headline: String::from("Breaking News"),
        content: String::from("Lorem ipsum dolor sit amet"),
    };

    println!("{:?}", article.headline);
    println!("{:?}", article.content);
    println!("{:?}", article.summarize());
}
