use mongodb::Client;
use mongodb::bson::{doc, Document};

fn main() {
    println!("starting ....");
    x();
    println!("done");
}

async fn x(){
    println!("calling mongo");
    let client = Client::with_uri_str("mongodb+srv://user:8WkirTWrRFZ5nnBAKvx9tq725DLLhxy3@javajoecluster.zixasoj.mongodb.net/test").await.unwrap();
    println!("calling result");
    let result = client.database("sample_airbnb").collection::<Document>("listingsAndReviews").aggregate([
        doc! {
            "$count": "string"
        }
    ], None).await.unwrap();
    println!("{:#?}", result);
}

