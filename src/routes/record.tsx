import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { Select, SelectContent, SelectItem, SelectTrigger } from '@/components/ui/select';
import { Button } from '@/components/ui/button';
import { CropIcon, Monitor, VideoIcon, VideoOff } from 'lucide-react';

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

export default function RecordPage() {
  const [captureSources, setCaptureSources] = useState<CaptureSource[]>([]);
  const [selectedSource, setSelectedSource] = useState<number>(0);
  const [outputPath, setOutputPath] = useState('recording.mp4');
  const [isRecording, setIsRecording] = useState(false);
  const [status, setStatus] = useState('');
  const [isLoadingSources, setIsLoadingSources] = useState(false);
  const [selectedRegion, setSelectedRegion] = useState<RegionCoordinates | null>(null);

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

  return (
    <div className="flex flex-col h-screen gap-y-4 pt-10 px-6 pb-6">
      <div className="flex gap-x-4 h-full w-full items-center justify-center">
        <div className="w-full h-full flex flex-col gap-y-4 items-center justify-center">
          <div className="text-center">
            <div className="w-16 h-16 bg-white/10 rounded-lg mb-4 mx-auto flex items-center justify-center">
              <Monitor className="w-8 h-8 text-white opacity-40" />
            </div>
            <p className="text-white/60 text-lg mb-2">Ready to Record</p>
            <p className="text-white/40 text-xs mt-1">Select a source and start recording</p>
            {selectedRegion && (
              <p className="text-green-400 text-xs mt-2">
                Selected region: {selectedRegion.width}x{selectedRegion.height} at ({selectedRegion.x},{' '}
                {selectedRegion.y})
              </p>
            )}
          </div>

          <div className="flex gap-x-4">
            <Button
              variant="outline"
              disabled={isLoadingSources || !selectedSourceInfo}
              onClick={isRecording ? stopRecording : startRecording}
            >
              {isRecording ? <VideoOff /> : <VideoIcon />}
              {isRecording ? 'Stop Recording' : 'Start Recording'}
            </Button>
            <Select
              onValueChange={(value) => setSelectedSource(Number(value))}
              value={selectedSource.toString()}
              disabled={isLoadingSources || isRecording}
            >
              <SelectTrigger>
                <Monitor />
                {`${selectedSourceInfo?.name.slice(0, 20)} (${selectedSourceInfo?.width}x${selectedSourceInfo?.height})` ||
                  'Select Source'}
              </SelectTrigger>
              <SelectContent>
                {captureSources.map((source) => (
                  <SelectItem key={source.handle} value={source.handle.toString()}>
                    {source.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <Button variant="outline" onClick={openRegionSelector} disabled={isRecording}>
              <CropIcon />
              Select Region
            </Button>
          </div>

          {status && (
            <div className="mt-4 p-2 bg-white/10 rounded text-white/80 text-sm max-w-md text-center">{status}</div>
          )}
        </div>
        {/* Maybe sidebar here? */}
      </div>
    </div>
  );
}
