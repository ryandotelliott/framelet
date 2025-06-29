import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Button } from '@/components/ui/button';

interface OutputSettingsProps {
  fileName: string;
  onFileNameChange: (fileName: string) => void;
  outputPath: string;
  onOutputPathChange: (outputPath: string) => void;
}

export function OutputSettings({ fileName, onFileNameChange, outputPath, onOutputPathChange }: OutputSettingsProps) {
  return (
    <div className="flex flex-col gap-y-4">
      <div className="flex flex-col gap-y-2">
        <Label htmlFor="fileName">File Name</Label>
        <Input
          id="fileName"
          placeholder="recording"
          value={fileName}
          onChange={(e) => onFileNameChange(e.target.value)}
        />
      </div>
      <div className="flex flex-col gap-y-2">
        <Label htmlFor="outputPath">Output Path</Label>
        <div className="flex gap-x-2">
          <Input
            id="outputPath"
            placeholder="~/Desktop"
            value={outputPath}
            onChange={(e) => onOutputPathChange(e.target.value)}
          />
          <Button variant="outline">Browse</Button>
        </div>
      </div>
    </div>
  );
}
