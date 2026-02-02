import { Button } from "@/components/ui/button.tsx";
import { Card, CardContent, CardHeader } from "@/components/ui/card.tsx";
import { useNavigate } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import type { User } from "@/types";
import { api } from "@/lib/api.ts";

export function DashboardPage() {
    const navigate = useNavigate();

    const {
        data: user,
        isLoading,
        isError,
    } = useQuery<User>({
        queryKey: ["currentUser"],
        queryFn: async () => {
            const res = await api.get("/users/me");
            return res.data;
        },
        retry: false,
    });

    const handleLogout = () => {
        localStorage.removeItem("token");
        navigate("/login");
    };

    if (isError) {
        navigate("/login");
        return null;
    }

    if (isLoading) {
        return <div className="p-8">読み込み中...</div>;
    }

    return (
        <div className="h-screen p-6 space-y-4">
            <div className="text-center text-slate-900 font-semibold text-xl">
                ようこそ、ダッシュボードページへ
            </div>
            <div>
                <Card className="rounded-md">
                    <CardHeader className="text-slate-900 text-center">
                        ログイン情報
                    </CardHeader>
                    <CardContent className="py-4">
                        <p className="text-slate-900">ID: {user?.id}</p>
                        <p className="text-slate-900">name: {user?.name}</p>
                        <p className="text-slate-900">email: {user?.email}</p>
                    </CardContent>
                </Card>
            </div>
            <div className="flex justify-center">
                <Button onClick={handleLogout} className="bg-slate-800">
                    ログアウト
                </Button>
            </div>
        </div>
    );
}
