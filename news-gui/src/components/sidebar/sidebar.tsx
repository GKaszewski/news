import { ScrollArea } from '@/components/ui/scroll-area';
import ThemeToggle from '@/components/theme-toggle';
import { Separator } from '@/components/ui/separator';
import SettingsDialog from '@/components/settings-dialog';

const Sidebar = () => {
  return (
    <ScrollArea className="h-full w-full min-h-screen max-h-screen">
      <div className="flex flex-col gap-2 w-full h-full min-h-screen p-2">
        <p>News 1</p>
        <p>News 2</p>
        <p>News 3</p>
        <span className="flex-1" />
        <Separator />
        <ThemeToggle />
        <SettingsDialog />
      </div>
    </ScrollArea>
  );
};

export default Sidebar;
