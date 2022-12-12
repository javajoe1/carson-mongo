use futures::executor::block_on;
use mongodb::bson::{doc, Document};
use mongodb::error::Error;
use mongodb::{Client, Cursor};

#[tokio::main]
async fn main() {
    let future = get_from_mongo();
    match block_on(future) {
        Ok(good) => println!("{:#?}", good),
        Err(e) => eprintln!("Error: {}", e),
    }
}

async fn get_from_mongo() -> Result<Cursor<Document>, Error> {
    let client = Client::with_uri_str("mongodb+srv://user:8WkirTWrRFZ5nnBAKvx9tq725DLLhxy3@javajoecluster.zixasoj.mongodb.net/test").await?;

    client
        .database("sample_airbnb")
        .collection::<Document>("listingsAndReviews")
        .aggregate(
            [doc! {
                "$count": "string"
            }],
            None,
        )
        .await
}
