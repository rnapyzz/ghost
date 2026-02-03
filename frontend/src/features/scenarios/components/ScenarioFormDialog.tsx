import type { Scenario } from "@/types";
import { useScenarioMutations } from "@/features/scenarios/api/useScenarios.ts";
import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
} from "@/components/ui/dialog.tsx";
import {
    Form,
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form.tsx";
import { useForm } from "react-hook-form";
import * as z from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { Input } from "@/components/ui/input.tsx";
import { useEffect } from "react";
import { Textarea } from "@/components/ui/textarea.tsx";
import { Button } from "@/components/ui/button.tsx";

// バリデーションのルールを作成
const formSchema = z
    .object({
        name: z.string().min(1, "シナリオ名は必須です"),
        description: z.string().optional(),
        start_date: z.string().min(1, "開始日は必須です"),
        end_date: z.string().min(1, "終了日は必須です"),
    })
    .refine((data) => new Date(data.start_date) <= new Date(data.end_date), {
        message: "終了日は開始日より後である必要があります",
        path: ["end_date"],
    });
// 型として定義
type FormValues = z.infer<typeof formSchema>;

// Propsの型を定義
type Props = {
    open: boolean; // dialogが開いている（表示されている）かどうか
    onOpenChange: (open: boolean) => void; //
    scenarioToEdit?: Scenario | null; // 編集対象のシナリオ情報
};

// コンポーネントを定義
export function ScenarioFormDialog({
    open,
    onOpenChange,
    scenarioToEdit,
}: Props) {
    const { createScenario, updateScenario } = useScenarioMutations();

    // useFormを使用してreact-hook-formの監視機能を用意
    const form = useForm({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: "",
            description: "",
            start_date: "",
            end_date: "",
        },
    });

    // コンポーネント生成時の処理（初期値のセット）
    useEffect(() => {
        if (open) {
            if (scenarioToEdit) {
                // 編集モード
                form.reset({
                    name: scenarioToEdit.name,
                    description: scenarioToEdit.description || "",
                    start_date: scenarioToEdit.start_date,
                    end_date: scenarioToEdit.end_date,
                });
            } else {
                form.reset({
                    name: "",
                    description: "",
                    start_date: "",
                    end_date: "",
                });
            }
        }
    }, [open, scenarioToEdit, form]);

    // submit時の処理
    const onSubmit = async (values: FormValues) => {
        try {
            if (scenarioToEdit) {
                await updateScenario.mutateAsync({
                    id: scenarioToEdit.id,
                    data: values,
                });
            } else {
                await createScenario.mutateAsync(values);
            }
            onOpenChange(false);
        } catch (error) {
            console.error(error);
        }
    };

    return (
        <Dialog open={open} onOpenChange={onOpenChange}>
            <DialogContent className="sm:max-w-[500px]">
                <DialogHeader>
                    <DialogTitle>
                        {scenarioToEdit
                            ? "シナリオの編集"
                            : "新しいシナリオの作成"}
                    </DialogTitle>
                </DialogHeader>

                <Form {...form}>
                    <form
                        onSubmit={form.handleSubmit(onSubmit)}
                        className="space-y-4"
                    >
                        {/* シナリオ名 */}
                        <FormField
                            name="name"
                            control={form.control}
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>シナリオ名</FormLabel>
                                    <FormControl>
                                        <Input
                                            placeholder="e.g. FY2026 期初計画"
                                            {...field}
                                        />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />

                        <div className="grid grid-cols-2 gap-4">
                            {/* 開始日*/}
                            <FormField
                                name="start_date"
                                control={form.control}
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>開始日</FormLabel>
                                        <FormControl>
                                            <Input type="date" {...field} />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            ></FormField>

                            {/* 終了日 */}
                            <FormField
                                name="end_date"
                                control={form.control}
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>終了日</FormLabel>
                                        <FormControl>
                                            <Input type="date" {...field} />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            ></FormField>
                        </div>

                        {/* シナリオの説明 */}
                        <FormField
                            name="description"
                            control={form.control}
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>説明 (任意)</FormLabel>
                                    <FormControl>
                                        <Textarea
                                            placeholder="シナリオの説明を記載"
                                            {...field}
                                        />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        ></FormField>

                        {/* ボタン */}
                        <div className="flex justify-end pt-4">
                            <Button type="submit">
                                {scenarioToEdit ? "保存する" : "作成する"}
                            </Button>
                        </div>
                    </form>
                </Form>
            </DialogContent>
        </Dialog>
    );
}
