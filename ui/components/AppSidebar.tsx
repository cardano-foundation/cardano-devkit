import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarHeader,
    SidebarMenuButton,
} from "@/components/ui/sidebar"
import cardanoBlack from "@/assets/cardanoBlack.svg"
import { ROUTES } from "@/constants/routes"
import { Home } from "lucide-react"

const items = [
    {
        title: "Home",
        url: ROUTES.LANDING,
        icon: Home,
    },

]
export function AppSidebar() {
    return (
        <Sidebar>
            <SidebarHeader >
                <div style={{ display: "flex" }}>
                    <img src={cardanoBlack} alt="Cardano logo" />
                    Cardano Devkit
                </div>
            </SidebarHeader>
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupLabel>Menu</SidebarGroupLabel>
                    <SidebarGroupContent>
                        {items.map((item) => (
                            <SidebarMenuButton asChild>
                                <a href={item.url}>
                                    <item.icon />
                                    <span>{item.title}</span>
                                </a>
                            </SidebarMenuButton>
                        ))}
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
            <SidebarFooter />
        </Sidebar>
    )
}