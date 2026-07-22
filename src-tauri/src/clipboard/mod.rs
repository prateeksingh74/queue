// Phase 2: Clipboard monitoring service.
// Polls NSPasteboard (macOS) / GetClipboardSequenceNumber (Windows) for changes,
// classifies content via intelligence::classify, and emits "clipboard-change" Tauri events.
