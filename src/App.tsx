import { useEffect, useState } from "react";
import "./App.css";
import { BeatriceModelInfo, RustInvoke } from "./rustInvoke";
import { ModelCard, SelectDirCard } from "./card";
import { Store, load as tauriStoreLoad } from '@tauri-apps/plugin-store';
import { TauriStoreDevice, tauriStoreDeviceKey, tauriStoreModelPathKey as tauriStoreModelPathKey, TauriStoreSliders, tauriStorSlidersKey } from "./tauriStore";

function App() {
    const [store, setStore] = useState<Store | null>(null);

    const [modelInfo, setModelInfo] = useState<BeatriceModelInfo[]>([]);
    const [selectModel, setSelectModel] = useState<BeatriceModelInfo | null>(null);
    const [selectSpeakerIdx, setSelectSpeakerIdx] = useState<number>(0);

    const [pitch, setPitch] = useState(0.0);
    const [inputGain, setInputGain] = useState(1.0);
    const [outputGain, setOutputGain] = useState(1.0);
    const [formantShift, setFormantShift] = useState(0.0);

    const [inputDevices, setInputDevices] = useState<string[]>([]);
    const [outputDevices, setOutputDevices] = useState<string[]>([]);
    const [monitorDevices, setMonitorDevices] = useState<string[]>([]);
    const [selectInputDevice, setSelectInputDevice] = useState<string | null>(null);
    const [selectOutputDevice, setSelectOutputDevice] = useState<string | null>(null);
    const [selectMonitorDevice, setSelectMonitorDevice] = useState<string | null>(null);

    // 初期設定
    useEffect(() => {
        const promise = async () => {
            const store = await tauriStoreLoad('store.json', { autoSave: true });
            setStore(store);

            // 入出力の設定
            const inputs = ["None", ...await RustInvoke.Cpal.getInputs()];
            const outputs = ["None", ...await RustInvoke.Cpal.getOutputs()];
            const monitors = [...outputs];

            setInputDevices(inputs);
            setOutputDevices(outputs);
            setMonitorDevices(monitors);

            setSelectInputDevice(null);
            setSelectOutputDevice(null);
            setSelectMonitorDevice(null);

            // デバイス設定
            if (store) {
                const storeDevices = await store.get<TauriStoreDevice>(tauriStoreDeviceKey);
                if (storeDevices) {
                    setSelectInputDevice(storeDevices.input);
                    setSelectOutputDevice(storeDevices.output);
                    setSelectMonitorDevice(storeDevices.monitor);
                }

                const storeModelPath = await store.get<string>(tauriStoreModelPathKey);
                if (storeModelPath) {
                    setModelInfo(await RustInvoke.Beatrice.searchModel(storeModelPath));
                }

                const storeSlider = await store.get<TauriStoreSliders>(tauriStorSlidersKey);
                if (storeSlider) {
                    setPitch(storeSlider.pitch);
                    setInputGain(storeSlider.inputGain);
                    setOutputGain(storeSlider.outputGain);
                    setFormantShift(storeSlider.formantShift);
                }
            }
        };
        promise()
    }, [])

    // モデルと話者変更時
    useEffect(() => {
        if (selectModel === null) { return; }

        RustInvoke.Beatrice.loadModel(selectModel.model_path);
        RustInvoke.Beatrice.setAverageSourcePitch(selectModel.voices[0].average_pitch);
        RustInvoke.Beatrice.setSpeaker(selectSpeakerIdx);
    }, [selectModel, selectSpeakerIdx]);

    // 入出力の変更時
    useEffect(() => {
        if (store) {
            const storeValue: TauriStoreDevice = {
                input: selectInputDevice,
                output: selectOutputDevice,
                monitor: selectMonitorDevice
            };

            store.set(tauriStoreDeviceKey, storeValue);
        }

        RustInvoke.Cpal.startVoiceChanger(selectInputDevice, selectOutputDevice, selectMonitorDevice)
    }, [selectInputDevice, selectOutputDevice, selectMonitorDevice])

    // 入出力音量の変更時
    useEffect(() => {
        if (store) {
            const storeValue: TauriStoreSliders = {
                pitch: pitch,
                inputGain: inputGain,
                outputGain: outputGain,
                formantShift: formantShift
            };

            store.set(tauriStorSlidersKey, storeValue);
        }

        RustInvoke.Cpal.setInputGain(inputGain);
        RustInvoke.Cpal.setOutputGain(outputGain);
        RustInvoke.Beatrice.setPitch(pitch);
        RustInvoke.Beatrice.setFormantShift(formantShift);
    }, [pitch, inputGain, outputGain, formantShift])

    return (
        <div className="container">
            <div className="cards">
                <SelectDirCard setModelInfo={setModelInfo} />

                {modelInfo.map((i) => {
                    return <ModelCard
                        key={i.model_path}
                        modelInfo={i}
                        selectModel={selectModel}
                        setSelectModel={setSelectModel}
                        setSelectSpeakerIdx={setSelectSpeakerIdx}
                    />;
                })}
            </div>

            <div className="controllers">
                <div className="pitch-control">
                    <div className="label-title">Pitch: {pitch}</div>
                    <input
                        className="slider"
                        type="range"
                        min={-24.0}
                        max={24.0}
                        step={0.125}
                        value={pitch}
                        onChange={(e) => setPitch(parseFloat(e.target.value))}
                    />
                </div>

                <div className="formant-control">
                    <div className="label-title">Formant: {formantShift}</div>
                    <input
                        className="slider"
                        type="range"
                        min={-2.0}
                        max={2.0}
                        step={0.5}
                        value={formantShift}
                        onChange={(e) => setFormantShift(parseFloat(e.target.value))}
                    />
                </div>

                <div className="input-gain-control">
                    <div className="label-title">InputGain: {inputGain}</div>
                    <input
                        className="slider"
                        type="range"
                        min={0.1}
                        max={10.0}
                        step={0.1}
                        value={inputGain}
                        onChange={(e) => setInputGain(parseFloat(e.target.value))}
                    />
                </div>

                <div className="output-gain-control">
                    <div className="label-title">OutputGain: {outputGain}</div>
                    <input
                        className="slider"
                        type="range"
                        min={0.1}
                        max={10.0}
                        step={0.1}
                        value={outputGain}
                        onChange={(e) => setOutputGain(parseFloat(e.target.value))}
                    />
                </div>

                <div className="output-controllers">
                    <div className="input-control">
                        <div className="label-title">Input:</div>
                        <select
                            className="select"
                            value={selectInputDevice || "None"}
                            onChange={(event) => { setSelectInputDevice(event.target.value) }}
                        >
                            {inputDevices.map((i) => { return <option key={i} value={i}>{i}</option> })}
                        </select>
                    </div>

                    <div className="output-control">
                        <div className="label-title">Output:</div>
                        <select
                            className="select"
                            value={selectOutputDevice || "None"}
                            onChange={(event) => { setSelectOutputDevice(event.target.value) }}
                        >
                            {outputDevices.map((i) => { return <option key={i} value={i}>{i}</option> })}
                        </select>
                    </div>

                    <div className="monitor-control">
                        <div className="label-title">Monitor:</div>
                        <select
                            className="select"
                            value={selectMonitorDevice || "None"}
                            onChange={(event) => { setSelectMonitorDevice(event.target.value) }}
                        >
                            {monitorDevices.map((i) => { return <option key={i} value={i}>{i}</option> })}
                        </select>
                    </div>
                </div>


                <div className="speaker-controller">
                    <div className="label-title">Speaker:</div>
                    <select
                        className="select speaker-select"
                        onChange={(event) => { setSelectSpeakerIdx(parseInt(event.target.value)) }}
                    >
                        {selectModel?.voices.map((i, idx) => { return <option key={i.name} value={idx}>{i.name}</option> })}
                    </select>
                </div>
            </div>
        </div>
    )
}

export default App;
