import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
} from "@/components/ui/card.tsx";
import { Button } from "@/components/ui/button.tsx";
import { Label } from "@/components/ui/label.tsx";
import { Input } from "@/components/ui/input.tsx";
import * as React from "react";

export function LoginForm() {
    const handleSubmit = (e: React.SubmitEvent) => {
        e.preventDefault();
        console.log(e.target.value);
    };
    return (
        <Card className="rounded-md w-[380px]">
            <CardContent>
                <form onSubmit={handleSubmit} className="py-6">
                    <div className="py-4 space-y-3">
                        <div>
                            <Label className="text-slate-900">
                                メールアドレス
                            </Label>
                            <Input id="email" name="email"></Input>
                        </div>
                        <div>
                            <Label className="text-slate-900">パスワード</Label>
                            <Input id="password" name="password"></Input>
                        </div>
                    </div>
                </form>
                <div className="flex justify-center items-center mb-4">
                    <a className="text-xs text-sky-700">パスワードを忘れた方</a>
                    <span className="text-xs text-slate-400 px-2">/</span>
                    <a className="text-xs text-sky-700">
                        ログインでお困りの場合
                    </a>
                </div>
                <Button
                    type="submit"
                    className="w-full py-6 rounded-sm bg-slate-800 text-white"
                >
                    ログイン
                </Button>
                <p className="text-xs text-slate-700 py-4">
                    続行することでGhostの利用規約に同意し、Ghostのプライバシーポリシーを確認したものとみなされます。
                </p>
            </CardContent>
            <div className="flex justify-between items-center p-6 border-t">
                <p className="text-sm text-slate-900">
                    アカウントが未登録ですか？
                </p>
                <a className="text-sm text-sky-700 font-bold">
                    新規アカウント登録
                </a>
            </div>
        </Card>
    );
}
