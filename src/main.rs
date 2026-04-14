#![allow(unused)]

use arboard::Clipboard;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
#[cfg(target_os = "macos")]
use tao::platform::macos::{ActivationPolicy, EventLoopExtMacOS, WindowExtMacOS};
use tao::window::WindowBuilder;
use muda::ContextMenu;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder,
};

const MAX_HISTORY: usize = 50;

#[derive(Clone)]
struct HistoryEntry {
    text: String,
    timestamp: u64,
}

struct AppState {
    entries: Vec<HistoryEntry>,
}

impl AppState {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    
    fn truncate_text(text: &str, max_len: usize) -> String {
        let single_line = text.replace('\n', " ");
        if single_line.chars().count() > max_len {
            let truncated: String = single_line.chars().take(max_len).collect();
            format!("{}...", truncated)
        } else {
            single_line
        }
    }
}

fn get_data_path() -> PathBuf {
    let mut path = std::env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("."));
    path.push(".clipboard_history.txt");
    path
}

fn load_history() -> Vec<HistoryEntry> {
    let path = get_data_path();
    let mut entries = Vec::new();
    if let Ok(data) = fs::read_to_string(&path) {
        let mut cursor = 0;
        while cursor < data.len() {
            let Some(ts_end) = data[cursor..].find('\n') else { break; };
            let ts_str = &data[cursor..cursor + ts_end];
            cursor += ts_end + 1;
            
            let Some(len_end) = data[cursor..].find('\n') else { break; };
            let len_str = &data[cursor..cursor + len_end];
            cursor += len_end + 1;
            
            if let (Ok(ts), Ok(len)) = (ts_str.parse::<u64>(), len_str.parse::<usize>()) {
                if cursor + len <= data.len() {
                    let text = data[cursor..cursor + len].to_string();
                    cursor += len;
                    if cursor < data.len() && data[cursor..].starts_with('\n') {
                        cursor += 1;
                    }
                    entries.push(HistoryEntry { text, timestamp: ts });
                } else { break; }
            } else { break; }
        }
    }
    entries
}

fn save_history(entries: &[HistoryEntry]) {
    let path = get_data_path();
    let mut out = String::new();
    for e in entries {
        out.push_str(&format!("{}\n{}\n{}\n", e.timestamp, e.text.len(), e.text));
    }
    let _ = fs::write(path, out);
}

enum CustomEvent {
    ClipboardUpdated(String),
}

