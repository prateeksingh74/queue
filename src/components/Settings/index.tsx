import { useEffect, useState } from "react";
import { useSettingsStore } from "../../stores/settingsStore";

// Maps a keydown event to a Tauri accelerator string, e.g. "CommandOrControl+Shift+V".
function eventToAccelerator(e: React.KeyboardEvent): string | null {
  const key = e.key;
  if (["Meta", "Control", "Alt", "Shift"].includes(key)) return null; // modifier alone

  const parts: string[] = [];
  if (e.metaKey) parts.push("CommandOrControl");
  if (e.ctrlKey && !e.metaKey) parts.push("Control");
  if (e.altKey) parts.push("Alt");
  if (e.shiftKey) parts.push("Shift");
  if (parts.length === 0) return null; // require at least one modifier for global keys

  const named: Record<string, string> = {
    " ": "Space",
    ArrowUp: "Up",
    ArrowDown: "Down",
    ArrowLeft: "Left",
    ArrowRight: "Right",
  };
  parts.push(named[key] ?? (key.length === 1 ? key.toUpperCase() : key));
  return parts.join("+");
}

export default function SettingsView() {
  const { hotkeys, maxHistoryItems, setMaxHistoryItems, loadHotkeys, setHotkey, resetHotkeys } =
    useSettingsStore();
  const [recording, setRecording] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadHotkeys();
  }, [loadHotkeys]);

  const handleKeyDown = async (e: React.KeyboardEvent) => {
    if (!recording) return;
    e.preventDefault();
    e.stopPropagation();

    if (e.key === "Escape") {
      setRecording(null);
      return;
    }
    if (e.key === "Backspace" || e.key === "Delete") {
      await setHotkey(recording, "");
      setRecording(null);
      return;
    }
    const accelerator = eventToAccelerator(e);
    if (!accelerator) return;

    const err = await setHotkey(recording, accelerator);
    setError(err);
    setRecording(null);
  };

  return (
    <div className="p-6 max-w-2xl" onKeyDown={handleKeyDown}>
      <div className="mb-6">
        <h2 className="text-lg font-semibold text-white">Settings</h2>
        <p className="text-xs text-neutral-500 mt-0.5">Hotkeys and preferences.</p>
      </div>

      <section className="mb-8">
        <div className="flex items-center justify-between mb-3">
          <h3 className="text-sm font-medium text-neutral-300">Hotkeys</h3>
          <button
            onClick={() => resetHotkeys()}
            className="text-xs text-neutral-500 hover:text-neutral-300 transition-colors cursor-default"
          >
            Reset to defaults
          </button>
        </div>
        <div className="space-y-1.5">
          {hotkeys.map((hotkey) => (
            <div
              key={hotkey.action}
              className="flex items-center justify-between px-3 py-2.5 rounded-lg bg-neutral-900 border border-neutral-800"
            >
              <p className="text-sm text-neutral-200">{hotkey.label}</p>
              <button
                onClick={() => {
                  setError(null);
                  setRecording(recording === hotkey.action ? null : hotkey.action);
                }}
                className={`text-xs font-mono px-2 py-1 rounded border transition-colors cursor-default ${
                  recording === hotkey.action
                    ? "border-indigo-500 bg-indigo-500/10 text-indigo-300 animate-pulse"
                    : "border-neutral-700 bg-neutral-800 text-neutral-400 hover:border-neutral-600"
                }`}
              >
                {recording === hotkey.action
                  ? "Press keys… (Esc to cancel, ⌫ to clear)"
                  : hotkey.keys.replace("CommandOrControl", "⌘") || "Not set"}
              </button>
            </div>
          ))}
        </div>
        {error && <p className="text-xs text-red-400 mt-2">{error}</p>}
        <p className="text-xs text-neutral-600 mt-2">
          Click a shortcut, then press the new key combination. Global shortcuts need at
          least one modifier key.
        </p>
      </section>

      <section>
        <h3 className="text-sm font-medium text-neutral-300 mb-3">History</h3>
        <div className="flex items-center justify-between px-3 py-2.5 rounded-lg bg-neutral-900 border border-neutral-800">
          <p className="text-sm text-neutral-200">Maximum history items</p>
          <input
            type="number"
            value={maxHistoryItems}
            onChange={(e) => setMaxHistoryItems(Number(e.target.value) || 0)}
            className="w-20 bg-neutral-800 border border-neutral-700 rounded px-2 py-1 text-sm text-neutral-200 text-right focus:outline-none focus:border-neutral-600"
          />
        </div>
      </section>
    </div>
  );
}
