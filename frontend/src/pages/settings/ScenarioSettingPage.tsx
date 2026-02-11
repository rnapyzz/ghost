import {
    useScenarioMutations,
    useScenarios,
} from "@/features/scenarios/api/useScenarios.ts";
import { Button } from "@/components/ui/button.tsx";
import { CheckCircle2, Circle, Loader2 } from "lucide-react";
import type { Scenario } from "@/types";
import { ScenarioRolloverDialog } from "@/features/scenarios/components/ScenarioRolloverDialog.tsx";

export function ScenarioSettingPage() {
    const { data: scenarios, isLoading } = useScenarios();
    const { activateScenario } = useScenarioMutations();

    if (isLoading)
        return (
            <div className="p-8 text-center">シナリオデータを読み込み中...</div>
        );

    return (
        <div className="p-4 max-w-5xl mx-auto">
            {/* header */}
            <div className="mt-3 mb-8">
                <h1 className="text-2xl text-slate-900 font-bold">
                    現在シナリオ設定
                </h1>
                <p className="text-slate-500 mt-2">
                    システムで「現在（Current）」として扱うシナリオを設定します。
                    <br />
                    設定されたシナリオに対する計画管理状態になります。
                    <br />
                    他のシナリオに対する計画は作成や編集が行えなくなります。
                </p>
            </div>

            {/* main */}
            <div className="bg-white rounded-lg border shadow-sm divide-y">
                {scenarios?.map((scenario: Scenario) => (
                    <div
                        key={scenario.id}
                        className="px-4 py-2 flex items-center justify-between hover:bg-slate-50 transition-colors"
                    >
                        <h3 className="text-slate-900 truncate font-">
                            {scenario.name}
                        </h3>
                        <div className="flex items-center gap-2">
                            <ScenarioRolloverDialog sourceScenario={scenario} />
                            <Button
                                variant={
                                    scenario.is_current ? "outline" : "default"
                                }
                                disabled={
                                    scenario.is_current ||
                                    activateScenario.isPending
                                }
                                onClick={() =>
                                    activateScenario.mutate(scenario.id)
                                }
                                className={
                                    scenario.is_current
                                        ? "w-32 border-green-400 bg-green-200 text-green-700 hover:bg-green-100"
                                        : "w-32 border-blue-200 bg-blue-500 text-white hover:bg-blue-600"
                                }
                            >
                                {activateScenario.isPending &&
                                !scenario.is_current ? (
                                    <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                                ) : scenario.is_current ? (
                                    <>
                                        <CheckCircle2 className="w-4 h-4 mr-2" />
                                        選択中
                                    </>
                                ) : (
                                    <>
                                        <Circle className="w-4 h-4 mr-2" />
                                        有効化
                                    </>
                                )}
                            </Button>
                        </div>
                    </div>
                ))}
                {scenarios?.length === 0 && (
                    <div className="p-8 text-center text-slate-500">
                        シナリオがありません。
                    </div>
                )}
            </div>
        </div>
    );
}
