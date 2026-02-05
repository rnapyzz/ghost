import { Link, Outlet, useLocation } from "react-router-dom";
import { Button } from "@/components/ui/button.tsx";
import { useState } from "react";
import { cn } from "@/lib/utils.ts";
import {
    BookMarked,
    ChevronLeft,
    ChevronsRight,
    LayoutDashboard,
    Network,
    Settings,
} from "lucide-react";

export function AppLayout() {
    const location = useLocation();

    const [isCollapsed, setIsCollapsed] = useState(false);

    const getLinkClass = (path: string) => {
        const isActive = location.pathname === path;
        return cn(
            "flex items-center rounded-md transition-all duration-200 text-sm",
            isCollapsed
                ? "justify-center w-10 h-10 p-0"
                : "w-full px-3 py-2 space-x-3",
            isActive
                ? "bg-slate-800 text-white shadow-sm"
                : "text-slate-700 hover:bg-slate-200 hover:text-slate-900",
        );
    };

    return (
        <div className="h-screen flex flex-col">
            {/* header */}
            <div className="h-14 shrink-0 py-2 px-4 border-b border-slate-200 flex items-center justify-between">
                <div className="text-slate-800 text-xl font-bold">Ghost</div>
                <div className="flex items-center space-x-2">
                    <p className="text-sm text-slate-500">
                        何かボタンを置くかもエリア
                    </p>
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
                <nav
                    className={cn(
                        "bg-slate-100 border-r border-slate-200 flex-col transition-all duration-100 ease-in-out relative",
                        isCollapsed ? "w-[60px]" : "w-56",
                    )}
                >
                    <button
                        onClick={() => setIsCollapsed(!isCollapsed)}
                        className="absolute -right-3 top-2 bg-white border border-slate-200 rounded-full p-1 shadow-sm hover:bg-slate-50 z-30 text-slate-500"
                    >
                        {isCollapsed ? (
                            <ChevronsRight size={14} />
                        ) : (
                            <ChevronLeft size={14} />
                        )}
                    </button>

                    <div className="flex-1 overflow-y-auto py-4 px-3 space-y-2">
                        <ul className="space-y-1">
                            <li>
                                <Link
                                    to="/dashboard"
                                    className={getLinkClass("/dashboard")}
                                >
                                    <LayoutDashboard
                                        size={20}
                                        className="shrink-0"
                                    />
                                    {!isCollapsed && (
                                        <span className="font-medium text-sm overflow-hidden whitespace-nowrap">
                                            ダッシュボード
                                        </span>
                                    )}
                                </Link>
                            </li>
                            <li>
                                <Link
                                    to="/nodes"
                                    className={getLinkClass("/nodes")}
                                >
                                    <Network size={20} className="shrink-0" />
                                    {!isCollapsed && (
                                        <span className="font-medium text-sm overflow-hidden whitespace-nowrap">
                                            ノード一覧
                                        </span>
                                    )}
                                </Link>
                            </li>
                            <li>
                                <Link
                                    to="/scenarios"
                                    className={getLinkClass("/scenarios")}
                                >
                                    <BookMarked
                                        size={20}
                                        className="shrink-0"
                                    />
                                    {!isCollapsed && (
                                        <span className="font-medium text-sm overflow-hidden whitespace-nowrap">
                                            シナリオ設定
                                        </span>
                                    )}
                                </Link>
                            </li>
                            <li>
                                <Link
                                    to="/services"
                                    className={getLinkClass("/services")}
                                >
                                    <Settings size={20} className="shrink-0" />
                                    {!isCollapsed && (
                                        <span className="font-medium text-sm overflow-hidden whitespace-nowrap">
                                            サービス設定
                                        </span>
                                    )}
                                </Link>
                            </li>
                        </ul>
                    </div>

                    <div className="p-3 border-t border-slate-100"></div>
                </nav>

                {/* main content */}
                <main className="flex-1 bg-white overflow-y-auto p-2">
                    <Outlet />
                </main>
            </div>
        </div>
    );
}
