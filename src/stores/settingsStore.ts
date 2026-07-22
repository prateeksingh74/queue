import { create } from "zustand";

export type HotkeyAction =
  | "queue-paste"
  | "queue-add"
  | "open-queue"
  | "clear-queue"
  | "store-slot"
  | "paste-slot"
  | "load-profile";

export interface Hotkey {
  action: HotkeyAction;
  label: string;
  keys: string;
  defaultKeys: string;
}

const DEFAULT_HOTKEYS: Hotkey[] = [
  { action: "queue-paste", label: "Queue Paste", keys: "CommandOrControl+Shift+V", defaultKeys: "CommandOrControl+Shift+V" },
  { action: "queue-add", label: "Add to Queue", keys: "CommandOrControl+Shift+C", defaultKeys: "CommandOrControl+Shift+C" },
  { action: "open-queue", label: "Open Queue", keys: "CommandOrControl+Shift+Q", defaultKeys: "CommandOrControl+Shift+Q" },
  { action: "clear-queue", label: "Clear Queue", keys: "", defaultKeys: "" },
  { action: "store-slot", label: "Store to Slot", keys: "", defaultKeys: "" },
  { action: "paste-slot", label: "Paste from Slot", keys: "", defaultKeys: "" },
  { action: "load-profile", label: "Load Profile", keys: "", defaultKeys: "" },
];

interface SettingsStore {
  hotkeys: Hotkey[];
  launchAtLogin: boolean;
  showInMenuBar: boolean;
  maxHistoryItems: number;
  theme: "dark" | "light" | "system";
  setHotkey: (action: HotkeyAction, keys: string) => void;
  resetHotkeys: () => void;
  setLaunchAtLogin: (v: boolean) => void;
  setMaxHistoryItems: (v: number) => void;
}

export const useSettingsStore = create<SettingsStore>((set) => ({
  hotkeys: DEFAULT_HOTKEYS,
  launchAtLogin: false,
  showInMenuBar: true,
  maxHistoryItems: 500,
  theme: "dark",

  setHotkey: (action, keys) =>
    set((state) => ({
      hotkeys: state.hotkeys.map((h) => (h.action === action ? { ...h, keys } : h)),
    })),

  resetHotkeys: () => set({ hotkeys: DEFAULT_HOTKEYS }),

  setLaunchAtLogin: (v) => set({ launchAtLogin: v }),

  setMaxHistoryItems: (v) => set({ maxHistoryItems: v }),
}));
