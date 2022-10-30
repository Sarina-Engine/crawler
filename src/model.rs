use super::deserializer::{CommentTemplate, ProductTemplate};
use super::schema::comments::{self, dsl::vid};
use super::schema::products;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Comment {
    pub vid: i32,
    pub id: i32,
    pub product_id: i32,
    pub title: String,
    pub body: String,
    pub ratting: i32,
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub id: i32,
    pub product_id: i32,
    pub title: String,
    pub body: String,
    pub rating: i32,
}

impl Comment {
    pub fn get(conn: &mut SqliteConnection, id: i32) -> Comment {
        match comments::table.filter(vid.eq(id)).load::<Comment>(conn) {
            Ok(mut image_vec) if !image_vec.is_empty() => image_vec.pop().unwrap(),
            _ => panic!("Error!, message"),
        }
    }

    pub fn add(comment: &CommentTemplate, product_id: i32, conn: &mut SqliteConnection) -> () {
        let com = NewComment {
            id: comment.id,
            product_id: product_id,
            body: String::from(&comment.body),
            rating: comment.rate,
            title: String::from(comment.title.as_ref().unwrap_or(&String::from(""))),
        };
        diesel::insert_into(comments::table)
            .values(com)
            .execute(conn)
            .unwrap();
        ()
    }
}

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct {
    id: i32,
    title_fa: String,
}

pub struct Product;

impl Product {
    pub fn add(product: &ProductTemplate, conn: &mut SqliteConnection) -> () {
        let product = NewProduct {
            id: product.id,
            title_fa: String::from(&product.title_fa),
        };
        diesel::insert_into(products::table)
            .values(product)
            .execute(conn)
            .unwrap();
    }
}
