import type { PlanNode } from "@/types";

// 元のPlanNode型に子Nodeを追加した型
export type PlanNodeWithChildren = PlanNode & {
    children: PlanNodeWithChildren[];
};

export const buildTree = (nodes: PlanNode[]): PlanNodeWithChildren[] => {
    const nodeMap = new Map<string, PlanNodeWithChildren>();
    const roots: PlanNodeWithChildren[] = [];

    nodes.forEach((node) => {
        nodeMap.set(node.id, { ...node, children: [] });
    });

    nodes.forEach((node) => {
        const currentNode = nodeMap.get(node.id);
        if (!currentNode) return;

        if (node.parent_id && nodeMap.has(node.parent_id)) {
            const parent = nodeMap.get(node.parent_id);
            parent?.children.push(currentNode);
        } else {
            roots.push(currentNode);
        }
    });

    const sortRecursive = (nodes: PlanNodeWithChildren[]) => {
        nodes.sort((a, b) => a.display_order - b.display_order);
        nodes.forEach((node) => {
            if (node.children.length > 0) {
                sortRecursive(node.children);
            }
        });
    };

    sortRecursive(roots);
    return roots;
};
