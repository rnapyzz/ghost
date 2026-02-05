import {
    Box,
    Compass,
    FileText,
    Folder,
    FolderOpen,
    PackageOpen,
} from "lucide-react";

export const getNodeIcon = (nodeType: string, isOpen: boolean) => {
    const props = { className: "w-4 h-4 text-slate-500" };

    switch (nodeType) {
        case "Initiative":
            return <Compass {...props} className="w-4 h-4 text-blue-500" />;
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
            return <Box {...props} className="w-4 h-4 text-orange-500" />;
        case "AdjustmentBuffer":
            return (
                <PackageOpen {...props} className="w-4- h-4 text-slate-300" />
            );
        default:
            return <FileText {...props} />;
    }
};
