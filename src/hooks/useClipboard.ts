import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useQueueStore } from "../stores/queueStore";
import type { ContentType } from "../stores/queueStore";

interface ClipboardChangeEvent {
  content: string;
  content_type: ContentType;
}

// Activated in Phase 2 when the Rust clipboard monitor emits "clipboard-change" events.
export function useClipboardMonitor(enabled = false) {
  const addItem = useQueueStore((state) => state.addItem);

  useEffect(() => {
    if (!enabled) return;

    const unlisten = listen<ClipboardChangeEvent>("clipboard-change", (event) => {
      addItem(event.payload.content, event.payload.content_type);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [enabled, addItem]);
}
