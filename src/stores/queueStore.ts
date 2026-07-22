import { create } from "zustand";

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

interface QueueStore {
  items: QueueItem[];
  cursor: number;
  addItem: (content: string, type?: ContentType) => void;
  removeItem: (id: string) => void;
  clear: () => void;
  advanceCursor: () => void;
  resetCursor: () => void;
}

export const useQueueStore = create<QueueStore>((set) => ({
  items: [],
  cursor: 0,

  addItem: (content, type = "text") =>
    set((state) => ({
      items: [
        ...state.items,
        { id: crypto.randomUUID(), content, type, addedAt: Date.now() },
      ],
    })),

  removeItem: (id) =>
    set((state) => {
      const items = state.items.filter((item) => item.id !== id);
      return { items, cursor: Math.min(state.cursor, Math.max(0, items.length - 1)) };
    }),

  clear: () => set({ items: [], cursor: 0 }),

  advanceCursor: () =>
    set((state) => ({
      cursor: state.cursor < state.items.length - 1 ? state.cursor + 1 : state.cursor,
    })),

  resetCursor: () => set({ cursor: 0 }),
}));
