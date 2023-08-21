//use serde::{self, Deserialize, Serialize};
//#[derive(Clone, Debug, Deserialize, Serialize)]
struct LessonElement {
    verse_spec: String,
    comments: String,
    element_type: String,
}
//#[derive(Clone, Debug, Deserialize, Serialize)]
struct Lesson {
    _id: ObjectId,
    title: String,
    lesson_date: DateTime,
    teacher: String,
    lesson_elements: Vec<LessonElement>,
}


 use mongodb::bson::{Document, oid::ObjectId};
use mongodb::bson::DateTime;
use mongodb::{Client, Collection, Cursor};
use mongodb::bson::document::ValueAccessResult;
use tokio;

// don't forget this!
use tokio::stream;
use futures::stream::StreamExt;


// use mongodb::bson::datetime;
#[tokio::main]
async fn main(){
    println!("Reading lessons from Mongo DB");
    let client_uri = String::from("mongodb://localhost:27017");

    let clien_opt = create_mongo_db(&client_uri).await;

    let client = clien_opt.expect("No client returned");
    println!("Got client - calling process()");

   let docs = process(client);
    // match create_mongo_db(&client_uri).await {
    //     Some(mongodb_client) => process(mongodb_client.to_owned()),
    //     None => None,
    // }
    let doc_vector_opt = docs.await;

    match doc_vector_opt {
        Some(doc_vector) => {
            let mut title_vec:Vec<String> = Vec::new();
            for doc in doc_vector  {
                let title =  doc.get_str("title").unwrap();
                println!("doc: {}", title);
                title_vec.push(title.to_string());
            }
            Some(title_vec)
        }
        _ => { None}
    };

    //Ok(title_vec)
}


async fn process(mongodb_client: Client) -> Option<Vec<Document>>{
    println!("in process()");


    let db = mongodb_client.database("lessons");
    println!("in process(): got db: {}", db.name());

    // let lesson_collection: mongodb::Collection<Lesson> = db.collection("lessons");\
    let collection:Collection<Document> = db.collection::<Document>("lessons");

    //let mut cursor:Cursor<Document> =  collection.find(None,None).await.ok()?;

    let cursor = match collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(_) => {
            let v:Vec<Document> = Vec::new();
            return Option::from(v);
        },
    };
    let results: Vec<Result<Document, _>> = cursor.collect().await;
   // let vec = cursor.try_collect().await.unwrap_or_else(|_| vec![]);

    let mut  doc_vec:Vec<Document> = Vec::new();
    for item in  results {
        println!("{:?}", item);
        let doc = item.unwrap();
        doc_vec.push(doc);
    }
    // let lessons_result_vector:Result<Document, _>  =  cursor.deserialize_current() ;
    // let mut lessons_vector: Vec<Document> = Vec::new();

    // println!("in processs(): {} documents", lessons_result_vector.le);
    for l in doc_vec.clone() {
        let lesson_doc: ValueAccessResult<&str> = l.get_str("title");
        let title = lesson_doc.unwrap_or("No title");
        println!("Lesson {}", title);
        //lessons_vector.push(l);
    }

    //
    // while let Some(result) =
    //     cursor.collect().await {
    //     let lesson = result?;
    //     lessons.push(lesson);
    //     println!("Lesson {}", lesson.get_str("title")?);
    // }
    Some(doc_vec)
}
  async fn create_mongo_db(client_uri: &str) -> Option<Client> {
    let result = Client::with_uri_str(client_uri);
    result.await.ok()
}
