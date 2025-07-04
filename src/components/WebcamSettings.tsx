import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { cn } from '@/lib/utils';

interface WebcamSettingsProps {
  enableWebcam: boolean;
  onEnableWebcamChange: (enabled: boolean) => void;
  webcamSource: string;
  onWebcamSourceChange: (source: string) => void;
  webcamSourcesList: string[];
}

// Utility for label disabled style
const labelDisabled = 'opacity-50';

export function WebcamSettings({
  enableWebcam,
  onEnableWebcamChange,
  webcamSource,
  onWebcamSourceChange,
  webcamSourcesList,
}: WebcamSettingsProps) {
  return (
    <Card className="col-span-1 flex w-full gap-y-4">
      <CardHeader>
        <CardTitle>Webcam Settings</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-y-4">
        <div className="flex items-center justify-between">
          <Label>Enable Webcam</Label>
          <Switch checked={enableWebcam} onCheckedChange={onEnableWebcamChange} />
        </div>

        <div className="flex flex-col gap-y-2">
          <Label htmlFor="webcamSource" className={cn({ [labelDisabled]: !enableWebcam })}>
            Webcam Source
          </Label>
          <Select value={webcamSource} onValueChange={onWebcamSourceChange} disabled={!enableWebcam}>
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select webcam source" />
            </SelectTrigger>
            <SelectContent>
              {webcamSourcesList.map((src) => (
                <SelectItem key={src} value={src}>
                  {src}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      </CardContent>
    </Card>
  );
}
