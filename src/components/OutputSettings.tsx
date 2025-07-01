import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Button } from '@/components/ui/button';
import { save } from '@tauri-apps/plugin-dialog';
import { videoDir } from '@tauri-apps/api/path';

interface OutputSettingsProps {
  outputPath: string;
  onOutputPathChange: (outputPath: string) => void;
}

export function OutputSettings({ outputPath, onOutputPathChange }: OutputSettingsProps) {
  const handleBrowser = async () => {
    const path = await save({
      title: 'Select Output Path',
      defaultPath: await videoDir(),
      filters: [{ name: 'MP4', extensions: ['mp4'] }],
    });

    if (path) {
      onOutputPathChange(path);
    }
  };

  return (
    <div className="flex flex-col gap-y-4">
      <div className="flex flex-col gap-y-2">
        <Label htmlFor="outputPath">Output Path</Label>
        <div className="flex gap-x-2">
          <Input
            id="outputPath"
            placeholder="/videos/recording.mp4"
            value={outputPath}
            onChange={(e) => onOutputPathChange(e.target.value)}
            onClick={handleBrowser}
          />
          <Button variant="outline" onClick={handleBrowser}>
            Browse
          </Button>
        </div>
      </div>
    </div>
  );
}
