import { ActionIcon, Box, Group, Text } from "@mantine/core";
import { IconX } from "@tabler/icons-react";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { DrawShape } from "@lichess-org/chessground/draw";
import type { Key } from "@lichess-org/chessground/types";
import { INITIAL_FEN } from "chessops/fen";
import { useEffect, useMemo, useState } from "react";
import { Chessground } from "@/chessground/Chessground";

export interface FloatBoardState {
  fen: string;
  lastMove: [string, string] | null;
  bestMove: { from: string; to: string } | null;
  scoreText: string | null;
  depth: number | null;
  turn: "white" | "black";
  orientation: "white" | "black";
}

const STORAGE_KEY = "chesslab-float-last";

const DEFAULT_STATE: FloatBoardState = {
  fen: INITIAL_FEN,
  lastMove: null,
  bestMove: null,
  scoreText: null,
  depth: null,
  turn: "white",
  orientation: "white",
};

function readInitialState(): FloatBoardState {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return DEFAULT_STATE;
    return { ...DEFAULT_STATE, ...(JSON.parse(raw) as Partial<FloatBoardState>) };
  } catch {
    return DEFAULT_STATE;
  }
}

export function FloatingOverlay() {
  const [state, setState] = useState<FloatBoardState>(readInitialState);

  useEffect(() => {
    const unlisten = listen<FloatBoardState>("chesslab-float-state", ({ payload }) => {
      setState({ ...DEFAULT_STATE, ...payload });
    });
    return () => {
      void unlisten.then((fn) => fn());
    };
  }, []);

  const shapes = useMemo<DrawShape[]>(() => {
    if (!state.bestMove) return [];
    return [
      {
        orig: state.bestMove.from as Key,
        dest: state.bestMove.to as Key,
        brush: "blue",
        modifiers: { lineWidth: 10 },
      },
    ];
  }, [state.bestMove]);

  const lastMove = (state.lastMove ?? undefined) as [Key, Key] | undefined;

  return (
    <Box
      style={{
        height: "100vh",
        width: "100vw",
        display: "flex",
        flexDirection: "column",
        background: "#1b1f27",
        color: "#e7e9ee",
        overflow: "hidden",
      }}
    >
      <Box
        data-tauri-drag-region
        style={{
          height: 32,
          flex: "0 0 32px",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          padding: "0 8px",
          background: "#20242e",
          borderBottom: "1px solid #333a48",
          cursor: "grab",
        }}
      >
        <Group gap={8} wrap="nowrap">
          <Text size="xs" fw={700} style={{ letterSpacing: 0.4 }}>
            ChessLab Float
          </Text>
          <Text size="xs" c="dimmed">
            {state.turn === "white" ? "White" : "Black"} to move
          </Text>
        </Group>
        <Group gap={6} wrap="nowrap">
          {state.scoreText && (
            <Text size="xs" fw={700}>
              {state.scoreText}
            </Text>
          )}
          {state.depth !== null && (
            <Text size="xs" c="dimmed">
              d{state.depth}
            </Text>
          )}
          <ActionIcon
            size="sm"
            variant="subtle"
            color="gray"
            aria-label="Close floating board"
            onClick={() => {
              void getCurrentWindow().close();
            }}
          >
            <IconX size="0.9rem" />
          </ActionIcon>
        </Group>
      </Box>

      <Box
        style={{
          flex: 1,
          minHeight: 0,
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          padding: 10,
        }}
      >
        <Box style={{ width: "100%", maxWidth: "min(100%, 72vh)" }}>
          <Chessground
            fen={state.fen}
            orientation={state.orientation}
            coordinates
            viewOnly
            lastMove={lastMove}
            animation={{ enabled: true, duration: 180 }}
            drawable={{
              enabled: false,
              visible: true,
              autoShapes: shapes,
            }}
          />
        </Box>
      </Box>
    </Box>
  );
}

export default FloatingOverlay;
