import { invoke } from "@tauri-apps/api/core";
import type { QueueItem } from "../stores/queueStore";
import type { Slot } from "../stores/slotsStore";
import type { Profile } from "../stores/profilesStore";

export const Commands = {
  getQueue: () => invoke<QueueItem[]>("get_queue"),
  clearQueue: () => invoke<void>("clear_queue"),
  getSlots: () => invoke<Slot[]>("get_slots"),
  getProfiles: () => invoke<Profile[]>("get_profiles"),
  searchHistory: (query: string) => invoke<QueueItem[]>("search_history", { query }),
};
