use std::fmt::format;
use cursive::views::{Dialog, LinearLayout, SelectView, TextView};
use cursive::{Cursive, CursiveExt};
use crate::controller::login::login;
use crate::controller::todos::{finished_todo, get_todos};
use crate::controller::user::{get_user, get_user_todo};
use crate::models::todo_model::Todo;
use crate::models::user_model::User;
use cursive::view::Nameable;

static give_finished: bool = true;

pub fn todo_list() -> Result<(),Box<dyn std::error::Error>>{
    let mut todo_app = Cursive::default();

    //Get user info and todos
    login("nem".to_string(),"proutprout".to_string())?;
    let (user,todo_list) = get_user_todo()?;

    todo_app.add_layer(recreate_layer());

    todo_app.add_global_callback('q', |s| s.quit());

    todo_app.run();
    Ok(())
}

pub fn user_view(s: &mut Cursive){
    let user = get_user();
    let user_info = "user id: ".to_string() + user.get_id().to_string().as_str() + "\n"
                         + "Username: " + user.get_username().as_str() + "\n"
                         + "Email: " + user.get_email().as_str();
    s.add_layer(Dialog::text(user_info).title("User infos")
                        .button("Ok", |s| {s.pop_layer();} ))

}

pub fn complete_todo(s: &mut Cursive, todo: &Todo){
    if todo.is_terminated(){
        s.add_layer(Dialog::info("Todo already terminated"));
        return;
    }
    let todo_id = todo.get_id();
    s.add_layer(Dialog::text(format!("Do you want terminate this todo\n{} ",todo.get_content()))
        .button("Yes ", move |s| {
            finished_todo(todo_id).unwrap();
            s.pop_layer();
            s.pop_layer();
            s.add_layer(recreate_layer())
        })
        .button("No", |s| {
            s.pop_layer();
        }));
}

pub fn recreate_layer() -> Dialog {
    let todo_list = get_todos();
    let mut description = format!("{:^5}|{:^20}|{:^18}","ID","content","category");
    if give_finished{
        description += "|finished";
    }
    let column = TextView::new(description);

    let mut todo_selector = SelectView::<Todo>::new()
        .on_submit(complete_todo);


    for todo in todo_list {
        if let Some( todo_string) = todo.to_format_string(20,give_finished) {
            todo_selector.add_item(todo_string,todo)
        }
    }
    let vlayout = LinearLayout::vertical()
        .child(column)
        .child(todo_selector.with_name("todo_selector"));

    Dialog::around(vlayout)
        .title("TODOAPP")
        .button("Quit", |s| s.quit())
        .button("User info",  user_view)
}