import { useEffect, useState } from "react";
import { tauriBridge, type AppStatus, type LazyMeBridge } from "./lib/tauri";

interface AppProps {
  bridge?: LazyMeBridge;
}

/** Root LazyMe shell displaying first-run onboarding or tray status. */
export default function App({ bridge = tauriBridge }: AppProps) {
  const [status, setStatus] = useState<AppStatus | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    bridge
      .getAppStatus()
      .then((next) => {
        if (!cancelled) setStatus(next);
      })
      .catch((err: unknown) => {
        if (!cancelled) {
          setError(err instanceof Error ? err.message : String(err));
        }
      });
    return () => {
      cancelled = true;
    };
  }, [bridge]);

  return (
    <main className="min-h-screen bg-slate-950 p-6 text-slate-100">
      <div className="mx-auto max-w-xl rounded-xl border border-slate-800 bg-slate-900/70 p-6 shadow-lg">
        <header className="mb-4 flex items-center justify-between">
          <h1 className="text-xl font-semibold tracking-tight">LazyMe</h1>
          <span className="rounded-full bg-slate-800 px-2.5 py-0.5 text-xs text-slate-300">
            Settings
          </span>
        </header>

        {error && (
          <p className="rounded-lg border border-red-800 bg-red-950/60 p-3 text-sm text-red-200">
            {error}
          </p>
        )}

        {!error && status === null && (
          <p className="text-sm text-slate-400">Checking setup...</p>
        )}

        {status && !status.hasApiKey && (
          <section className="rounded-lg border border-amber-700/60 bg-amber-950/40 p-4">
            <h2 className="text-sm font-medium text-amber-200">
              No API key configured
            </h2>
            <p className="mt-1 text-xs text-amber-100/80">
              Add a Provider Profile API key to start transforming Raw Input via
              your global hotkey.
            </p>
          </section>
        )}

        {status && status.hasApiKey && (
          <section className="rounded-lg border border-emerald-800/60 bg-emerald-950/40 p-4">
            <h2 className="text-sm font-medium text-emerald-200">
              Running in tray
            </h2>
            <p className="mt-1 text-xs text-emerald-100/80">
              Closing this window keeps LazyMe active in the Windows system
              tray.
            </p>
          </section>
        )}
      </div>
    </main>
  );
}
