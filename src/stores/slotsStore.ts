import { create } from "zustand";
import type { ContentType } from "./queueStore";
import { Commands } from "../lib/commands";

export interface Slot {
  id: string;
  name: string;
  content: string;
  type: ContentType;
  updatedAt: number;
}

// Backend-owned like the queue: mutations invoke commands, and the store
// refreshes from "slots-updated" events (see useSlotsSync).
interface SlotsStore {
  slots: Slot[];
  applySlots: (slots: Slot[]) => void;
  createSlot: (name: string) => Promise<void>;
  setSlotContent: (id: string, content: string) => Promise<void>;
  renameSlot: (id: string, name: string) => Promise<void>;
  deleteSlot: (id: string) => Promise<void>;
  captureSlot: (id: string) => Promise<void>;
}

export const useSlotsStore = create<SlotsStore>((set) => ({
  slots: [],

  applySlots: (slots) => set({ slots }),

  createSlot: (name) => Commands.createSlot(name),
  setSlotContent: (id, content) => Commands.setSlotContent(id, content),
  renameSlot: (id, name) => Commands.renameSlot(id, name),
  deleteSlot: (id) => Commands.deleteSlot(id),
  captureSlot: (id) => Commands.captureSlot(id),
}));
