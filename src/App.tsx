import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface MonitorInfo {
    id: number;
    name: string;
    width: number;
    height: number;
}

function App() {
    const [monitors, setMonitors] = useState<MonitorInfo[]>([]);
    const [selectedMonitor, setSelectedMonitor] = useState<number>(0);
    const [outputPath, setOutputPath] = useState("recording.mp4");
    const [isRecording, setIsRecording] = useState(false);
    const [status, setStatus] = useState("");

    useEffect(() => {
        loadMonitors();
    }, []);

    async function loadMonitors() {
        try {
            const monitorList = await invoke<MonitorInfo[]>("get_monitors");
            setMonitors(monitorList);
            setStatus(`Found ${monitorList.length} monitor(s)`);
        } catch (error) {
            setStatus(`Error loading monitors: ${error}`);
        }
    }

    async function startRecording() {
        try {
            setStatus("Starting recording...");
            const result = await invoke<string>("start_recording", {
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
            setStatus("Stopping recording...");
            const result = await invoke<string>("stop_recording");
            setStatus(result);
            setIsRecording(false);
        } catch (error) {
            setStatus(`Error stopping recording: ${error}`);
        }
    }

    return (
        <main className="container">
            <h1>Framelet - Screen Recorder</h1>

            <div className="recording-controls">
                <div className="form-group">
                    <label htmlFor="monitor-select">Select Monitor:</label>
                    <select
                        id="monitor-select"
                        value={selectedMonitor}
                        onChange={(e) => setSelectedMonitor(parseInt(e.target.value))}
                        disabled={isRecording}
                    >
                        {monitors.map((monitor) => (
                            <option key={monitor.id} value={monitor.id}>
                                {monitor.name} ({monitor.width}x{monitor.height})
                            </option>
                        ))}
                    </select>
                </div>

                <div className="form-group">
                    <label htmlFor="output-path">Output File:</label>
                    <input
                        id="output-path"
                        type="text"
                        value={outputPath}
                        onChange={(e) => setOutputPath(e.target.value)}
                        disabled={isRecording}
                        placeholder="recording.mp4"
                    />
                </div>

                <div className="button-group">
                    {!isRecording ? (
                        <button onClick={startRecording} disabled={monitors.length === 0} className="start-button">
                            Start Recording
                        </button>
                    ) : (
                        <button onClick={stopRecording} className="stop-button">
                            Stop Recording
                        </button>
                    )}

                    <button onClick={loadMonitors} disabled={isRecording}>
                        Refresh Monitors
                    </button>
                </div>

                {status && <div className={`status ${isRecording ? "recording" : ""}`}>{status}</div>}
            </div>
        </main>
    );
}

export default App;
