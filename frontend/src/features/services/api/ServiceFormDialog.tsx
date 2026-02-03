import * as z from "zod";
import type { Service } from "@/types";
import { useServiceMutations } from "@/features/services/api/useServices.ts";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { useEffect } from "react";
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
} from "@/components/ui/form.tsx";
import { Input } from "@/components/ui/input.tsx";
import { Button } from "@/components/ui/button.tsx";

const formSchema = z.object({
    name: z.string().min(1, "サービス名は必須です"),
    slug: z.string().min(1, "url表示名称は必須です"),
    display_order: z.coerce.number().min(0, "0以上の数値を入力してください"),
});

type FormValues = z.infer<typeof formSchema>;

type Props = {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    serviceToEdit?: Service | null;
};

export function ServiceFormDialog({
    open,
    onOpenChange,
    serviceToEdit,
}: Props) {
    const { createService } = useServiceMutations();

    const form = useForm({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: "",
            slug: "",
            display_order: 0,
        },
    });

    useEffect(() => {
        if (open) {
            if (serviceToEdit) {
                form.reset({
                    name: serviceToEdit.name,
                    slug: serviceToEdit.slug,
                    display_order: serviceToEdit.display_order,
                });
            } else {
                form.reset({
                    name: "",
                    slug: "",
                    display_order: 0,
                });
            }
        }
    }, [open, serviceToEdit, form]);

    const onSubmit = async (values: FormValues) => {
        try {
            if (serviceToEdit) {
                console.log("not implemented");
            } else {
                await createService.mutateAsync(values);
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
                        {serviceToEdit
                            ? "サービスの編集"
                            : "新しいサービスの作成"}
                    </DialogTitle>
                </DialogHeader>
                <Form {...form}>
                    <form
                        onSubmit={form.handleSubmit(onSubmit)}
                        className="space-y-4"
                    >
                        <FormField
                            name="name"
                            control={form.control}
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>
                                        サービス名
                                        <span className="text-red-700 text-sm">
                                            {" *"}
                                        </span>
                                    </FormLabel>
                                    <FormControl>
                                        <Input {...field} />
                                    </FormControl>
                                </FormItem>
                            )}
                        ></FormField>
                        <FormField
                            name="slug"
                            control={form.control}
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>
                                        slug
                                        <span className="text-red-700 text-sm">
                                            {" *"}
                                        </span>
                                    </FormLabel>
                                    <FormControl>
                                        <Input
                                            {...field}
                                            placeholder="e.g.  special-service"
                                        />
                                    </FormControl>
                                    <span className="text-xs text-slate-500">
                                        ※
                                        URLの末尾に付くサービス名を識別する固有の文字列です。
                                    </span>
                                </FormItem>
                            )}
                        ></FormField>
                        <FormField
                            name="display_order"
                            control={form.control}
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>
                                        表示順
                                        <span className="text-red-700 text-sm">
                                            {" *"}
                                        </span>
                                    </FormLabel>
                                    <FormControl>
                                        <Input
                                            type="number"
                                            {...field}
                                            value={(field.value as number) || 0}
                                            className="w-32 text-right"
                                        />
                                    </FormControl>
                                </FormItem>
                            )}
                        ></FormField>
                        <div className="flex justify-end p-4">
                            <Button type="submit">
                                {serviceToEdit ? "保存する" : "作成する"}
                            </Button>
                        </div>
                    </form>
                </Form>
            </DialogContent>
        </Dialog>
    );
}
