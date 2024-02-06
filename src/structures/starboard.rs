use futures_lite::stream::StreamExt;
use mongodb::bson::{doc, from_bson, to_document, Bson};
use mongodb::Client;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StarStruct {
    pub _id: i64,
    pub stars: Vec<i64>,
    pub starboard_id: Option<i64>,
    pub author_id: i64,
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

    pub async fn add_star(&self, id: i64, user_id: i64, author_id: i64) -> StarStruct {
        let mut cursor = self.collection.find(doc! {"_id": id}, None).await.unwrap();

        // cursor.next() is None if it's the first star added to the message
        let Some(doc) = cursor.next().await else {
            let doc = doc! {
                "_id": id,
                "stars": [user_id],
                "starboard_id": None::<i64>,
                "author_id": author_id,
            };
            self.collection.insert_one(&doc, None).await.unwrap();
            return from_bson(Bson::Document(doc)).unwrap();
        };

        let doc = doc.unwrap();
        let mut stars = doc
            .get_array("stars")
            .unwrap()
            .iter()
            .map(|x| x.as_i64().unwrap())
            .collect::<Vec<i64>>();
        stars.push(user_id);
        let star_struct = StarStruct {
            _id: id,
            stars,
            starboard_id: doc.get_i64("starboard_id").ok(),
            author_id,
        };

        self.collection
            .update_one(
                doc! {"_id": id},
                doc! {"$set": &to_document(&star_struct).unwrap()},
                None,
            )
            .await
            .unwrap();

        star_struct
    }

    pub async fn remove_star(&self, id: i64, user_id: i64) -> Option<StarStruct> {
        let mut cursor = self.collection.find(doc! {"_id": id}, None).await.unwrap();
        let doc = cursor.next().await.unwrap().ok()?;

        let mut stars = doc
            .get_array("stars")
            .unwrap()
            .iter()
            .map(|x| x.as_i64().unwrap())
            .collect::<Vec<i64>>();
        stars.retain(|x| *x != user_id);

        let star_struct = StarStruct {
            _id: id,
            stars,
            starboard_id: doc.get_i64("starboard_id").ok(),
            author_id: doc.get_i64("author_id").unwrap(),
        };

        self.collection
            .update_one(
                doc! {"_id": id},
                doc! {"$set": &to_document(&star_struct).unwrap()},
                None,
            )
            .await
            .unwrap();

        Some(star_struct)
    }

    pub async fn get_random_star_message_id(&self) -> Option<i64> {
        let mut cursor = self
            .collection
            .find(doc! {"starboard_id": {"$ne": None::<i64>}}, None)
            .await
            .unwrap();
        let mut docs = Vec::new();

        while let Some(doc) = cursor.next().await {
            let Ok(doc) = doc else {
                continue;
            };
            docs.push(doc.get_i64("starboard_id").unwrap());
        }
        docs.choose(&mut rand::thread_rng()).map(|x| *x)
    }

    pub async fn update_starboard_message(&self, id: i64, starboard_id: i64) {
        self.collection
            .update_one(
                doc! {"_id": id},
                doc! {"$set": {"starboard_id": starboard_id}},
                None,
            )
            .await
            .unwrap();
    }
}
