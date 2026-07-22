import { useCallback, useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { Commands } from "../../lib/commands";
import type { HistoryItem } from "../../lib/commands";

function timeAgo(ts: number): string {
  const seconds = Math.floor((Date.now() - ts) / 1000);
  if (seconds < 60) return "just now";
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`;
  if (seconds < 86400) return `${Math.floor(seconds / 3600)}h ago`;
  return `${Math.floor(seconds / 86400)}d ago`;
}

export default function HistoryView() {
  const [query, setQuery] = useState("");
  const [items, setItems] = useState<HistoryItem[]>([]);

  const refresh = useCallback(() => {
    Commands.searchHistory(query).then(setItems).catch(console.error);
  }, [query]);

  useEffect(() => {
    refresh();
    const unlisten = listen("history-updated", refresh);
    return () => {
      unlisten.then((fn) => fn());
    };
  }, [refresh]);

  return (
    <div className="p-6 h-full flex flex-col">
      <div className="flex items-start justify-between mb-6">
        <div>
          <h2 className="text-lg font-semibold text-white">History</h2>
          <p className="text-xs text-neutral-500 mt-0.5">
            Everything you've copied, searchable.
          </p>
        </div>
        {items.length > 0 && (
          <button
            onClick={() => Commands.clearHistory()}
            title="Pinned items are kept"
            className="text-xs text-neutral-500 hover:text-red-400 transition-colors px-2.5 py-1.5 rounded border border-neutral-800 hover:border-red-400/30 cursor-default"
          >
            Clear History
          </button>
        )}
      </div>

      <input
        type="text"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        placeholder="Search history..."
        className="w-full bg-neutral-900 border border-neutral-800 rounded-lg px-3 py-2 text-sm text-neutral-200 placeholder-neutral-600 focus:outline-none focus:border-neutral-700 mb-4"
      />

      {items.length === 0 ? (
        <div className="flex-1 flex items-center justify-center">
          <div className="text-center">
            <div className="w-10 h-10 rounded-lg bg-neutral-900 border border-neutral-800 flex items-center justify-center mx-auto mb-3">
              <svg className="w-5 h-5 text-neutral-700" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <p className="text-neutral-400 text-sm font-medium">
              {query ? "No matches" : "No history yet"}
            </p>
            <p className="text-neutral-600 text-xs mt-1">
              {query ? "Try a different search" : "Copied items appear here automatically"}
            </p>
          </div>
        </div>
      ) : (
        <div className="space-y-1.5 overflow-auto flex-1">
          {items.map((item) => (
            <div
              key={item.id}
              className="group flex items-center gap-3 px-3 py-2.5 rounded-lg bg-neutral-900 border border-neutral-800 hover:border-neutral-700 transition-colors"
            >
              <button
                onClick={() => Commands.setHistoryPinned(item.id, !item.pinned)}
                title={item.pinned ? "Unpin" : "Pin"}
                className={`flex-shrink-0 transition-colors cursor-default ${
                  item.pinned
                    ? "text-yellow-400"
                    : "text-neutral-700 hover:text-neutral-400 opacity-0 group-hover:opacity-100"
                }`}
              >
                <svg className="w-3.5 h-3.5" fill={item.pinned ? "currentColor" : "none"} viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z" />
                </svg>
              </button>
              <p className="flex-1 text-sm text-neutral-200 truncate min-w-0">{item.content}</p>
              <span className="text-xs text-neutral-600 flex-shrink-0">{timeAgo(item.copiedAt)}</span>
              <div className="flex items-center gap-1.5 opacity-0 group-hover:opacity-100 transition-opacity flex-shrink-0">
                <button
                  onClick={() => Commands.restoreHistoryItem(item.id)}
                  title="Add back to queue"
                  className="text-neutral-600 hover:text-indigo-400 transition-colors cursor-default"
                >
                  <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M3 10h10a4 4 0 014 4v1m-14-5l4-4m-4 4l4 4" />
                  </svg>
                </button>
                <button
                  onClick={() => Commands.deleteHistoryItem(item.id)}
                  className="text-neutral-600 hover:text-red-400 transition-colors cursor-default"
                >
                  <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
