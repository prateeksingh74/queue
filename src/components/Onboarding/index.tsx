import { useEffect, useState } from "react";
import { useSettingsStore } from "../../stores/settingsStore";

const STEPS = ["Welcome", "What Queue Does", "Hotkeys", "Preferences", "Done"] as const;

export default function Onboarding() {
  const { hotkeys, loadHotkeys, maxHistoryItems, setMaxHistoryItems, setOnboardingComplete } =
    useSettingsStore();
  const [step, setStep] = useState(0);

  useEffect(() => {
    loadHotkeys();
  }, [loadHotkeys]);

  const last = step === STEPS.length - 1;

  return (
    <div className="fixed inset-0 z-50 bg-neutral-950 flex flex-col items-center justify-center select-none">
      <div className="w-full max-w-md px-8">
        {step === 0 && (
          <div className="text-center">
            <div className="w-14 h-14 rounded-2xl bg-indigo-500/15 border border-indigo-500/30 flex items-center justify-center mx-auto mb-5">
              <svg className="w-7 h-7 text-indigo-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
              </svg>
            </div>
            <h1 className="text-2xl font-semibold text-white mb-2">Welcome to Queue</h1>
            <p className="text-sm text-neutral-400">
              The programmable clipboard for modern workflows. Let's get you set up — it
              takes less than two minutes.
            </p>
          </div>
        )}

        {step === 1 && (
          <div>
            <h2 className="text-xl font-semibold text-white mb-4 text-center">What Queue does</h2>
            <div className="space-y-3">
              {[
                ["Queue Clipboard", "Every copy is saved in order. Paste them back one at a time — no more losing what you copied."],
                ["Slots", "Persistent locations for your email, address, links — paste them anytime without recopying."],
                ["Profiles", "Reusable sets of info (name, email, phone…) that fill the queue in one click. Perfect for forms."],
                ["History", "Everything you've copied, searchable and restorable."],
              ].map(([title, desc]) => (
                <div key={title} className="px-4 py-3 rounded-lg bg-neutral-900 border border-neutral-800">
                  <p className="text-sm font-medium text-neutral-200">{title}</p>
                  <p className="text-xs text-neutral-500 mt-0.5">{desc}</p>
                </div>
              ))}
            </div>
          </div>
        )}

        {step === 2 && (
          <div>
            <h2 className="text-xl font-semibold text-white mb-1 text-center">Your hotkeys</h2>
            <p className="text-xs text-neutral-500 text-center mb-4">
              These work system-wide, in any app. You can change them anytime in Settings.
            </p>
            <div className="space-y-1.5">
              {hotkeys
                .filter((h) => h.keys)
                .map((h) => (
                  <div key={h.action} className="flex items-center justify-between px-4 py-3 rounded-lg bg-neutral-900 border border-neutral-800">
                    <p className="text-sm text-neutral-200">{h.label}</p>
                    <span className="text-xs font-mono px-2 py-1 rounded border border-neutral-700 bg-neutral-800 text-neutral-400">
                      {h.keys.replace("CommandOrControl", "⌘")}
                    </span>
                  </div>
                ))}
            </div>
            <p className="text-xs text-yellow-500/80 mt-4 text-center">
              macOS will ask for Accessibility permission the first time Queue pastes —
              that's required to simulate ⌘V.
            </p>
          </div>
        )}

        {step === 3 && (
          <div>
            <h2 className="text-xl font-semibold text-white mb-1 text-center">Preferences</h2>
            <p className="text-xs text-neutral-500 text-center mb-4">Sensible defaults — tweak if you like.</p>
            <div className="flex items-center justify-between px-4 py-3 rounded-lg bg-neutral-900 border border-neutral-800">
              <div>
                <p className="text-sm text-neutral-200">Maximum history items</p>
                <p className="text-xs text-neutral-600 mt-0.5">Older unpinned items are dropped</p>
              </div>
              <input
                type="number"
                value={maxHistoryItems}
                onChange={(e) => setMaxHistoryItems(Number(e.target.value) || 0)}
                className="w-20 bg-neutral-800 border border-neutral-700 rounded px-2 py-1 text-sm text-neutral-200 text-right focus:outline-none focus:border-neutral-600"
              />
            </div>
          </div>
        )}

        {step === 4 && (
          <div className="text-center">
            <div className="w-14 h-14 rounded-2xl bg-green-500/15 border border-green-500/30 flex items-center justify-center mx-auto mb-5">
              <svg className="w-7 h-7 text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
              </svg>
            </div>
            <h2 className="text-2xl font-semibold text-white mb-2">You're all set</h2>
            <p className="text-sm text-neutral-400">
              Copy a few things and watch your queue fill up. Then press{" "}
              <span className="font-mono text-neutral-300">⌘⇧V</span> anywhere to paste
              them back in order.
            </p>
          </div>
        )}

        <div className="flex items-center justify-between mt-8">
          <button
            onClick={() => setStep(step - 1)}
            className={`text-xs text-neutral-500 hover:text-neutral-300 transition-colors px-3 py-2 cursor-default ${
              step === 0 ? "invisible" : ""
            }`}
          >
            Back
          </button>

          <div className="flex gap-1.5">
            {STEPS.map((_, i) => (
              <span
                key={i}
                className={`w-1.5 h-1.5 rounded-full ${i === step ? "bg-indigo-400" : "bg-neutral-800"}`}
              />
            ))}
          </div>

          <button
            onClick={() => (last ? setOnboardingComplete(true) : setStep(step + 1))}
            className="text-xs text-indigo-400 hover:text-indigo-300 transition-colors px-3 py-2 rounded border border-indigo-400/30 hover:border-indigo-400/50 cursor-default"
          >
            {last ? "Start using Queue" : "Continue"}
          </button>
        </div>
      </div>
    </div>
  );
}
