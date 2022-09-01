use std::error;
use std::io::stdin;
extern crate dotenv;
use dotenv::dotenv;
use std::env;
use serde_json;
use serde::Deserialize;
use colour::{dark_cyan, yellow, e_blue};


#[derive(Deserialize, Debug)]
pub struct Articles{
    pub articles: Vec<Article>
}

#[derive(Deserialize, Debug)]
pub struct Article{
    pub title: String,
    pub description: String,
    pub url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn error::Error>>{
    let response = ureq::get(&url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;
    Ok(articles)
}

fn render_articles(articles: &Articles){
    for a in &articles.articles {
        dark_cyan!("> {}\n", a.title);
        e_blue!("   +{}\n", a.description);
        yellow!("    +{}\n\n\n", a.url);


    }
}


fn main() -> Result<(), Box<dyn error::Error>>{
    dotenv();
    println!("Welcome to the news CLI tool");
    println!("Please enter a topic you're looking for");


    //get input
    let mut q = String::new();
    stdin().read_line(&mut q)?;
    println!("You're looking for {}", q);

    //create string
    let mut request_string = format!("https://newsapi.org/v2/everything?q={}&from=2022-08-30&to=2022-08-30&sortBy=popularity&language=en&apiKey=",q);
    //get apikey
    let api_key = env::var("NEWSAPI_KEY")?;
    //build and format request string
    request_string = format!("{}{}", request_string, api_key);
    // println!("{}", request_string);

    let articles = get_articles(&request_string)?;
    render_articles(&articles);
    



    Ok(())
}


