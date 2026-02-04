import {
    FileText,
    Folder,
    FolderOpen,
    Hammer,
    Scale,
    Target,
} from "lucide-react";

export const getNodeIcon = (nodeType: string, isOpen: boolean) => {
    const props = { className: "w-4 h-4 text-slate-500" };

    switch (nodeType) {
        case "Initiative":
            return <Target {...props} className="w-4 h-4 text-blue-500" />;
        case "Project":
            return isOpen ? (
                <FolderOpen {...props} className="w-4 h-4 text-blue-500" />
            ) : (
                <Folder {...props} className="w-4 h-4 text-blue-500" />
            );
        case "SubProject":
            return isOpen ? (
                <FolderOpen {...props} className="w-4 h-4 text-blue-500" />
            ) : (
                <Folder {...props} className="w-4 h-4 text-blue-500" />
            );
        case "Job":
            return <Hammer {...props} className="w-4 h-4 text-blue-500" />;
        case "AdjustmentBuffer":
            return <Scale {...props} className="w-4- h-4 text-orange-400" />;
        default:
            return <FileText {...props} />;
    }
};
