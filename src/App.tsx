import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import "./App.css";
import { useQueueSync, useSlotsSync, useProfilesSync } from "./hooks/useClipboard";
import Onboarding from "./components/Onboarding";
import { useSettingsStore } from "./stores/settingsStore";
import Sidebar from "./components/Layout/Sidebar";
import QueueView from "./components/Queue";
import SlotsView from "./components/Slots";
import ProfilesView from "./components/Profiles";
import HistoryView from "./components/History";
import SettingsView from "./components/Settings";

export type View = "queue" | "slots" | "profiles" | "history" | "settings";

function App() {
  const [queueError, setQueueError] = useState("");
  useEffect(() => {
    const unlisten = listen<string>("queue-error", (event) => setQueueError(event.payload));
    return () => { void unlisten.then((fn) => fn()); };
  }, []);
  const [activeView, setActiveView] = useState<View>("queue");
  useQueueSync();
  useSlotsSync();
  useProfilesSync();
  const onboardingComplete = useSettingsStore((s) => s.onboardingComplete);

  if (!onboardingComplete) {
    return <Onboarding />;
  }

  const views: Record<View, React.ReactNode> = {
    queue: <QueueView />,
    slots: <SlotsView />,
    profiles: <ProfilesView />,
    history: <HistoryView />,
    settings: <SettingsView />,
  };

  return (
    <div className="flex h-screen bg-neutral-950 text-neutral-100 overflow-hidden select-none">
      <Sidebar activeView={activeView} onNavigate={setActiveView} />
      <main className="flex-1 overflow-auto">
        {queueError && <div role="alert" className="m-4 p-3 rounded border border-red-400/30 text-red-300 text-sm">
          {queueError}
          <button onClick={() => setQueueError("")} className="ml-3 underline">Dismiss</button>
        </div>}
        {views[activeView]}
      </main>
    </div>
  );
}

export default App;