fn main() {
    let mut app_state = AppState::new();
    app_state.entries = load_history();

    // Setup event loop
    let mut event_loop_builder = EventLoopBuilder::<CustomEvent>::with_user_event();
    let mut event_loop = event_loop_builder.build();
    #[cfg(target_os = "macos")]
    event_loop.set_activation_policy(ActivationPolicy::Accessory);
    let proxy = event_loop.create_proxy();

    let invisible_window = WindowBuilder::new()
        .with_visible(false)
        .with_decorations(false)
        .with_always_on_top(true)
        .with_title("ClipHistoryPopup")
        .build(&event_loop)
        .unwrap();

    // Menu Map to link Menu ID to History text
    let mut menu_map: HashMap<String, String> = HashMap::new();

    // Build the tray menu
    let menu = Menu::new();
    
    // Function to populate menu
    let populate_menu = |menu: &Menu, entries: &[HistoryEntry], map: &mut HashMap<String, String>| {
        // clear map
        map.clear();
        
        // Remove all items? We have to do it by creating a new menu or just appending.
        // Muda / tray-icon doesn't have an easy clear(). We will just return a new Menu instead!
    };

    // Rebuild Menu helper
    let rebuild_menu = |entries: &[HistoryEntry], map: &mut HashMap<String, String>| -> Menu {
        map.clear();
        let new_menu = Menu::new();
        
        let title_item = MenuItem::new("📋 Clipboard History", false, None);
        let _ = new_menu.append(&title_item);
        let _ = new_menu.append(&PredefinedMenuItem::separator());
        
        let mut add_separator = false;
        if entries.is_empty() {
             let empty_item = MenuItem::new("No history yet", false, None);
             let _ = new_menu.append(&empty_item);
        } else {
            for (idx, entry) in entries.iter().enumerate() {
                let preview = AppState::truncate_text(&entry.text, 40);
                let menu_item = MenuItem::new(format!("{}. {}", idx + 1, preview), true, None);
                map.insert(menu_item.id().0.clone(), entry.text.clone());
                let _ = new_menu.append(&menu_item);
            }
            add_separator = true;
        }

        if add_separator {
             let _ = new_menu.append(&PredefinedMenuItem::separator());
        }
        let clear_item = MenuItem::new("Clear History", true, None);
        map.insert(clear_item.id().0.clone(), "___CLEAR___".to_string());
        let _ = new_menu.append(&clear_item);
        
        let quit_item = MenuItem::new("Quit", true, None);
        map.insert(quit_item.id().0.clone(), "___QUIT___".to_string());
        let _ = new_menu.append(&quit_item);
        
        new_menu
    };

    let initial_menu = rebuild_menu(&app_state.entries, &mut menu_map);

    let mut tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(initial_menu))
            .with_title("📋")
            .build()
            .unwrap(),
    );

    let menu_channel = MenuEvent::receiver();
    let hotkey_channel = GlobalHotKeyEvent::receiver();

    // Register Cmd+Shift+V
    let hotkey_manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyV);
    hotkey_manager.register(hotkey).unwrap();

    // Background thread for clipboard polling
    let mut latest_clipboard_text = app_state.entries.first().map(|e| e.text.clone()).unwrap_or_default();
    
    thread::spawn(move || {
        let mut clipboard = match Clipboard::new() {
            Ok(cb) => cb,
            Err(e) => {
                eprintln!("Failed to init clipboard monitor: {}", e);
                return;
            }
        };

        let mut last_text = String::new();
        // pre-fill last_text without emitting event immediately
        if let Ok(text) = clipboard.get_text() {
            last_text = text;
        }

        loop {
            thread::sleep(Duration::from_millis(500));
            if let Ok(text) = clipboard.get_text() {
                let trimmed = text.trim();
                // If it's different and not empty
                if !trimmed.is_empty() && text != last_text {
                    last_text = text.clone();
                    let _ = proxy.send_event(CustomEvent::ClipboardUpdated(text));
                }
            }
        }
    });

    let mut clipboard_writer = Clipboard::new().unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(100));

        match event {
            Event::UserEvent(CustomEvent::ClipboardUpdated(new_text)) => {
                // Ignore if it matches the very first entry precisely (avoids loop when we select menu item)
                if let Some(first) = app_state.entries.first() {
                    if first.text == new_text {
                        return;
                    }
                }

                // Add to history
                app_state.entries.retain(|e| e.text != new_text); // remove duplicates
                app_state.entries.insert(0, HistoryEntry {
                    text: new_text.clone(),
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                });
                app_state.entries.truncate(MAX_HISTORY);
                
                save_history(&app_state.entries);

                // Update Menu
                if let Some(tray) = &mut tray_icon {
                    let new_m = rebuild_menu(&app_state.entries, &mut menu_map);
                    tray.set_menu(Some(Box::new(new_m)));
                }
            }
            _ => {}
        }

        // Process Menu Events
        if let Ok(event) = menu_channel.try_recv() {
            if let Some(action) = menu_map.get(&event.id.0) {
                if action == "___QUIT___" {
                    tray_icon.take();
                    *control_flow = ControlFlow::Exit;
                } else if action == "___CLEAR___" {
                    app_state.entries.clear();
                    save_history(&app_state.entries);
                    if let Some(tray) = &mut tray_icon {
                        let new_m = rebuild_menu(&app_state.entries, &mut menu_map);
                        tray.set_menu(Some(Box::new(new_m)));
                    }
                } else {
                    // It's a clipboard item
                    // Write to clipboard
                    if let Err(e) = clipboard_writer.set_text(action) {
                        eprintln!("Failed to write to clipboard: {}", e);
                    }
                    // Move to top
                    let selected_text = action.clone();
                    app_state.entries.retain(|e| e.text != selected_text); // remove duplicates
                    app_state.entries.insert(0, HistoryEntry {
                        text: selected_text.clone(),
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    });
                    save_history(&app_state.entries);
                    
                    if let Some(tray) = &mut tray_icon {
                        let new_m = rebuild_menu(&app_state.entries, &mut menu_map);
                        tray.set_menu(Some(Box::new(new_m)));
                    }
                }
            }
        }

        // Process Hotkey Events
        if let Ok(event) = hotkey_channel.try_recv() {
            if event.id == hotkey.id() && event.state == global_hotkey::HotKeyState::Released {
                let current_entries = app_state.entries.clone();
                let popup_menu = rebuild_menu(&current_entries, &mut menu_map);
                #[cfg(target_os = "macos")]
                unsafe {
                    popup_menu.show_context_menu_for_nsview(invisible_window.ns_view(), None);
                }
            }
        }
    });
}
