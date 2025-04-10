import { useAtom, useAtomValue } from "jotai"
import { jotaiAtoms } from "../../atoms"

export const SpeakerSelector = () => {
    const selectModel = useAtomValue(jotaiAtoms.selectModel)
    const [selectSpeakerIdx, setSelectSpeakerIdx] = useAtom(jotaiAtoms.selectSpeakerIdx)

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

                <div className="description">
                    <label>{selectModel?.voices[selectSpeakerIdx].description}</label>
                </div>
            </div>
        </>)
}