import { useState } from "react";

export default function HistoryView() {
  const [query, setQuery] = useState("");

  return (
    <div className="p-6 h-full flex flex-col">
      <div className="mb-6">
        <h2 className="text-lg font-semibold text-white">History</h2>
        <p className="text-xs text-neutral-500 mt-0.5">
          Everything you've copied, searchable.
        </p>
      </div>

      <input
        type="text"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        placeholder="Search history..."
        className="w-full bg-neutral-900 border border-neutral-800 rounded-lg px-3 py-2 text-sm text-neutral-200 placeholder-neutral-600 focus:outline-none focus:border-neutral-700 mb-4"
      />

      <div className="flex-1 flex items-center justify-center">
        <div className="text-center">
          <div className="w-10 h-10 rounded-lg bg-neutral-900 border border-neutral-800 flex items-center justify-center mx-auto mb-3">
            <svg className="w-5 h-5 text-neutral-700" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
          <p className="text-neutral-400 text-sm font-medium">No history yet</p>
          <p className="text-neutral-600 text-xs mt-1">Copied items appear here once monitoring starts</p>
        </div>
      </div>
    </div>
  );
}
