import { useSettingsStore } from "../../stores/settingsStore";

export default function SettingsView() {
  const { hotkeys, maxHistoryItems, setMaxHistoryItems, resetHotkeys } = useSettingsStore();

  return (
    <div className="p-6 max-w-2xl">
      <div className="mb-6">
        <h2 className="text-lg font-semibold text-white">Settings</h2>
        <p className="text-xs text-neutral-500 mt-0.5">Hotkeys and preferences.</p>
      </div>

      <section className="mb-8">
        <div className="flex items-center justify-between mb-3">
          <h3 className="text-sm font-medium text-neutral-300">Hotkeys</h3>
          <button
            onClick={resetHotkeys}
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
              <span className="text-xs font-mono px-2 py-1 rounded border border-neutral-700 bg-neutral-800 text-neutral-400">
                {hotkey.keys || "Not set"}
              </span>
            </div>
          ))}
        </div>
        <p className="text-xs text-neutral-600 mt-2">Shortcut recording arrives in Phase 4.</p>
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
