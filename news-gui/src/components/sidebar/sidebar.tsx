import { ScrollArea } from '@/components/ui/scroll-area';
import ThemeToggle from '@/components/theme-toggle';
import { Separator } from '@/components/ui/separator';
import SettingsDialog from '@/components/settings-dialog';
import useRssItems from '@/hooks/rss-items/use-rss-items';
import { Skeleton } from '@/components/ui/skeleton';
import { RssItem } from '@/lib/types';
import useStore from '@/lib/store/store';
import { useMemo } from 'react';
import { getGroupedRssItemsBySource } from '@/lib/utils';
import {
  Collapsible,
  CollapsibleTrigger,
  CollapsibleContent,
} from '@/components/ui/collapsible';

import BBCIcon from '@/assets/sites_icons/bbc.webp';
import NYCTimesIcon from '@/assets/sites_icons/new_york_times.webp';
import WyborczaIcon from '@/assets/sites_icons/wyborcza.webp';
import RssFeedGroup from '../rss-feed-group';

const Sidebar = () => {
  const {
    data: rssItems,
    isLoading: isRssItemsLoading,
    isError: isRssItemsError,
    error: rssItemsError,
  } = useRssItems();

  const setSelectedText = useStore((state) => state.setSelectedText);
  const setSelectedRssItem = useStore((state) => state.setSelectedRssItem);

  const groupedRssItems = useMemo(
    () => getGroupedRssItemsBySource(rssItems || []),
    [rssItems]
  );

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
          <RssFeedGroup
            groupedRssItems={groupedRssItems}
            handleItemClick={handleItemClick}
            iconUrl={BBCIcon}
            groupName="BBC World News"
          />
          <RssFeedGroup
            groupedRssItems={groupedRssItems}
            handleItemClick={handleItemClick}
            iconUrl={NYCTimesIcon}
            groupName="New York Times World News"
          />
          <RssFeedGroup
            groupedRssItems={groupedRssItems}
            handleItemClick={handleItemClick}
            iconUrl={WyborczaIcon}
            groupName="Gazeta Wyborcza"
          />
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
