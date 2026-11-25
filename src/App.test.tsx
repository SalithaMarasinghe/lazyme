import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import App from "./App";
import type { LazyMeBridge } from "./lib/tauri";

describe("App shell", () => {
  it("shows first-run onboarding notice when no API key is configured", async () => {
    const bridge: LazyMeBridge = {
      getAppStatus: async () => ({ hasApiKey: false }),
    };

    render(<App bridge={bridge} />);

    expect(await screen.findByText(/LazyMe/i)).toBeInTheDocument();
    expect(
      await screen.findByText(/No API key configured/i),
    ).toBeInTheDocument();
  });

  it("shows ready status when an API key is already configured", async () => {
    const bridge: LazyMeBridge = {
      getAppStatus: async () => ({ hasApiKey: true }),
    };

    render(<App bridge={bridge} />);

    expect(
      await screen.findByText(/Running in tray/i),
    ).toBeInTheDocument();
  });
});
