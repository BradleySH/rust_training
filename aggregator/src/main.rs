use aggregator::{self, NewsArticle, Summary};
//use chapter10::{self, NewsArticle};

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course as you probabaly know, people"),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());

  let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburg, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
      "The Pittsburg Penguins once again are the best \
      hockey team in the NHL.",
    ),
  };

  println!("New article available! {}", article.summarize());
}
