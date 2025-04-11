import { useAtom, useAtomValue } from "jotai"
import { jotaiAtoms } from "../../atoms"
import { useEffect, useState } from "react"
import { RustInvoke } from "../../rustInvoke"

export const SpeakerSelector = () => {
    const selectModel = useAtomValue(jotaiAtoms.selectModel)
    const [selectSpeakerIdx, setSelectSpeakerIdx] = useAtom(jotaiAtoms.selectSpeakerIdx)

    const [imageSrc, setImageSrc] = useState("");

    useEffect(() => {
        const loadImage = async () => {
            const base64Image = await RustInvoke.otherReadImage(
                `${selectModel?.model_path}/${selectModel?.voices[selectSpeakerIdx].portrait_path}`
            );
            setImageSrc(base64Image);
        }
        loadImage();
    }, [selectModel, selectSpeakerIdx]);

    return (
        <>
            <div className="label-title">Speaker:</div>

            <div className="speaker-controller">
                <select
                    className="select speaker-select"
                    onChange={(event) => { setSelectSpeakerIdx(parseInt(event.target.value)) }}
                >
                    {selectModel?.voices.map((i, idx) => { return <option key={i.name} value={idx}>{i.name}</option> })}
                </select>

                <div className="speaker-portrait">
                    <img src={imageSrc}></img>
                </div>

                <div className="speaker-portrait-description">
                    画像の説明:{`\n${selectModel?.voices[selectSpeakerIdx].portrait_description}`}

                </div>

                <div className="description">
                    <label>{selectModel?.voices[selectSpeakerIdx].description}</label>
                </div>


            </div>
        </>)
}