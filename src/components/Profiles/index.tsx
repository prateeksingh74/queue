import { useState } from "react";
import { useProfilesStore } from "../../stores/profilesStore";
import type { Profile } from "../../stores/profilesStore";

export default function ProfilesView() {
  const { profiles, createProfile } = useProfilesStore();
  const [expanded, setExpanded] = useState<string | null>(null);

  return (
    <div className="p-6">
      <div className="flex items-start justify-between mb-6">
        <div>
          <h2 className="text-lg font-semibold text-white">Profiles</h2>
          <p className="text-xs text-neutral-500 mt-0.5">
            Reusable collections that fill the queue in one action.
          </p>
        </div>
        <button
          onClick={() => createProfile(`Profile ${profiles.length + 1}`)}
          className="text-xs text-indigo-400 hover:text-indigo-300 transition-colors px-2.5 py-1.5 rounded border border-indigo-400/30 hover:border-indigo-400/50 cursor-default"
        >
          + New Profile
        </button>
      </div>

      {profiles.length === 0 ? (
        <div className="flex items-center justify-center h-48">
          <div className="text-center">
            <div className="w-10 h-10 rounded-lg bg-neutral-900 border border-neutral-800 flex items-center justify-center mx-auto mb-3">
              <svg className="w-5 h-5 text-neutral-700" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
            </div>
            <p className="text-neutral-400 text-sm font-medium">No profiles yet</p>
            <p className="text-neutral-600 text-xs mt-1">Create a profile for job applications, forms, and more</p>
          </div>
        </div>
      ) : (
        <div className="space-y-1.5">
          {profiles.map((profile) => (
            <ProfileCard
              key={profile.id}
              profile={profile}
              expanded={expanded === profile.id}
              onToggle={() =>
                setExpanded(expanded === profile.id ? null : profile.id)
              }
            />
          ))}
        </div>
      )}
    </div>
  );
}

function ProfileCard({
  profile,
  expanded,
  onToggle,
}: {
  profile: Profile;
  expanded: boolean;
  onToggle: () => void;
}) {
  const { renameProfile, deleteProfile, duplicateProfile, loadProfile, addItem, updateItem, removeItem } =
    useProfilesStore();
  const [editingName, setEditingName] = useState(false);
  const [newLabel, setNewLabel] = useState("");
  const [newContent, setNewContent] = useState("");

  const handleAdd = () => {
    if (!newLabel.trim() || !newContent.trim()) return;
    addItem(profile.id, newLabel.trim(), newContent);
    setNewLabel("");
    setNewContent("");
  };

  return (
    <div className="bg-neutral-900 border border-neutral-800 rounded-lg hover:border-neutral-700 transition-colors">
      <div className="group flex items-center gap-3 px-3 py-2.5">
        <button onClick={onToggle} className="text-neutral-600 cursor-default">
          <svg
            className={`w-3 h-3 transition-transform ${expanded ? "rotate-90" : ""}`}
            fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}
          >
            <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
          </svg>
        </button>

        <div className="flex-1 min-w-0">
          {editingName ? (
            <input
              autoFocus
              defaultValue={profile.name}
              onBlur={(e) => {
                if (e.target.value.trim()) renameProfile(profile.id, e.target.value.trim());
                setEditingName(false);
              }}
              onKeyDown={(e) => e.key === "Enter" && e.currentTarget.blur()}
              className="text-sm font-medium bg-neutral-800 border border-neutral-700 rounded px-1.5 py-0.5 text-neutral-200 focus:outline-none"
            />
          ) : (
            <p
              onDoubleClick={() => setEditingName(true)}
              className="text-sm font-medium text-neutral-200 truncate"
              title="Double-click to rename"
            >
              {profile.name}
            </p>
          )}
          <p className="text-xs text-neutral-600 mt-0.5">
            {profile.items.length} item{profile.items.length === 1 ? "" : "s"}
          </p>
        </div>

        <button
          onClick={() => loadProfile(profile.id)}
          disabled={profile.items.length === 0}
          className="text-xs text-indigo-400 hover:text-indigo-300 disabled:text-neutral-700 transition-colors px-2 py-1 rounded border border-indigo-400/30 disabled:border-neutral-800 cursor-default"
          title="Replace the queue with this profile's items"
        >
          Load
        </button>
        <div className="flex items-center gap-1.5 opacity-0 group-hover:opacity-100 transition-opacity">
          <button
            onClick={() => duplicateProfile(profile.id)}
            title="Duplicate"
            className="text-neutral-600 hover:text-neutral-300 transition-colors cursor-default"
          >
            <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
            </svg>
          </button>
          <button
            onClick={() => deleteProfile(profile.id)}
            className="text-neutral-600 hover:text-red-400 transition-all cursor-default"
          >
            <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>

      {expanded && (
        <div className="border-t border-neutral-800 px-3 py-2.5 space-y-1.5">
          {profile.items.map((item, index) => (
            <div key={item.id} className="group/item flex items-center gap-2">
              <span className="text-xs text-neutral-700 font-mono w-4 text-right flex-shrink-0">
                {index + 1}
              </span>
              <input
                defaultValue={item.label}
                onBlur={(e) =>
                  e.target.value !== item.label &&
                  updateItem(profile.id, item.id, e.target.value, item.content)
                }
                className="w-32 text-xs bg-neutral-800/50 border border-transparent hover:border-neutral-700 focus:border-neutral-600 rounded px-1.5 py-1 text-neutral-400 focus:outline-none"
              />
              <input
                defaultValue={item.content}
                onBlur={(e) =>
                  e.target.value !== item.content &&
                  updateItem(profile.id, item.id, item.label, e.target.value)
                }
                className="flex-1 text-xs bg-neutral-800/50 border border-transparent hover:border-neutral-700 focus:border-neutral-600 rounded px-1.5 py-1 text-neutral-200 focus:outline-none"
              />
              <button
                onClick={() => removeItem(profile.id, item.id)}
                className="opacity-0 group-hover/item:opacity-100 text-neutral-600 hover:text-red-400 transition-all cursor-default"
              >
                <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
          ))}

          <div className="flex items-center gap-2 pt-1">
            <span className="w-4 flex-shrink-0" />
            <input
              value={newLabel}
              onChange={(e) => setNewLabel(e.target.value)}
              placeholder="Label"
              className="w-32 text-xs bg-neutral-800 border border-neutral-700 rounded px-1.5 py-1 text-neutral-200 placeholder-neutral-600 focus:outline-none focus:border-neutral-600"
            />
            <input
              value={newContent}
              onChange={(e) => setNewContent(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && handleAdd()}
              placeholder="Content"
              className="flex-1 text-xs bg-neutral-800 border border-neutral-700 rounded px-1.5 py-1 text-neutral-200 placeholder-neutral-600 focus:outline-none focus:border-neutral-600"
            />
            <button
              onClick={handleAdd}
              className="text-xs text-indigo-400 hover:text-indigo-300 transition-colors cursor-default"
            >
              Add
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
