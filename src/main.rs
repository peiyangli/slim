#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

use i_slint_backend_winit::WinitWindowAccessor;
use rand::Rng;
use slint::{Model, ModelExt};



enum SearchResult{
    Some(usize),
    None(usize),
}


fn lower_bound<T: Clone + 'static, F>(model: &slint::VecModel<T>, compare: F) -> SearchResult 
where
    F: Fn(&T) -> std::cmp::Ordering
{
    let mut left = 0;
    let mut right = model.row_count();

    while left < right {
        let mid = left + (right - left) / 2;
        if let Some(current) = model.row_data(mid) {
            match compare(&current){
                std::cmp::Ordering::Less => right = mid,
                std::cmp::Ordering::Equal => return SearchResult::Some(mid),
                std::cmp::Ordering::Greater => left = mid + 1,
            }
        } else {
            return SearchResult::None(right);
        }
    }
    SearchResult::None(left)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new().unwrap()));
    let app: AppWindow = AppWindow::new()?;

    {
        //窗口居中
        let win = app.window();
        let window_size = win.size();
        win.with_winit_window(move|winit_window|{
            if let Some(monitor) = winit_window.current_monitor() {
                let screen_size = monitor.size();
                
                // Calculate center position
                let x = (screen_size.width as i32 - window_size.width as i32) / 2;
                let y = (screen_size.height as i32 - window_size.height as i32) / 2;
                
                // Set the window position
                win.set_position(slint::PhysicalPosition::new(x, y));
            }
        });
    }

    {
        let defAvatar = slint::Image::load_from_path(std::path::Path::new("ui/assets/images/img.jpg")).unwrap();
        //test list
        let model = slint::VecModel::<MyListViewItem>::from(vec![
            MyListViewItem { id: 2i64, from_type: 1,  text: slint::SharedString::from("Item 1"), avatar: defAvatar.clone() },
            MyListViewItem { id: 3i64, from_type: 1, text: slint::SharedString::from("Item 2"), avatar: defAvatar.clone() },
            MyListViewItem { id: 5i64, from_type: 1, text: slint::SharedString::from("Item 3"), avatar: defAvatar.clone() },
            MyListViewItem { id: 7i64, from_type: 1, text: slint::SharedString::from("Item 4"), avatar: defAvatar.clone() },
            MyListViewItem { id: 11i64, from_type: 2, text: slint::SharedString::from("Item 5"), avatar: defAvatar.clone() },
            MyListViewItem { id: 13i64, from_type: 2, text: slint::SharedString::from("Item 6"), avatar: defAvatar.clone() },
            MyListViewItem { id: 17i64, from_type: 2, text: slint::SharedString::from("Item 7"), avatar: defAvatar.clone() },
            MyListViewItem { id: 19i64, from_type: 2, text: slint::SharedString::from("Item 8"), avatar: defAvatar.clone() },
            MyListViewItem { id: 23i64, from_type: 1, text: slint::SharedString::from("Item 9"), avatar: defAvatar.clone() },
        ]);
        for i in 10..1000{
           model.push(MyListViewItem { id: i as i64, from_type: i%2+1, text: slint::SharedString::from(format!("Hello, {}! go and find what is not just so easy to do so", i).as_str()), avatar: defAvatar.clone()});
        }
        let model = std::rc::Rc::new(model);

        app.set_list_history(slint::ModelRc::new(model.clone().reverse()));

        // let mut rng = rand::rng();

        let handle = app.as_weak();
        app.on_item_clicked(move|opt, item|{
            if let Some(app) = handle.upgrade(){
                match opt {
                    MyListViewOperate::Remove =>{
                        match lower_bound(&model, |val|{
                            if item.id == val.id{return  std::cmp::Ordering::Equal;}
                            if item.id < val.id{return std::cmp::Ordering::Less;}
                            std::cmp::Ordering::Greater
                        }){
                            SearchResult::None(_index)=>{
                                println!("=============> not found: {}", item.id)
                            },
                            SearchResult::Some(index)=>{
                                model.remove(index);
                                println!("=============> remove: {}", item.id);

                                //remove is ok
                                // app.set_list_model_data(app.get_list_model_data()+1);
                            }
                        }
                    },
                    MyListViewOperate::Add =>{

                        let id  = if(model.row_count()<1){1} else{model.row_data(model.row_count()-1).unwrap().id+1};
                            //rng.random();
                        let item = MyListViewItem { id: id, from_type: id as i32 %2 + 1, text: slint::SharedString::from(format!("Hello, {}!", id).as_str()), avatar: defAvatar.clone() };
                        match lower_bound(&model, |val|{
                            if item.id == val.id{return  std::cmp::Ordering::Equal;}
                            if item.id < val.id{return std::cmp::Ordering::Less;}
                            std::cmp::Ordering::Greater
                        }){
                            SearchResult::None(index)=>{
                                model.insert(index, item);
                                println!("=============> insert: {}-{}", index, id);

                                // let handle = app.as_weak();
                                // slint::invoke_from_event_loop(move||{
                                //     if let Some(app) = handle.upgrade(){
                                //         // app.set_listViewY(std::cmp::min(0, app.get_listVisibleHeight() as i32 -app.get_listViewHeight() as i32) as f32);
                                //         app.set_list_model_data(app.get_list_model_data()+1);
                                //     }
                                // }).unwrap();
                            },
                            SearchResult::Some(index)=>{
                                println!("=============> inserted: {}", index)
                            }
                        }
                    }
                }
            }
        });
    }

    let handle = app.as_weak();
    app.on_title_clicked(move |evt|{
        if let Some(app) = handle.upgrade(){
            match evt {
                1 =>{
                    let _ = app.hide();
                },
                2|4 =>{
                    let tmaxminzed = app.window().is_maximized();
                    app.window().set_maximized(!tmaxminzed);
                },
                3 => {
                    app.window().set_minimized(true);
                },
                _ =>{}
            }
        }
    });

    let handle = app.as_weak();
    app.on_move_window(move |offset_x, offset_y|{
        let app = handle.upgrade().unwrap();
        let logical_pos = app.window().position().to_logical(app.window().scale_factor());
        app.window().set_position(slint::LogicalPosition::new(logical_pos.x + offset_x, logical_pos.y + offset_y));
    });
    app.show()?;

    slint::run_event_loop()?;
    Ok(())
}