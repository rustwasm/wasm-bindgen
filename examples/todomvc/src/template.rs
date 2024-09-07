use crate::store::{ItemList, ItemListTrait};
use askama::Template as AskamaTemplate;

#[derive(AskamaTemplate)]
#[template(path = "row.html")]
struct RowTemplate<'a> {
    id: &'a str,
    title: &'a str,
    completed: bool,
}

#[derive(AskamaTemplate)]
#[template(path = "itemsLeft.html")]
struct ItemsLeftTemplate {
    active_todos: usize,
}

pub struct Template {}

impl Template {
    /// Format the contents of a todo list.
    ///
    /// items `ItemList` contains keys you want to find in the template to replace.
    /// Returns the contents for a todo list
    ///
    pub fn item_list(items: ItemList) -> String {
        let mut output = String::from("");
        for item in items.iter() {
            let row = RowTemplate {
                id: &item.id,
                completed: item.completed,
                title: &item.title,
            };
            if let Ok(res) = row.render() {
                output.push_str(&res);
            }
        }
        output
    }

    ///
    /// Format the contents of an "items left" indicator.
    ///
    /// `active_todos` Number of active todos
    ///
    /// Returns the contents for an "items left" indicator
    pub fn item_counter(active_todos: usize) -> String {
        let items_left = ItemsLeftTemplate { active_todos };
        items_left.render().unwrap_or_default()
    }
}
