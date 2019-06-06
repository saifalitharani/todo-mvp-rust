use super::schema::todos;

#[derive(Queryable)]
pub struct Todo {
    pub done: bool,
    pub name: String,
    pub id: i32,
}

#[derive(Insertable)]
#[table_name="todos"]
pub struct NewTodo<'a> {
    pub name: &'a str,
}