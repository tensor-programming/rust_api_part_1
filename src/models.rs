use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::books;
use schema::books::dsl::books as all_books;

#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "books"]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub published: bool,
}

impl Book {
    pub fn show(id: i32, conn: &PgConnection) -> Vec<Book> {
        all_books
            .find(id)
            .load::<Book>(conn)
            .expect("Error loading book")
    }

    pub fn all(conn: &PgConnection) -> Vec<Book> {
        all_books
            .order(books::id.desc())
            .load::<Book>(conn)
            .expect("error loading the books")
    }

    pub fn update_by_id(id: i32, conn: &PgConnection, book: NewBook) -> bool {
        use schema::books::dsl::{author as a, published as p, title as t};
        let NewBook {
            title,
            author,
            published,
        } = book;

        diesel::update(all_books.find(id))
            .set((a.eq(author), p.eq(published), t.eq(title)))
            .get_result::<Book>(conn)
            .is_ok()
    }

    pub fn insert(book: NewBook, conn: &PgConnection) -> bool {
        diesel::insert_into(books::table)
            .values(&book)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if Book::show(id, conn).is_empty() {
            return false;
        };
        diesel::delete(all_books.find(id)).execute(conn).is_ok()
    }

    pub fn all_by_author(author: String, conn: &PgConnection) -> Vec<Book> {
        all_books
            .filter(books::author.eq(author))
            .load::<Book>(conn)
            .expect("Error loading books by author")
    }
}
