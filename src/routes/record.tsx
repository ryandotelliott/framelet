import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { SourceSettings } from '@/components/SourceSettings';
import { OutputSettings } from '@/components/OutputSettings';
import { AudioSettings } from '@/components/AudioSettings';
import { WebcamSettings } from '@/components/WebcamSettings';
import { CaptureSource, Region } from '@/types/recording';

export default function RecordPage() {
  const [captureSources, setCaptureSources] = useState<CaptureSource[]>([]);
  const [selectedSource, setSelectedSource] = useState<number>(0);
  const [outputPath, setOutputPath] = useState('');
  const [isRecording, setIsRecording] = useState(false);
  const [isLoadingSources, setIsLoadingSources] = useState(false);
  const [selectedRegion, setSelectedRegion] = useState<Region | null>(null);
  const [isRegionSelectorOpen, setIsRegionSelectorOpen] = useState(false);

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

  const [monitorCaptureMode, setMonitorCaptureMode] = useState<'full' | 'custom'>('full');

  useEffect(() => {
    loadCaptureSources();

    // Listen for region selection events
    const unlistenRegionSelected = listen<Region>('region-selected', (event) => {
      console.log('Region selected:', event.payload);
      setSelectedRegion(event.payload);
      setIsRegionSelectorOpen(false);
    });

    // Listen for region selector cancellation
    const unlistenRegionCancelled = listen('region-selector-closed', () => {
      console.log('Region selector closed without selection');
      setIsRegionSelectorOpen(false);
      // Reset to full monitor mode if we were in custom mode and the selector was open
      if (monitorCaptureMode === 'custom' && isRegionSelectorOpen) {
        setMonitorCaptureMode('full');
      }
    });

    return () => {
      unlistenRegionSelected.then((unlisten) => unlisten());
      unlistenRegionCancelled.then((unlisten) => unlisten());
    };
  }, [monitorCaptureMode, isRegionSelectorOpen]);

  async function loadCaptureSources() {
    setIsLoadingSources(true);
    try {
      const sources = await invoke<CaptureSource[]>('get_capture_sources');
      setCaptureSources(sources);
      if (sources.length > 0) {
        setSelectedSource(sources[0].handle);
      }
    } catch (error) {
      console.error('Error loading capture sources:', error);
    } finally {
      setIsLoadingSources(false);
    }
  }

  async function startRecording() {
    const source = captureSources.find((s) => s.handle === selectedSource);
    if (!source) {
      console.error('Error: no capture source selected');
      return;
    }

    try {
      const result = await invoke<string>('start_recording', {
        handle: source.handle,
        sourceType: source.source_type,
        outputPath: outputPath,
        region: monitorCaptureMode === 'custom' ? selectedRegion : null,
      });
      console.log('Recording started:', result);
      setIsRecording(true);
    } catch (error) {
      console.error('Error starting recording:', error);
    }
  }

  async function stopRecording() {
    try {
      await invoke<string>('stop_recording');
      setIsRecording(false);
    } catch (error) {
      console.error('Error stopping recording:', error);
    }
  }

  async function openRegionSelector() {
    try {
      setIsRegionSelectorOpen(true);
      await invoke('open_region_selector', {
        monitorHandle: selectedSource,
      });
    } catch (error) {
      setIsRegionSelectorOpen(false);
      console.error('Error opening region selector:', error);
    }
  }

  const handleMonitorCaptureModeChange = (mode: 'full' | 'custom') => {
    setMonitorCaptureMode(mode);
    if (mode === 'full') {
      setSelectedRegion(null);
    }
  };

  const handleSourceChange = (source: number) => {
    setSelectedSource(source);

    if (monitorCaptureMode === 'custom') {
      setSelectedRegion(null);
    }
  };

  return (
    <div className="flex h-screen flex-col items-center justify-center gap-y-4 px-6 pt-10 pb-6">
      <div className="grid w-full max-w-[850px] grid-cols-3 gap-x-3">
        <Card className="col-span-2 flex w-full gap-y-4">
          <CardHeader>
            <CardTitle>Source Settings</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-y-4">
            <SourceSettings
              captureSources={captureSources}
              selectedSource={selectedSource}
              onSourceChange={handleSourceChange}
              monitorCaptureMode={monitorCaptureMode}
              onMonitorCaptureModeChange={handleMonitorCaptureModeChange}
              selectedRegion={selectedRegion}
              onOpenRegionSelector={openRegionSelector}
            />
          </CardContent>

          <CardContent className="flex flex-col gap-y-4">
            <Separator />
            <CardTitle>Output Settings</CardTitle>
            <OutputSettings outputPath={outputPath} onOutputPathChange={setOutputPath} />
          </CardContent>

          <CardContent className="mt-auto flex flex-col gap-y-4">
            <Separator />
            <Button onClick={isRecording ? stopRecording : startRecording}>
              {isRecording ? 'Stop Recording' : 'Start Recording'}
            </Button>
          </CardContent>
        </Card>

        <div className="col-span-1 flex w-full flex-col gap-y-4">
          <AudioSettings
            recordAudio={recordAudio}
            onRecordAudioChange={setRecordAudio}
            audioSource={audioSource}
            onAudioSourceChange={setAudioSource}
            microphoneEnabled={microphoneEnabled}
            onMicrophoneEnabledChange={setMicrophoneEnabled}
            inputSource={inputSource}
            onInputSourceChange={setInputSource}
            audioSourcesList={audioSourcesList}
            microphoneSourcesList={microphoneSourcesList}
          />

          <WebcamSettings
            enableWebcam={enableWebcam}
            onEnableWebcamChange={setEnableWebcam}
            webcamSource={webcamSource}
            onWebcamSourceChange={setWebcamSource}
            webcamSourcesList={webcamSourcesList}
          />
        </div>
      </div>
    </div>
  );
}
