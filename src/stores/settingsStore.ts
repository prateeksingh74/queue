import { create } from "zustand";
import { persist } from "zustand/middleware";
import { Commands } from "../lib/commands";

export interface HotkeyBinding {
  action: string;
  label: string;
  keys: string;
}

// Hotkeys live in the Rust backend (they must be registered with the OS);
// simple UI preferences persist locally via zustand.
interface SettingsStore {
  hotkeys: HotkeyBinding[];
  maxHistoryItems: number;
  onboardingComplete: boolean;
  loadHotkeys: () => Promise<void>;
  setHotkey: (action: string, keys: string) => Promise<string | null>;
  resetHotkeys: () => Promise<void>;
  setMaxHistoryItems: (v: number) => void;
  setOnboardingComplete: (v: boolean) => void;
}

export const useSettingsStore = create<SettingsStore>()(
  persist(
    (set) => ({
      hotkeys: [],
      maxHistoryItems: 500,
      onboardingComplete: false,

      loadHotkeys: async () => {
        set({ hotkeys: await Commands.getHotkeys() });
      },

      // Returns an error message on registration failure (e.g. conflict), null on success.
      setHotkey: async (action, keys) => {
        try {
          set({ hotkeys: await Commands.setHotkey(action, keys) });
          return null;
        } catch (e) {
          return String(e);
        }
      },

      resetHotkeys: async () => {
        set({ hotkeys: await Commands.resetHotkeys() });
      },

      setMaxHistoryItems: (v) => {
        set({ maxHistoryItems: v });
        Commands.setMaxHistoryItems(v).catch(console.error);
      },

      setOnboardingComplete: (v) => set({ onboardingComplete: v }),
    }),
    {
      name: "queue-settings",
      partialize: (state) => ({
        maxHistoryItems: state.maxHistoryItems,
        onboardingComplete: state.onboardingComplete,
      }),
    }
  )
);
