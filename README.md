# Queue

Queue is a macOS desktop clipboard workspace. Copy several pieces of text, browse them like terminal command history, and paste the selected item as many times as you need. Pasting keeps the selection; moving to another item is a separate action.

Use it to collect research snippets, reuse replies, or fill forms from saved collections. Queue includes:

- **Queue:** an ordered list of captured text with a selected-item preview.
- **Slots:** named storage for frequently used values such as an email or address.
- **Profiles:** reusable collections that replace the queue with a predefined set of items.
- **History:** searchable captured text, with pinning and restoration to the queue.
- **Settings:** customizable global shortcuts and a history retention limit.

Built with Tauri 2, Rust, React 19, TypeScript, Tailwind CSS 4, Zustand, and SQLite. The current target platform is **macOS**. Clipboard capture supports **text**, not images or files.

## Setup

### Prerequisites

Install these before running the project:

| Requirement | Details |
| --- | --- |
| Node.js and npm | Node **22.12+**, or Node **20.19+ within the 20.x series**, matching Vite's engine requirement. |
| Rust and Cargo | Install the stable Rust toolchain using [rustup](https://rustup.rs/). |
| Xcode Command Line Tools | Run `xcode-select --install` if they are not already installed. |

Check your tools:

```sh
node --version
npm --version
rustc --version
cargo --version
xcode-select -p
```

### Install and start

From the project root, the directory containing `package.json`:

```sh
npm install
npm run tauri dev
```

This starts Vite, compiles the Rust backend, and opens the desktop app. The first Rust build can take several minutes. Complete onboarding on the first launch, and keep the development terminal running while you use the app.

**Use `npm run tauri dev` for the functional app.** `npm run dev` only starts the frontend in a browser; it does not provide the native clipboard, database, or global-shortcut backend.

### Allow pasting on macOS

Queue pastes by placing the selected text on the system clipboard and sending Command+V to the focused application. This requires Accessibility permission.

1. Open **System Settings → Privacy & Security → Accessibility**.
2. Enable Queue. During development, macOS may associate the permission with the terminal or app hosting the process; enable the relevant entry shown by macOS.
3. Fully quit and restart Queue after changing permission.

Copy capture and queue navigation can work even when paste permission is missing. Queue checks permission before pasting and displays an error when it is unavailable; do not rely on an automatic permission prompt.

## First use

1. Start Queue with `npm run tauri dev` and finish onboarding.
2. In a text editor, copy three different snippets with **Command+C**, pausing briefly between copies. They should appear in Queue within about a second.
3. Click any item in Queue to select it. Its full text appears in the preview.
4. Click into a destination text field in another app.
5. Press **Command+Shift+V**, then **release all keys**. Queue waits for the shortcut modifiers to be released before sending the paste.
6. Press and release the same shortcut again to paste the same item again.
7. Use **Command+Shift+Up/Down** to move through saved queue items, then paste the new selection.

The destination text field must have focus. Navigation alone does not paste or change the system clipboard. Pasting replaces the clipboard's current text with the selected item.

## Controls

### Global keyboard shortcuts

These defaults work while Queue is running, including when another app has focus. **⌘ = Command**, **⇧ = Shift**. Custom bindings in Settings take precedence over these defaults.

| Action | Default shortcut | Behavior |
| --- | --- | --- |
| Paste selected item | **⌘⇧V** | Paste without removing the item or advancing the selection. Release all shortcut keys to complete the paste. |
| Previous item | **⌘⇧↑** | Select the previous queue item. Stops at the first item. |
| Next item | **⌘⇧↓** | Select the next queue item. Stops at the last item. |
| Add Clipboard to Queue | **⌘⇧C** | Add the current clipboard text, even when automatic recording is paused. Can add a duplicate. |
| Open Queue | **⌘⇧Q** | Show and focus the Queue window. |
| Clear Queue | **Unbound** | Assign a shortcut in Settings if desired. Removes queue items and resets the selection. |

Plain arrow keys are not registered globally. The arrow shortcuts browse the **active queue**; to reuse an entry from the separate History view, restore it to the queue first.

### Queue view

| Control | Action |
| --- | --- |
| Click an item's text | Select it for the next paste. The selected row is highlighted. |
| Selected-item preview | Read the full text; longer content can be scrolled. |
| **↑ Previous / ↓ Next** | Move the selection, stopping at the list boundaries. |
| **Restart** | Select the first item. Shown when the selection is beyond the first item. |
| **● Recording / ❚❚ Paused** | Toggle automatic clipboard capture. Recording starts enabled when the app launches. |
| Row **×** button on hover | Remove that item from the queue. |
| **Clear All** | Empty the queue and reset the selection. History remains available. |

Queue appends newly detected clipboard text in copy order and assigns content-type badges such as email, URL, phone, or text. Consecutive copies with unchanged clipboard text are ignored by the monitor; this is not deduplication of the entire queue. Text already on the clipboard at startup is not automatically captured—use **⌘⇧C** to add it explicitly.

### Slots view

| Control | Action |
| --- | --- |
| **+ New Slot** | Create a named storage slot. |
| Double-click a slot's name | Rename it; click away or press Enter to save. |
| Double-click its content | Edit the text; click away to save. |
| **+** button on hover | Replace the slot's value with the current clipboard text. |
| **×** button on hover | Delete the slot. |

Slots persist across restarts. The current Slots interface provides storage and editing; it does not expose a slot-paste button or per-slot global shortcuts.

### Profiles view

| Control | Action |
| --- | --- |
| **+ New Profile** | Create a collection. |
| Chevron beside a profile | Expand or collapse its items. |
| Double-click the profile name | Rename it; click away or press Enter to save. |
| New **Label** and **Content** fields | Enter an item, then click **Add** or press Enter in Content. Both fields are required. |
| Existing label/content fields | Edit in place; click away to save. |
| Item **×** button on hover | Remove that item from the profile. |
| **Load** | **Replace the current queue** with the profile's items in order and select the first item. |
| Duplicate icon on hover | Create an independent copy of the profile and its items. |
| Profile **×** button on hover | Delete the profile. |

For example, create Name, Email, and Phone entries, then click **Load**. Focus the first form field and paste. Move to the next field yourself, select the next queue item with **⌘⇧↓**, and paste again. Queue does not automatically tab between fields or advance after pasting.

### History view

| Control | Action |
| --- | --- |
| Search box | Filter saved history by text. |
| Bookmark icon | Pin or unpin an item. Pinned items sort first, then entries sort newest first. |
| Restore arrow on hover | Append that item's text to the queue. Select it in Queue before pasting. |
| **×** button on hover | Delete an individual history entry, including a pinned entry. |
| **Clear History** | Delete unpinned history entries. Leaves the active queue intact. |

History records captured text, including explicit clipboard additions. Paused automatic recording does not capture new copies into history. The default retention limit is **500 unpinned items**; pinned entries are exempt. Each history search returns up to **200 results**.

### Settings

- Click a shortcut chip, then press a new combination containing a modifier. It takes effect globally and is saved across restarts.
- While recording a shortcut, **Esc** cancels; **Backspace/Delete** removes the binding.
- **Reset to defaults** restores the shortcuts listed above.
- **Maximum history items** changes the unpinned retention limit and trims older entries. A value of `0` retains no unpinned entries after trimming.

## Build a standalone app

From the project root:

```sh
npm run tauri build
```

The build compiles the frontend and Rust backend and creates release bundles under:

```text
src-tauri/target/release/bundle/
```

The application is normally at `src-tauri/target/release/bundle/macos/Queue.app`; disk-image output is under `bundle/dmg/`. A built app runs without the development server. Grant the built app its own Accessibility permission if needed.

## Troubleshooting

| Symptom | What to check |
| --- | --- |
| Browser UI opens but native features fail | Start the desktop app with `npm run tauri dev`, rather than `npm run dev`. |
| Frontend build reports an unsupported Node version | Check the Node requirement above and the version reported by `node --version`. |
| Copying does not add items | Confirm recording is enabled, copy nonempty text, and allow time for the 300 ms polling interval. Images and files are ignored. |
| Navigation works but nothing pastes | Focus an editable field in another app, release all shortcut keys, check Accessibility permission, and inspect any error displayed by Queue. |
| Error asks you to release shortcut keys | Release Command, Shift, Control, and Option, then try again. The modifier wait times out after about two seconds. |
| Shortcut has no effect | Inspect its binding in Settings and check for conflicts with other apps or macOS shortcuts. Check the development terminal for registration errors. |
| An old build says a paste is already in progress | Fully quit the old Queue process and restart the current code. Current code ignores overlapping requests rather than displaying that error. |
| Changes are not reflected in the running app | Fully stop and restart `npm run tauri dev`; make sure you are not using a previously bundled app. |
| Native keyboard-event test cannot create an event source | Run it in a normal logged-in macOS desktop session with access to WindowServer; a restricted sandbox or headless runner may prevent event creation. |

During development, paste shortcut receipt and failures are logged in the terminal running Tauri. Successful event creation does not prove that a destination app accepted the paste; verify the workflow in a text editor.

## Saved data

Queue items, the selected position, slots, profiles, history, and custom shortcuts are stored locally in SQLite:

```text
~/Library/Application Support/com.prateeksingh.queue/queue.db
```

Onboarding completion and the frontend history-limit preference are stored in the webview's local storage. Fully quit Queue before copying the database for a backup. Pausing capture does not delete existing saved text.

## Development checks

Run these from the project root:

```sh
npm run build
cargo check --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml
```

The frontend build runs TypeScript checking and Vite bundling. Rust tests cover classification, saved-hotkey migration, legacy cursor restoration, overlapping-paste handling, and native macOS paste-event construction. The native event test creates and inspects events without posting keystrokes. These checks do not replace manually testing pasting into another app.

For a quick manual check, capture three texts, browse both directions, paste one item twice, paste a different item, then quit and relaunch to verify the queue and selection persist.

### Project layout

```text
src/
  components/         Queue, Slots, Profiles, History, Settings, onboarding
  stores/             Zustand state and frontend preferences
  hooks/              Backend event subscriptions
  lib/commands.ts     Tauri command wrappers
src-tauri/
  src/clipboard/      Text clipboard monitor (arboard, 300 ms polling)
  src/paste/          Native macOS Command+V events and modifier-release wait
  src/hotkeys/        Global shortcut registration and actions
  src/intelligence/   Text classification
  src/db/             SQLite schema and persistence
  src/commands.rs     Frontend command handlers
  src/state.rs        Shared state and serialized models
  tauri.conf.json     Desktop window and build configuration
```

Rust owns the queue and other persisted application data; frontend stores mirror it through Tauri commands and update events. Paste uses native Core Graphics events on macOS, with self-capture suppression so Queue's own clipboard writes are skipped by the monitor.
