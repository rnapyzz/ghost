import { Link, Outlet, useLocation } from "react-router-dom";
import { Button } from "@/components/ui/button.tsx";

export function AppLayout() {
    const location = useLocation();

    const getLinkClass = (path: string) => {
        const isActive = location.pathname === path;
        return `block w-full p-2 rounded shadow-sm text-sm transition-colors ${
            isActive
                ? "bg-slate-400 text-white font-semibold"
                : "bg-white text-slate-900 hover:bg-slate-100"
        }`;
    };

    return (
        <div className="h-screen flex flex-col">
            {/* header */}
            <div className="h-14 shrink-0 py-2 px-4 border-b border-slate-200 flex items-center justify-between">
                <div className="text-slate-800 text-xl font-bold">Ghost</div>
                <div className="text-sm text-slate-800">
                    何かボタンを置くかもエリア
                </div>
            </div>

            <div className="flex-1 flex overflow-hidden">
                {/* sidebar 1 */}
                <nav className="py-4 px-2 bg-slate-800 flex flex-col justify-start items-center space-y-6 overflow-y-auto">
                    <div className="flex flex-col justify-start items-center space-y-4">
                        <Button className="h-11 w-11 rounded-xl bg-slate-700 hover:bg-slate-600">
                            1
                        </Button>
                        <Button className="h-11 w-11 rounded-xl bg-slate-700 hover:bg-slate-600">
                            2
                        </Button>
                        <Button className="h-11 w-11 rounded-xl bg-slate-700 hover:bg-slate-600">
                            3
                        </Button>
                    </div>
                </nav>
                {/* sidebar 2 */}
                <nav className="w-56 p-4 bg-slate-50 border-r border-slate-200 overflow-y-auto">
                    <ul className="space-y-2">
                        <li>
                            <Link
                                to="/dashboard"
                                className={getLinkClass("/dashboard")}
                            >
                                ダッシュボード
                            </Link>
                        </li>
                        <li>
                            <Link
                                to="/nodes"
                                className={getLinkClass("/nodes")}
                            >
                                ノード一覧
                            </Link>
                        </li>
                    </ul>
                </nav>
                <main className="flex-1 bg-white overflow-y-auto p-2">
                    <Outlet />
                </main>
            </div>
        </div>
    );
}
