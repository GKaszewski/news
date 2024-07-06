import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTrigger,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Newspaper, Plus, Settings } from 'lucide-react';
import { Label } from '@/components/ui/label';
import {
  Collapsible,
  CollapsibleTrigger,
  CollapsibleContent,
} from '../ui/collapsible';
import useFeeds from '@/hooks/feeds/use-feeds';
import { Skeleton } from '../ui/skeleton';

const SettingsDialog = () => {
  const {
    data: feeds,
    isLoading: fetchingFeeds,
    isError: fetchignFeedsError,
  } = useFeeds();

  return (
    <Dialog>
      <div className="inline-flex items-center gap-2">
        <DialogTrigger asChild>
          <Button variant="outline" size="icon">
            <Settings className="w-4 h-4" />
          </Button>
        </DialogTrigger>
        <Label>Settings</Label>
      </div>
      <DialogContent className="sm:max-w-[425px] w-[720px] min-h-[400px] flex flex-col">
        <DialogHeader>
          <Label className="text-lg font-bold">Settings</Label>
        </DialogHeader>
        <div className="grid grid-cols-2 gap-2">
          <div className="flex flex-col gap-2">
            <Collapsible>
              <div className="inline-flex items-center gap-2">
                <CollapsibleTrigger>
                  <Button variant="outline" size="icon">
                    <Plus className="w-4 h-4" />
                  </Button>
                </CollapsibleTrigger>
                <Label>Add RSS feed</Label>
              </div>
              <CollapsibleContent>
                <p>Add RSS feed content here</p>
              </CollapsibleContent>
            </Collapsible>
          </div>
          <div className="flex flex-col gap-2">
            <Collapsible>
              <CollapsibleTrigger className="inline-flex items-center gap-2 font-semibold">
                <Newspaper className="w-4 h-4" /> RSS Feeds
              </CollapsibleTrigger>
              <CollapsibleContent className="flex flex-col w-full gap-1">
                {fetchingFeeds && <Skeleton className="w-full h-10" />}
                {fetchignFeedsError && <p>Error fetching feeds</p>}
                {feeds && feeds.length === 0 && <p>No feeds available</p>}
                {feeds &&
                  feeds.map((feed, index) => (
                    <a
                      href={feed.url}
                      target="_blank"
                      className="underline truncate"
                      key={`feeed-${index}`}
                    >
                      {feed.name}
                    </a>
                  ))}
              </CollapsibleContent>
            </Collapsible>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
};

export default SettingsDialog;
