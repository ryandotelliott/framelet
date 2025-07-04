import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';

interface AudioSettingsProps {
  recordAudio: boolean;
  onRecordAudioChange: (enabled: boolean) => void;
  audioSource: string;
  onAudioSourceChange: (source: string) => void;
  microphoneEnabled: boolean;
  onMicrophoneEnabledChange: (enabled: boolean) => void;
  inputSource: string;
  onInputSourceChange: (source: string) => void;
  audioSourcesList: string[];
  microphoneSourcesList: string[];
}

// Utility for label disabled style
const labelDisabled = 'opacity-50';

export function AudioSettings({
  recordAudio,
  onRecordAudioChange,
  audioSource,
  onAudioSourceChange,
  microphoneEnabled,
  onMicrophoneEnabledChange,
  inputSource,
  onInputSourceChange,
  audioSourcesList,
  microphoneSourcesList,
}: AudioSettingsProps) {
  return (
    <Card className="col-span-1 flex w-full gap-y-4">
      <CardHeader>
        <CardTitle>Audio Settings</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-y-4">
        {/* Record Audio toggle */}
        <div className="flex items-center justify-between">
          <Label htmlFor="recordAudio">Record Audio</Label>
          <Switch checked={recordAudio} onCheckedChange={onRecordAudioChange} />
        </div>

        <div className="flex flex-col gap-y-2">
          <Label htmlFor="audioSource" className={cn({ [labelDisabled]: !recordAudio })}>
            Audio Source
          </Label>
          <Select value={audioSource} onValueChange={onAudioSourceChange} disabled={!recordAudio}>
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select audio source" />
            </SelectTrigger>
            <SelectContent>
              {audioSourcesList.map((src) => (
                <SelectItem key={src} value={src}>
                  {src}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <Separator />

        {/* Microphone toggle */}
        <div className="flex items-center justify-between">
          <Label htmlFor="microphone">Microphone</Label>
          <Switch checked={microphoneEnabled} onCheckedChange={onMicrophoneEnabledChange} />
        </div>

        <div className="flex flex-col gap-y-2">
          <Label htmlFor="inputSource" className={cn({ [labelDisabled]: !microphoneEnabled })}>
            Input Source
          </Label>
          <Select value={inputSource} onValueChange={onInputSourceChange} disabled={!microphoneEnabled}>
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select input source" />
            </SelectTrigger>
            <SelectContent>
              {microphoneSourcesList.map((src) => (
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
