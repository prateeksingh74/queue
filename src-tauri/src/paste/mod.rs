// Keyboard simulation engine. Writes content to the OS clipboard, then
// simulates Cmd+V in the frontmost app. Requires the Accessibility
// permission on macOS (System Settings > Privacy & Security > Accessibility).
#[cfg(not(target_os = "macos"))]
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

use crate::state::AppState;

// Small pause between clipboard write and keystroke so the target app
// sees the new clipboard contents.
const CLIPBOARD_SETTLE: Duration = Duration::from_millis(80);

pub fn paste_text(state: &AppState, content: &str) -> Result<(), String> {
    let _guard = match state.paste_lock.try_lock() {
        Ok(guard) => guard,
        // Shortcut repeats must not steal focus from the in-flight paste.
        Err(std::sync::TryLockError::WouldBlock) => return Ok(()),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("Paste worker failed. Restart Queue and try again.".into()),
    };
    wait_for_modifiers()?;
    #[cfg(target_os = "macos")]
    unsafe {
        if !AXIsProcessTrusted() {
            return Err("Allow Queue (or your terminal in development) in System Settings → Privacy & Security → Accessibility, then restart Queue.".into());
        }
    }
    // Mark our own clipboard write so the monitor doesn't re-enqueue it.
    *state.suppress.lock().unwrap() = Some(content.to_string());

    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(content).map_err(|e| e.to_string())?;
    std::thread::sleep(CLIPBOARD_SETTLE);

    simulate_paste()?;
    Ok(())
}

// Paste the selection without consuming it, so repeated pastes are supported.
pub fn paste_next(app: &AppHandle) -> Result<(), String> {
    let state = app.state::<AppState>();
    let content = {
        let queue = state.queue.lock().unwrap();
        let cursor = state.cursor.lock().unwrap();
        queue.get(*cursor).map(|item| item.content.clone())
            .ok_or_else(|| "Queue is empty. Copy some text first.".to_string())?
    };
    paste_text(&state, &content)?;
    let _ = app.emit("queue-error", "");
    Ok(())
}

#[cfg(target_os = "macos")]
#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn CGEventSourceFlagsState(state: i32) -> u64;
    fn AXIsProcessTrusted() -> bool;
}

// Wait on a worker thread until the user's shortcut modifiers are released.
// Otherwise Shift/Option can change the simulated Cmd+V into another command.
fn wait_for_modifiers() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        for _ in 0..200 {
            if unsafe { CGEventSourceFlagsState(1) } & 0x1e0000 == 0 {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        return Err("Release the shortcut keys and try pasting again.".into());
    }
    #[cfg(not(target_os = "macos"))]
    Ok(())
}

#[cfg(target_os = "macos")]
fn paste_events() -> Result<Vec<core_graphics::event::CGEvent>, String> {
    use core_graphics::event::{CGEvent, CGEventFlags};
    use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

    let source = CGEventSource::new(CGEventSourceStateID::Private)
        .map_err(|_| "Could not create the macOS keyboard event source.".to_string())?;
    // Physical macOS keycodes: left Command = 55, V = 9.
    // Set flags explicitly: posting a modifier key does not reliably update
    // the flags inherited by subsequently created synthetic events.
    [(55, true, true), (9, true, true), (9, false, true), (55, false, false)]
        .into_iter()
        .map(|(key, down, command)| {
            let event = CGEvent::new_keyboard_event(source.clone(), key, down)
                .map_err(|_| "Could not create a macOS paste key event.".to_string())?;
            event.set_flags(if command {
                CGEventFlags::CGEventFlagCommand
            } else {
                CGEventFlags::CGEventFlagNull
            });
            Ok(event)
        }).collect()
}

#[cfg(target_os = "macos")]
fn simulate_paste() -> Result<(), String> {
    use core_graphics::event::CGEventTapLocation;
    // Allocate all events before posting any so creation failure cannot leave
    // a synthetic modifier held down.
    for event in paste_events()? {
        event.post(CGEventTapLocation::HID);
        std::thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn simulate_paste() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;
    enigo.key(Key::Control, Direction::Press).map_err(|e| e.to_string())?;
    let result = enigo.key(Key::Unicode('v'), Direction::Click);
    let release = enigo.key(Key::Control, Direction::Release);
    result.map_err(|e| e.to_string())?;
    release.map_err(|e| e.to_string())
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::*;
    use core_graphics::event::{CGEventFlags, CGEventType, EventField};

    #[test]
    fn overlapping_paste_is_ignored_without_touching_clipboard_or_selection() {
        let state = AppState::new(crate::db::open(std::path::Path::new(":memory:")).unwrap());
        let _active_paste = state.paste_lock.lock().unwrap();
        assert!(paste_text(&state, "duplicate request").is_ok());
        assert!(state.suppress.lock().unwrap().is_none());
        assert_eq!(*state.cursor.lock().unwrap(), 0);
    }

    #[test]
    fn paste_events_include_command_on_v_and_release_it_at_the_end() {
        // Construct and inspect real native events without posting keystrokes.
        let events = paste_events().unwrap();
        assert_eq!(events.len(), 4);
        for (event, (key, down, command)) in events.iter().zip([
            (55, true, true), (9, true, true), (9, false, true), (55, false, false),
        ]) {
            assert_eq!(event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE), key);
            let expected_type = if key == 55 { CGEventType::FlagsChanged } else if down { CGEventType::KeyDown } else { CGEventType::KeyUp };
            assert_eq!(event.get_type() as u32, expected_type as u32);
            assert_eq!(event.get_flags().contains(CGEventFlags::CGEventFlagCommand), command);
            assert!(!event.get_flags().intersects(CGEventFlags::CGEventFlagShift | CGEventFlags::CGEventFlagAlternate | CGEventFlags::CGEventFlagControl));
        }
    }
}
