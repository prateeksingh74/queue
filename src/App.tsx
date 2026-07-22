import { useState } from "react";
import "./App.css";
import Sidebar from "./components/Layout/Sidebar";
import QueueView from "./components/Queue";
import SlotsView from "./components/Slots";
import ProfilesView from "./components/Profiles";
import HistoryView from "./components/History";
import SettingsView from "./components/Settings";

export type View = "queue" | "slots" | "profiles" | "history" | "settings";

function App() {
  const [activeView, setActiveView] = useState<View>("queue");

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
      <main className="flex-1 overflow-auto">{views[activeView]}</main>
    </div>
  );
}

export default App;
