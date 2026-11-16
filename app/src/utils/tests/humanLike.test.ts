import { describe, expect, it } from "vitest";
import { applyHumanLikeOptions, HUMAN_LIKE_ELO, HUMAN_LIKE_SKILL_LEVEL } from "../humanLike";

type Option = { name: string; value: string };

describe("applyHumanLikeOptions", () => {
    it("keeps the original options when the preset is disabled", () => {
        const options: Option[] = [
            { name: "Threads", value: "4" },
            { name: "UCI_LimitStrength", value: "false" },
        ];

        expect(applyHumanLikeOptions(options, false)).toBe(options);
    });

    it("adds the preset when no strength options exist", () => {
        const result = applyHumanLikeOptions([{ name: "Threads", value: "4" }], true);

        expect(result).toEqual(
            expect.arrayContaining([
                { name: "Threads", value: "4" },
                { name: "UCI_LimitStrength", value: "true" },
                { name: "UCI_Elo", value: String(HUMAN_LIKE_ELO) },
                { name: "Skill Level", value: String(HUMAN_LIKE_SKILL_LEVEL) },
            ]),
        );
    });

    it("overrides stored full-strength settings without mutating the input", () => {
        const options: Option[] = [
            { name: "UCI_LimitStrength", value: "false" },
            { name: "UCI_Elo", value: "3000" },
            { name: "Skill Level", value: "20" },
        ];

        const result = applyHumanLikeOptions(options, true);

        expect(result).toEqual([
            { name: "UCI_LimitStrength", value: "true" },
            { name: "UCI_Elo", value: String(HUMAN_LIKE_ELO) },
            { name: "Skill Level", value: String(HUMAN_LIKE_SKILL_LEVEL) },
        ]);
        expect(options).toEqual([
            { name: "UCI_LimitStrength", value: "false" },
            { name: "UCI_Elo", value: "3000" },
            { name: "Skill Level", value: "20" },
        ]);
    });
});
