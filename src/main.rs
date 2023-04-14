extern crate gtk;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label, Orientation};
use notify_rust::Notification;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

fn main() {
    let app = Application::builder()
        .application_id("com.areco.pomodoro-timer")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn add_input_box(labelstr: String, countertime: Rc<RefCell<u32>>) -> gtk::Box {
    let inputbox = Box::new(Orientation::Horizontal, 1);
    let label = Label::builder()
        .label(&labelstr)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .build();
    let timer = Label::builder()
        .label(&((*countertime.as_ref().borrow_mut() / 60) % 60).to_string())
        .margin_top(12)
        .margin_bottom(12)
        .build();
    let minutes = Label::builder()
        .label(" Minutes")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .build();
    let add_button = Button::builder()
        .label("+")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .build();
    let remove_button = Button::builder()
        .label("-")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .build();

    inputbox.append(&label);
    inputbox.append(&timer);
    inputbox.append(&minutes);
    inputbox.append(&remove_button);
    inputbox.append(&add_button);

    let timer_clone = timer.clone();
    let countertime_clone = countertime.clone();
    add_value(add_button, countertime_clone, timer_clone);

    let timer_clone = timer.clone();
    let countertime_clone = countertime.clone();
    remove_value(remove_button, countertime_clone, timer_clone);

    inputbox
}

fn add_value(add_button: Button, countertime: Rc<RefCell<u32>>, timer: Label) {
    add_button.connect_clicked(move |_| {
        *countertime.as_ref().borrow_mut() += 60;
        timer.set_text(&((*countertime.as_ref().borrow_mut() / 60) % 60).to_string());
    });
}
fn remove_value(remove_button: Button, countertime: Rc<RefCell<u32>>, timer: Label) {
    remove_button.connect_clicked(move |_| {
        *countertime.as_ref().borrow_mut() -= 60;

        timer.set_text(&((*countertime.as_ref().borrow_mut() / 60) % 60).to_string());
    });
}

fn start_timer(
    countertime: Rc<RefCell<u32>>,
    breaktime: Rc<RefCell<u32>>,
    content: Box,
    label3: Label,
) {
    let content_copy = content.clone();
    let label3_copy = label3.clone();

    let end = u64::try_from(*countertime.clone().as_ref().borrow_mut()).unwrap();
    let endbreak = u64::try_from(*breaktime.clone().as_ref().borrow_mut()).unwrap();
    let time = Instant::now();

    start_step(end, endbreak, time, content_copy, label3_copy, false);
}
fn start_step(end: u64, endbreak: u64, time: Instant, content: Box, label3: Label, is_last: bool) {
    let mut owned_string: String = "Remaining time: ".to_owned();
    let borrowed_string: &str = &(((end - time.elapsed().as_secs()) / 60) % 60).to_string();
    owned_string.push_str(borrowed_string);
    let borrowed_string: &str = " Minutes";
    owned_string.push_str(borrowed_string);

    let label2 = Label::builder()
        .label(&owned_string)
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .build();

    content.remove(&label3);
    content.append(&label2);
    let content2 = content.clone();

    let tick = move || {
        if (end - time.elapsed().as_secs()) == 0 {
            content2.remove(&label2);

            let time = Instant::now();
            let content_copy = content.clone();
            let label3_copy = label3.clone();

            if is_last == false {
                let _notification = Notification::new()
                    .summary("Pomodoro Timer")
                    .body("Work time is over")
                    .show();
                start_step(endbreak, end, time, content_copy, label3_copy, true);
            } else {
                let _notification = Notification::new()
                    .summary("Pomodoro Timer")
                    .body("Break time is over")
                    .show();
            }
            glib::Continue(false)
        } else {
            let mut owned_string: String = "Remaining time: ".to_owned();
            let borrowed_string: &str = &(((end - time.elapsed().as_secs()) / 60) % 60).to_string();
            owned_string.push_str(borrowed_string);
            let borrowed_string: &str = " Minutes";
            owned_string.push_str(borrowed_string);

            label2.set_text(&owned_string);
            glib::Continue(true)
        }
    };
    glib::timeout_add_seconds_local(1, tick);
}

fn build_ui(app: &Application) {
    //Construct title
    let title = Label::builder()
        .label("Pomodoro Timer")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .build();

    //Create content box
    let content = Box::new(Orientation::Vertical, 1);

    //Create Window
    let window = ApplicationWindow::builder()
        .title("UI Demo")
        .application(app)
        .child(&content)
        .build();

    //Create Start Button
    let start_button = Button::builder()
        .label("Start")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .build();

    //Create label
    let label3 = Label::builder()
        .label("Press Start to begin timer")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .build();

    //Create Input timer for work time
    let countertime: Rc<RefCell<u32>> = Rc::new(RefCell::new(25 * 60));

    let countertime_copy = countertime.clone();
    let workinputbox = add_input_box("Work Time: ".to_string(), countertime_copy);

    //Create Input for break time
    let breaktime: Rc<RefCell<u32>> = Rc::new(RefCell::new(5 * 60));
    let breaktime_copy = breaktime.clone();
    let breakinputbox = add_input_box("Break Time: ".to_string(), breaktime_copy);

    content.append(&title);
    content.append(&workinputbox);
    content.append(&breakinputbox);
    content.append(&start_button);
    content.append(&label3);

    start_button.connect_clicked(move |_| {
        let countertime_copy = countertime.clone();
        let content_copy = content.clone();
        let label3_copy = label3.clone();
        let breaktime_copy = breaktime.clone();
        start_timer(countertime_copy, breaktime_copy, content_copy, label3_copy);
    });

    window.show();
}
