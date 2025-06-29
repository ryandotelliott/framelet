import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Button } from '@/components/ui/button';
import { CropIcon, MonitorIcon, AppWindowMacIcon } from 'lucide-react';
import { CaptureSource } from '@/types/recording';

interface SourceSettingsProps {
  captureSources: CaptureSource[];
  selectedSource: number;
  onSourceChange: (source: number) => void;
  monitorCaptureMode: 'full' | 'custom';
  onMonitorCaptureModeChange: (mode: 'full' | 'custom') => void;
}

export function SourceSettings({
  captureSources,
  selectedSource,
  onSourceChange,
  monitorCaptureMode,
  onMonitorCaptureModeChange,
}: SourceSettingsProps) {
  const monitorSources = captureSources.filter((source) => source.source_type === 'monitor');
  const windowSources = captureSources.filter((source) => source.source_type === 'window');

  return (
    <Tabs defaultValue="monitor" className="w-full gap-y-4">
      <TabsList className="w-full">
        <TabsTrigger value="monitor">
          <MonitorIcon /> Monitor
        </TabsTrigger>
        <TabsTrigger value="window">
          <AppWindowMacIcon /> Window
        </TabsTrigger>
      </TabsList>
      <TabsContent value="monitor" className="flex flex-col gap-y-4">
        <Select onValueChange={(value) => onSourceChange(Number(value))}>
          <SelectTrigger className="w-full">
            <SelectValue placeholder="Select a monitor" />
          </SelectTrigger>
          <SelectContent>
            {monitorSources.map((source) => (
              <SelectItem key={source.handle} value={source.handle.toString()}>
                {source.name} ({source.width}x{source.height})
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <div className="flex gap-x-2">
          <Button
            variant={monitorCaptureMode === 'full' ? 'default' : 'outline'}
            className="grow border"
            onClick={() => onMonitorCaptureModeChange('full')}
          >
            <MonitorIcon /> Full Monitor
          </Button>
          <Button
            variant={monitorCaptureMode === 'custom' ? 'default' : 'outline'}
            className="grow border"
            onClick={() => onMonitorCaptureModeChange('custom')}
          >
            <CropIcon /> Custom Region
          </Button>
        </div>
      </TabsContent>
      <TabsContent value="window">
        <Select onValueChange={(value) => onSourceChange(Number(value))}>
          <SelectTrigger className="w-full">
            <SelectValue placeholder="Select a window" />
          </SelectTrigger>
          <SelectContent>
            {windowSources.map((source) => (
              <SelectItem key={source.handle} value={source.handle.toString()}>
                {source.name} ({source.width}x{source.height})
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </TabsContent>
    </Tabs>
  );
}
