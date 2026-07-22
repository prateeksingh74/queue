import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useQueueStore } from "../stores/queueStore";
import type { QueueSnapshot } from "../stores/queueStore";
import { useSlotsStore } from "../stores/slotsStore";
import type { Slot } from "../stores/slotsStore";
import { Commands } from "../lib/commands";

// Keeps the frontend queue store in sync with the Rust backend: hydrates on
// mount, then applies every "queue-updated" snapshot the backend emits.
export function useQueueSync() {
  const applySnapshot = useQueueStore((state) => state.applySnapshot);

  useEffect(() => {
    Commands.getQueueState().then(applySnapshot).catch(console.error);

    const unlisten = listen<QueueSnapshot>("queue-updated", (event) => {
      applySnapshot(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [applySnapshot]);
}

export function useSlotsSync() {
  const applySlots = useSlotsStore((state) => state.applySlots);

  useEffect(() => {
    Commands.getSlots().then(applySlots).catch(console.error);

    const unlisten = listen<Slot[]>("slots-updated", (event) => {
      applySlots(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [applySlots]);
}
