import type { PlanNodeWithChildren } from "@/features/nodes/utils/tree.ts";
import { Calendar, Pencil, Tag, Trash2, User } from "lucide-react";
import { usePlanNodeMutations } from "@/features/nodes/api/usePlanNodes.ts";
import { useState } from "react";
import { Button } from "@/components/ui/button.tsx";
import { PlanNodeFormDialog } from "@/features/nodes/components/PlanNodeFormDialog.tsx";
import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
} from "@/components/ui/alert-dialog.tsx";

type Props = {
    node: PlanNodeWithChildren | null;
    onDeleted?: () => void;
};

export function NodeDetailPanel({ node, onDeleted }: Props) {
    const { deletePlanNode } = usePlanNodeMutations();
    const [isEditOpen, setIsEditOpen] = useState(false);
    const [isDeleteAlertOpen, setIsDeleteAlertOpen] = useState(false);

    const executeDelete = async () => {
        if (!node) return;
        try {
            await deletePlanNode.mutateAsync(node.id);
            onDeleted?.();
            setIsDeleteAlertOpen(false);
        } catch (error: any) {
            const message = error.response?.data || "削除に失敗しました";
            alert(message);
        }
    };

    if (!node) {
        return (
            <div className="h-full flex items-center justify-center text-slate-400 bg-slate-50/50">
                <div className="text-center">
                    <p>ノードを選択して詳細を表示</p>
                </div>
            </div>
        );
    }
    return (
        <div className="h-full bg-white p-6 overflow-y-auto">
            {/* header */}
            <div className="mb-6 flex justify-between items-start">
                <h2 className="text-xl font-semibold text-slate-900 leading-tight">
                    {node.title}
                </h2>

                <div className="flex gap-2">
                    <Button
                        variant="outline"
                        size="icon"
                        onClick={() => setIsEditOpen(true)}
                    >
                        <Pencil className="w-4- h-4 text-slate-600" />
                    </Button>
                    <Button
                        variant="outline"
                        size="icon"
                        className="hover:bg-red-50 hover:text-red-600 hover:border-red-200"
                        onClick={() => setIsDeleteAlertOpen(true)}
                    >
                        <Trash2 className="w-4 h-4 text-slate-400 hover:text-red-600" />
                    </Button>
                </div>
            </div>

            {/* meta data*/}
            <div className="grid grid-cols-2 gap-4 mb-6">
                <div className="space-y-1">
                    <div className="flex items-center text-sm text-slate-500">
                        <Calendar className="w-4 h-4 mr-2" />
                        作成日
                    </div>
                    <p className="text-sm font-medium text-slate-900">
                        {new Date(node.created_at).toLocaleDateString()}
                    </p>
                </div>
                <div className="space-y-1">
                    <div className="flex tems-center text-sm text-slate-500">
                        <User />
                        作成者
                    </div>
                    <p className="text-sm text-slate-900">
                        {node.created_by.slice(0, 18)}...
                    </p>
                </div>
            </div>

            {/* description */}
            <div className="space-y-2">
                <div className="flex items-center text-sm text-slate-500">
                    <Tag className="w-4 h-4 mr-2" />
                    説明
                </div>
                <div className="p-4 bg-slate-50 rounded-md text-sm text-slate-700 min-h-[100px] whitespace-pre-warp">
                    {node.description || "説明は設定されていません。"}
                </div>
            </div>

            {/* 編集用ダイアログ */}
            <PlanNodeFormDialog
                open={isEditOpen}
                onOpenChange={setIsEditOpen}
                scenarioId={node.scenario_id}
                nodeToEdit={node}
            />

            {/* 削除用ダイアログ */}
            <AlertDialog
                open={isDeleteAlertOpen}
                onOpenChange={setIsDeleteAlertOpen}
            >
                <AlertDialogContent>
                    <AlertDialogHeader>
                        <AlertDialogTitle>
                            本当に削除しますか？
                        </AlertDialogTitle>
                        <AlertDialogDescription>
                            "
                            <span className="text-slate-800 font-semibold">
                                {node.title}
                            </span>
                            " ({node.node_type}ノード)
                            を削除しようとしています。
                            <br />
                            子ノードや数値データが紐づいている場合、削除は失敗します。
                            <br />
                            <span className="text-red-600">
                                この操作は取り消せません。
                            </span>
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel>キャンセル</AlertDialogCancel>
                        <AlertDialogAction
                            onClick={(e) => {
                                e.preventDefault();
                                executeDelete();
                            }}
                            className="bg-red-600 hover:bg-red-700 text-white"
                        >
                            削除する
                        </AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialogContent>
            </AlertDialog>
        </div>
    );
}
