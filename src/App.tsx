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

    const setLoadedModelVersion = useSetAtom(jotaiAtoms.loadedModelVersion);

    const [pitch, setPitch] = useAtom(jotaiAtoms.pitch);
    const [formantShift, setFormantShift] = useAtom(jotaiAtoms.formantShift);
    const [inputGain, setInputGain] = useAtom(jotaiAtoms.inputGain);
    const [outputGain, setOutputGain] = useAtom(jotaiAtoms.outputGain);
    
    const [isDisplayAdvancedSettings, setIsDisplayAdvancedSettings] = useAtom(jotaiAtoms.isDisplayAdvancedSettings);
    const [averageSourcePitch, setAverageSourcePitch] = useAtom(jotaiAtoms.averageSourcePitch);
    const [intonationIntensity, setIntonationIntensity] = useAtom(jotaiAtoms.intonationIntensity);
    const [pitchCorrection, setPitchCorrection] = useAtom(jotaiAtoms.pitchCorrection);
    const [pitchCorrectionType, setPitchCorrectionType] = useAtom(jotaiAtoms.pitchCorrectionType);
    const [minSourcePitch, setMinSourcePitch] = useAtom(jotaiAtoms.minSourcePitch);
    const [maxSourcePitch, setMaxSourcePitch] = useAtom(jotaiAtoms.maxSourcePitch);
    const [vqNumNeighbors, setVqNumNeighbors] = useAtom(jotaiAtoms.vqNumNeighbors);

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

                setIsDisplayAdvancedSettings(storeSlider.isDisplayAdvancedSettings || false);
                setAverageSourcePitch(storeSlider.averageSourcePitch || 52.0);
                setIntonationIntensity(storeSlider.intonationIntensity || 1.0);
                setPitchCorrection(storeSlider.pitchCorrection || 0.0);
                setPitchCorrectionType(storeSlider.pitchCorrectionType || 0);
                setMinSourcePitch(storeSlider.minSourcePitch || 33.125);
                setMaxSourcePitch(storeSlider.maxSourcePitch || 80.875);
                setVqNumNeighbors(storeSlider.vqNumNeighbors || 0);
            }
        };
        promise()
    }, [])

    // モデルと話者変更時
    useEffect(() => {
        const promise = async () => {
            if (selectModel === null) { return; }

            await RustInvoke.Beatrice.loadModel(selectModel.model_path);
            setLoadedModelVersion(await RustInvoke.Beatrice.getModelVersion());
            await RustInvoke.Beatrice.setAverageSourcePitch(selectModel.voices[selectSpeakerIdx].average_pitch);
            setAverageSourcePitch(selectModel.voices[selectSpeakerIdx].average_pitch);
            await RustInvoke.Beatrice.setSpeaker(selectSpeakerIdx);


            // 最初だけピッチやフォルマントがおかしくなるのを臨時で修正 (あまりよくないけど)
            if (tauriStore) {
                const storeSlider = await tauriStore.get<TauriStoreSliders>(tauriStorSlidersKey);
                if (storeSlider) {
                    RustInvoke.Cpal.setInputGain(storeSlider.inputGain || 0.0);
                    RustInvoke.Cpal.setOutputGain(storeSlider.outputGain || 1.0);
                    RustInvoke.Beatrice.setPitch(storeSlider.pitch || 1.0);
                    RustInvoke.Beatrice.setFormantShift(storeSlider.formantShift || 0.0);

                    RustInvoke.Beatrice.setAverageSourcePitch(storeSlider.averageSourcePitch || 52.0);
                    RustInvoke.Beatrice.setIntonationIntensity(storeSlider.intonationIntensity || 1.0);
                    RustInvoke.Beatrice.setPitchCorrection(storeSlider.pitchCorrection || 0.0);
                    RustInvoke.Beatrice.setPitchCorrectionType(storeSlider.pitchCorrectionType || 0);
                    RustInvoke.Beatrice.setMinSourcePitch(storeSlider.minSourcePitch || 33.125);
                    RustInvoke.Beatrice.setMaxSourcePitch(storeSlider.maxSourcePitch || 80.875);
                    RustInvoke.Beatrice.setVqNumNeighbors(storeSlider.vqNumNeighbors || 0);
                }
            }
        }
        promise()
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

    // 入出力音量などの変更時
    useEffect(() => {
        if (tauriStore) {
            const storeValue: TauriStoreSliders = {
                pitch: pitch,
                inputGain: inputGain,
                outputGain: outputGain,
                formantShift: formantShift,

                isDisplayAdvancedSettings: isDisplayAdvancedSettings,
                averageSourcePitch: averageSourcePitch,
                intonationIntensity: intonationIntensity,
                pitchCorrection: pitchCorrection,
                pitchCorrectionType: pitchCorrectionType,
                minSourcePitch: minSourcePitch,
                maxSourcePitch: maxSourcePitch,
                vqNumNeighbors: vqNumNeighbors
            };

            tauriStore.set(tauriStorSlidersKey, storeValue);
        }

        RustInvoke.Cpal.setInputGain(inputGain);
        RustInvoke.Cpal.setOutputGain(outputGain);
        RustInvoke.Beatrice.setPitch(pitch);
        RustInvoke.Beatrice.setFormantShift(formantShift);

        RustInvoke.Beatrice.setAverageSourcePitch(averageSourcePitch);
        RustInvoke.Beatrice.setIntonationIntensity(intonationIntensity);
        RustInvoke.Beatrice.setPitchCorrection(pitchCorrection);
        RustInvoke.Beatrice.setPitchCorrectionType(pitchCorrectionType);
        RustInvoke.Beatrice.setMinSourcePitch(minSourcePitch);
        RustInvoke.Beatrice.setMaxSourcePitch(maxSourcePitch);
        RustInvoke.Beatrice.setVqNumNeighbors(vqNumNeighbors);
    }, [pitch, inputGain, outputGain, formantShift, 
        isDisplayAdvancedSettings, averageSourcePitch, intonationIntensity,
        pitchCorrection, pitchCorrectionType, minSourcePitch, maxSourcePitch, vqNumNeighbors])

    return <></>
}

export default App;
