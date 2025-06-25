import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { CropIcon, MonitorIcon, AppWindowMacIcon } from 'lucide-react';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';

interface CaptureSource {
  handle: number;
  name: string;
  width: number;
  height: number;
  source_type: string;
}

interface RegionCoordinates {
  x: number;
  y: number;
  width: number;
  height: number;
}

// Utility for label disabled style
const labelDisabled = 'opacity-50';

export default function RecordPage() {
  const [captureSources, setCaptureSources] = useState<CaptureSource[]>([]);
  const [selectedSource, setSelectedSource] = useState<number>(0);
  const [outputPath, setOutputPath] = useState('recording.mp4');
  const [isRecording, setIsRecording] = useState(false);
  const [status, setStatus] = useState('');
  const [isLoadingSources, setIsLoadingSources] = useState(false);
  const [selectedRegion, setSelectedRegion] = useState<RegionCoordinates | null>(null);

  /* ---------------------------------- Audio ---------------------------------- */
  const [recordAudio, setRecordAudio] = useState(false);
  const [audioSource, setAudioSource] = useState('');
  const audioSourcesList = ['All System Audio'];

  const [microphoneEnabled, setMicrophoneEnabled] = useState(false);
  const [inputSource, setInputSource] = useState('');
  const microphoneSourcesList = ['Default - Microphone'];

  /* --------------------------------- Webcam ---------------------------------- */
  const [enableWebcam, setEnableWebcam] = useState(false);
  const [webcamSource, setWebcamSource] = useState('');
  const webcamSourcesList = ['Logitech 4K Webcam'];

  useEffect(() => {
    loadCaptureSources();

    // Listen for region selection events
    const unlistenRegionSelected = listen<RegionCoordinates>('region-selected', (event) => {
      console.log('Region selected:', event.payload);
      setSelectedRegion(event.payload);
      setStatus(
        `Region selected: ${event.payload.width}x${event.payload.height} at (${event.payload.x}, ${event.payload.y})`,
      );
    });

    return () => {
      unlistenRegionSelected.then((unlisten) => unlisten());
    };
  }, []);

  async function loadCaptureSources() {
    setIsLoadingSources(true);
    try {
      const sources = await invoke<CaptureSource[]>('get_capture_sources');
      setCaptureSources(sources);
      if (sources.length > 0) {
        setSelectedSource(sources[0].handle);
      }
      setStatus('');
    } catch (error) {
      setStatus(`Error loading capture sources: ${error}`);
    } finally {
      setIsLoadingSources(false);
    }
  }

  async function startRecording() {
    const source = captureSources.find((s) => s.handle === selectedSource);
    if (!source) {
      setStatus('Error: no capture source selected');
      return;
    }
    try {
      setStatus('Starting recording...');
      const result = await invoke<string>('start_recording', {
        handle: source.handle,
        sourceType: source.source_type,
        outputPath,
      });
      setStatus(result);
      setIsRecording(true);
    } catch (error) {
      setStatus(`Error starting recording: ${error}`);
    }
  }

  async function stopRecording() {
    try {
      setStatus('Stopping recording...');
      const result = await invoke<string>('stop_recording');
      setStatus(result);
      setIsRecording(false);
    } catch (error) {
      setStatus(`Error stopping recording: ${error}`);
    }
  }

  async function openRegionSelector() {
    try {
      await invoke('open_region_selector', {
        monitorHandle: selectedSource,
      });
      setStatus('Region selector opened - click and drag to select an area');
    } catch (error) {
      setStatus(`Error opening region selector: ${error}`);
    }
  }

  const selectedSourceInfo = captureSources.find((source) => source.handle === selectedSource);

  const monitorSources = captureSources.filter((source) => source.source_type === 'monitor');
  const windowSources = captureSources.filter((source) => source.source_type === 'window');

  const [monitorCaptureMode, setMonitorCaptureMode] = useState<'full' | 'custom'>('full');

  return (
    <div className="flex h-screen flex-col items-center justify-center gap-y-4 px-6 pt-10 pb-6">
      <div className="grid w-full max-w-[850px] grid-cols-3 gap-x-3">
        <Card className="col-span-2 flex w-full gap-y-4">
          <CardHeader>
            <CardTitle>Source Settings</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-y-4">
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
                <Select onValueChange={(value) => setSelectedSource(Number(value))}>
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
                  {/* Note that both buttons should have a button in order to keep the width the same*/}
                  <Button
                    variant={monitorCaptureMode === 'full' ? 'default' : 'outline'}
                    className="grow border"
                    onClick={() => setMonitorCaptureMode('full')}
                  >
                    <MonitorIcon /> Full Monitor
                  </Button>
                  <Button
                    variant={monitorCaptureMode === 'custom' ? 'default' : 'outline'}
                    className="grow border"
                    onClick={() => setMonitorCaptureMode('custom')}
                  >
                    <CropIcon /> Custom Region
                  </Button>
                </div>
              </TabsContent>
              <TabsContent value="window">
                <Select onValueChange={(value) => setSelectedSource(Number(value))}>
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
          </CardContent>

          <CardContent className="flex flex-col gap-y-4">
            <Separator />
            <CardTitle>Output Settings</CardTitle>
            <div className="flex flex-col gap-y-2">
              <Label htmlFor="fileName">File Name</Label>
              <Input id="fileName" placeholder="recording" />
            </div>
            <div className="flex flex-col gap-y-2">
              <Label htmlFor="outputPath">Output Path</Label>
              <div className="flex gap-x-2">
                <Input id="outputPath" placeholder="~/Desktop" />
                <Button variant="outline">Browse</Button>
              </div>
            </div>
          </CardContent>

          <CardContent className="flex flex-col gap-y-4">
            <Separator />
            <Button>Start Recording</Button>
          </CardContent>
        </Card>
        <div className="col-span-1 flex w-full flex-col gap-y-4">
          {/* ----------------------------- Audio Settings ----------------------------- */}
          <Card className="col-span-1 flex w-full gap-y-4">
            <CardHeader>
              <CardTitle>Audio Settings</CardTitle>
            </CardHeader>
            <CardContent className="flex flex-col gap-y-4">
              {/* Record Audio toggle */}
              <div className="flex items-center justify-between">
                <Label>Record Audio</Label>
                <Switch checked={recordAudio} onCheckedChange={(v) => setRecordAudio(v)} />
              </div>

              <div className="flex flex-col gap-y-2">
                <Label htmlFor="audioSource" className={!recordAudio ? labelDisabled : ''}>
                  Audio Source
                </Label>
                <Select value={audioSource} onValueChange={setAudioSource} disabled={!recordAudio}>
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
                <Label>Microphone</Label>
                <Switch checked={microphoneEnabled} onCheckedChange={(v) => setMicrophoneEnabled(v)} />
              </div>

              <div className="flex flex-col gap-y-2">
                <Label htmlFor="inputSource" className={!microphoneEnabled ? labelDisabled : ''}>
                  Input Source
                </Label>
                <Select value={inputSource} onValueChange={setInputSource} disabled={!microphoneEnabled}>
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

          {/* ---------------------------- Webcam Settings ---------------------------- */}
          <Card className="col-span-1 flex w-full gap-y-4">
            <CardHeader>
              <CardTitle>Webcam Settings</CardTitle>
            </CardHeader>
            <CardContent className="flex flex-col gap-y-4">
              <div className="flex items-center justify-between">
                <Label>Enable Webcam</Label>
                <Switch checked={enableWebcam} onCheckedChange={(v) => setEnableWebcam(v)} />
              </div>

              <div className="flex flex-col gap-y-2">
                <Label htmlFor="webcamSource" className={!enableWebcam ? labelDisabled : ''}>
                  Webcam Source
                </Label>
                <Select value={webcamSource} onValueChange={setWebcamSource} disabled={!enableWebcam}>
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
        </div>
      </div>
    </div>
  );
}
