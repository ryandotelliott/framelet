import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Button } from '@/components/ui/button';

interface MonitorInfo {
  id: number;
  name: string;
  width: number;
  height: number;
}

function Home() {
  const [monitors, setMonitors] = useState<MonitorInfo[]>([]);
  const [selectedMonitor, setSelectedMonitor] = useState<number>(0);
  const [outputPath, setOutputPath] = useState('recording.mp4');
  const [isRecording, setIsRecording] = useState(false);
  const [status, setStatus] = useState('');

  useEffect(() => {
    loadMonitors();
  }, []);

  async function loadMonitors() {
    try {
      const monitorList = await invoke<MonitorInfo[]>('get_monitors');
      setMonitors(monitorList);
    } catch (error) {
      setStatus(`Error loading monitors: ${error}`);
    }
  }

  async function startRecording() {
    try {
      setStatus('Starting recording...');
      const result = await invoke<string>('start_recording', {
        monitorId: selectedMonitor,
        outputPath: outputPath,
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

  return (
    <div className="flex flex-col h-screen gap-y-4">
      <h1 className="text-2xl font-bold">Screen Recorder</h1>

      <div className="flex flex-col gap-4">
        <div className="flex flex-col gap-2">
          <Label htmlFor="monitor-select">Select Monitor:</Label>
          <Select
            name="monitor-select"
            value={selectedMonitor.toString()}
            onValueChange={(value) => setSelectedMonitor(parseInt(value))}
            disabled={isRecording}
          >
            <SelectTrigger>
              <SelectValue placeholder="Select Monitor" />
            </SelectTrigger>
            <SelectContent>
              {monitors.map((monitor) => (
                <SelectItem key={monitor.id} value={monitor.id.toString()}>
                  {monitor.name} ({monitor.width}x{monitor.height})
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <div className="flex flex-col gap-2">
          <Label htmlFor="output-path">Output File:</Label>
          <Input
            name="output-path"
            type="text"
            value={outputPath}
            onChange={(event) => setOutputPath(event.target.value)}
            disabled={isRecording}
            placeholder="recording.mp4"
          />
        </div>

        <div className="flex gap-2">
          {!isRecording ? (
            <Button onClick={startRecording} disabled={monitors.length === 0} className="start-button">
              Start Recording
            </Button>
          ) : (
            <Button onClick={stopRecording} className="stop-button">
              Stop Recording
            </Button>
          )}
        </div>

        {status && <div className={`status ${isRecording ? 'recording' : ''}`}>{status}</div>}
      </div>
    </div>
  );
}

export default Home;
