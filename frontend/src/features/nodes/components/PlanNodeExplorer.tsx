import { usePlanNodes } from "@/features/nodes/api/usePlanNodes.ts";
import {
    buildTree,
    type PlanNodeWithChildren,
} from "@/features/nodes/utils/tree.ts";
import { ChevronDown, ChevronsRight } from "lucide-react";
import { useState } from "react";
import { getNodeIcon } from "@/features/nodes/components/NodeIcon.tsx";

const TreeNode = ({
    node,
    level = 0,
}: {
    node: PlanNodeWithChildren;
    level?: number;
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
                className="flex items-center py-1 px-2 hover:bg-slate-100 cursor-pointer rounded select-none transition-colors"
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
                </span>

                {/* icon */}
                <span className="mr-2">
                    {getNodeIcon(node.node_type, isOpen)}
                </span>

                {/* title */}
                <span className="text-sm text-slate-700 truncate">
                    {node.title}
                </span>

                {/* id (for Debug)*/}
                <span className="ml-2 text-xs text-slate-200">{node.id}</span>
            </div>

            {isOpen && hasChildren && (
                <div>
                    {node.children.map((child) => (
                        <TreeNode
                            key={child.id}
                            node={child}
                            level={level + 1}
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
    // 無加工のNodeデータを取得
    const { data: flatNodes, isLoading, isError } = usePlanNodes();

    if (isLoading) return <div className="p-4">読み込み中...</div>;

    if (isError)
        return (
            <div className="p-4 text-red-500">
                Nodeツリーの取得に失敗しました
            </div>
        );

    const treeNodes = buildTree(flatNodes);

    return (
        <div>
            {/* header */}
            <div className="">Ex</div>

            {/* node tree */}
            {treeNodes.length === 0 && <div>No Nodes</div>}
            {treeNodes.map((node) => (
                <TreeNode key={node.id} node={node} />
            ))}
        </div>
    );
}
