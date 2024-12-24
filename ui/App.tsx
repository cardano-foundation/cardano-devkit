import { useLocation } from "react-router-dom";
import "./App.css";
import { AppSidebar } from "./components/AppSidebar";
import Header from "./components/header";
import Router from "./router";
import { ROUTES_TITLES } from "./constants/routes";

function App() {
  const location = useLocation();

  const currentTitle = ROUTES_TITLES[location.pathname] || "Cardano Devkit";

  return (
    <>
      <AppSidebar />
      <div className="flex-1 flex flex-col">
        <Header title={currentTitle} />
        <Router />
      </div>
    </>
  );
}

export default App;
