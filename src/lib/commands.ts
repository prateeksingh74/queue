import { invoke } from "@tauri-apps/api/core";
import type { ContentType, QueueSnapshot } from "../stores/queueStore";
import type { Slot } from "../stores/slotsStore";
import type { Profile } from "../stores/profilesStore";
import type { HotkeyBinding } from "../stores/settingsStore";

export interface HistoryItem {
  id: string;
  content: string;
  type: ContentType;
  copiedAt: number;
  pinned: boolean;
}

export const Commands = {
  getQueueState: () => invoke<QueueSnapshot>("get_queue_state"),
  removeQueueItem: (id: string) => invoke<void>("remove_queue_item", { id }),
  clearQueue: () => invoke<void>("clear_queue"),
  selectQueueItem: (position: number) => invoke<void>("select_queue_item", { position }),
  resetCursor: () => invoke<void>("reset_cursor"),
  setMonitoring: (enabled: boolean) => invoke<void>("set_monitoring", { enabled }),
  queuePaste: () => invoke<void>("queue_paste"),
  getSlots: () => invoke<Slot[]>("get_slots"),
  createSlot: (name: string) => invoke<void>("create_slot", { name }),
  setSlotContent: (id: string, content: string) =>
    invoke<void>("set_slot_content", { id, content }),
  renameSlot: (id: string, name: string) => invoke<void>("rename_slot", { id, name }),
  deleteSlot: (id: string) => invoke<void>("delete_slot", { id }),
  captureSlot: (id: string) => invoke<void>("capture_slot", { id }),
  pasteSlot: (id: string) => invoke<void>("paste_slot", { id }),
  getProfiles: () => invoke<Profile[]>("get_profiles"),
  createProfile: (name: string) => invoke<void>("create_profile", { name }),
  renameProfile: (id: string, name: string) =>
    invoke<void>("rename_profile", { id, name }),
  deleteProfile: (id: string) => invoke<void>("delete_profile", { id }),
  duplicateProfile: (id: string) => invoke<void>("duplicate_profile", { id }),
  addProfileItem: (profileId: string, label: string, content: string) =>
    invoke<void>("add_profile_item", { profileId, label, content }),
  updateProfileItem: (profileId: string, itemId: string, label: string, content: string) =>
    invoke<void>("update_profile_item", { profileId, itemId, label, content }),
  removeProfileItem: (profileId: string, itemId: string) =>
    invoke<void>("remove_profile_item", { profileId, itemId }),
  loadProfile: (id: string) => invoke<void>("load_profile", { id }),
  searchHistory: (query: string) => invoke<HistoryItem[]>("search_history", { query }),
  setHistoryPinned: (id: string, pinned: boolean) =>
    invoke<void>("set_history_pinned", { id, pinned }),
  deleteHistoryItem: (id: string) => invoke<void>("delete_history_item", { id }),
  clearHistory: () => invoke<void>("clear_history"),
  restoreHistoryItem: (id: string) => invoke<void>("restore_history_item", { id }),
  getHotkeys: () => invoke<HotkeyBinding[]>("get_hotkeys"),
  setHotkey: (action: string, keys: string) =>
    invoke<HotkeyBinding[]>("set_hotkey", { action, keys }),
  resetHotkeys: () => invoke<HotkeyBinding[]>("reset_hotkeys"),
  setMaxHistoryItems: (maxItems: number) =>
    invoke<void>("set_max_history_items", { maxItems }),
};
