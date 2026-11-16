import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

export const FLOAT_WINDOW_LABEL = "float";

/**
 * Opens the frameless always-on-top mirror board, or focuses the existing
 * window when it is already open. The window renders the same frontend bundle;
 * App.tsx detects it by its window label and mounts the lightweight overlay.
 */
export async function openFloatWindow(): Promise<void> {
    const existing = await WebviewWindow.getByLabel(FLOAT_WINDOW_LABEL);
    if (existing) {
        await existing.show();
        await existing.setFocus();
        return;
    }

    const float = new WebviewWindow(FLOAT_WINDOW_LABEL, {
        url: "index.html",
        title: "ChessLab Float",
        width: 520,
        height: 600,
        minWidth: 320,
        minHeight: 380,
        alwaysOnTop: true,
        decorations: false,
        resizable: true,
        skipTaskbar: true,
        focus: true,
    });

    void float.once("tauri://error", (event) => {
        console.error("Failed to open the floating board", event);
    });
}
