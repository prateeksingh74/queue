# Queue

*The programmable clipboard for modern workflows.*

Queue transforms the single-value system clipboard into a persistent, ordered workspace. Copy multiple items and paste them back sequentially, keep frequently used values in permanent slots, and fill entire forms from reusable profiles — all driven by global hotkeys that work in any app.

Built with **Tauri 2** (Rust backend) + **React 19** + **TypeScript** + **Tailwind CSS 4** + **Zustand**, with **SQLite** persistence. Currently macOS-only.

---

## Getting Started

### Prerequisites

- Node.js 18+ and npm
- Rust toolchain (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Xcode Command Line Tools (`xcode-select --install`)

### Run in development

```sh
npm install
npm run tauri dev
```

The first launch compiles the Rust backend (~1–2 minutes); subsequent launches are fast.

### Build a release app

```sh
npm run tauri build
```

The bundled `.app` lands in `src-tauri/target/release/bundle/`.

### macOS Accessibility permission

Pasting works by simulating ⌘V, which requires the **Accessibility** permission
(System Settings → Privacy & Security → Accessibility). macOS prompts on the first
paste attempt. In dev mode, the permission may need to be granted to your **terminal
app** rather than Queue itself — approve whichever app appears in the prompt.

Until permission is granted, everything else works; only paste simulation fails.

---

## Demo Walkthrough

Follow these in order — later steps use data from earlier ones.

### 1. Onboarding (first launch only)

A five-step flow: Welcome → What Queue Does → Hotkeys → Preferences → Finish.
**Expected:** after finishing, the main app appears, and onboarding never shows again
(quit and relaunch to confirm).

### 2. Queue capture — the core feature

Open Notes (or any app) and copy three different lines of text.

**Expected:**
- Each copy appears in the Queue view within ~1 second, numbered in order.
- Content is auto-classified with a colored badge: `email` (blue), `url` (purple),
  `phone` (green), `date`/`address`, or plain `text`. Try copying `test@email.com`
  and `https://example.com`.
- Copying identical text twice adds it only once.
- Item 1 has an indigo highlight — that's the paste cursor.
- Clicking **● Recording** pauses capture (copies are ignored until resumed).

### 3. Sequential paste

Click into any text field, then press **⌘⇧V** repeatedly.

**Expected:**
- Each press pastes the next queue item in order; the highlight in Queue advances.
- Pasted items do **not** re-enter the queue (self-capture suppression).
- After the last item, further presses do nothing.
- The **Restart** button resets the cursor to item 1.

### 4. Persistence

Quit fully (⌘Q) and relaunch.

**Expected:** queue items, cursor position, slots, and profiles all survive. Data
lives in `~/Library/Application Support/com.prateeksingh.queue/queue.db`.

### 5. Slots

Create a slot, double-click its name to rename it, double-click the content area to
type a value (e.g., your email).

**Expected:** content is saved on click-away and classified. The **+** button on
hover captures the current clipboard into the slot. Slots survive restart.

### 6. Profiles — the marquee demo

Create a profile, expand it with the chevron, and add items in order:
`Name / John Smith`, `Email / john@email.com`, `Phone / 555-1234`. Then press **Load**.

**Expected:** the queue is replaced with those items in order. Click into a form and
press ⌘⇧V three times — name, email, phone are pasted sequentially. This is the
job-application workflow. **Duplicate** (copy icon) deep-copies a profile.

### 7. History

**Expected:**
- Everything ever copied is listed, newest first, with relative timestamps.
- The search box filters live.
- The bookmark icon pins an item (pinned items sort to the top and survive
  **Clear History**).
- The restore arrow re-adds an item to the end of the queue.

### 8. Hotkey recorder (Settings)

Click any shortcut chip.

**Expected:** it pulses ("Press keys…"). Press a new combo (must include a modifier)
— it takes effect globally, immediately. Esc cancels; Backspace clears the binding;
**Reset to defaults** restores all four. Custom bindings survive restart.

### Default hotkeys

| Action | Shortcut |
|--------|----------|
| Queue Paste (paste next item) | ⌘⇧V |
| Add Clipboard to Queue | ⌘⇧C |
| Open Queue window | ⌘⇧Q |
| Clear Queue | unbound |

---

## Architecture

The Rust backend owns all state; the React frontend is a thin mirror.

- **State + persistence:** `AppState` (Mutex-guarded) backed by SQLite (`rusqlite`,
  bundled). Six tables: queue_items, slots, profiles, profile_items, history, settings.
- **Clipboard monitor:** background thread polls via `arboard` every 300 ms,
  classifies content with regex (`intelligence` module), appends to queue + history.
- **Paste engine:** `enigo` writes to the clipboard and simulates ⌘V; a suppression
  flag prevents the monitor from re-capturing Queue's own pastes.
- **Sync model:** every mutation goes through a Tauri command; the backend emits
  `queue-updated` / `slots-updated` / `profiles-updated` / `history-updated` events
  that refresh the Zustand stores.

```
src/                  React frontend (components, stores, hooks)
src-tauri/src/
  clipboard/          clipboard monitor thread
  paste/              keyboard-simulation paste engine
  hotkeys/            global shortcut registration + actions
  intelligence/       regex content classification (unit-tested)
  db/                 SQLite layer
  commands.rs         Tauri IPC commands
  state.rs            shared AppState + models
```

## Development

```sh
npm run build        # type-check + bundle frontend
cargo check          # (in src-tauri/) compile-check backend
cargo test           # run backend unit tests
```

Branching: `main` holds tagged releases (`v1.0.0` = MVP), `dev` is the integration
branch, and each phase was built on a `phase/N-name` branch squash-merged into `dev`.
