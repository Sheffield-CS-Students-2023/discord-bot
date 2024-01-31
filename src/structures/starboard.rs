use crate::MongoClient;
use mongodb::bson::{doc, Bson, from_bson};
use mongodb::Client;
use rand::seq::SliceRandom;
use futures_lite::stream::StreamExt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StarStruct {
    pub id: i64,
    pub stars: Vec<i64>,
    pub starboard_id: Option<i64>,
    pub author_id: i64,
}

// Implement Clone for StarStruct
impl Clone for StarStruct {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            stars: self.stars.clone(),
            starboard_id: self.starboard_id,
            author_id: self.author_id,
        }
    }
}

// Create a starboard class
pub struct Starboard {
    pub collection: mongodb::Collection<mongodb::bson::Document>,
}

impl Starboard {
    pub async fn new(client: &Client) -> Self {
        Self {
            collection: client.database("starboard").collection("starboard"),
        }
    }

    pub async fn get_starboard_message(&self, id: i64) -> Option<StarStruct> {
        let mut cursor = self.collection.find(doc! {"_id": id}, None).await.unwrap();
        let doc = cursor.next().await.unwrap();
        if !doc.is_ok() {
            return None;
        }
        let doc = doc.unwrap();
        let doc = doc! {
            "id": doc.get_i64("_id").unwrap(),
            "stars": doc.get_array("stars").unwrap().iter().map(|x| x.as_i64().unwrap()).collect::<Vec<i64>>(),
            "starboard_id": doc.get_i64("starboard_id").unwrap(),
            "author_id": doc.get_i64("author_id").unwrap(),
        };
        Some(from_bson(Bson::Document(doc)).unwrap())
    }

    pub async fn add_star(&self, id: i64, user_id: i64, author_id: i64) -> StarStruct {
        let mut cursor = self.collection.find(doc! {"_id": id}, None).await.unwrap();
        let doc = cursor.next().await.unwrap();
        if !doc.is_ok() {
            let doc = doc! {
                "_id": id,
                "stars": [user_id],
                "starboard_id": None::<i64>,
                "author_id": author_id,
            };
            self.collection.insert_one(&doc, None).await.unwrap();
            return from_bson(Bson::Document(doc)).unwrap();
        }
        let doc = doc.unwrap();
        let mut stars = doc.get_array("stars").unwrap().iter().map(|x| x.as_i64().unwrap()).collect::<Vec<i64>>();
        stars.push(user_id);
        let doc = doc! {
            "_id": id,
            "stars": stars,
            "starboard_id": doc.get_i64("starboard_id").unwrap(),
            "author_id": author_id,
        };
        self.collection.update_one(doc! {"_id": id}, doc! {"$set": &doc}, None).await.unwrap();
        from_bson(Bson::Document(doc)).unwrap()
    }

    pub async fn remove_star(&self, id: i64, user_id: i64) -> Option<StarStruct> {
        let mut cursor = self.collection.find(doc! {"_id": id}, None).await.unwrap();
        let doc = cursor.next().await.unwrap();
        if !doc.is_ok() {
            return None;
        }
        let doc = doc.unwrap();
        let mut stars = doc.get_array("stars").unwrap().iter().map(|x| x.as_i64().unwrap()).collect::<Vec<i64>>();
        stars.retain(|x| *x != user_id);
        let doc = doc! {
            "_id": id,
            "stars": stars,
            "starboard_id": doc.get_i64("starboard_id").unwrap(),
            "author_id": doc.get_i64("author_id").unwrap(),
        };
        self.collection.update_one(doc! {"_id": id}, doc! {"$set": &doc}, None).await.unwrap();
        Some(from_bson(Bson::Document(doc)).unwrap())
    }

    pub async fn get_random_star_message_id(&self) -> Option<i64> {
        let mut cursor = self.collection.find(doc! {"starboard_id": {"$ne": None::<i64>}}, None).await.unwrap();
        let mut docs = Vec::new();
        while let Some(doc) = cursor.next().await {
            if !doc.is_ok() {
                continue;
            }
            let doc = doc.unwrap();
            docs.push(doc.get_i64("_id").unwrap());
        }
        docs.choose(&mut rand::thread_rng()).map(|x| *x)
    }

    pub async fn update_starboard_message(&self, id: i64, starboard_id: i64) {
        self.collection.update_one(doc! {"_id": id}, doc! {"$set": {"starboard_id": starboard_id}}, None).await.unwrap();
    }
}