import { invoke } from "@tauri-apps/api/core";
import type { QueueItem, QueueSnapshot } from "../stores/queueStore";
import type { Slot } from "../stores/slotsStore";
import type { Profile } from "../stores/profilesStore";

export const Commands = {
  getQueueState: () => invoke<QueueSnapshot>("get_queue_state"),
  removeQueueItem: (id: string) => invoke<void>("remove_queue_item", { id }),
  clearQueue: () => invoke<void>("clear_queue"),
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
  searchHistory: (query: string) => invoke<QueueItem[]>("search_history", { query }),
};
