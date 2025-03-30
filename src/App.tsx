import { useEffect, useState } from "react";
import "./App.css";
import { RustInvoke } from "./rustInvoke";

function App() {
    const [modelPath, setModelPath] = useState("");
    const [pitch, setPitch] = useState(0.0);
    const [inputDevices, setInputDevices] = useState<string[]>([]);
    const [outputDevices, setOutputDevices] = useState<string[]>([]);

    const [selectInputDevice, setSelectInputDevice] = useState<string | null>(null);
    const [selectOutputDevice, setSelectOutputDevice] = useState<string | null>(null);

    useEffect(() => {
        const promise = async () => {
            const inputs = await RustInvoke.cpalGetInputs();
            const outputs = await RustInvoke.cpalGetOutputs();

            setInputDevices(inputs);
            setOutputDevices(outputs);
            setSelectInputDevice(inputs[0]);
            setSelectOutputDevice(outputs[0]);
        };
        promise()
    }, [])

    useEffect(() => {
        const promise = async () => {
            await RustInvoke.cpalStartVoiceChanger(selectInputDevice, selectOutputDevice)
        };
        promise()
    }, [selectInputDevice, selectOutputDevice])

    useEffect(() => {
        const promise = async () => {
            await RustInvoke.beatriceSetPitch(pitch);
        };
        promise()
    }, [pitch])


    return (
        <div className="main">
            <div className="input-model-path">
                <label>ModelPath: </label>
                <input
                    type="text"
                    value={modelPath}
                    onChange={(e) => setModelPath(e.target.value)}
                />
                <button className="model-load-button"
                    onClick={() => { RustInvoke.beatriceLoadModel(modelPath) }}
                >Load</button>
            </div>

            <div className="pitch-control">
                <label>Pitch: </label>
                <input
                    type="range"
                    min={-24.0}
                    max={24.0}
                    step={0.1}
                    value={pitch}
                    onChange={(e) => setPitch(parseFloat(e.target.value))}
                />
                <span>{pitch}</span>
            </div>

            <div className="input-control">
                <label>InputDevice: </label>
                <select onChange={(event) => { setSelectInputDevice(event.target.value) }}>
                    {inputDevices.map((v) => { return <option value={v}>{v}</option> })}
                </select>
            </div>

            <div className="output-control">
                <label>OutputDevice: </label>
                <select onChange={(event) => { setSelectOutputDevice(event.target.value) }}>
                    {outputDevices.map((v) => { return <option key={v} value={v}>{v}</option> })}
                </select>
            </div>
        </div>
    );
}

export default App;
