import {
    Table,
    TableBody,
    TableCaption,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table.tsx";
import { useScenarios } from "@/features/scenarios/api/useScenarios.ts";
import { useState } from "react";
import type { Scenario } from "@/types";
import { ScenarioFormDialog } from "@/features/scenarios/components/ScenarioFormDialog.tsx";
import { Button } from "@/components/ui/button.tsx";

export function ScenarioList() {
    const { data: scenarios, isLoading, isError } = useScenarios();
    // const { deleteScenario } = useScenarioMutations();

    const [isDialogOpen, setIsDialogOpen] = useState(false);
    const [editingScenario, setEditingScenario] = useState<Scenario | null>(
        null,
    );

    const handleCreate = () => {
        setEditingScenario(null);
        setIsDialogOpen(true);
    };

    if (isLoading) return <div className="p-4">読み込み中...</div>;

    if (isError)
        return (
            <div className="p-4 text-red-500">
                シナリオデータの取得に失敗しました
            </div>
        );

    return (
        <div>
            <div className="flex items-center justify-end">
                <Button onClick={handleCreate}>+ シナリオの作成</Button>
            </div>
            <div>
                <Table>
                    <TableCaption></TableCaption>
                    <TableHeader>
                        <TableRow>
                            <TableHead>id</TableHead>
                            <TableHead>name</TableHead>
                            <TableHead>start_date</TableHead>
                            <TableHead>end_date</TableHead>
                            <TableHead>is_locked</TableHead>
                            <TableHead>updated_at</TableHead>
                            <TableHead>updated_by</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {scenarios?.length === 0 && (
                            <TableRow>
                                <TableCell
                                    colSpan={7}
                                    className="text-center text-slate-400 py-8"
                                >
                                    シナリオデータはありません
                                </TableCell>
                            </TableRow>
                        )}
                        {scenarios?.map((s: Scenario) => (
                            <TableRow>
                                <TableCell>{s.id}</TableCell>
                                <TableCell>{s.name}</TableCell>
                                <TableCell>{s.start_date}</TableCell>
                                <TableCell>{s.end_date}</TableCell>
                                <TableCell>{s.is_locked}</TableCell>
                                <TableCell>{s.updated_at}</TableCell>
                                <TableCell>{s.updated_by}</TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </div>

            <ScenarioFormDialog
                open={isDialogOpen}
                onOpenChange={setIsDialogOpen}
                scenarioToEdit={editingScenario}
            />
        </div>
    );
}
