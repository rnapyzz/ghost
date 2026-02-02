import { usePlanNodes } from "@/features/nodes/api/usePlanNodes.ts";

export function NodeList() {
    const { data: node, isLoading, isError } = usePlanNodes();

    if (isLoading)
        return <div className="p-4 ">ノードデータの読み込み中...</div>;

    if (isError)
        return <div className="p-4 text-red-500">エラーが発生しました</div>;

    return <div>Node List Here.</div>;
}
