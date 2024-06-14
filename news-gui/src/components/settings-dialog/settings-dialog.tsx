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

const SettingsDialog = () => {
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
          <Label className="font-bold text-lg">Settings</Label>
        </DialogHeader>
        <div className="grid grid-cols-2 gap-2">
          <div className="flex flex-col gap-2">
            <Collapsible>
              <div className="inline-flex gap-2 items-center">
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
              <CollapsibleTrigger className="inline-flex gap-2 items-center font-semibold">
                <Newspaper className="w-4 h-4" /> RSS Feeds
              </CollapsibleTrigger>
              <CollapsibleContent>
                <p>RSS Feeds content here</p>
              </CollapsibleContent>
            </Collapsible>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
};

export default SettingsDialog;
