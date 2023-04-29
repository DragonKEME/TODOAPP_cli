use std::error::Error;
use cursive::views::{Button, Checkbox, Dialog, DummyView, EditView, LinearLayout, SelectView, TextView};
use cursive::{Cursive, CursiveExt};
use cursive::align::HAlign;
use cursive::traits::{Nameable, Resizable};
use crate::controller::login::login;
use crate::controller::{category, todos};
use crate::controller::user;
use crate::models::todo_model::Todo;
use cursive::view::{Scrollable};
use cursive_aligned_view::Alignable;
use crate::controller::register::register;
use crate::models::category_model::Category;

static DEFAULT_GIVE_ALL: bool = false;

pub fn todo_list(){
    let mut todo_app = Cursive::default();

    if user::get_user().get_id() == 0{
        todo_app.add_layer(login_layer());
    }else {
        todo_app.add_layer(todos_layer(DEFAULT_GIVE_ALL));
    }

    todo_app.add_global_callback('q', quit_popup);

    todo_app.run();
}

pub fn user_view(s: &mut Cursive){
    let user = user::get_user();
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
            match todos::finished_todo(todo_id) {
                Ok(()) => (),
                Err(e) => error_popup(s,e)
            }
            s.pop_layer();
            refresh_todo_list(s);
        })
        .button("No", |s| {
            s.pop_layer();
        })
        .title("Finished ?"));
}

pub fn todos_layer(give_finished: bool) -> Dialog {
    let todo_list = todos::get_todos();
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

    let mut checkbox = Checkbox::new().on_change(toggle_show_todo);
    if give_finished{
        checkbox.check();
    }else{
        checkbox.uncheck();
    }

    let v_layout = LinearLayout::vertical()
        .child(column)
        .child(todo_selector.with_name("todo_list").scrollable())
        .child(DummyView)
        .child(LinearLayout::horizontal()
                    .child(checkbox.with_name("give_all"))
                    .child(TextView::new(" See finished Todos"))
        );


    Dialog::around(v_layout)
        .title("TODOAPP")
        .button("Add todo", |s| {s.add_layer(add_todo_layer());})
        .button("User info",  user_view)
        .button("Disconnect", disconnect)
        .button("Quit", quit_popup)

}

pub fn refresh_todo_list(s: &mut Cursive){
    let todo_list = todos::get_todos();
    let state = s.call_on_name("give_all", |view: &mut Checkbox| {
        view.is_checked()
    }).unwrap();

    let mut todo_selector = s.find_name::<SelectView<Todo>>("todo_list").unwrap();
    todo_selector.clear();
    for todo in todo_list {
        if let Some( todo_string) = todo.to_format_string(20,state) {
            todo_selector.add_item(todo_string,todo)
        }
    }
}

pub fn toggle_show_todo(s: &mut Cursive, state: bool){
    s.pop_layer();
    s.add_layer(todos_layer(state));
}


pub fn login_layer() -> Dialog{
    #[allow(unused_variables)]
    let login_layer = LinearLayout::vertical()
        .child(TextView::new("Username:"))
        .child(EditView::new().on_submit(|s , str| connect_todoapp(s)).with_name("username_login").fixed_width(40))
        .child(DummyView)
        .child(TextView::new("Password:"))
        .child(EditView::new().on_submit(|s , str| connect_todoapp(s))
            .secret().with_name("password_login").fixed_width(40));
    Dialog::around(login_layer)
        .button("Connect",connect_todoapp)
        .button("Register",|s| {s.pop_layer();s.add_layer(register_view());})
        .button("Quit",quit_popup)
        .title("TODOAPP: Login")
}

pub fn connect_todoapp(s: &mut Cursive){
    let username = s.call_on_name("username_login", |view: &mut EditView| {
            view.get_content()
        }).unwrap();
    let password = s.call_on_name("password_login", |view: &mut EditView| {
        view.get_content()
    }).unwrap();
    match login(username.to_string(),password.to_string()) {
        Ok(()) => {s.pop_layer();s.add_layer(todos_layer(DEFAULT_GIVE_ALL));}
        Err(e) => error_popup(s,e)
    }
}

