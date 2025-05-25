trait Summarizable {
    fn summary(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

impl Summarizable for Article {
    fn summary(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

struct Reporter;

impl Reporter {
    // This method accepts anything that implements Summarizable
    fn announce<T: Summarizable>(&self, item: T) {
        println!("Breaking News: {}", item.summary());
    }
}

fn main() {
    let article = Article {
        title: "Rust Hits 2.0!".to_string(),
        author: "Jane Doe".to_string(),
    };

    let reporter = Reporter;
    reporter.announce(article);
}
