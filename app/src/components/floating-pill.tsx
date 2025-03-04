import { ModeToggle } from "@/components/mode-toggle";
import { SidebarTrigger } from "@/components/ui/sidebar";

export function FloatingPill() {
  return (
    <div className="fixed left-4 bottom-4 z-50">
      <div className="bg-background border rounded-full shadow-lg p-1 flex items-center w-[42px] hover:w-[98px] transition-all duration-300 ease-in-out">
        <div className="flex-shrink-0 h-8 w-8 flex items-center justify-center">
          <SidebarTrigger />
        </div>
        <div className="flex items-center overflow-hidden">
          <div className="w-[1px] h-4 mx-1 bg-border flex-shrink-0" />
          <div className="flex-shrink-0 h-8 w-10 flex items-center justify-center">
            <ModeToggle />
          </div>
        </div>
      </div>
    </div>
  );
}
