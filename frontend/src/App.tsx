import { BrowserRouter, Routes, Route } from "react-router-dom";
import { LoginPage } from "@/pages/auth/LoginPage.tsx";
import { DashboardPage } from "@/pages/dashboard/DashboardPage.tsx";

function App() {
    return (
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<DashboardPage />} />
                <Route path="/dashboard" element={<DashboardPage />} />
                <Route path="/login" element={<LoginPage />} />
            </Routes>
        </BrowserRouter>
    );
}

export default App;
