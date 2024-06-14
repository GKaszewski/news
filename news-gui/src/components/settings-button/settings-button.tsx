import { Button } from '@/components/ui/button';
import { Settings } from 'lucide-react';
import { Label } from '@/components/ui/label';

const SettingsButton = () => {
  return (
    <div className="inline-flex gap-2 items-center">
      <Button variant="outline" size="icon">
        <Settings className="w-4 h-4" />
      </Button>
      <Label>Settings</Label>
    </div>
  );
};

export default SettingsButton;
