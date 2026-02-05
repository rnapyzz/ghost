import {
    Table,
    TableBody,
    TableCaption,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table.tsx";
import { useServices } from "@/features/services/api/useServices.ts";
import { useState } from "react";
import type { Service } from "@/types";
import { Button } from "@/components/ui/button.tsx";
import { Plus } from "lucide-react";
import { ServiceFormDialog } from "@/features/services/components/ServiceFormDialog.tsx";

export function ServiceList() {
    const { data: services, isLoading, isError } = useServices();

    const [isDialogOpen, setIsDialogOpen] = useState(false);
    const [editingService, setEditingService] = useState<Service | null>(null);

    const handleCreate = () => {
        setEditingService(null);
        setIsDialogOpen(true);
    };

    if (isLoading) return <div className="p-4">読み込み中...</div>;

    if (isError)
        return (
            <div className="p-4 text-red-500">サービスの取得に失敗しました</div>
        );

    return (
        <div className="space-y-4">
            <div className="flex items-center justify-between">
                <h2 className="text-xl font-bold text-slate-800">
                    サービス設定
                </h2>
                <Button onClick={handleCreate}>
                    <Plus className="h-4 w-4" />
                    サービスの追加
                </Button>
            </div>
            <div>
                <Table>
                    <TableCaption></TableCaption>
                    <TableHeader>
                        <TableRow>
                            <TableHead>name</TableHead>
                            <TableHead>#slug</TableHead>
                            <TableHead>updated_at</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {services?.length === 0 && (
                            <TableRow>
                                <TableCell
                                    colSpan={3}
                                    className="text-center text-slate-400 py-8"
                                >
                                    サービスは作成されていません
                                </TableCell>
                            </TableRow>
                        )}
                        {services?.map((s: Service) => (
                            <TableRow key={s.id}>
                                <TableCell>{s.name}</TableCell>
                                <TableCell>#{s.slug}</TableCell>
                                <TableCell>#{s.updated_at}</TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </div>

            <ServiceFormDialog
                open={isDialogOpen}
                onOpenChange={setIsDialogOpen}
                serviceToEdit={editingService}
            />
        </div>
    );
}
