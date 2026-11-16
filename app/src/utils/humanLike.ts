/**
 * ChessLab human-like engine preset.
 *
 * Stockfish can be capped with UCI_LimitStrength + UCI_Elo and softened with
 * Skill Level. These options are applied on top of the user's engine settings
 * so the preset wins over a stored full-strength configuration.
 */
export const HUMAN_LIKE_ELO = 1800;
export const HUMAN_LIKE_SKILL_LEVEL = 15;

const HUMAN_LIKE_OPTIONS = [
    { name: "UCI_LimitStrength", value: "true" },
    { name: "UCI_Elo", value: String(HUMAN_LIKE_ELO) },
    { name: "Skill Level", value: String(HUMAN_LIKE_SKILL_LEVEL) },
] as const;

export function applyHumanLikeOptions<T extends { name: string; value: unknown }>(
    options: T[],
    enabled: boolean,
): T[] {
    if (!enabled) {
        return options;
    }

    const merged = new Map<string, T>(options.map((option) => [option.name, option]));
    for (const option of HUMAN_LIKE_OPTIONS) {
        merged.set(option.name, option as unknown as T);
    }
    return [...merged.values()];
}
