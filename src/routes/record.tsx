import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Button } from '@/components/ui/button';
import { CameraIcon, FolderOpen, Monitor, RefreshCw, VideoIcon } from 'lucide-react';
import { save } from '@tauri-apps/plugin-dialog';

interface CaptureSource {
  handle: number;
  name: string;
  width: number;
  height: number;
  source_type: string;
}

export default function RecordPage() {
  const [captureSources, setCaptureSources] = useState<CaptureSource[]>([]);
  const [selectedSource, setSelectedSource] = useState<number>(0);
  const [outputPath, setOutputPath] = useState('recording.mp4');
  const [isRecording, setIsRecording] = useState(false);
  const [status, setStatus] = useState('');
  const [isLoadingSources, setIsLoadingSources] = useState(false);

  useEffect(() => {
    loadCaptureSources();
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

  const selectedSourceInfo = captureSources.find((source) => source.handle === selectedSource);

  return (
    <div className="flex flex-col h-screen gap-y-4 pt-10 px-6 pb-6">
      <div className="flex gap-x-4 h-full w-full items-center justify-center">
        <div className="w-full h-full flex flex-col gap-y-4 items-center justify-center">
          <div className="aspect-video rounded-lg border-2 border-dashed border-white/20 flex items-center justify-center relative overflow-hidden w-full max-w-2xl">
            <div className="text-center">
              <div className="w-16 h-16 bg-white/10 rounded-lg mb-4 mx-auto flex items-center justify-center">
                <Monitor className="w-8 h-8 text-white/40" />
              </div>
              <p className="text-white/60 text-lg mb-2">Screen Preview</p>
              <p className="text-white/40 text-sm">Your screen content will appear here</p>
              <p className="text-white/40 text-xs mt-1">Select a source and start recording</p>
            </div>
          </div>
          <Button variant="default" size="lg">
            <VideoIcon />
            Start Recording
          </Button>
        </div>
        <div className="h-full w-full max-w-xs rounded-lg bg-card"></div>
      </div>
    </div>
  );
}
