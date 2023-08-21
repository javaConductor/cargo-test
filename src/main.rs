use mongodb::{Client, Collection};
use mongodb::bson::{Document, oid::ObjectId, DateTime, document::ValueAccessResult};
use tokio;
use bson::{bson, doc, Bson};
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
struct LessonElement {
    #[serde(rename(deserialize = "verseSpec"))]
    verse_spec: String,
    comments: Option<String>,
    #[serde(rename(deserialize = "elementType"))]
    element_type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]

struct Lesson {
    _id: ObjectId,
    title: String,
    #[serde(rename(deserialize = "lessonDate"))]
    lesson_date: Option<DateTime>,
    teacher: String,
    #[serde(rename(deserialize = "lessonElements"))]
    lesson_elements: Vec<LessonElement>,
}

// use mongodb::bson::datetime;
#[tokio::main]
async fn main() {
    println!("Reading lessons from Mongo DB");
    let client_uri = String::from("mongodb://localhost:27017");
    let clien_opt = create_mongo_db(&client_uri).await;
    let client = clien_opt.expect("No client returned");
    println!("Got client - calling process()");
    let docs = process(client);
    let doc_vector_opt = docs.await;
    println!("Back from process()");

    match doc_vector_opt {
        Some(doc_vector) => {
            let mut title_vec: Vec<String> = Vec::new();
            for lesson in doc_vector {
                let title = lesson.title;
                println!("lesson title: {}", title);
                title_vec.push(title.to_string());
            }
            Some(title_vec)
        }
        _ => { None }
    };
}

async fn process(mongodb_client: Client) -> Option<Vec<Lesson>> {
    println!("in process()");

    let db = mongodb_client.database("lessons");
    println!("in process(): got db: {}", db.name());

    // let lesson_collection: mongodb::Collection<Lesson> = db.collection("lessons");\
    let collection: Collection<Lesson> = db.collection::<Lesson>("lessons");
    let cursor = match collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(_) => {
            let v: Vec<Lesson> = Vec::new();
            return Option::from(v);
        }
    };


    // let results: Vec<Result<Document, _>> = cursor.collect().await?.try;
    // let vec = cursor.try_collect().await.unwrap_or_else(|_| vec![]);

    let results: Vec<Result<Lesson, _>> = cursor.collect().await;
    // let vec = cursor.try_collect().await.unwrap_or_else(|_| vec![]);

    let mut lesson_vec: Vec<Lesson> = Vec::new();
    for item in results {
        println!("{:?}\n", item);

        let lesson = item.unwrap();
        //let doc_str = doc.to_string();
        println!("{:?}\n", lesson);
        lesson_vec.push(lesson);
    }

    for l in lesson_vec.clone() {
        let lesson_title:  &str = l.title.as_str();
        println!("Lesson {}", lesson_title);
    }

    Some(lesson_vec)
}

async fn create_mongo_db(client_uri: &str) -> Option<Client> {
    let result = Client::with_uri_str(client_uri);
    result.await.ok()
}
