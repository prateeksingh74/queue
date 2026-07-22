import { useSlotsStore } from "../../stores/slotsStore";

export default function SlotsView() {
  const { slots, createSlot, deleteSlot } = useSlotsStore();

  return (
    <div className="p-6">
      <div className="flex items-start justify-between mb-6">
        <div>
          <h2 className="text-lg font-semibold text-white">Slots</h2>
          <p className="text-xs text-neutral-500 mt-0.5">
            Persistent clipboard locations for frequently used content.
          </p>
        </div>
        <button
          onClick={() => createSlot(`Slot ${slots.length + 1}`)}
          className="text-xs text-indigo-400 hover:text-indigo-300 transition-colors px-2.5 py-1.5 rounded border border-indigo-400/30 hover:border-indigo-400/50 cursor-default"
        >
          + New Slot
        </button>
      </div>

      {slots.length === 0 ? (
        <div className="flex items-center justify-center h-48">
          <div className="text-center">
            <div className="w-10 h-10 rounded-lg bg-neutral-900 border border-neutral-800 flex items-center justify-center mx-auto mb-3">
              <svg className="w-5 h-5 text-neutral-700" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z" />
              </svg>
            </div>
            <p className="text-neutral-400 text-sm font-medium">No slots yet</p>
            <p className="text-neutral-600 text-xs mt-1">Create slots for your email, phone, address, etc.</p>
          </div>
        </div>
      ) : (
        <div className="grid grid-cols-2 gap-3">
          {slots.map((slot) => (
            <div key={slot.id} className="group bg-neutral-900 border border-neutral-800 rounded-lg p-3 hover:border-neutral-700 transition-colors">
              <div className="flex items-center justify-between mb-2">
                <p className="text-sm font-medium text-neutral-200">{slot.name}</p>
                <button
                  onClick={() => deleteSlot(slot.id)}
                  className="opacity-0 group-hover:opacity-100 text-neutral-600 hover:text-red-400 transition-all cursor-default"
                >
                  <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
              <p className="text-xs text-neutral-600 truncate">
                {slot.content || "Empty — hotkey assignment coming in Phase 3"}
              </p>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
