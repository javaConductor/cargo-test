use serde::{self, Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
struct LessonElement {
    verse_spec: String,
    comments: String,
    element_type: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Lesson {
    _id: ObjectId,
    title: String,
    lesson_date: DateTime,
    teacher: String,
    lesson_elements: Vec<LessonElement>,
}

use serde_json::json;
// use serde_json::Result;

use mongodb::bson::{
    de::Error,
    oid::ObjectId,
    RawDocumentBuf
};
use mongodb::bson::DateTime;
use mongodb::{Client, Cursor};
use std::convert::Infallible;

// use mongodb::bson::datetime;
fn main() {
    println!("Reading lessons from Mongo DB");
    let client_uri = String::from("mongodb://localhost:27017");

    let r = create_mongo_db(&client_uri);

    match create_mongo_db(&client_uri) {
        Some(mongodb_client) => process(mongodb_client),
        None => print!("Unable to connect to Mongo DB at {}", client_uri),
    }
}


async fn process<T>(mongodb_client: Client) {
    let db = mongodb_client.database("lessons");
    // let lesson_collection: mongodb::Collection<Lesson> = db.collection("lessons");\
    let collection = db.collection::<Lesson>("lessons");

    let docs: Result<mongodb::Cursor<Lesson>, mongodb::error::Error> =
        collection.find(None, None).await;

    let lessons: Cursor<Lesson> = docs.unwrap();

    // let stuff = lessons.try_collect().await.unwrap_or_else(|_| vec![]);
    let stuff = lessons
        .deserialize_current()
        .await
        .unwrap_or_else(|_| vec![]);

    for l in stuff {
        println!("Lesson id {}", l._id);
    }
    // into_iter()
    //     .map(|raw| bson::from_slice(raw.try_into(self::Lesson)).unwrap())
    //     .collect();

    //println!("{} lessons found", lessons.len());
}
async fn create_mongo_db(client_uri: &str) -> Option<Client> {
    let result = Client::with_uri_str(client_uri).await?;
    Some(result)
}
