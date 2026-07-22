import { useProfilesStore } from "../../stores/profilesStore";

export default function ProfilesView() {
  const { profiles, createProfile, deleteProfile } = useProfilesStore();

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
            <div key={profile.id} className="group flex items-center gap-3 px-3 py-2.5 rounded-lg bg-neutral-900 border border-neutral-800 hover:border-neutral-700 transition-colors">
              <div className="flex-1 min-w-0">
                <p className="text-sm font-medium text-neutral-200 truncate">{profile.name}</p>
                <p className="text-xs text-neutral-600 mt-0.5">
                  {profile.items.length} item{profile.items.length === 1 ? "" : "s"}
                </p>
              </div>
              <button
                onClick={() => deleteProfile(profile.id)}
                className="opacity-0 group-hover:opacity-100 text-neutral-600 hover:text-red-400 transition-all cursor-default"
              >
                <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
