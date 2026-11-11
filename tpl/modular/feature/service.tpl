use crate::features::{{name}}::model::{ {{name_pascal_case}}, Create{{name_pascal_case}}, Update{{name_pascal_case}} };

pub async fn get_all() -> Vec<{{name_pascal_case}}> {
    // TODO: fetch from your database
    vec![]
}

pub async fn get_by_id(id: &str) -> Option<{{name_pascal_case}}> {
    // TODO: fetch from your database
    let _ = id;
    None
}

pub async fn create(data: Create{{name_pascal_case}}) -> {{name_pascal_case}} {
    // TODO: insert into your database
    let _ = data;
    {{name_pascal_case}} {
        id: None,
        // map fields from data here
    }
}

pub async fn update(id: &str, data: Update{{name_pascal_case}}) -> Option<{{name_pascal_case}}> {
    // TODO: update in your database
    let _ = (id, data);
    None
}

pub async fn delete(id: &str) -> bool {
    // TODO: delete from your database
    let _ = id;
    false
}