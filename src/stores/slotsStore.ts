import { create } from "zustand";
import type { ContentType } from "./queueStore";

export interface Slot {
  id: string;
  name: string;
  content: string;
  type: ContentType;
  updatedAt: number;
}

interface SlotsStore {
  slots: Slot[];
  createSlot: (name: string) => void;
  setSlotContent: (id: string, content: string, type?: ContentType) => void;
  renameSlot: (id: string, name: string) => void;
  deleteSlot: (id: string) => void;
}

export const useSlotsStore = create<SlotsStore>((set) => ({
  slots: [],

  createSlot: (name) =>
    set((state) => ({
      slots: [
        ...state.slots,
        { id: crypto.randomUUID(), name, content: "", type: "text", updatedAt: Date.now() },
      ],
    })),

  setSlotContent: (id, content, type = "text") =>
    set((state) => ({
      slots: state.slots.map((s) =>
        s.id === id ? { ...s, content, type, updatedAt: Date.now() } : s
      ),
    })),

  renameSlot: (id, name) =>
    set((state) => ({
      slots: state.slots.map((s) => (s.id === id ? { ...s, name } : s)),
    })),

  deleteSlot: (id) =>
    set((state) => ({ slots: state.slots.filter((s) => s.id !== id) })),
}));
