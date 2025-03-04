import { AppSidebar } from "@/components/app-sidebar";
import { FloatingPill } from "@/components/floating-pill";
import { SidebarProvider } from "@/components/ui/sidebar";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <SidebarProvider>
      <AppSidebar />
      <main className="relative min-h-screen">{children}</main>
      <FloatingPill />
    </SidebarProvider>
  );
}
