import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { LoginPage } from "@/pages/auth/LoginPage.tsx";
import { DashboardPage } from "@/pages/dashboard/DashboardPage.tsx";
import { AppLayout } from "@/components/layout/AppLayout.tsx";
import { NodesPage } from "@/pages/nodes/NodesPage.tsx";
import { ScenariosPage } from "@/pages/scenarios/ScenariosPage.tsx";
import { ServicesPage } from "@/pages/services/ServicesPage.tsx";
import { ScenarioSettingPage } from "@/pages/settings/ScenarioSettingPage.tsx";

function App() {
    return (
        <BrowserRouter>
            <Routes>
                <Route path="/login" element={<LoginPage />} />

                <Route element={<AppLayout />}>
                    <Route path="/" element={<DashboardPage />} />
                    <Route path="/dashboard" element={<DashboardPage />} />
                    <Route path="/nodes" element={<NodesPage />} />
                    <Route path="/scenarios" element={<ScenariosPage />} />
                    <Route path="/services" element={<ServicesPage />} />
                    <Route
                        path="/settings/scenario"
                        element={<ScenarioSettingPage />}
                    />
                </Route>

                <Route path="*" element={<Navigate to="/login" />}></Route>
            </Routes>
        </BrowserRouter>
    );
}

export default App;
