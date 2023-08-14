use std::rc::Rc;

use crate::io_utils::wait_for_key_press;

pub mod db;

mod io_utils;
mod models;
mod navigator;
mod ui;

fn main() {
    // TODO: create database and navigator
    let database = db::JiraDatabase::new("./data/db.json");
    let mut navigator = navigator::Navigator::new(Rc::new(database));

    loop {
        clearscreen::clear().unwrap();

        let page = navigator.get_current_page();
        if page.is_none() {
            break;
        }

        if let Some(page) = page {
            if let Err(error) = page.draw_page() {
                println!(
                    "Error rendering page: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
            }

            let input = io_utils::get_user_input();
            match page.handle_input(input.trim()) {
                Err(error) => {
                    println!(
                        "Error processing input: {}\nPress any key to continue...",
                        error
                    );
                    wait_for_key_press();
                }
                Ok(act) => {
                    if let Some(action) = act {
                        if let Err(error) = navigator.handle_action(action) {
                            println!(
                                "Error processing action: {}\nPress any key to continue...",
                                error
                            );
                            wait_for_key_press();
                        }
                    }
                }
            }
        }

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        // 2. render page
        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action
    }
}
