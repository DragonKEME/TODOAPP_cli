use cursive::views::{Checkbox, Dialog, DummyView, EditView, LinearLayout, SelectView, TextView};
use cursive::{Cursive, CursiveExt};
use cursive::traits::{Nameable, Resizable};
use crate::controller::login::login;
use crate::controller::todos;
use crate::controller::user;
use crate::models::todo_model::Todo;
use cursive::view::{Scrollable};
use crate::controller::register::register;

static DEFAULT_GIVE_ALL: bool = false;

pub fn todo_list() -> Result<(),Box<dyn std::error::Error>>{
    let mut todo_app = Cursive::default();

    if user::get_user().get_id() == 0{
        todo_app.add_layer(login_layer());
    }else {
        todo_app.add_layer(todos_layer(DEFAULT_GIVE_ALL));
    }

    todo_app.add_global_callback('q', |s| s.quit());

    todo_app.run();
    Ok(())
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
            todos::finished_todo(todo_id).unwrap();
            s.pop_layer();
            refresh_todo_layer(s);
        })
        .button("No", |s| {
            s.pop_layer();
        }));
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

    let mut checkbox = Checkbox::new().on_change(toogle_show_todo);
    if give_finished{
        checkbox.check();
    }else{
        checkbox.uncheck();
    }

    let v_layout = LinearLayout::vertical()
        .child(column)
        .child(todo_selector.scrollable())
        .child(DummyView)
        .child(LinearLayout::horizontal()
                    .child(checkbox.with_name("give_all"))
                    .child(TextView::new(" See finished Todos"))
        );


    Dialog::around(v_layout)
        .title("TODOAPP")
        .button("User info",  user_view)
        .button("Disconnect", disconnect)
        .button("Quit", |s| s.quit())

}

pub fn refresh_todo_layer(s: &mut Cursive){
    let state = s.call_on_name("give_all", |view: &mut Checkbox| {
        view.is_checked()
    }).unwrap();
    s.pop_layer();
    s.add_layer(todos_layer(state));
}

pub fn toogle_show_todo(s: &mut Cursive, state: bool){
    s.pop_layer();
    s.add_layer(todos_layer(state));
}

pub fn login_layer() -> Dialog{
    let login_layer = LinearLayout::vertical()
        .child(TextView::new("Username:"))
        .child(EditView::new().with_name("username_login").fixed_width(40))
        .child(DummyView)
        .child(TextView::new("Password:"))
        .child(EditView::new().with_name("password_login").fixed_width(40));
    Dialog::around(login_layer)
        .button("Connect",connect_todoapp)
        .button("Register",|s| {s.pop_layer();s.add_layer(register_view());})
        .button("Quit",|s| s.quit())
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
        Err(e) => s.add_layer(Dialog::info(e.to_string()))
    }
}

pub fn register_view() -> Dialog{
    let register_layer = LinearLayout::vertical()
        .child(TextView::new("Email:"))
        .child(EditView::new().with_name("email_register").fixed_width(40))
        .child(DummyView)
        .child(TextView::new("Username:"))
        .child(EditView::new().with_name("username_register").fixed_width(40))
        .child(DummyView)
        .child(TextView::new("Password:"))
        .child(EditView::new().with_name("password_register").fixed_width(40));
    Dialog::around(register_layer)
        .button("Login", |s| {s.pop_layer();s.add_layer(login_layer())})
        .button("Register",register_todoapp)
        .button("Quit",|s| s.quit())
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
            s.add_layer(Dialog::info("   You register successful   \nPlease now login you"));}
        Err(e) => s.add_layer(Dialog::info(e.to_string()))
    }
}

pub fn add_todo(s: &mut Cursive){

}

pub fn disconnect(s: &mut Cursive){
    user::reset_user();
    todos::reset_todos();
    s.pop_layer();
    s.add_layer(login_layer())
}