pub fn register_view() -> Dialog{
    #[allow(unused_variables)]
    let register_layer = LinearLayout::vertical()
        .child(TextView::new("Email:"))
        .child(EditView::new().on_submit(|s , str| register_todoapp(s))
            .with_name("email_register").fixed_width(40))
        .child(DummyView)
        .child(TextView::new("Username:"))
        .child(EditView::new().on_submit(|s , str| register_todoapp(s))
            .with_name("username_register").fixed_width(40))
        .child(DummyView)
        .child(TextView::new("Password:"))
        .child(EditView::new().secret().on_submit(|s , str| register_todoapp(s))
            .with_name("password_register").fixed_width(40));
    Dialog::around(register_layer)
        .button("Register",register_todoapp)
        .button("Back", |s| {s.pop_layer();s.add_layer(login_layer())})
        .button("Quit",quit_popup)
        .title("TODOAPP: Register")
}

pub fn register_todoapp(s: &mut Cursive){
    let username = s.call_on_name("username_register", |view: &mut EditView| {
        view.get_content()
    }).unwrap();
    let password = s.call_on_name("password_register", |view: &mut EditView| {
        view.get_content()
    }).unwrap();
    let email = s.call_on_name("email_register", |view: &mut EditView| {
        view.get_content()
    }).unwrap();
    match register(username.to_string(),email.to_string(),password.to_string()) {
        Ok(()) => {s.pop_layer();
            s.add_layer(login_layer());
            s.add_layer(Dialog::info("   You register successful   \nPlease now login you")
                                .title("Welcome"));}
        Err(e) => error_popup(s,e),
    }
}

pub fn add_todo_layer() -> Dialog {
    let add_todo_layer = LinearLayout::vertical()
        .child(TextView::new("Content:"))
        .child(EditView::new().with_name("new_todo_content").fixed_width(40))
        .child(DummyView)
        .child(TextView::new("Category:"))
        .child(SelectView::<Option<Category>>::new()
            .on_submit(category_selector)
            .item("Chose a Category (enter)", None)
            .with_name("new_todo_category")
            .fixed_height(1))
        .child(DummyView)
        //Buttons
        .child(LinearLayout::horizontal()
            .child(Button::new("Create", add_new_todo).disabled().with_name("create_button"))
            .child(Button::new("Cancel", |s| { s.pop_layer(); }))
            .align_bottom_right());

    Dialog::around(add_todo_layer).title("Add Todo")
}

pub fn add_new_todo(s: &mut Cursive){
    let select = s.find_name::<SelectView<Option<Category>>>("new_todo_category").unwrap();
    let content = s.call_on_name("new_todo_content", |view: &mut EditView| {
        view.get_content()
    }).unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("Please set a category")),
        Some(category_id) => {
            match todos::make_new_todo(content.to_string(), select.get_item(category_id).unwrap().1.as_ref().unwrap()) {
                Ok(()) => { s.pop_layer(); refresh_todo_list(s);},
                Err(e) => error_popup(s,e),
            }
        }
    }
}

pub fn category_selector(s: &mut Cursive, selected: &Option<Category>){
    let categories  = match category::get_categories(){
        Ok(cat) => cat,
        Err(e) => return error_popup(s,e),
    };
    let mut categories_selector = SelectView::<Category>::new()
        .on_submit(category_selected)
        .h_align(HAlign::Center);

    let cat_selected_id = match selected{
        Some(selected_category) => selected_category.get_id() - 1 ,
        None => 0,
    };
    for category in categories {
        categories_selector.add_item(category.to_string(), category)
    }
    s.add_layer(Dialog::around(categories_selector.selected(cat_selected_id).scrollable()));
}

pub fn category_selected(s: &mut Cursive, selected: &Category){
    s.call_on_name("new_todo_category", |view: &mut SelectView<Option<Category>>|{
        view.clear();
        view.add_item(selected.to_string(),Some(selected.clone()))
    }).unwrap();
    s.call_on_name("create_button", |button: &mut Button|{
        button.set_enabled(true)
    }).unwrap();
    s.pop_layer();
}

pub fn disconnect(s: &mut Cursive){
    user::reset_user();
    todos::reset_todos();
    s.pop_layer();
    s.add_layer(login_layer())
}

pub fn quit_popup(s: &mut Cursive) {
    s.add_layer(Dialog::text("\n    Are you sure you want to leave?   \n")
        .button("No", |s| {s.pop_layer();})
        .button("Yes", |s| s.quit())
        .title("Quit ?"));
}

pub fn error_popup(s: &mut Cursive, error: Box<dyn Error>){
    s.add_layer(Dialog::info(error.to_string()).title("ERROR"))
}