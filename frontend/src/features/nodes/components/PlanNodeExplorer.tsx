import { usePlanNodes } from "@/features/nodes/api/usePlanNodes.ts";
import {
    buildTree,
    type PlanNodeWithChildren,
} from "@/features/nodes/utils/tree.ts";
import { ChevronDown, ChevronsRight, Plus } from "lucide-react";
import { useState } from "react";
import { getNodeIcon } from "@/features/nodes/components/NodeIcon.tsx";
import { Button } from "@/components/ui/button.tsx";
import { cn } from "@/lib/utils.ts";
import { PlanNodeFormDialog } from "@/features/nodes/components/PlanNodeFormDialog.tsx";

const TreeNode = ({
    node,
    level = 0,
    onCreateChild,
}: {
    node: PlanNodeWithChildren;
    level?: number;
    onCreateChild: (node: PlanNodeWithChildren) => void;
}) => {
    const [isOpen, setIsOpen] = useState(false);

    const hasChildren = node.children && node.children.length > 0;

    const isContainer = ["Initiative", "Project", "SubProject"].includes(
        node.node_type,
    );

    const handleToggle = () => {
        if (isContainer || hasChildren) {
            setIsOpen(!isOpen);
        }
    };

    return (
        <div>
            <div
                className={cn(
                    `group flex items-center py-1 px-2 hover:bg-slate-100 cursor-pointer select-none transition-colors ${node.node_type === "Initiative" ? "bg-blue-50" : ""}`,
                )}
                style={{ paddingLeft: `${level * 16 + 8}px` }}
                onClick={handleToggle}
            >
                <span className="w-4 h-4 mr-1 flex items-center justify-center font-semibold bg-slate-50 rounded-full text-slate-500">
                    {(hasChildren || isContainer) &&
                        (isOpen ? (
                            <ChevronDown className="w-3 h-3" />
                        ) : (
                            <ChevronsRight className="w-3 h-3" />
                        ))}
                    {!hasChildren && !isContainer && (
                        <span className="font-light">-</span>
                    )}
                </span>

                {/* icon */}
                <span className="w-4 h-4 mr-2 flex items-center justify-center shrink-0">
                    {getNodeIcon(node.node_type, isOpen)}
                </span>

                {/* title */}
                <span className="text-sm text-slate-700 truncate">
                    {node.title}
                </span>

                {isContainer && (
                    <Button
                        variant="ghost"
                        size="icon"
                        className="w-6 h-6 opacity-0 group-hover:opacity-100 transition-opacity ml-2"
                        onClick={(e) => {
                            e.stopPropagation();
                            onCreateChild(node);
                        }}
                    >
                        <Plus className="w-3 h-3 text-slate-500" />
                    </Button>
                )}
            </div>

            {isOpen && hasChildren && (
                <div>
                    {node.children.map((child) => (
                        <TreeNode
                            key={child.id}
                            node={child}
                            level={level + 1}
                            onCreateChild={onCreateChild}
                        />
                    ))}
                </div>
            )}

            {isOpen && isContainer && !hasChildren && (
                <div
                    className="text-xs text-slate-300 py-1"
                    style={{ paddingLeft: `${(level + 1) * 16 + 8}px` }}
                >
                    (Empty)
                </div>
            )}
        </div>
    );
};

export function PlanNodeExplorer() {
    const { data: flatNodes, isLoading, isError } = usePlanNodes();

    const [isDialogOpen, setIsDialogOpen] = useState(false);
    const [targetParent, setTargetParent] =
        useState<PlanNodeWithChildren | null>(null);

    const currentScenarioId = "00000000-0000-0000-0000-000000000000";

    const handleCreateRoot = () => {
        setTargetParent(null);
        setIsDialogOpen(true);
    };

    const handleCreateChild = (parentNode: PlanNodeWithChildren) => {
        setTargetParent(parentNode);
        setIsDialogOpen(true);
    };

    if (isLoading) return <div className="p-4">読み込み中...</div>;

    if (isError)
        return (
            <div className="p-4 text-red-500">
                Nodeツリーの取得に失敗しました
            </div>
        );

    // tree構造を生成
    const treeNodes = buildTree(flatNodes);

    return (
        <div className="border rounded-md bg-white min-h-[400px] py-2 relative">
            {/* header */}
            <div className="px-4 py-2 border-b mb-2 flex justify-between items-center">
                <span className="text-xs font-bold text-slate-500 uppercase tracking-wider">
                    Explorer
                </span>
                <Button variant="ghost" size="sm" onClick={handleCreateRoot}>
                    <Plus className="w-4 h-4 mr-1" />
                    Node
                </Button>
            </div>

            {/* node tree */}
            {treeNodes.length === 0 && <div>No Nodes</div>}
            {treeNodes.map((node) => (
                <TreeNode
                    key={node.id}
                    node={node}
                    onCreateChild={handleCreateChild}
                />
            ))}

            {/* ここに dialog を 設置*/}
            <PlanNodeFormDialog
                open={isDialogOpen}
                onOpenChange={setIsDialogOpen}
                scenarioId={currentScenarioId}
                parentNode={targetParent}
            />
        </div>
    );
}
