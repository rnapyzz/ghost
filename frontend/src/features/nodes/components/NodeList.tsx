import { usePlanNodes } from "@/features/nodes/api/usePlanNodes.ts";
import { FileText, Layers } from "lucide-react";

export function NodeList() {
    const { data: nodes, isLoading, isError } = usePlanNodes();

    if (isLoading)
        return <div className="p-4 ">ノードデータの読み込み中...</div>;

    if (isError)
        return <div className="p-4 text-red-500">エラーが発生しました</div>;

    const getTypeIcon = (type: string) => {
        switch (type) {
            case "Project":
                return <Layers />;
            default:
                return <FileText />;
        }
    };

    return <div>Node List Here.</div>;
}
