import { type SyntheticEvent, useState } from "react";
import { useNavigate } from "react-router-dom";

import { Card, CardContent } from "@/components/ui/card.tsx";
import { Button } from "@/components/ui/button.tsx";
import { Label } from "@/components/ui/label.tsx";
import { Input } from "@/components/ui/input.tsx";
import { api } from "@/lib/api.ts";

export function LoginForm() {
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");

    const navigate = useNavigate();

    const handleSubmit = async (e: SyntheticEvent) => {
        e.preventDefault();

        try {
            const response = await api.post("/auth/login", {
                email,
                password,
            });

            localStorage.setItem("token", response.data.access_token);

            navigate("/dashboard");
        } catch (error) {
            console.error("ログイン失敗...", error);
            alert(
                "ログインに失敗しました。メールアドレスかパスワードが間違っています。",
            );
        }
    };
    return (
        <Card className="rounded-md w-[380px]">
            <CardContent>
                <form onSubmit={handleSubmit} className="py-6">
                    <div className="py-6  space-y-3">
                        <div>
                            <Label className="text-slate-900">
                                メールアドレス
                            </Label>
                            <Input
                                type="email"
                                id="email"
                                name="email"
                                value={email}
                                onChange={(e) => setEmail(e.target.value)}
                            />
                        </div>
                        <div>
                            <Label className="text-slate-900">パスワード</Label>
                            <Input
                                type="password"
                                id="password"
                                name="password"
                                value={password}
                                onChange={(e) => setPassword(e.target.value)}
                            />
                        </div>
                    </div>
                    <Button
                        type="submit"
                        className="w-full py-6 rounded-sm bg-slate-800 text-white"
                    >
                        ログイン
                    </Button>
                    <p className="text-xs text-slate-700 pt-4">
                        続行することでGhostの利用規約に同意し、Ghostのプライバシーポリシーを確認したものとみなされます。
                    </p>
                </form>
                <div className="flex justify-center items-center">
                    <a className="text-xs text-sky-700">パスワードを忘れた方</a>
                    <span className="text-xs text-slate-400 px-2">/</span>
                    <a className="text-xs text-sky-700">
                        ログインでお困りの場合
                    </a>
                </div>
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
