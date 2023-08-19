use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)] 
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}


fn main() {
    let article: Article = Article {
        article: String::from("How to use json in rust"),
        author: String::from("Chinedu"),
        paragraph: vec![
            Paragraph {
                name: String::from("lorem10")
            },
            Paragraph {
                name: String::from("lorem10")
            },
            Paragraph {
                name: String::from("lorem10")
            },
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("The json is: {}", json);
}
