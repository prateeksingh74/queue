import { create } from "zustand";
import { Commands } from "../lib/commands";

export type ContentType =
  | "text"
  | "email"
  | "phone"
  | "url"
  | "address"
  | "date"
  | "unknown";

export interface QueueItem {
  id: string;
  content: string;
  type: ContentType;
  addedAt: number;
}

export interface QueueSnapshot {
  items: QueueItem[];
  cursor: number;
  monitoring: boolean;
}

// The Rust backend owns queue state; every mutation goes through a command
// and the store is refreshed from "queue-updated" events (see useQueueSync).
interface QueueStore extends QueueSnapshot {
  applySnapshot: (snapshot: QueueSnapshot) => void;
  removeItem: (id: string) => Promise<void>;
  clear: () => Promise<void>;
  resetCursor: () => Promise<void>;
  selectItem: (position: number) => Promise<void>;
  setMonitoring: (enabled: boolean) => Promise<void>;
}

export const useQueueStore = create<QueueStore>((set) => ({
  items: [],
  cursor: 0,
  monitoring: true,

  applySnapshot: (snapshot) => set(snapshot),

  removeItem: (id) => Commands.removeQueueItem(id),
  clear: () => Commands.clearQueue(),
  resetCursor: () => Commands.resetCursor(),
  selectItem: (position) => Commands.selectQueueItem(position),
  setMonitoring: (enabled) => Commands.setMonitoring(enabled),
}));
