import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  navigationMenuTriggerStyle,
} from "@/components/ui/navigation-menu";

export default function NavHeader() {
  const [token, setToken] = useState<string | null>("pending");

  useEffect(() => {
    setToken(localStorage.getItem("token"));
  }, []);

  return (
    <header className="border-b bg-background">
      <div className="flex h-16 items-center px-4 container mx-auto">
        <div className="flex items-center gap-6 flex-1">
          <a href="/landing" className="flex items-center gap-2 no-underline">
            <h1 className="text-3xl font-bold text-primary">Flash QC</h1>
          </a>

          {token && token !== "pending" && (
            <NavigationMenu>
              <NavigationMenuList>
                <NavigationMenuItem>
                  <NavigationMenuLink
                    href="/"
                    className={navigationMenuTriggerStyle()}
                  >
                    Study
                  </NavigationMenuLink>
                </NavigationMenuItem>
                <NavigationMenuItem>
                  <NavigationMenuLink
                    href="/"
                    className={navigationMenuTriggerStyle()}
                  >
                    Library
                  </NavigationMenuLink>
                </NavigationMenuItem>
                <NavigationMenuItem>
                  <NavigationMenuLink
                    href="/"
                    className={navigationMenuTriggerStyle()}
                  >
                    Plugins
                  </NavigationMenuLink>
                </NavigationMenuItem>
              </NavigationMenuList>
            </NavigationMenu>
          )}
        </div>

        <nav className="flex items-center gap-4">
          {token === null && (
            <Button asChild variant="default">
              <a href="/login">Login</a>
            </Button>
          )}
          {token && token !== "pending" && (
            <>
              <Button asChild variant="ghost">
                <a href="/logout">Logout</a>
              </Button>
              <Button asChild variant="outline" size="icon">
                <a href="/profile">
                  <img
                    src="/circle-user.svg"
                    className="h-5 w-5"
                    alt="Profile"
                  />
                </a>
              </Button>
            </>
          )}
        </nav>
      </div>
    </header>
  );
}
