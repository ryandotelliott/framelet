import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Button } from '@/components/ui/button';
import { FolderOpen, RefreshCw } from 'lucide-react';
import { save } from '@tauri-apps/plugin-dialog';

interface CaptureSource {
  handle: number;
  name: string;
  width: number;
  height: number;
  source_type: string;
}

function Home() {
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
    <div className="flex flex-col h-screen gap-y-4 p-6">
      <h1 className="text-3xl font-bold">Screen Recorder</h1>
      <p className="text-muted-foreground">Record your screen or capture specific application windows</p>

      <div className="flex flex-col gap-6">
        <div className="flex flex-col gap-3">
          <div className="flex items-center justify-between">
            <Label htmlFor="source-select">Select Capture Source:</Label>
            <Button
              variant="outline"
              size="sm"
              onClick={loadCaptureSources}
              disabled={isRecording || isLoadingSources}
              className="flex items-center gap-2"
            >
              <RefreshCw className={`h-4 w-4 ${isLoadingSources ? 'animate-spin' : ''}`} />
              Refresh
            </Button>
          </div>

          <Select
            name="source-select"
            value={selectedSource.toString()}
            onValueChange={(value) => setSelectedSource(parseInt(value))}
            disabled={isRecording || isLoadingSources}
          >
            <SelectTrigger>
              <SelectValue placeholder="Select a monitor or window to record" />
            </SelectTrigger>
            <SelectContent>
              {captureSources.map((source) => (
                <SelectItem key={source.handle} value={source.handle.toString()}>
                  <div className="flex items-center gap-2">
                    <span>{source.name}</span>
                    <span className="text-sm text-muted-foreground">
                      ({source.width}x{source.height})
                    </span>
                  </div>
                </SelectItem>
              ))}
            </SelectContent>
          </Select>

          {selectedSourceInfo && (
            <div className="p-3 bg-muted rounded-lg">
              <div className="text-sm">
                <strong>Selected:</strong> {selectedSourceInfo.name}
              </div>
              <div className="text-sm text-muted-foreground">
                Resolution: {selectedSourceInfo.width}x{selectedSourceInfo.height} | Type:{' '}
                {selectedSourceInfo.source_type === 'monitor' ? 'Monitor' : 'Application Window'}
              </div>
            </div>
          )}
        </div>

        <div className="flex flex-col gap-3">
          <Label htmlFor="output-path">Output File:</Label>
          <Input
            name="output-path"
            type="text"
            value={outputPath}
            onChange={(event) => setOutputPath(event.target.value)}
            disabled={isRecording}
            placeholder="recording.mp4"
            className="font-mono"
          />
          <Button
            variant="outline"
            size="sm"
            onClick={async () => {
              const result = await save({
                filters: [{ name: 'Video', extensions: ['mp4'] }],
              });
              if (result) {
                setOutputPath(result);
              }
            }}
          >
            <FolderOpen className="h-4 w-4" />
            Browse
          </Button>
          <p className="text-sm text-muted-foreground">
            Specify the path and filename for your recording. Supports .mp4, .avi, and other video formats.
          </p>
        </div>

        <div className="flex gap-3">
          {!isRecording ? (
            <Button
              onClick={startRecording}
              disabled={captureSources.length === 0 || isLoadingSources}
              className="bg-red-600 hover:bg-red-700 text-white"
              size="lg"
            >
              Start Recording
            </Button>
          ) : (
            <Button onClick={stopRecording} className="bg-gray-600 hover:bg-gray-700 text-white" size="lg">
              Stop Recording
            </Button>
          )}
        </div>

        {status && (
          <div
            className={`p-3 rounded-lg ${
              isRecording
                ? 'bg-red-50 text-red-800 border border-red-200'
                : status.includes('Error')
                  ? 'bg-red-50 text-red-800 border border-red-200'
                  : 'bg-green-50 text-green-800 border border-green-200'
            }`}
          >
            {status}
          </div>
        )}

        {captureSources.length === 0 && !isLoadingSources && (
          <div className="p-4 bg-yellow-50 text-yellow-800 border border-yellow-200 rounded-lg">
            <p className="font-medium">No capture sources available</p>
            <p className="text-sm">
              Make sure you have monitors connected or applications running, then click Refresh.
            </p>
          </div>
        )}

        {isLoadingSources && (
          <div className="p-4 bg-blue-50 text-blue-800 border border-blue-200 rounded-lg">
            <p>Loading available capture sources...</p>
          </div>
        )}
      </div>
    </div>
  );
}

export default Home;
