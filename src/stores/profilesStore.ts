import { create } from "zustand";
import type { ContentType } from "./queueStore";

export interface ProfileItem {
  id: string;
  label: string;
  content: string;
  type: ContentType;
}

export interface Profile {
  id: string;
  name: string;
  items: ProfileItem[];
  createdAt: number;
  updatedAt: number;
}

interface ProfilesStore {
  profiles: Profile[];
  createProfile: (name: string) => void;
  deleteProfile: (id: string) => void;
  updateProfile: (id: string, updates: Partial<Pick<Profile, "name" | "items">>) => void;
  addProfileItem: (profileId: string, label: string, content: string, type?: ContentType) => void;
  removeProfileItem: (profileId: string, itemId: string) => void;
}

export const useProfilesStore = create<ProfilesStore>((set) => ({
  profiles: [],

  createProfile: (name) =>
    set((state) => ({
      profiles: [
        ...state.profiles,
        { id: crypto.randomUUID(), name, items: [], createdAt: Date.now(), updatedAt: Date.now() },
      ],
    })),

  deleteProfile: (id) =>
    set((state) => ({ profiles: state.profiles.filter((p) => p.id !== id) })),

  updateProfile: (id, updates) =>
    set((state) => ({
      profiles: state.profiles.map((p) =>
        p.id === id ? { ...p, ...updates, updatedAt: Date.now() } : p
      ),
    })),

  addProfileItem: (profileId, label, content, type = "text") =>
    set((state) => ({
      profiles: state.profiles.map((p) =>
        p.id === profileId
          ? {
              ...p,
              items: [...p.items, { id: crypto.randomUUID(), label, content, type }],
              updatedAt: Date.now(),
            }
          : p
      ),
    })),

  removeProfileItem: (profileId, itemId) =>
    set((state) => ({
      profiles: state.profiles.map((p) =>
        p.id === profileId
          ? { ...p, items: p.items.filter((i) => i.id !== itemId), updatedAt: Date.now() }
          : p
      ),
    })),
}));
