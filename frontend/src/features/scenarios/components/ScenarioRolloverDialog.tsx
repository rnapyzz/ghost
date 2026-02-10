import * as z from "zod";
import type { Scenario } from "@/types";
import { format, addYears } from "date-fns";
import { useState } from "react";
import { useScenarioMutations } from "@/features/scenarios/api/useScenarios.ts";
import { Form, useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog.tsx";
import { Copy, Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button.tsx";
import {
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form.tsx";
import { Input } from "@/components/ui/input.tsx";

const formSchema = z.object({
    name: z.string().min(1, "シナリオ名は必須です"),
    start_date: z
        .string()
        .regex(/^\d{4}-\d{2}-\d{2}$/, "正しい日付を入力してください"),
    end_date: z
        .string()
        .regex(/^\d{4}-\d{2}-\d{2}$/, "正しい日付を入力してください"),
});

type Props = {
    sourceScenario: Scenario;
};

export function ScenarioRolloverDialog({ sourceScenario }: Props) {
    const [open, setOpen] = useState(false);
    const { rolloverScenario } = useScenarioMutations();

    const defaultStartDate = format(
        addYears(new Date(sourceScenario.start_date), 1),
        "yyyy-MM-dd",
    );
    const defaultEndDate = format(
        addYears(new Date(sourceScenario.end_date), 1),
        "yyyy-MM-dd",
    );

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: `Next Year Plan (from ${sourceScenario.name})`,
            start_date: defaultStartDate,
            end_date: defaultEndDate,
        },
    });

    const onSubmit = async (values: z.infer<typeof formSchema>) => {
        try {
            await rolloverScenario.mutateAsync({
                sourceId: sourceScenario.id,
                data: values,
            });
            setOpen(false);
        } catch (error) {
            console.error(error);
            // TODO: エラーハンドリング
        }
    };

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
                <Button variant="outline" size="sm" className="gap-2">
                    <Copy className="w-4 h-4" />
                    翌期作成（コピー）
                </Button>
            </DialogTrigger>
            <DialogContent className="sm:max-x-[425px]">
                <DialogHeader>
                    <DialogTitle>シナリオの繰り返し（コピー）</DialogTitle>
                    <DialogDescription>
                        「{sourceScenario.name}
                        」の構造と数値をコピーして、新しいシナリオを作成します。
                        <br />
                        作成後、自動的に新しいシナリオが「Current
                        Scenario」として設定されます。
                    </DialogDescription>
                </DialogHeader>
            </DialogContent>

            <Form {...form}>
                <form
                    onSubmit={form.handleSubmit(onSubmit)}
                    className="space-y-4"
                >
                    <FormField
                        control={form.control}
                        name="name"
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>新しいシナリオ名</FormLabel>
                                <FormControl>
                                    <Input {...field} />
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    ></FormField>
                    <div className="grid grid-cols-2 gap-4">
                        <FormField
                            control={form.control}
                            name="start_date"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>開始日</FormLabel>
                                    <FormControl>
                                        <Input type="date" {...field} />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <FormField
                            control={form.control}
                            name="end_date"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>終了日</FormLabel>
                                    <FormControl>
                                        <Input type="date" {...field} />
                                    </FormControl>
                                </FormItem>
                            )}
                        />
                    </div>

                    <DialogFooter>
                        <Button
                            type="submit"
                            disabled={rolloverScenario.isPending}
                        >
                            {rolloverScenario.isPending && (
                                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                            )}
                            シナリオを作成して切り替え
                        </Button>
                    </DialogFooter>
                </form>
            </Form>
        </Dialog>
    );
}
