import { ScrollArea } from "@/components/ui/scroll-area";
import ThemeToggle from "@/components/theme-toggle";
import { Separator } from "@/components/ui/separator";
import SettingsDialog from "@/components/settings-dialog";
import useRssItems from "@/hooks/rss-items/use-rss-items";
import { Skeleton } from "@/components/ui/skeleton";
import { RssItem } from "@/lib/types";
import useStore from "@/lib/store/store";

const Sidebar = () => {
  const {
    data: rssItems,
    isLoading: isRssItemsLoading,
    isError: isRssItemsError,
    error: rssItemsError,
  } = useRssItems();

  const setSelectedText = useStore((state) => state.setSelectedText);
  const setSelectedRssItem = useStore((state) => state.setSelectedRssItem);

  const handleItemClick = (item: RssItem) => {
    setSelectedText(item.description);
    setSelectedRssItem(item);
  };

  return (
    <ScrollArea className="w-full h-full max-h-screen min-h-screen">
      <div className="flex flex-col w-full h-full min-h-screen gap-2 p-2">
        <div className="flex flex-col max-h-[calc(100vh-125px)] h-full overflow-y-auto">
          {isRssItemsLoading && (
            <div className="flex flex-col gap-1">
              <Skeleton className="w-full h-10" />
              <Skeleton className="w-full h-10" />
              <Skeleton className="w-full h-10" />
            </div>
          )}
          {isRssItemsError && (
            <p>Error fetching rss items: {rssItemsError.message}</p>
          )}
          {rssItems &&
            rssItems.map((rssItem, index) => (
              <p
                className="transition-all duration-300 cursor-pointer hover:underline hover:text-blue-500"
                key={`rss-item-${index}`}
                onClick={() => handleItemClick(rssItem)}
              >
                {rssItem.title}
              </p>
            ))}
        </div>
        <span className="flex-1" />
        <Separator />
        <ThemeToggle />
        <SettingsDialog />
      </div>
    </ScrollArea>
  );
};

export default Sidebar;
