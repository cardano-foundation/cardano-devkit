import "./App.css";
import { AppSidebar } from "./components/AppSidebar";
import { SidebarTrigger } from "./components/ui/sidebar";
import Router from "./router";

function App() {

  return (
    <>
      <AppSidebar />
      <SidebarTrigger />
      <Router />
    </>
  );
}

export default App;
