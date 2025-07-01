import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Button } from '@/components/ui/button';
import { CropIcon, MonitorIcon, AppWindowMacIcon } from 'lucide-react';
import { CaptureSource, Region } from '@/types/recording';
import { useEffect, useState } from 'react';

interface SourceSettingsProps {
  captureSources: CaptureSource[];
  selectedSource: number;
  onSourceChange: (source: number) => void;
  monitorCaptureMode: 'full' | 'custom';
  onMonitorCaptureModeChange: (mode: 'full' | 'custom') => void;
  selectedRegion: Region | null;
  onOpenRegionSelector?: () => void;
}

type SourceTab = 'monitor' | 'window';

export function SourceSettings({
  captureSources,
  selectedSource,
  onSourceChange,
  monitorCaptureMode,
  onMonitorCaptureModeChange,
  selectedRegion,
  onOpenRegionSelector,
}: SourceSettingsProps) {
  const monitorSources = captureSources.filter((source) => source.source_type === 'monitor');
  const windowSources = captureSources.filter((source) => source.source_type === 'window');
  const [selectedTab, setSelectedTab] = useState<SourceTab>('monitor');

  // Auto-select first available source when switching tabs
  useEffect(() => {
    const currentSources = selectedTab === 'monitor' ? monitorSources : windowSources;
    if (currentSources.length > 0) {
      const firstSource = currentSources[0];
      // Only change if the current selection is not in the current tab's sources
      const isCurrentSourceValid = currentSources.some((source) => source.handle === selectedSource);
      if (!isCurrentSourceValid) {
        onSourceChange(firstSource.handle);
      }
    }
  }, [selectedTab, monitorSources, windowSources, selectedSource, onSourceChange]);

  const handleTabChange = (value: string) => {
    const newTab = value as SourceTab;
    setSelectedTab(newTab);

    const newSources = newTab === 'monitor' ? monitorSources : windowSources;
    if (newSources.length > 0) {
      onSourceChange(newSources[0].handle);
    }
  };

  const currentSources = selectedTab === 'monitor' ? monitorSources : windowSources;
  const hasSources = currentSources.length > 0;
  const emptyMessage = selectedTab === 'monitor' ? 'No Monitors Found' : 'No Windows Found';

  return (
    <Tabs className="w-full gap-y-4" onValueChange={handleTabChange} value={selectedTab}>
      <TabsList className="w-full">
        <TabsTrigger value="monitor">
          <MonitorIcon /> Monitor
        </TabsTrigger>
        <TabsTrigger value="window">
          <AppWindowMacIcon /> Window
        </TabsTrigger>
      </TabsList>
      <TabsContent value="monitor" className="flex flex-col gap-y-4">
        <Select
          onValueChange={(value) => onSourceChange(Number(value))}
          value={hasSources ? selectedSource.toString() : undefined}
          disabled={!hasSources}
        >
          <SelectTrigger className="w-full">
            <SelectValue placeholder={hasSources ? 'Select a monitor' : emptyMessage} />
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
            disabled={!hasSources}
          >
            <MonitorIcon /> Full Monitor
          </Button>
          <Button
            variant={monitorCaptureMode === 'custom' ? 'default' : 'outline'}
            className="grow border"
            onClick={() => {
              onMonitorCaptureModeChange('custom');
              onOpenRegionSelector?.();
            }}
            disabled={!hasSources}
          >
            <CropIcon /> Custom Region
          </Button>
        </div>
        {monitorCaptureMode === 'custom' && selectedRegion && (
          <div className="text-muted-foreground text-sm">
            Selected Region: {selectedRegion.width} &#x00d7; {selectedRegion.height}
          </div>
        )}
      </TabsContent>
      <TabsContent value="window">
        <Select
          onValueChange={(value) => onSourceChange(Number(value))}
          value={hasSources ? selectedSource.toString() : undefined}
          disabled={!hasSources}
        >
          <SelectTrigger className="w-full">
            <SelectValue placeholder={hasSources ? 'Select a window' : emptyMessage} />
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
