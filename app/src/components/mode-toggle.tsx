import { useTheme } from "@/components/providers/theme-provider";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { invoke } from "@tauri-apps/api/core";
import { Moon, Sun } from "lucide-react";
import React, { useEffect } from "react";

export const ModeToggle = React.forwardRef<React.ComponentRef<typeof Button>, React.ComponentProps<typeof Button>>(
  ({ className, ...props }, ref) => {
    const { setTheme, theme } = useTheme();

    useEffect(() => {
      invoke("set_window_theme", {
        theme,
      });
    }, [theme]);

    return (
      <Button
        ref={ref}
        className={cn("h-7 w-7", className)}
        variant="ghost"
        size="icon"
        onClick={() => setTheme(theme === "light" ? "dark" : "light")}
        {...props}
      >
        <Sun className="h-[1.5rem] w-[1.3rem] dark:hidden" />
        <Moon className="hidden h-5 w-5 dark:block" />
        <span className="sr-only">Toggle theme</span>
      </Button>
    );
  },
);
