import { useQueueStore } from "../../stores/queueStore";
import type { ContentType } from "../../stores/queueStore";

const TYPE_BADGE: Record<ContentType, string> = {
  email: "bg-blue-500/20 text-blue-300 border-blue-500/30",
  phone: "bg-green-500/20 text-green-300 border-green-500/30",
  url: "bg-purple-500/20 text-purple-300 border-purple-500/30",
  address: "bg-yellow-500/20 text-yellow-300 border-yellow-500/30",
  date: "bg-orange-500/20 text-orange-300 border-orange-500/30",
  text: "bg-neutral-700/40 text-neutral-400 border-neutral-700/40",
  unknown: "bg-neutral-700/40 text-neutral-400 border-neutral-700/40",
};

export default function QueueView() {
  const { items, cursor, clear, removeItem } = useQueueStore();

  return (
    <div className="p-6 h-full flex flex-col">
      <div className="flex items-start justify-between mb-6">
        <div>
          <h2 className="text-lg font-semibold text-white">Queue</h2>
          <p className="text-xs text-neutral-500 mt-0.5">
            {items.length === 0
              ? "Copy items to build your queue"
              : `${items.length} item${items.length === 1 ? "" : "s"} — next paste at position ${cursor + 1}`}
          </p>
        </div>
        {items.length > 0 && (
          <button
            onClick={clear}
            className="text-xs text-neutral-500 hover:text-red-400 transition-colors px-2.5 py-1.5 rounded border border-neutral-800 hover:border-red-400/30 cursor-default"
          >
            Clear All
          </button>
        )}
      </div>

      {items.length === 0 ? (
        <div className="flex-1 flex items-center justify-center">
          <div className="text-center">
            <div className="w-10 h-10 rounded-lg bg-neutral-900 border border-neutral-800 flex items-center justify-center mx-auto mb-3">
              <svg className="w-5 h-5 text-neutral-700" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
              </svg>
            </div>
            <p className="text-neutral-400 text-sm font-medium">Queue is empty</p>
            <p className="text-neutral-600 text-xs mt-1">Clipboard monitoring connects in Phase 2</p>
          </div>
        </div>
      ) : (
        <div className="space-y-1.5 overflow-auto flex-1">
          {items.map((item, index) => (
            <div
              key={item.id}
              className={`group flex items-center gap-3 px-3 py-2.5 rounded-lg border transition-colors ${
                index === cursor
                  ? "bg-indigo-500/10 border-indigo-500/25"
                  : "bg-neutral-900 border-neutral-800 hover:border-neutral-700"
              }`}
            >
              <span className="text-xs text-neutral-700 font-mono w-4 text-right flex-shrink-0">
                {index + 1}
              </span>
              <p className="flex-1 text-sm text-neutral-200 truncate min-w-0">{item.content}</p>
              <span className={`text-xs px-1.5 py-0.5 rounded border font-medium flex-shrink-0 ${TYPE_BADGE[item.type]}`}>
                {item.type}
              </span>
              <button
                onClick={() => removeItem(item.id)}
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
