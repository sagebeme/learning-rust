# Review: "Commands for the staff screen"

A teammate opens this PR: *"Adds the Tauri commands for the staff screen, and SMS sending so supervisors can text staff from the app. Permissions set up so nothing gets blocked."* Review the Rust and the capabilities file.

```rust
use std::sync::Mutex;

const AT_API_KEY: &str = "atsk_4f9c...e21"; // Africa's Talking

pub struct AppState {
    pub staff: Mutex<Vec<Staff>>,
}

#[tauri::command]
fn get_staff(state: tauri::State<'_, AppState>, staff_number: u32) -> Staff {
    let staff = state.staff.lock().unwrap();
    staff.iter().find(|s| s.staff_number == staff_number).unwrap().clone()
}

#[tauri::command]
fn mark_qualified(state: tauri::State<'_, AppState>, staff_number: u32, code: String) {
    let mut staff = state.staff.lock().unwrap();
    if let Some(s) = staff.iter_mut().find(|s| s.staff_number == staff_number) {
        s.codes.push(code);
    }
}

#[tauri::command]
fn send_sms(to: String, message: String) -> Result<(), String> {
    sms::send(AT_API_KEY, &to, &message).map_err(|e| e.to_string())
}
```

```json
{
  "identifier": "default",
  "windows": ["*"],
  "permissions": ["core:default", "shell:allow-execute", "fs:default", "allow-get-staff", "allow-mark-qualified", "allow-send-sms"]
}
```
