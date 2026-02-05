import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
} from "@/components/ui/dialog.tsx";
import * as z from "zod";
import type { CreatePlanNodeDTO, PlanNode } from "@/types";
import { usePlanNodeMutations } from "@/features/nodes/api/usePlanNodes.ts";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { useEffect } from "react";
import {
    Form,
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form.tsx";
import { Input } from "@/components/ui/input.tsx";
import { Button } from "@/components/ui/button.tsx";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select.tsx";
import { Textarea } from "@/components/ui/textarea.tsx";

const NODE_TYPES = [
    { value: "Initiative", label: "Initiative (戦略/方針/領域)" },
    { value: "Project", label: "Project (プロジェクト)" },
    { value: "SubProject", label: "Sub Project (子プロジェクト)" },
    { value: "Job", label: "Job (ジョブ/施策)" },
    { value: "AdjustmentBuffer", label: "AdjustmentBuffer (バッファ)" },
];

const formSchema = z.object({
    title: z.string().min(1, "タイトルは必須です"),
    node_type: z.string().min(1, "Nodeタイプを選択してください"),
    description: z.string().optional(),
    display_order: z.coerce.number(),
});

type FormValues = z.infer<typeof formSchema>;

type Props = {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    scenarioId: string;
    parentNode?: PlanNode | null;
    nodeToEdit?: PlanNode | null;
};

export function PlanNodeFormDialog({
    open,
    onOpenChange,
    scenarioId,
    parentNode,
    nodeToEdit,
}: Props) {
    const { createPlanNode } = usePlanNodeMutations();

    const form = useForm({
        resolver: zodResolver(formSchema),
        defaultValues: {
            title: "",
            node_type: "Project",
            description: "",
            display_order: 0,
        },
    });

    useEffect(() => {
        if (open) {
            if (nodeToEdit) {
                form.reset({
                    title: nodeToEdit.title,
                    node_type: nodeToEdit.node_type,
                    description: nodeToEdit.description,
                    display_order: nodeToEdit.display_order,
                });
            } else {
                form.reset({
                    title: "",
                    node_type: "Project",
                    description: "",
                    display_order: 0,
                });
            }
        }
    }, [open, nodeToEdit, form]);

    const onSubmit = async (values: FormValues) => {
        try {
            if (nodeToEdit) {
                console.log("Update not implemented yet");
            } else {
                const dto: CreatePlanNodeDTO = {
                    scenario_id: scenarioId,
                    parent_id: parentNode?.id,
                    title: values.title,
                    node_type: values.node_type,
                    description: values.description,
                    display_order: values.display_order,
                };
                await createPlanNode.mutateAsync(dto);
            }
            onOpenChange(false);
        } catch (error) {
            console.error(error);
        }
    };

    const dialogTitle = nodeToEdit
        ? "ノード編集"
        : parentNode
          ? `"${parentNode.title}"の下に子ノードを作成`
          : "ルートノードを作成";

    return (
        <Dialog open={open} onOpenChange={onOpenChange}>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>{dialogTitle}</DialogTitle>
                </DialogHeader>
                <Form {...form}>
                    <form
                        onSubmit={form.handleSubmit(onSubmit)}
                        className="space-y-4"
                    >
                        {/* title */}
                        <FormField
                            control={form.control}
                            name="title"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>タイトル</FormLabel>
                                    <FormControl>
                                        <Input
                                            placeholder="ノード名を入力"
                                            {...field}
                                        />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />

                        {/* node type*/}
                        <FormField
                            control={form.control}
                            name="node_type"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>ノードタイプ</FormLabel>
                                    <Select
                                        onValueChange={field.onChange}
                                        defaultValue={field.value}
                                    >
                                        <FormControl>
                                            <SelectTrigger>
                                                <SelectValue placeholder="タイプを選択" />
                                            </SelectTrigger>
                                        </FormControl>
                                        <SelectContent>
                                            {NODE_TYPES.map((type) => (
                                                <SelectItem
                                                    key={type.value}
                                                    value={type.value}
                                                >
                                                    {type.label}
                                                </SelectItem>
                                            ))}
                                        </SelectContent>
                                    </Select>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />

                        {/* display_order */}
                        <FormField
                            control={form.control}
                            name="display_order"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>表示順</FormLabel>
                                    <FormControl>
                                        <Input
                                            type="number"
                                            {...field}
                                            value={(field.value as number) || 0}
                                            className="w-32 text-right"
                                        />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />

                        {/* description*/}
                        <FormField
                            control={form.control}
                            name="description"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>説明</FormLabel>
                                    <FormControl>
                                        <Textarea {...field} />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />

                        <div className="flex justify-end pt-4">
                            <Button type="submit">作成する</Button>
                        </div>
                    </form>
                </Form>
            </DialogContent>
        </Dialog>
    );
}
