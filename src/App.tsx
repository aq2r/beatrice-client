import { useEffect } from "react";
import "./App.css";
import { RustInvoke } from "./rustInvoke";
import { CardList } from "./components/card";
import { Store as TauriStore } from '@tauri-apps/plugin-store';
import { TauriStoreDevice, tauriStoreDeviceKey, tauriStoreModelPathKey, TauriStoreSliders, tauriStorSlidersKey } from "./tauriStore";
import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { jotaiAtoms } from "./atoms";
import { Controllers } from "./components/controllers";

function App() {
    return (
        <>
            <BeatriceEffects />

            <div className="container">
                <CardList />
                <Controllers />
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
                setPitch(storeSlider.pitch || 0.0);
                setInputGain(storeSlider.inputGain || 1.0);
                setOutputGain(storeSlider.outputGain || 1.0);
                setFormantShift(storeSlider.formantShift || 0.0);
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
