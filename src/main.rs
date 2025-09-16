#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use device_query::DeviceQuery;
use enigo::MouseControllable;

fn main() {
    println!("Curcor bouncing is running.");

    let mut mouse_controller = enigo::Enigo::new();
    let screen_size = mouse_controller.main_display_size();
    let mouse_listner = device_query::DeviceState::new();

    let initial_mouse_position = mouse_listner.get_mouse().coords;
    
    let mut virtual_x = initial_mouse_position.0;
    let mut virtual_y = initial_mouse_position.1;

    let mut dx = 5;
    let mut dy = -5;

    let tolerance = 5;

    let mut last_middle_click = std::time::Instant::now();
    let middle_click_inteval = std::time::Duration::from_secs(60);

    loop {

        if last_middle_click.elapsed() >= middle_click_inteval{
            mouse_controller.mouse_click(enigo::MouseButton::Middle);
            last_middle_click = std::time::Instant::now();
            println!("Middle click simulated");
        }

        virtual_x += dx;
        virtual_y += dy;

        let real_mouse_pos = mouse_listner.get_mouse().coords;

        let estimated_mouse_pos_x = real_mouse_pos.0 + dx;
        let estimated_mouse_pos_y = real_mouse_pos.1 + dy;

        let diff_x = (estimated_mouse_pos_x - virtual_x).abs();
        let diff_y = (estimated_mouse_pos_y - virtual_y).abs();

        if diff_x > tolerance || diff_y > tolerance {
            println!("Mouse moved. Exiting...");
            break;
        }

        mouse_controller.mouse_move_to(virtual_x, virtual_y);

        if virtual_x >= screen_size.0 || virtual_x <= 0 {
            dx = -dx;
        }
        if virtual_y >= screen_size.1 || virtual_y <= 0 {
            dy = -dy;
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
