import { useEffect } from "react";
import "./App.css";
import { RustInvoke } from "./rustInvoke";
import { ModelCard, SelectDirCard } from "./card";
import { Store as TauriStore } from '@tauri-apps/plugin-store';
import { TauriStoreDevice, tauriStoreDeviceKey, tauriStoreModelPathKey, TauriStoreSliders, tauriStorSlidersKey } from "./tauriStore";
import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { jotaiAtoms } from "./atoms";

function App() {
    const allModelInfo = useAtomValue(jotaiAtoms.allModelInfo);
    const selectModel = useAtomValue(jotaiAtoms.selectModel)
    const [selectSpeakerIdx, setSelectSpeakerIdx] = useAtom(jotaiAtoms.selectSpeakerIdx)

    const [pitch, setPitch] = useAtom(jotaiAtoms.pitch);
    const [formantShift, setFormantShift] = useAtom(jotaiAtoms.formantShift);
    const [inputGain, setInputGain] = useAtom(jotaiAtoms.inputGain);
    const [outputGain, setOutputGain] = useAtom(jotaiAtoms.outputGain);

    const inputDevices = useAtomValue(jotaiAtoms.inputDevices);
    const outputDevices = useAtomValue(jotaiAtoms.outputDevices);
    const monitorDevices = useAtomValue(jotaiAtoms.monitorDevices);
    const [selectInputDevice, setSelectInputDevice] = useAtom(jotaiAtoms.selectInputDevice);
    const [selectOutputDevice, setSelectOutputDevice] = useAtom(jotaiAtoms.selectOutputDevice);
    const [selectMonitorDevice, setSelectMonitorDevice] = useAtom(jotaiAtoms.selectMonitorDevice);

    return (
        <>
            <BeatriceEffects />

            <div className="container">
                <div className="cards">
                    <SelectDirCard />

                    {allModelInfo.map((i) => {
                        return <ModelCard
                            key={i.model_path}
                            modelInfo={i}
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

                    <div className="label-title">Speaker:</div>

                    <div className="speaker-controller">
                        <select
                            className="select speaker-select"
                            onChange={(event) => { setSelectSpeakerIdx(parseInt(event.target.value)) }}
                        >
                            {selectModel?.voices.map((i, idx) => { return <option key={i.name} value={idx}>{i.name}</option> })}
                        </select>

                        <div className="description">
                            <label>{selectModel?.voices[selectSpeakerIdx].description}</label>
                        </div>
                    </div>
                </div>
            </div>
        </>
    )
}

// pitchなどが更新されたときにbeatrice側にも反映する
function BeatriceEffects() {
    const [tauriStore, setTauriStore] = useAtom(jotaiAtoms.tauriStore);

    const setAllModelInfo = useSetAtom(jotaiAtoms.allModelInfo);
    const selectModel = useAtomValue(jotaiAtoms.selectModel)
    const selectSpeakerIdx = useAtomValue(jotaiAtoms.selectSpeakerIdx)

    const [pitch, setPitch] = useAtom(jotaiAtoms.pitch);
    const [formantShift, setFormantShift] = useAtom(jotaiAtoms.formantShift);
    const [inputGain, setInputGain] = useAtom(jotaiAtoms.inputGain);
    const [outputGain, setOutputGain] = useAtom(jotaiAtoms.outputGain);

    const setInputDevices = useSetAtom(jotaiAtoms.inputDevices);
    const setOutputDevices = useSetAtom(jotaiAtoms.outputDevices);
    const setMonitorDevices = useSetAtom(jotaiAtoms.monitorDevices);
    const [selectInputDevice, setSelectInputDevice] = useAtom(jotaiAtoms.selectInputDevice);
    const [selectOutputDevice, setSelectOutputDevice] = useAtom(jotaiAtoms.selectOutputDevice);
    const [selectMonitorDevice, setSelectMonitorDevice] = useAtom(jotaiAtoms.selectMonitorDevice);

    // 初期設定
    useEffect(() => {
        const promise = async () => {
            const store = await TauriStore.load('store.json', { autoSave: true });
            setTauriStore(store);

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
            const storeDevices = await store.get<TauriStoreDevice>(tauriStoreDeviceKey);
            if (storeDevices) {
                setSelectInputDevice(storeDevices.input);
                setSelectOutputDevice(storeDevices.output);
                setSelectMonitorDevice(storeDevices.monitor);
            }

            const storeModelPath = await store.get<string>(tauriStoreModelPathKey);
            if (storeModelPath) {
                setAllModelInfo(await RustInvoke.Beatrice.searchModel(storeModelPath));
            }

            const storeSlider = await store.get<TauriStoreSliders>(tauriStorSlidersKey);
            if (storeSlider) {
                setPitch(storeSlider.pitch);
                setInputGain(storeSlider.inputGain);
                setOutputGain(storeSlider.outputGain);
                setFormantShift(storeSlider.formantShift);
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
        if (tauriStore) {
            const storeValue: TauriStoreDevice = {
                input: selectInputDevice,
                output: selectOutputDevice,
                monitor: selectMonitorDevice
            };

            tauriStore.set(tauriStoreDeviceKey, storeValue);
        }

        RustInvoke.Cpal.startVoiceChanger(selectInputDevice, selectOutputDevice, selectMonitorDevice)
    }, [selectInputDevice, selectOutputDevice, selectMonitorDevice])

    // 入出力音量の変更時
    useEffect(() => {
        if (tauriStore) {
            const storeValue: TauriStoreSliders = {
                pitch: pitch,
                inputGain: inputGain,
                outputGain: outputGain,
                formantShift: formantShift
            };

            tauriStore.set(tauriStorSlidersKey, storeValue);
        }

        RustInvoke.Cpal.setInputGain(inputGain);
        RustInvoke.Cpal.setOutputGain(outputGain);
        RustInvoke.Beatrice.setPitch(pitch);
        RustInvoke.Beatrice.setFormantShift(formantShift);
    }, [pitch, inputGain, outputGain, formantShift])

    return <></>
}

export default App;
