import { create } from "zustand";
import type { ContentType } from "./queueStore";
import { Commands } from "../lib/commands";

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
}

// Backend-owned: mutations invoke commands, store refreshes from
// "profiles-updated" events (see useProfilesSync).
interface ProfilesStore {
  profiles: Profile[];
  applyProfiles: (profiles: Profile[]) => void;
  createProfile: (name: string) => Promise<void>;
  renameProfile: (id: string, name: string) => Promise<void>;
  deleteProfile: (id: string) => Promise<void>;
  duplicateProfile: (id: string) => Promise<void>;
  addItem: (profileId: string, label: string, content: string) => Promise<void>;
  updateItem: (
    profileId: string,
    itemId: string,
    label: string,
    content: string
  ) => Promise<void>;
  removeItem: (profileId: string, itemId: string) => Promise<void>;
  loadProfile: (id: string) => Promise<void>;
}

export const useProfilesStore = create<ProfilesStore>((set) => ({
  profiles: [],

  applyProfiles: (profiles) => set({ profiles }),

  createProfile: (name) => Commands.createProfile(name),
  renameProfile: (id, name) => Commands.renameProfile(id, name),
  deleteProfile: (id) => Commands.deleteProfile(id),
  duplicateProfile: (id) => Commands.duplicateProfile(id),
  addItem: (profileId, label, content) =>
    Commands.addProfileItem(profileId, label, content),
  updateItem: (profileId, itemId, label, content) =>
    Commands.updateProfileItem(profileId, itemId, label, content),
  removeItem: (profileId, itemId) => Commands.removeProfileItem(profileId, itemId),
  loadProfile: (id) => Commands.loadProfile(id),
}));
