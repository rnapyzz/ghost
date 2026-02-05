import type { PlanNodeWithChildren } from "@/features/nodes/utils/tree.ts";
import { Badge, Calendar, Tag, User } from "lucide-react";

type Props = {
    node: PlanNodeWithChildren | null;
};

export function NodeDetailPanel({ node }: Props) {
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
            <div className="mb-6 ">
                <div className="flex items-center gap-2 mb-2">
                    <Badge className="text-xs font-normal text-slate-500">
                        {node.node_type}
                    </Badge>
                    <span className="text-xs text-slate-400">
                        ID: {node.id.slice(0, 18)}...
                    </span>
                </div>
                <span className="text-xl font-semibold text-slate-900">
                    {node.title}
                </span>
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
        </div>
    );
}
